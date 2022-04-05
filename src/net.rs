//! Network of AudioUnits connected together.

use super::audionode::*;
use super::audiounit::*;
use super::buffer::*;
use super::signal::*;
use super::*;
use duplicate::duplicate_item;

pub type NodeIndex = usize;
pub type PortIndex = usize;

/// Input or output port.
#[derive(Clone, Copy)]
pub enum Port {
    /// Node input or output.
    Local(NodeIndex, PortIndex),
    /// Global input or output.
    Global(PortIndex),
    /// Unconnected input. Unconnected output ports are not marked anywhere.
    Zero,
}

#[derive(Clone, Copy)]
pub struct Edge {
    pub source: Port,
    pub target: Port,
}

/// Create an edge from source to target.
pub fn edge(source: Port, target: Port) -> Edge {
    Edge { source, target }
}

#[duplicate_item(
    f48       Vertex48       AudioUnit48;
    [ f64 ]   [ Vertex64 ]   [ AudioUnit64 ];
    [ f32 ]   [ Vertex32 ]   [ AudioUnit32 ];
)]
/// Individual AudioUnits are vertices in the graph.
pub struct Vertex48 {
    /// The unit.
    pub unit: Box<dyn AudioUnit48>,
    /// Edges connecting into this vertex. The length indicates the number of inputs.
    pub source: Vec<Edge>,
    /// Input buffers. The length indicates the number of inputs.
    pub input: Buffer<f48>,
    /// Output buffers. The length indicates the number of outputs.
    pub output: Buffer<f48>,
    /// Input for tick iteration. The length indicates the number of inputs.
    pub tick_input: Vec<f48>,
    /// Output for tick iteration. The length indicates the number of outputs.
    pub tick_output: Vec<f48>,
    /// Index or ID of this unit. This equals unit index in graph.
    pub id: NodeIndex,
}

#[duplicate_item(
    f48       Vertex48       AudioUnit48;
    [ f64 ]   [ Vertex64 ]   [ AudioUnit64 ];
    [ f32 ]   [ Vertex32 ]   [ AudioUnit32 ];
)]
impl Vertex48 {
    pub fn new(id: NodeIndex, inputs: usize, outputs: usize) -> Self {
        Self {
            unit: Box::new(super::prelude::pass()),
            source: vec![],
            input: Buffer::with_size(inputs),
            output: Buffer::with_size(outputs),
            tick_input: vec![0.0; inputs],
            tick_output: vec![0.0; outputs],
            id,
        }
    }

    pub fn inputs(&self) -> usize {
        self.input.buffers()
    }

    pub fn outputs(&self) -> usize {
        self.output.buffers()
    }
}

#[duplicate_item(
    f48       Net48       Vertex48       AudioUnit48;
    [ f64 ]   [ Net64 ]   [ Vertex64 ]   [ AudioUnit64 ];
    [ f32 ]   [ Net32 ]   [ Vertex32 ]   [ AudioUnit32 ];
)]
/// Network unit. It can contain other units and maintain connections between them.
/// Outputs of the network are sourced from user specified unit outputs or global inputs.
pub struct Net48 {
    /// Global input buffers.
    input: Buffer<f48>,
    /// Global output buffers.
    output: Buffer<f48>,
    /// Sources of global outputs.
    output_edge: Vec<Edge>,
    /// Vertices of the graph.
    vertex: Vec<Vertex48>,
    /// Ordering of vertex evaluation.
    order: Vec<NodeIndex>,
    ordered: bool,
}

#[duplicate_item(
    f48       Net48       Vertex48       AudioUnit48;
    [ f64 ]   [ Net64 ]   [ Vertex64 ]   [ AudioUnit64 ];
    [ f32 ]   [ Net32 ]   [ Vertex32 ]   [ AudioUnit32 ];
)]
impl Net48 {
    /// Create new network with the given number of inputs and outputs.
    /// The number of inputs and outputs is fixed after construction.
    pub fn new(inputs: usize, outputs: usize) -> Self {
        let mut net = Self {
            input: Buffer::with_size(inputs),
            output: Buffer::with_size(outputs),
            output_edge: vec![],
            vertex: vec![],
            order: vec![],
            ordered: true,
        };
        for channel in 0..outputs {
            net.output_edge
                .push(edge(Port::Zero, Port::Global(channel)));
        }
        net
    }

    fn determine_order(&mut self) {
        self.ordered = true;
        let mut order = Vec::new();
        self.determine_order_in(&mut order);
        self.order.clear();
        std::mem::swap(&mut order, &mut self.order);
    }

    fn determine_order_in(&self, order: &mut Vec<NodeIndex>) {
        let mut vertices_left = self.vertex.len();
        let mut vertex_left = vec![true; self.vertex.len()];
        // Note about contents of the edge vector.
        // Each node input appears there exactly once.
        // Sources, however, are not unique or guaranteed to appear.
        let mut all_edges: Vec<Edge> = Vec::new();
        for vertex in self.vertex.iter() {
            for edge in &vertex.source {
                all_edges.push(*edge);
            }
        }

        let mut inputs_left = vec![0; self.vertex.len()];
        for i in 0..inputs_left.len() {
            inputs_left[i] = self.vertex[i].unit.inputs();
            if inputs_left[i] == 0 {
                vertex_left[i] = false;
                order.push(i);
                vertices_left -= 1;
            }
        }

        // Start from network inputs.
        for (_, edge) in all_edges.iter().enumerate() {
            if let (Port::Global(_) | Port::Zero, Port::Local(vertex, _)) =
                (edge.source, edge.target)
            {
                if vertex_left[vertex] {
                    inputs_left[vertex] -= 1;
                    if inputs_left[vertex] == 0 {
                        vertex_left[vertex] = false;
                        order.push(vertex);
                        vertices_left -= 1;
                    }
                }
            }
        }
        while vertices_left > 0 {
            let mut progress = false;
            for (_i, edge) in all_edges.iter().enumerate() {
                if let (Port::Local(source, _), Port::Local(target, _)) = (edge.source, edge.target)
                {
                    if !vertex_left[source] && vertex_left[target] {
                        progress = true;
                        inputs_left[target] -= 1;
                        if inputs_left[target] == 0 {
                            vertex_left[target] = false;
                            order.push(target);
                            vertices_left -= 1;
                        }
                    }
                }
            }
            if !progress {
                panic!("Cycle detected.");
            }
        }
    }

    /// Add a new unit to the network. Return its ID handle.
    /// ID handles are always consecutive numbers starting from zero.
    pub fn add(&mut self, unit: Box<dyn AudioUnit48>) -> NodeIndex {
        let id = self.vertex.len();
        let inputs = unit.inputs();
        let outputs = unit.outputs();
        let mut vertex = Vertex48 {
            unit,
            source: vec![],
            input: Buffer::with_size(inputs),
            output: Buffer::with_size(outputs),
            tick_input: vec![0.0; inputs],
            tick_output: vec![0.0; outputs],
            id,
        };
        for i in 0..vertex.inputs() {
            vertex
                .source
                .push(edge(Port::Zero, Port::Local(id as usize, i)));
        }
        self.vertex.push(vertex);
        self.ordered = false;
        id
    }

    /// Connect the given output (`source`, `source_port`)
    /// to the given input (`target`, `target_port`).
    pub fn connect(
        &mut self,
        source: NodeIndex,
        source_port: PortIndex,
        target: NodeIndex,
        target_port: PortIndex,
    ) {
        self.vertex[target].source[target_port] = edge(
            Port::Local(source, source_port),
            Port::Local(target, target_port),
        );
        self.ordered = false;
    }

    /// Connect the node input (`target`, `target_port`) to the global input `global_input`.
    pub fn connect_input(
        &mut self,
        global_input: PortIndex,
        target: NodeIndex,
        target_port: PortIndex,
    ) {
        self.vertex[target].source[target_port] =
            edge(Port::Global(global_input), Port::Local(target, target_port));
        self.ordered = false;
    }

    /// Pipe global input to node `target`.
    pub fn pipe_input(&mut self, target: NodeIndex) {
        assert!(self.vertex[target].inputs() == self.inputs());
        for i in 0..self.inputs() {
            self.vertex[target].source[i] = edge(Port::Global(i), Port::Local(target, i));
        }
        self.ordered = false;
    }

    /// Connect node input (`source`, `source_port`) to global output `global_output`.
    pub fn connect_output(
        &mut self,
        source: NodeIndex,
        source_port: PortIndex,
        global_output: PortIndex,
    ) {
        self.output_edge[global_output] = edge(
            Port::Local(source, source_port),
            Port::Global(global_output),
        );
        self.ordered = false;
    }

    /// Pipe node outputs to global outputs.
    /// The number of outputs and number of global outputs must match.
    pub fn pipe_output(&mut self, source: NodeIndex) {
        assert!(self.vertex[source].outputs() == self.outputs());
        for i in 0..self.outputs() {
            self.output_edge[i] = edge(Port::Local(source, i), Port::Global(i));
        }
        self.ordered = false;
    }

    /// Add an arbitrary edge to the network.
    pub fn join(&mut self, edge: Edge) {
        match edge.target {
            Port::Global(global_output) => self.output_edge[global_output] = edge,
            Port::Local(target, target_port) => self.vertex[target].source[target_port] = edge,
            _ => (),
        }
        self.ordered = false;
    }

    /// Connect `source` to `target`.
    /// The number of outputs in `source` and number of inputs in `target` must match.
    pub fn pipe(&mut self, source: NodeIndex, target: NodeIndex) {
        assert!(self.vertex[source].outputs() == self.vertex[target].inputs());
        for i in 0..self.vertex[target].inputs() {
            self.vertex[target].source[i] = edge(Port::Local(source, i), Port::Local(target, i));
        }
        self.ordered = false;
    }

    /// Assuming this network is a chain of processing units ordered by increasing node ID,
    /// add a new unit to the chain. The global outputs will be assigned to the outputs of the unit.
    pub fn chain(&mut self, unit: Box<dyn AudioUnit48>) {
        assert!(unit.inputs() == unit.outputs() && self.outputs() == unit.outputs());
        let id = self.add(unit);
        self.pipe_output(id);
        if id > 0 {
            self.pipe(id - 1, id);
        } else {
            self.pipe_input(id);
        }
    }
}

#[duplicate_item(
    f48       Net48       Vertex48       AudioUnit48;
    [ f64 ]   [ Net64 ]   [ Vertex64 ]   [ AudioUnit64 ];
    [ f32 ]   [ Net32 ]   [ Vertex32 ]   [ AudioUnit32 ];
)]
impl AudioUnit48 for Net48 {
    fn inputs(&self) -> usize {
        self.input.buffers()
    }

    fn outputs(&self) -> usize {
        self.output.buffers()
    }

    fn reset(&mut self, sample_rate: Option<f64>) {
        for vertex in &mut self.vertex {
            vertex.unit.reset(sample_rate);
        }
    }

    fn tick(&mut self, input: &[f48], output: &mut [f48]) {
        if !self.ordered {
            self.determine_order();
        }
        // Iterate units in network order.
        for node_index in self.order.iter() {
            let mut vertex = Vertex48::new(*node_index, 0, 0);

            std::mem::swap(&mut vertex, &mut self.vertex[*node_index]);
            for channel in 0..vertex.inputs() {
                match vertex.source[channel].source {
                    Port::Zero => vertex.tick_input[channel] = 0.0,
                    Port::Global(port) => vertex.tick_input[channel] = input[port],
                    Port::Local(source, port) => {
                        vertex.tick_input[channel] = self.vertex[source].tick_output[port]
                    }
                }
            }
            vertex
                .unit
                .tick(&vertex.tick_input, &mut vertex.tick_output);
            std::mem::swap(&mut vertex, &mut self.vertex[*node_index]);
        }

        // Then we set the global outputs.
        for channel in 0..output.len() {
            match self.output_edge[channel].source {
                Port::Global(port) => output[channel] = input[port],
                Port::Local(node, port) => output[channel] = self.vertex[node].tick_output[port],
                Port::Zero => output[channel] = 0.0,
            }
        }
    }

    fn process(&mut self, size: usize, input: &[&[f48]], output: &mut [&mut [f48]]) {
        if !self.ordered {
            self.determine_order();
        }
        // Iterate units in network order.
        for node_index in self.order.iter() {
            let mut vertex = Vertex48::new(*node_index, 0, 0);

            std::mem::swap(&mut vertex, &mut self.vertex[*node_index]);
            for channel in 0..vertex.inputs() {
                match vertex.source[channel].source {
                    Port::Zero => vertex.input.mut_at(channel)[..size].fill(0.0),
                    Port::Global(port) => {
                        vertex.input.mut_at(channel)[..size].copy_from_slice(&input[port][..size])
                    }
                    Port::Local(source, port) => {
                        vertex.input.mut_at(channel)[..size]
                            .copy_from_slice(&self.vertex[source].output.at(port)[..size]);
                    }
                }
            }
            vertex
                .unit
                .process(size, vertex.input.self_ref(), vertex.output.self_mut());
            std::mem::swap(&mut vertex, &mut self.vertex[*node_index]);
        }

        // Then we set the global outputs.
        for channel in 0..output.len() {
            match self.output_edge[channel].source {
                Port::Global(port) => output[channel][..size].copy_from_slice(&input[port][..size]),
                Port::Local(node, port) => output[channel][..size]
                    .copy_from_slice(&self.vertex[node].output.at(port)[..size]),
                Port::Zero => output[channel][..size].fill(0.0),
            }
        }
    }

    fn route(&self, input: &SignalFrame, frequency: f64) -> SignalFrame {
        let mut order = vec![];
        self.determine_order_in(&mut order);
        let mut inner_signal: Vec<SignalFrame> = vec![];
        for vertex in self.vertex.iter() {
            inner_signal.push(new_signal_frame(vertex.unit.outputs()));
        }
        for unit_index in order {
            let mut input_signal = new_signal_frame(self.vertex[unit_index].unit.inputs());
            for channel in 0..self.vertex[unit_index].unit.inputs() {
                match self.vertex[unit_index].source[channel].source {
                    Port::Local(j, port) => input_signal[channel] = inner_signal[j][port],
                    Port::Global(j) => input_signal[channel] = input[j],
                    Port::Zero => input_signal[channel] = Signal::Value(0.0),
                }
            }
            inner_signal[unit_index] = self.vertex[unit_index].unit.route(&input_signal, frequency);
        }

        // Then we set the global outputs.
        let mut output_signal = new_signal_frame(self.outputs());
        for channel in 0..self.outputs() {
            match self.output_edge[channel].source {
                Port::Global(port) => output_signal[channel] = input[port],
                Port::Local(node, port) => {
                    output_signal[channel] = inner_signal[node][port];
                }
                Port::Zero => output_signal[channel] = Signal::Value(0.0),
            }
        }
        output_signal
    }

    fn set(&mut self, parameter: audionode::Tag, value: f64) {
        for vertex in &mut self.vertex {
            vertex.unit.set(parameter, value);
        }
    }

    fn get(&self, parameter: Tag) -> Option<f64> {
        for vertex in &self.vertex {
            if let Some(value) = vertex.unit.get(parameter) {
                return Some(value);
            }
        }
        None
    }
}