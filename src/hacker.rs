//! The hacker prelude, a fully 64-bit environment for audio processing.

pub use super::audionode::*;
pub use super::audiounit::*;
pub use super::buffer::*;
pub use super::combinator::*;
pub use super::delay::*;
pub use super::dynamics::*;
pub use super::envelope::*;
pub use super::feedback::*;
pub use super::filter::*;
pub use super::fir::*;
pub use super::follow::*;
pub use super::gen::*;
pub use super::granular::*;
pub use super::math::*;
pub use super::moog::*;
pub use super::net::*;
pub use super::noise::*;
pub use super::oscillator::*;
pub use super::oversample::*;
pub use super::pan::*;
pub use super::realnet::*;
pub use super::realseq::*;
pub use super::resample::*;
pub use super::rez::*;
pub use super::sequencer::*;
pub use super::setting::*;
pub use super::shape::*;
pub use super::shared::*;
pub use super::signal::*;
pub use super::slot::*;
pub use super::snoop::*;
pub use super::svf::*;
pub use super::system::*;
pub use super::wave::*;
pub use super::wave_stream::*;
pub use super::wavetable::*;
pub use super::*;

#[cfg(feature = "files")]
pub use super::read::*;
#[cfg(feature = "files")]
pub use super::read_stream::*;

use std::sync::Arc;

// Combinator environment.
// We like to define all kinds of useful functions here.

// Import some typenum integers for reporting arities.
pub type U0 = numeric_array::typenum::U0;
pub type U1 = numeric_array::typenum::U1;
pub type U2 = numeric_array::typenum::U2;
pub type U3 = numeric_array::typenum::U3;
pub type U4 = numeric_array::typenum::U4;
pub type U5 = numeric_array::typenum::U5;
pub type U6 = numeric_array::typenum::U6;
pub type U7 = numeric_array::typenum::U7;
pub type U8 = numeric_array::typenum::U8;
pub type U9 = numeric_array::typenum::U9;
pub type U10 = numeric_array::typenum::U10;
pub type U11 = numeric_array::typenum::U11;
pub type U12 = numeric_array::typenum::U12;
pub type U13 = numeric_array::typenum::U13;
pub type U14 = numeric_array::typenum::U14;
pub type U15 = numeric_array::typenum::U15;
pub type U16 = numeric_array::typenum::U16;
pub type U17 = numeric_array::typenum::U17;
pub type U18 = numeric_array::typenum::U18;
pub type U19 = numeric_array::typenum::U19;
pub type U20 = numeric_array::typenum::U20;
pub type U21 = numeric_array::typenum::U21;
pub type U22 = numeric_array::typenum::U22;
pub type U23 = numeric_array::typenum::U23;
pub type U24 = numeric_array::typenum::U24;
pub type U25 = numeric_array::typenum::U25;
pub type U26 = numeric_array::typenum::U26;
pub type U27 = numeric_array::typenum::U27;
pub type U28 = numeric_array::typenum::U28;
pub type U29 = numeric_array::typenum::U29;
pub type U30 = numeric_array::typenum::U30;
pub type U31 = numeric_array::typenum::U31;
pub type U32 = numeric_array::typenum::U32;
pub type U33 = numeric_array::typenum::U33;
pub type U34 = numeric_array::typenum::U34;
pub type U35 = numeric_array::typenum::U35;
pub type U36 = numeric_array::typenum::U36;
pub type U37 = numeric_array::typenum::U37;
pub type U38 = numeric_array::typenum::U38;
pub type U39 = numeric_array::typenum::U39;
pub type U40 = numeric_array::typenum::U40;
pub type U41 = numeric_array::typenum::U41;
pub type U42 = numeric_array::typenum::U42;
pub type U43 = numeric_array::typenum::U43;
pub type U44 = numeric_array::typenum::U44;
pub type U45 = numeric_array::typenum::U45;
pub type U46 = numeric_array::typenum::U46;
pub type U47 = numeric_array::typenum::U47;
pub type U48 = numeric_array::typenum::U48;
pub type U49 = numeric_array::typenum::U49;
pub type U50 = numeric_array::typenum::U50;
pub type U51 = numeric_array::typenum::U51;
pub type U52 = numeric_array::typenum::U52;
pub type U53 = numeric_array::typenum::U53;
pub type U54 = numeric_array::typenum::U54;
pub type U55 = numeric_array::typenum::U55;
pub type U56 = numeric_array::typenum::U56;
pub type U57 = numeric_array::typenum::U57;
pub type U58 = numeric_array::typenum::U58;
pub type U59 = numeric_array::typenum::U59;
pub type U60 = numeric_array::typenum::U60;
pub type U61 = numeric_array::typenum::U61;
pub type U62 = numeric_array::typenum::U62;
pub type U63 = numeric_array::typenum::U63;
pub type U64 = numeric_array::typenum::U64;
pub type U65 = numeric_array::typenum::U65;
pub type U66 = numeric_array::typenum::U66;
pub type U67 = numeric_array::typenum::U67;
pub type U68 = numeric_array::typenum::U68;
pub type U69 = numeric_array::typenum::U69;
pub type U70 = numeric_array::typenum::U70;
pub type U71 = numeric_array::typenum::U71;
pub type U72 = numeric_array::typenum::U72;
pub type U73 = numeric_array::typenum::U73;
pub type U74 = numeric_array::typenum::U74;
pub type U75 = numeric_array::typenum::U75;
pub type U76 = numeric_array::typenum::U76;
pub type U77 = numeric_array::typenum::U77;
pub type U78 = numeric_array::typenum::U78;
pub type U79 = numeric_array::typenum::U79;
pub type U80 = numeric_array::typenum::U80;
pub type U81 = numeric_array::typenum::U81;
pub type U82 = numeric_array::typenum::U82;
pub type U83 = numeric_array::typenum::U83;
pub type U84 = numeric_array::typenum::U84;
pub type U85 = numeric_array::typenum::U85;
pub type U86 = numeric_array::typenum::U86;
pub type U87 = numeric_array::typenum::U87;
pub type U88 = numeric_array::typenum::U88;
pub type U89 = numeric_array::typenum::U89;
pub type U90 = numeric_array::typenum::U80;
pub type U91 = numeric_array::typenum::U91;
pub type U92 = numeric_array::typenum::U92;
pub type U93 = numeric_array::typenum::U93;
pub type U94 = numeric_array::typenum::U94;
pub type U95 = numeric_array::typenum::U95;
pub type U96 = numeric_array::typenum::U96;
pub type U97 = numeric_array::typenum::U97;
pub type U98 = numeric_array::typenum::U98;
pub type U99 = numeric_array::typenum::U99;
pub type U100 = numeric_array::typenum::U100;
pub type U101 = numeric_array::typenum::U101;
pub type U102 = numeric_array::typenum::U102;
pub type U103 = numeric_array::typenum::U103;
pub type U104 = numeric_array::typenum::U104;
pub type U105 = numeric_array::typenum::U105;
pub type U106 = numeric_array::typenum::U106;
pub type U107 = numeric_array::typenum::U107;
pub type U108 = numeric_array::typenum::U108;
pub type U109 = numeric_array::typenum::U109;
pub type U110 = numeric_array::typenum::U110;
pub type U111 = numeric_array::typenum::U111;
pub type U112 = numeric_array::typenum::U112;
pub type U113 = numeric_array::typenum::U113;
pub type U114 = numeric_array::typenum::U114;
pub type U115 = numeric_array::typenum::U115;
pub type U116 = numeric_array::typenum::U116;
pub type U117 = numeric_array::typenum::U117;
pub type U118 = numeric_array::typenum::U118;
pub type U119 = numeric_array::typenum::U119;
pub type U120 = numeric_array::typenum::U120;
pub type U121 = numeric_array::typenum::U121;
pub type U122 = numeric_array::typenum::U122;
pub type U123 = numeric_array::typenum::U123;
pub type U124 = numeric_array::typenum::U124;
pub type U125 = numeric_array::typenum::U125;
pub type U126 = numeric_array::typenum::U126;
pub type U127 = numeric_array::typenum::U127;
pub type U128 = numeric_array::typenum::U128;

/// Constant node. The constant can be scalar, tuple, or a Frame.
/// Synonymous with [`dc`].
/// - Output(s): constant
///
/// ### Example: Sine Oscillator
/// ```
/// use fundsp::hacker::*;
/// constant(440.0) >> sine();
/// ```
pub fn constant<X: ConstantFrame<Sample = f64>>(x: X) -> An<Constant<X::Size, f64>>
where
    X::Size: Size<f64>,
{
    An(Constant::new(x.convert()))
}

/// Constant node. The constant can be scalar, tuple, or a Frame.
/// Synonymous with [`constant`].
/// (DC stands for "direct current", which is an electrical engineering term used with signals.)
/// - Output(s): constant
///
/// ### Example: Dual Sine Oscillator
/// ```
/// use fundsp::hacker::*;
/// dc((220.0, 440.0)) >> (sine() + sine());
/// ```
pub fn dc<X: ConstantFrame<Sample = f64>>(x: X) -> An<Constant<X::Size, f64>>
where
    X::Size: Size<f64>,
{
    An(Constant::new(x.convert()))
}

/// Zero generator.
/// - Output 0: zero
///
/// ### Example: Pluck Oscillator
/// ```
/// use fundsp::hacker::*;
/// zero() >> pluck(220.0, db_amp(-6.0), 0.5);
/// ```
pub fn zero() -> An<Constant<U1, f64>> {
    constant(0.0)
}

/// Multichannel zero generator.
/// - Output(s): zero
///
/// ### Example: Stereo Pluck Oscillator
/// ```
/// use fundsp::hacker::*;
/// multizero() >> (pluck(220.0, db_amp(-6.0), 0.5) | pluck(220.0, db_amp(-6.0), 0.5));
/// ```
pub fn multizero<N: Size<f64>>() -> An<Constant<N, f64>> {
    An(Constant::new(Frame::splat(0.0)))
}

/// Update enclosed node `x` with approximately `dt` seconds between updates.
/// The update function is `f(t, dt, x)` where `t` is current time,
/// `dt` is time from previous update, and `x` is the enclosed node.
pub fn update<X: AudioNode, F: FnMut(f64, f64, &mut X) + Clone + Send + Sync>(
    x: An<X>,
    dt: f64,
    f: F,
) -> An<System<f64, X, F>> {
    An(System::new(x, dt, f))
}

/// Mono pass-through.
/// - Input 0: signal
/// - Output 0: signal
///
/// ### Example: Add Feedback Delay
/// ```
/// use fundsp::hacker::*;
/// pass() & 0.2 * feedback(delay(1.0) * db_amp(-3.0));
/// ```
pub fn pass() -> An<Pass<f64>> {
    An(Pass::new())
}

/// Multichannel pass-through.
/// - Input(s): signal
/// - Output(s): signal
///
/// ### Example: Add Feedback Delay In Stereo
/// ```
/// use fundsp::hacker::*;
/// multipass::<U2>() & 0.2 * feedback((delay(1.0) | delay(1.0)) * db_amp(-3.0));
/// ```
pub fn multipass<N: Size<f64>>() -> An<MultiPass<N, f64>> {
    An(MultiPass::new())
}

/// Monitor node. Passes through input. Communicates via the shared variable
/// an aspect of the input signal according to the chosen metering mode.
/// - Input 0: signal
/// - Output 0: signal
///
/// ### Example
/// ```
/// use fundsp::hacker::*;
/// let rms = shared(0.0);
/// monitor(&rms, Meter::Rms(0.1));
/// ```
pub fn monitor(shared: &Shared<f64>, meter: Meter) -> An<Monitor<f64>> {
    An(Monitor::new(shared, meter))
}

/// Meter node.
/// Outputs a summary of the input according to the chosen metering mode.
/// - Input 0: signal
/// - Output 0: summary
///
/// ### Example
/// ```
/// use fundsp::hacker::*;
/// meter(Meter::Rms(0.1));
/// ```
pub fn meter(meter: Meter) -> An<MeterNode<f64>> {
    An(MeterNode::new(meter))
}

/// Mono sink. Input is discarded.
/// -Input 0: signal
pub fn sink() -> An<Sink<U1, f64>> {
    An(Sink::new())
}

/// Multichannel sink. Inputs are discarded.
/// -Input(s): signal
pub fn multisink<N: Size<f64>>() -> An<Sink<N, f64>> {
    An(Sink::new())
}

/// Reverse channel order.
/// - Input(s): signal
/// - Output(s): signal in reverse channel order
///
/// ### Example: Ping-Pong Delay
/// ```
/// use fundsp::hacker::*;
/// feedback((delay(1.0) | delay(1.0)) >> reverse() * db_amp(-3.0));
/// ```
pub fn reverse<N: Size<f64>>() -> An<Reverse<N, f64>> {
    An(Reverse::new())
}

/// Sine oscillator.
/// - Input 0: frequency (Hz)
/// - Output 0: sine wave
///
/// ### Example: Vibrato
/// ```
/// use fundsp::hacker::*;
/// lfo(|t| 110.0 + lerp11(-2.0, 2.0, sin_hz(t, 5.0))) >> sine();
/// ```
pub fn sine() -> An<Sine<f64>> {
    An(Sine::new(DEFAULT_SR))
}

/// Fixed sine oscillator at `f` Hz.
/// - Output 0: sine wave
///
/// ### Example
/// ```
/// use fundsp::hacker::*;
/// sine_hz(440.0);
/// ```
pub fn sine_hz(f: f64) -> An<Pipe<f64, Constant<U1, f64>, Sine<f64>>> {
    super::prelude::sine_hz(f)
}

/// Rossler dynamical system oscillator.
/// - Input 0: frequency. The Rossler oscillator exhibits peaks at multiples of this frequency.
/// - Output 0: system output
///
/// ### Example
/// ```
/// use fundsp::hacker::*;
/// lfo(|t| 440.0 + 10.0 * sin_hz(6.0, t)) >> rossler();
/// ```
pub fn rossler() -> An<Rossler<f64>> {
    An(Rossler::new())
}

/// Lorenz dynamical system oscillator.
/// - Input 0: frequency. The Lorenz system exhibits slight frequency effects.
/// - Output 0: system output
///
/// ### Example
/// ```
/// use fundsp::hacker::*;
/// lfo(|t| 110.0 + 5.0 * sin_hz(5.0, t)) >> lorenz();
/// ```
pub fn lorenz() -> An<Lorenz<f64>> {
    An(Lorenz::new())
}

/// Add constant to signal.
/// - Input(s): signal
/// - Output(s): signal plus constant
pub fn add<X: ConstantFrame<Sample = f64>>(
    x: X,
) -> An<Binop<f64, FrameAdd<X::Size, f64>, MultiPass<X::Size, f64>, Constant<X::Size, f64>>>
where
    X::Size: Size<f64> + Add<U0>,
    <X::Size as Add<U0>>::Output: Size<f64>,
{
    An(MultiPass::<X::Size, f64>::new()) + dc(x)
}

/// Subtract constant from signal.
/// - Input(s): signal
/// - Output(s): signal minus constant
pub fn sub<X: ConstantFrame<Sample = f64>>(
    x: X,
) -> An<Binop<f64, FrameSub<X::Size, f64>, MultiPass<X::Size, f64>, Constant<X::Size, f64>>>
where
    X::Size: Size<f64> + Add<U0>,
    <X::Size as Add<U0>>::Output: Size<f64>,
{
    An(MultiPass::<X::Size, f64>::new()) - dc(x)
}

/// Multiply signal with constant.
/// - Input(s): signal
/// - Output(s): signal times constant
pub fn mul<X: ConstantFrame<Sample = f64>>(
    x: X,
) -> An<Binop<f64, FrameMul<X::Size, f64>, MultiPass<X::Size, f64>, Constant<X::Size, f64>>>
where
    X::Size: Size<f64> + Add<U0>,
    <X::Size as Add<U0>>::Output: Size<f64>,
{
    An(MultiPass::<X::Size, f64>::new()) * dc(x)
}

/// Butterworth lowpass filter (2nd order).
/// - Input 0: audio
/// - Input 1: cutoff frequency (Hz)
/// - Output 0: filtered audio
///
/// ### Example: Filtered Noise
/// ```
/// use fundsp::hacker::*;
/// (noise() | dc(1000.0)) >> butterpass();
/// ```
pub fn butterpass() -> An<ButterLowpass<f64, f64, U2>> {
    An(ButterLowpass::new(440.0))
}

/// Butterworth lowpass filter (2nd order) with fixed cutoff frequency `f` Hz.
/// - Input 0: audio
/// - Output 0: filtered audio
pub fn butterpass_hz(f: f64) -> An<ButterLowpass<f64, f64, U1>> {
    super::prelude::butterpass_hz(f)
}

/// One-pole lowpass filter (1st order).
/// - Input 0: audio
/// - Input 1: cutoff frequency (Hz)
/// - Output 0: filtered audio
///
/// ### Example: Brown Noise
/// ```
/// use fundsp::hacker::*;
/// (noise() | dc(10.0)) >> lowpole();
/// ```
pub fn lowpole() -> An<Lowpole<f64, f64, U2>> {
    An(Lowpole::new(440.0))
}

/// One-pole lowpass filter (1st order) with fixed cutoff frequency `f` Hz.
/// - Input 0: audio
/// - Output 0: filtered audio
///
/// ### Example: Brown Noise
/// ```
/// use fundsp::hacker::*;
/// noise() >> lowpole_hz(10.0);
/// ```
pub fn lowpole_hz(f: f64) -> An<Lowpole<f64, f64, U1>> {
    super::prelude::lowpole_hz(f)
}

/// Allpass filter (1st order) with adjustable delay (delay > 0) in samples at DC.
/// - Input 0: audio
/// - Input 1: delay in samples
/// - Output 0: filtered audio
pub fn allpole() -> An<Allpole<f64, f64, U2>> {
    An(Allpole::new(1.0))
}

/// Allpass filter (1st order) with `delay` (`delay` > 0) in samples at DC.
/// - Input 0: audio
/// - Output 0: filtered audio
pub fn allpole_delay(delay: f64) -> An<Allpole<f64, f64, U1>> {
    An(Allpole::new(delay))
}

/// One-pole, one-zero highpass filter (1st order).
/// - Input 0: audio
/// - Input 1: cutoff frequency (Hz)
/// - Output 0: filtered audio
pub fn highpole() -> An<Highpole<f64, f64, U2>> {
    An(Highpole::new(440.0))
}

/// One-pole, one-zero highpass filter (1st order) with fixed cutoff frequency f.
/// - Input 0: audio
/// - Output 0: filtered audio
pub fn highpole_hz(f: f64) -> An<Highpole<f64, f64, U1>> {
    An(Highpole::new(f))
}

/// Constant-gain bandpass resonator.
/// - Input 0: audio
/// - Input 1: cutoff frequency (Hz)
/// - Input 2: bandwidth (Hz)
/// - Output 0: filtered audio
///
/// ### Example: Filtered Noise Tone
/// ```
/// use fundsp::hacker::*;
/// (noise() | dc((440.0, 5.0))) >> resonator();
/// ```
pub fn resonator() -> An<Resonator<f64, f64, U3>> {
    An(Resonator::new(440.0, 110.0))
}

/// Constant-gain bandpass resonator with fixed `center` frequency (Hz) and `bandwidth` (Hz).
/// - Input 0: audio
/// - Output 0: filtered audio
///
/// ### Example: Filtered Noise Tone
/// ```
/// use fundsp::hacker::*;
/// noise() >> resonator_hz(440.0, 5.0);
/// ```
pub fn resonator_hz(center: f64, bandwidth: f64) -> An<Resonator<f64, f64, U1>> {
    super::prelude::resonator_hz(center, bandwidth)
}

/// An arbitrary biquad filter with coefficients in normalized form.
/// - Input 0: signal
/// - Output 0: filtered signal
pub fn biquad(a1: f64, a2: f64, b0: f64, b1: f64, b2: f64) -> An<Biquad<f64, f64>> {
    An(Biquad::with_coefs(BiquadCoefs::arbitrary(
        a1, a2, b0, b1, b2,
    )))
}

/// Moog resonant lowpass filter.
/// - Input 0: input signal
/// - Input 1: cutoff frequency (Hz)
/// - Input 2: Q
/// - Output 0: filtered signal
pub fn moog() -> An<Moog<f64, f64, U3>> {
    An(Moog::new(DEFAULT_SR, 1000.0, 0.1))
}

/// Moog resonant lowpass filter with fixed Q.
/// - Input 0: input signal
/// - Input 1: cutoff frequency (Hz)
/// - Output 0: filtered signal
pub fn moog_q(
    q: f64,
) -> An<Pipe<f64, Stack<f64, MultiPass<U2, f64>, Constant<U1, f64>>, Moog<f64, f64, U3>>> {
    (multipass::<U2>() | dc(q)) >> An(Moog::new(convert(DEFAULT_SR), 1000.0, q))
}

/// Moog resonant lowpass filter with fixed cutoff frequency and Q.
/// - Input 0: input signal
/// - Output 0: filtered signal
pub fn moog_hz(frequency: f64, q: f64) -> An<Moog<f64, f64, U1>> {
    An(Moog::new(DEFAULT_SR, frequency, q))
}

/// Morphing filter that morphs between lowpass, peak and highpass modes.
/// - Input 0: input signal
/// - Input 1: center frequency (Hz)
/// - Input 2: Q
/// - Input 3: morph in -1...1 (-1 = lowpass, 0 = peak, 1 = highpass)
/// - Output 0: filtered signal
pub fn morph() -> An<super::prelude::Morph<f64, f64>> {
    super::prelude::morph()
}

/// Morphing filter with center frequency `f`, Q value `q`, and morph `morph`
/// (-1 = lowpass, 0 = peaking, 1 = highpass).
/// - Input 0: input signal
/// - Output 0: filtered signal
pub fn morph_hz(
    f: f64,
    q: f64,
    morph: f64,
) -> An<Pipe<f64, Stack<f64, Pass<f64>, Constant<U3, f64>>, super::prelude::Morph<f64, f64>>> {
    super::prelude::morph_hz(f, q, morph)
}

/// Control envelope from time-varying function `f(t)` with `t` in seconds.
/// Spaces samples using pseudorandom jittering.
/// Synonymous with [`fn@lfo`].
/// - Output(s): envelope linearly interpolated from samples at 2 ms intervals (average).
///
/// ### Example: Mixing Pink And Brown Noise
/// ```
/// use fundsp::hacker::*;
/// envelope(|t| (sin_hz(1.0, t), cos_hz(1.0, t))) * (pink() | brown()) >> join();
/// ```
pub fn envelope<E, R>(f: E) -> An<Envelope<f64, f64, E, R>>
where
    E: Fn(f64) -> R + Clone + Send + Sync,
    R: ConstantFrame<Sample = f64>,
    R::Size: Size<f64>,
{
    An(Envelope::new(0.002, DEFAULT_SR, f))
}

/// Control envelope from time-varying function `f(t)` with `t` in seconds.
/// Spaces samples using pseudorandom jittering.
/// Synonymous with [`fn@envelope`].
/// - Output(s): envelope linearly interpolated from samples at 2 ms intervals (average).
///
/// ### Example: Exponentially Decaying White Noise
/// ```
/// use fundsp::hacker::*;
/// lfo(|t| exp(-t)) * white();
/// ```
pub fn lfo<E, R>(f: E) -> An<Envelope<f64, f64, E, R>>
where
    E: Fn(f64) -> R + Clone + Send + Sync,
    R: ConstantFrame<Sample = f64>,
    R::Size: Size<f64>,
{
    An(Envelope::new(0.002, DEFAULT_SR, f))
}

/// Control envelope from time-varying, input dependent function `f(t, x)` with `t` in seconds.
/// Spaces samples using pseudorandom jittering.
/// Synonymous with `lfo2`.
/// - Input 0: x
/// - Output(s): envelope linearly interpolated from samples at 2 ms intervals (average).
///
/// ### Example (LFO Speed Control)
/// ```
/// use fundsp::hacker::*;
/// let speed = shared(1.0);
/// var(&speed) >> envelope2(|t, speed| exp(-t * speed));
/// ```
pub fn envelope2<E, R>(
    f: E,
) -> An<EnvelopeIn<f64, f64, impl Fn(f64, &Frame<f64, U1>) -> R + Sized + Clone, U1, R>>
where
    E: Fn(f64, f64) -> R + Clone + Send + Sync,
    R: ConstantFrame<Sample = f64>,
    R::Size: Size<f64>,
{
    An(EnvelopeIn::new(
        0.002,
        DEFAULT_SR,
        move |t, i: &Frame<f64, U1>| f(t, i[0]),
    ))
}

/// Control envelope from time-varying, input dependent function `f(t, x)` with `t` in seconds.
/// Spaces samples using pseudorandom jittering.
/// Synonymous with `envelope2`.
/// - Input 0: x
/// - Output(s): envelope linearly interpolated from samples at 2 ms intervals (average).
///
/// ### Example (Amplitude Control)
/// ```
/// use fundsp::hacker::*;
/// let amp = shared(1.0);
/// var(&amp) >> lfo2(|t, amp| amp * exp(-t));
/// ```
pub fn lfo2<E, R>(
    f: E,
) -> An<EnvelopeIn<f64, f64, impl Fn(f64, &Frame<f64, U1>) -> R + Sized + Clone, U1, R>>
where
    E: Fn(f64, f64) -> R + Clone + Send + Sync,
    R: ConstantFrame<Sample = f64>,
    R::Size: Size<f64>,
{
    An(EnvelopeIn::new(
        0.002,
        DEFAULT_SR,
        move |t, i: &Frame<f64, U1>| f(t, i[0]),
    ))
}

/// Control envelope from time-varying, input dependent function `f(t, x, y)` with `t` in seconds.
/// Spaces samples using pseudorandom jittering.
/// Synonymous with `lfo3`.
/// - Input 0: x
/// - Input 1: y
/// - Output(s): envelope linearly interpolated from samples at 2 ms intervals (average).
pub fn envelope3<E, R>(
    f: E,
) -> An<EnvelopeIn<f64, f64, impl Fn(f64, &Frame<f64, U2>) -> R + Sized + Clone, U2, R>>
where
    E: Fn(f64, f64, f64) -> R + Clone + Send + Sync,
    R: ConstantFrame<Sample = f64>,
    R::Size: Size<f64>,
{
    An(EnvelopeIn::new(
        0.002,
        DEFAULT_SR,
        move |t, i: &Frame<f64, U2>| f(t, i[0], i[1]),
    ))
}

/// Control envelope from time-varying, input dependent function `f(t, x, y)` with `t` in seconds.
/// Spaces samples using pseudorandom jittering.
/// Synonymous with `envelope3`.
/// - Input 0: x
/// - Input 1: y
/// - Output(s): envelope linearly interpolated from samples at 2 ms intervals (average).
///
/// ### Example (Clamped Sine)
/// ```
/// use fundsp::hacker::*;
/// let min = shared(-1.0);
/// let max = shared(1.0);
/// (var(&min) | var(&max)) >> lfo3(|t, min, max| clamp(min, max, sin_hz(110.0, t)));
/// max.set(0.5);
/// ```
pub fn lfo3<E, R>(
    f: E,
) -> An<EnvelopeIn<f64, f64, impl Fn(f64, &Frame<f64, U2>) -> R + Sized + Clone, U2, R>>
where
    E: Fn(f64, f64, f64) -> R + Clone + Send + Sync,
    R: ConstantFrame<Sample = f64>,
    R::Size: Size<f64>,
{
    An(EnvelopeIn::new(
        0.002,
        DEFAULT_SR,
        move |t, i: &Frame<f64, U2>| f(t, i[0], i[1]),
    ))
}

/// Control envelope from time-varying, input dependent function `f(t, i)` with `t` in seconds
/// and `i` of type `&Frame<f64, I>` where `I` is the number of input channels.
/// Spaces samples using pseudorandom jittering.
/// Synonymous with `lfo_in`.
/// - Inputs: i
/// - Output(s): envelope linearly interpolated from samples at 2 ms intervals (average).
pub fn envelope_in<E, I, R>(f: E) -> An<EnvelopeIn<f64, f64, E, I, R>>
where
    E: Fn(f64, &Frame<f64, I>) -> R + Clone + Send + Sync,
    I: Size<f64>,
    R: ConstantFrame<Sample = f64>,
    R::Size: Size<f64>,
    R::Size: Size<f64>,
{
    An(EnvelopeIn::new(0.002, DEFAULT_SR, f))
}

/// Control envelope from time-varying, input dependent function `f(t, i)` with `t` in seconds
/// and `i` of type `&Frame<f64, I>` where `I` is the number of input channels.
/// Spaces samples using pseudorandom jittering.
/// Synonymous with `envelope_in`.
/// - Inputs: i
/// - Output(s): envelope linearly interpolated from samples at 2 ms intervals (average).
pub fn lfo_in<E, I, R>(f: E) -> An<EnvelopeIn<f64, f64, E, I, R>>
where
    E: Fn(f64, &Frame<f64, I>) -> R + Clone + Send + Sync,
    I: Size<f64>,
    R: ConstantFrame<Sample = f64>,
    R::Size: Size<f64>,
    R::Size: Size<f64>,
{
    An(EnvelopeIn::new(0.002, DEFAULT_SR, f))
}

/// ADSR envelope.
///
/// When a positive value is given by the input, its output increases from 0.0 to 1.0 in the time
/// interval denoted by `attack`. It then decreases from 1.0 to the `sustain` level in the time
/// interval denoted by `decay`. It remains at the `sustain` level until a zero or negative value
/// is given by the input, after which it decreases from the `sustain` level to 0.0 in a time
/// interval denoted by `release`.
///
/// - Input 0: control start of attack and release
/// - Output 0: scaled ADSR value from 0.0 to 1.0
///
/// See [live_adsr.rs](https://github.com/SamiPerttu/fundsp/blob/master/examples/live_adsr.rs) for
/// a program that uses this function to control the volume of live notes from a MIDI instrument.
pub fn adsr_live(
    attack: f64,
    decay: f64,
    sustain: f64,
    release: f64,
) -> An<EnvelopeIn<f64, f64, impl Fn(f64, &Frame<f64, U1>) -> f64 + Sized + Clone, U1, f64>> {
    super::adsr::adsr_live(attack, decay, sustain, release)
}

/// Maximum Length Sequence noise generator from an `n`-bit sequence (1 <= `n` <= 31).
/// - Output 0: repeating white noise sequence of only -1 and 1 values.
///
/// ### Example
/// ```
/// use fundsp::hacker::*;
/// mls_bits(31);
/// ```
pub fn mls_bits(n: i64) -> An<Mls<f64>> {
    An(Mls::new(MlsState::new(n as u32)))
}

/// Default Maximum Length Sequence noise generator.
/// - Output 0: repeating white noise sequence of only -1 and 1 values.
///
/// ### Example
/// ```
/// use fundsp::hacker::*;
/// mls();
/// ```
pub fn mls() -> An<Mls<f64>> {
    mls_bits(29)
}

/// White noise generator.
/// Synonymous with `white`.
/// - Output 0: white noise.
///
/// ### Example
/// ```
/// use fundsp::hacker::*;
/// noise();
/// ```
pub fn noise() -> An<Noise<f64>> {
    An(Noise::new())
}

/// White noise generator.
/// Synonymous with `noise`.
/// - Output 0: white noise.
///
/// ### Example
/// ```
/// use fundsp::hacker::*;
/// white();
/// ```
pub fn white() -> An<Noise<f64>> {
    An(Noise::new())
}

/// Sample-and-hold component. Sampling frequency `variability` is in 0...1.
/// - Input 0: signal.
/// - Input 1: sampling frequency (Hz).
/// - Output 0: sampled signal.
///
/// ### Example (Sampled And Held Noise)
/// ```
/// use fundsp::hacker::*;
/// (pink() | dc(440.0)) >> hold(0.5);
/// ```
pub fn hold(variability: f64) -> An<Hold<f64>> {
    An(Hold::new(variability))
}

/// Sample-and-hold component. Sampling frequency `variability` is in 0...1.
/// - Input 0: signal.
/// - Output 0: sampled signal.
pub fn hold_hz(
    f: f64,
    variability: f64,
) -> An<Pipe<f64, Stack<f64, Pass<f64>, Constant<U1, f64>>, Hold<f64>>> {
    (pass() | dc(f)) >> hold(variability)
}

/// FIR filter.
/// - Input 0: signal.
/// - Output 0: filtered signal.
///
/// ### Example: 3-Point Lowpass Filter
/// ```
/// use fundsp::hacker::*;
/// fir((0.5, 1.0, 0.5));
/// ```
pub fn fir<X: ConstantFrame<Sample = f64>>(weights: X) -> An<Fir<f64, X::Size>> {
    An(Fir::new(weights))
}

/// Create a 3-point symmetric FIR from desired `gain` (`gain` >= 0) at the Nyquist frequency.
/// Results in a monotonic low-pass filter when `gain` < 1.
/// - Input 0: signal.
/// - Output 0: filtered signal.
pub fn fir3(gain: f64) -> Fir<f64, U3> {
    super::prelude::fir3(gain)
}

/// Single sample delay.
/// - Input 0: signal.
/// - Output 0: delayed signal.
///
/// ### Example: 2-Point Sum Filter
/// ```
/// use fundsp::hacker::*;
/// tick() & pass();
/// ```
pub fn tick() -> An<Tick<U1, f64>> {
    An(Tick::new())
}

/// Multichannel single sample delay.
/// - Inputs: signal.
/// - Outputs: delayed signal.
///
/// ### Example
/// ```
/// use fundsp::hacker::*;
/// multitick::<U2>();
/// ```
pub fn multitick<N: Size<f64>>() -> An<Tick<N, f64>> {
    An(Tick::new())
}

/// Fixed delay of `t` seconds.
/// Delay time is rounded to the nearest sample.
/// Allocates: the delay line.
/// - Input 0: signal.
/// - Output 0: delayed signal.
///
/// ### Example
/// ```
/// use fundsp::hacker::*;
/// delay(1.0);
/// ```
pub fn delay(t: f64) -> An<Delay<f64>> {
    An(Delay::new(t))
}

/// Tapped delay line with cubic interpolation.
/// Minimum and maximum delay times are in seconds.
/// Allocates: the delay line.
/// - Input 0: signal.
/// - Input 1: delay time in seconds.
/// - Output 0: delayed signal.
///
/// ### Example: Variable Delay
/// ```
/// use fundsp::hacker::*;
/// pass() & (pass() | lfo(|t| lerp11(0.01, 0.1, spline_noise(0, t)))) >> tap(0.01, 0.1);
/// ```
pub fn tap(min_delay: f64, max_delay: f64) -> An<Tap<U1, f64>> {
    An(Tap::new(min_delay, max_delay))
}

/// Tapped delay line with cubic interpolation.
/// The number of taps is `N`.
/// Minimum and maximum delay times are in seconds.
/// Allocates: the delay line.
/// - Input 0: signal.
/// - Inputs 1...N: delay time in seconds.
/// - Output 0: delayed signal.
///
/// ### Example: Dual Variable Delay
/// ```
/// use fundsp::hacker::*;
/// (pass() | lfo(|t| (lerp11(0.01, 0.1, spline_noise(0, t)), lerp11(0.1, 0.2, spline_noise(1, t))))) >> multitap::<U2>(0.01, 0.2);
/// ```
pub fn multitap<N>(min_delay: f64, max_delay: f64) -> An<Tap<N, f64>>
where
    N: Size<f64> + Add<U1>,
    <N as Add<U1>>::Output: Size<f64>,
{
    An(Tap::new(min_delay, max_delay))
}

/// 2x oversample enclosed `node`.
/// - Inputs and outputs: from `node`.
///
/// ### Example: Oversampled FM Oscillator
/// ```
/// use fundsp::hacker::*;
/// let f = 440.0;
/// let m = 1.0;
/// oversample(sine_hz(f) * f * m + f >> sine());
/// ```
pub fn oversample<X>(node: An<X>) -> An<Oversampler<f64, X>>
where
    X: AudioNode<Sample = f64>,
    X::Inputs: Size<f64>,
    X::Outputs: Size<f64>,
    X::Inputs: Size<Frame<f64, U128>>,
    X::Outputs: Size<Frame<f64, U128>>,
{
    An(Oversampler::new(DEFAULT_SR, node.0))
}

/// Resample enclosed generator `node` using cubic interpolation
/// at speed obtained from input 0, where 1 is the original speed.
/// Input 0: Sampling speed.
/// Output(s): Resampled outputs of contained generator.
///
/// ### Example: Resampled Pink Noise
/// ```
/// use fundsp::hacker::*;
/// lfo(|t| xerp11(0.5, 2.0, spline_noise(1, t))) >> resample(pink());
/// ```
pub fn resample<X>(node: An<X>) -> An<Resampler<f64, X>>
where
    X: AudioNode<Sample = f64, Inputs = U0>,
    X::Outputs: Size<f64>,
    X::Outputs: Size<Frame<f64, U128>>,
{
    An(Resampler::new(DEFAULT_SR, node.0))
}

/// Mix output of enclosed circuit `node` back to its input.
/// Feedback circuit `node` must have an equal number of inputs and outputs.
/// - Input(s): signal.
/// - Output(s): signal with feedback.
///
/// ### Example: Feedback Delay With Lowpass
/// ```
/// use fundsp::hacker::*;
/// pass() & feedback(delay(1.0) >> lowpass_hz(1000.0, 1.0));
/// ```
pub fn feedback<N, X>(node: An<X>) -> An<Feedback<N, f64, X, FrameId<N, f64>>>
where
    X: AudioNode<Sample = f64, Inputs = N, Outputs = N>,
    X::Inputs: Size<f64>,
    X::Outputs: Size<f64>,
    N: Size<f64>,
{
    An(Feedback::new(node.0, FrameId::new()))
}

/// Mix output of enclosed circuit `node` back to its input
/// with extra `loopback` feedback loop processing.
/// Feedback circuits `node` and `loopback` must have an equal number of inputs and outputs.
/// - Input(s): signal.
/// - Output(s): signal with feedback.
///
/// ### Example: Feedback Delay With Lowpass
/// ```
/// use fundsp::hacker::*;
/// pass() & feedback2(delay(1.0), lowpass_hz(1000.0, 1.0));
/// ```
pub fn feedback2<N, X, Y>(
    node: An<X>,
    loopback: An<Y>,
) -> An<Feedback2<N, f64, X, Y, FrameId<N, f64>>>
where
    N: Size<f64>,
    X: AudioNode<Sample = f64, Inputs = N, Outputs = N>,
    X::Inputs: Size<f64>,
    X::Outputs: Size<f64>,
    Y: AudioNode<Sample = f64, Inputs = N, Outputs = N>,
    Y::Inputs: Size<f64>,
    Y::Outputs: Size<f64>,
{
    An(Feedback2::new(node.0, loopback.0, FrameId::new()))
}

/// Transform channels freely. Accounted as non-linear processing for signal flow.
///
/// ### Example: Max Operator
/// ```
/// use fundsp::hacker::*;
/// map(|i: &Frame<f64, U2>| max(i[0], i[1]));
/// ```
pub fn map<M, I, O>(f: M) -> An<Map<f64, M, I, O>>
where
    M: Fn(&Frame<f64, I>) -> O + Clone + Send + Sync,
    I: Size<f64>,
    O: ConstantFrame<Sample = f64>,
    O::Size: Size<f64>,
{
    An(Map::new(f, Routing::Arbitrary))
}

/// Keeps a signal zero centered.
/// Filter `cutoff` (in Hz) is usually somewhere below the audible range.
/// The default blocker cutoff is 10 Hz.
/// - Input 0: input signal
/// - Output 0: filtered signal
///
/// ### Example
/// ```
/// use fundsp::hacker::*;
/// dcblock_hz(8.0);
/// ```
pub fn dcblock_hz(cutoff: f64) -> An<DCBlock<f64, f64>> {
    An(DCBlock::new(cutoff))
}

/// Keeps a signal zero centered. The cutoff of the filter is 10 Hz.
/// - Input 0: input signal
/// - Output 0: filtered signal
///
/// ### Example: Stereo DC Blocker
/// ```
/// use fundsp::hacker::*;
/// dcblock() | dcblock();
/// ```
pub fn dcblock() -> An<DCBlock<f64, f64>> {
    dcblock_hz(10.0)
}

/// Apply 10 ms of fade-in to signal at time zero.
/// - Input 0: input signal
/// - Output 0: signal with fade-in
pub fn declick() -> An<Declick<f64, f64>> {
    super::prelude::declick()
}

/// Apply `t` seconds of fade-in to signal at time zero.
/// - Input 0: input signal
/// - Output 0: signal with fade-in
pub fn declick_s(t: f64) -> An<Declick<f64, f64>> {
    super::prelude::declick_s(t)
}

/// Shape signal with a waveshaper function.
/// - Input 0: input signal
/// - Output 0: shaped signal
pub fn shape_fn<S: Fn(f64) -> f64 + Clone + Send + Sync>(f: S) -> An<ShaperFn<f64, S>> {
    super::prelude::shape_fn(f)
}

/// Shape signal according to shaping mode.
/// - Input 0: input signal
/// - Output 0: shaped signal
///
/// ### Example: Tanh Distortion
/// ```
/// use fundsp::hacker::*;
/// shape(Shape::Tanh(1.0));
/// ```
pub fn shape(mode: Shape<f64>) -> An<Shaper<f64>> {
    super::prelude::shape(mode)
}

/// Clip signal to -1...1.
/// - Input 0: input signal
/// - Output 0: clipped signal
pub fn clip() -> An<Shaper<f64>> {
    super::prelude::clip()
}

/// Clip signal to `minimum`...`maximum`.
/// - Input 0: input signal
/// - Output 0: clipped signal
pub fn clip_to(minimum: f64, maximum: f64) -> An<Shaper<f64>> {
    super::prelude::clip_to(minimum, maximum)
}

/// Equal power mono-to-stereo panner.
/// - Input 0: input signal
/// - Input 1: pan in -1...1 (left to right).
/// - Output 0: left channel
/// - Output 1: right channel
///
/// ### Example: Panning Noise
/// ```
/// use fundsp::hacker::*;
/// (noise() | sine_hz(0.5)) >> panner();
/// ```
pub fn panner() -> An<Panner<f64, U2>> {
    An(Panner::new(0.0))
}

/// Fixed equal power mono-to-stereo panner with `pan` value in -1...1 (left to right).
/// - Input 0: input signal
/// - Output 0: left channel
/// - Output 1: right channel
///
/// ### Example (Center Panned Saw Wave)
/// ```
/// use fundsp::hacker::*;
/// saw_hz(440.0) >> pan(0.0);
/// ```
pub fn pan(pan: f64) -> An<Panner<f64, U1>> {
    An(Panner::new(pan))
}

/// Parameter follower filter with halfway response time `t` seconds.
/// - Input 0: input signal
/// - Output 0: smoothed signal
///
/// ### Example (Smoothed Atomic Parameter)
/// ```
/// use fundsp::hacker::*;
/// let parameter = shared(1.0);
/// var(&parameter) >> follow(0.01);
/// ```
pub fn follow<S: ScalarOrPair<Sample = f64>>(t: S) -> An<AFollow<f64, f64, S>> {
    An(AFollow::new(DEFAULT_SR, t))
}

/// Look-ahead limiter with `(attack, release)` times in seconds.
/// Look-ahead is equal to the attack time.
/// Allocates: look-ahead buffers.
/// - Input 0: signal
/// - Output 0: signal limited to -1...1
pub fn limiter<S: ScalarOrPair<Sample = f64>>(time: S) -> An<Limiter<f64, U1, S>> {
    An(Limiter::new(DEFAULT_SR, time))
}

/// Stereo look-ahead limiter with `(attack, release)` times in seconds.
/// Look-ahead is equal to the attack time.
/// Allocates: look-ahead buffers.
/// - Input 0: left signal
/// - Input 1: right signal
/// - Output 0: left signal limited to -1...1
/// - Output 1: right signal limited to -1...1
pub fn limiter_stereo<S: ScalarOrPair<Sample = f64>>(time: S) -> An<Limiter<f64, U2, S>> {
    An(Limiter::new(DEFAULT_SR, time))
}

/// Pinking filter.
/// - Input 0: input signal
/// - Output 0: filtered signal
pub fn pinkpass() -> An<Pinkpass<f64, f64>> {
    An(Pinkpass::new())
}

/// Pink noise.
/// - Output 0: pink noise
pub fn pink() -> An<Pipe<f64, Noise<f64>, Pinkpass<f64, f64>>> {
    super::prelude::pink()
}

/// Brown noise.
/// - Output 0: brown noise
pub fn brown() -> An<
    Pipe<f64, Noise<f64>, Binop<f64, FrameMul<U1, f64>, Lowpole<f64, f64, U1>, Constant<U1, f64>>>,
> {
    // Empirical normalization factor.
    white() >> lowpole_hz(10.0) * dc(13.7)
}

/// Feedback delay network.
/// Mix output of enclosed circuit `x` back to its input.
/// The output is diffused with a Hadamard matrix for feedback.
/// Feedback circuit `x` must have an equal number of inputs and outputs.
/// - Inputs: input signal.
/// - Outputs: `x` output signal.
///
/// *** Example: Mono Reverb
/// ```
/// use fundsp::hacker::*;
/// split() >> fdn::<U16, _>(stack::<U16, _, _>(|i| { delay(lerp(0.01, 0.03, rnd(i))) >> fir((0.2, 0.4, 0.2)) })) >> join();
/// ```
pub fn fdn<N, X>(x: An<X>) -> An<Feedback<N, f64, X, FrameHadamard<N, f64>>>
where
    X: AudioNode<Sample = f64, Inputs = N, Outputs = N>,
    X::Inputs: Size<f64>,
    X::Outputs: Size<f64>,
    N: Size<f64>,
{
    An(Feedback::new(x.0, FrameHadamard::new()))
}

/// Feedback delay network.
/// Mix output of enclosed circuit `x` back to its input,
/// using `y` for extra feedback processing. The feedforward path does not include `y`.
/// After `y`, the feedback signal is diffused with a Hadamard matrix.
/// Feedback circuits `x` and `y` must have an equal number of inputs and outputs.
/// - Input(s): signal.
/// - Output(s): signal with feedback.
pub fn fdn2<N, X, Y>(x: An<X>, y: An<Y>) -> An<Feedback2<N, f64, X, Y, FrameHadamard<N, f64>>>
where
    N: Size<f64>,
    X: AudioNode<Sample = f64, Inputs = N, Outputs = N>,
    X::Inputs: Size<f64>,
    X::Outputs: Size<f64>,
    Y: AudioNode<Sample = f64, Inputs = N, Outputs = N>,
    Y::Inputs: Size<f64>,
    Y::Outputs: Size<f64>,
{
    An(Feedback2::new(x.0, y.0, FrameHadamard::new()))
}

/// Bus `N` similar nodes from indexed generator `f`.
/// - Input(s): from `f`.
/// - Output(s): from `f`.
///
/// ### Example (Sine Bundle)
/// ```
/// use fundsp::hacker::*;
/// bus::<U20, _, _>(|i| sine_hz(110.0 * exp(lerp(-0.2, 0.2, rnd(i)))));
/// ```
pub fn bus<N, X, F>(f: F) -> An<MultiBus<N, f64, X>>
where
    N: Size<f64>,
    N: Size<X>,
    X: AudioNode<Sample = f64>,
    X::Inputs: Size<f64>,
    X::Outputs: Size<f64>,
    F: Fn(i64) -> An<X>,
{
    super::prelude::bus(f)
}

/// Bus `N` similar nodes from fractional generator `f`.
/// The fractional generator is given values in the range 0...1.
/// - Input(s): from `f`.
/// - Output(s): from `f`.
///
/// ### Example (Noise Bundle)
/// ```
/// use fundsp::hacker::*;
/// busf::<U20, _, _>(|t| (noise() | dc((xerp(100.0, 1000.0, t), 20.0))) >> !resonator() >> resonator());
/// ```
pub fn busf<N, X, F>(f: F) -> An<MultiBus<N, f64, X>>
where
    N: Size<f64>,
    N: Size<X>,
    X: AudioNode<Sample = f64>,
    X::Inputs: Size<f64>,
    X::Outputs: Size<f64>,
    F: Fn(f64) -> An<X>,
{
    super::prelude::busf(f)
}

/// Stack `N` similar nodes from indexed generator `f`.
/// - Input(s): `N` times `f`.
/// - Output(s): `N` times `f`.
pub fn stack<N, X, F>(f: F) -> An<MultiStack<N, f64, X>>
where
    N: Size<f64>,
    N: Size<X>,
    X: AudioNode<Sample = f64>,
    X::Inputs: Size<f64> + Mul<N>,
    X::Outputs: Size<f64> + Mul<N>,
    <X::Inputs as Mul<N>>::Output: Size<f64>,
    <X::Outputs as Mul<N>>::Output: Size<f64>,
    F: Fn(i64) -> An<X>,
{
    super::prelude::stack(f)
}

/// Stack `N` similar nodes from fractional generator `f`.
/// The fractional generator is given values in the range 0...1.
/// - Input(s): `N` times `f`.
/// - Output(s): `N` times `f`.
pub fn stackf<N, X, F>(f: F) -> An<MultiStack<N, f64, X>>
where
    N: Size<f64>,
    N: Size<X>,
    X: AudioNode<Sample = f64>,
    X::Inputs: Size<f64> + Mul<N>,
    X::Outputs: Size<f64> + Mul<N>,
    <X::Inputs as Mul<N>>::Output: Size<f64>,
    <X::Outputs as Mul<N>>::Output: Size<f64>,
    F: Fn(f64) -> An<X>,
{
    super::prelude::stackf(f)
}

/// Branch into `N` similar nodes from indexed generator `f`.
/// - Input(s): from `f`.
/// - Output(s): `N` times `f`.
pub fn branch<N, X, F>(f: F) -> An<MultiBranch<N, f64, X>>
where
    N: Size<f64>,
    N: Size<X>,
    X: AudioNode<Sample = f64>,
    X::Inputs: Size<f64>,
    X::Outputs: Size<f64> + Mul<N>,
    <X::Outputs as Mul<N>>::Output: Size<f64>,
    F: Fn(i64) -> An<X>,
{
    super::prelude::branch(f)
}

/// Branch into `N` similar nodes from fractional generator `f`.
/// The fractional generator is given values in the range 0...1.
/// - Input(s): from `f`.
/// - Output(s): `N` times `f`.
pub fn branchf<N, X, F>(f: F) -> An<MultiBranch<N, f64, X>>
where
    N: Size<f64>,
    N: Size<X>,
    X: AudioNode<Sample = f64>,
    X::Inputs: Size<f64>,
    X::Outputs: Size<f64> + Mul<N>,
    <X::Outputs as Mul<N>>::Output: Size<f64>,
    F: Fn(f64) -> An<X>,
{
    super::prelude::branchf(f)
}

/// Mix together `N` similar nodes from indexed generator `f`.
/// - Input(s): `N` times `f`.
/// - Output(s): from `f`.
pub fn sum<N, X, F>(f: F) -> An<Reduce<N, f64, X, FrameAdd<X::Outputs, f64>>>
where
    N: Size<f64>,
    N: Size<X>,
    X: AudioNode<Sample = f64>,
    X::Inputs: Size<f64> + Mul<N>,
    X::Outputs: Size<f64>,
    <X::Inputs as Mul<N>>::Output: Size<f64>,
    F: Fn(i64) -> An<X>,
{
    super::prelude::sum(f)
}

/// Mix together `N` similar nodes from fractional generator `f`.
/// The fractional generator is given values in the range 0...1.
/// - Input(s): `N` times `f`.
/// - Output(s): from `f`.
pub fn sumf<N, X, F>(f: F) -> An<Reduce<N, f64, X, FrameAdd<X::Outputs, f64>>>
where
    N: Size<f64>,
    N: Size<X>,
    X: AudioNode<Sample = f64>,
    X::Inputs: Size<f64> + Mul<N>,
    X::Outputs: Size<f64>,
    <X::Inputs as Mul<N>>::Output: Size<f64>,
    F: Fn(f64) -> An<X>,
{
    super::prelude::sumf(f)
}

/// Chain together `N` similar nodes from indexed generator `f`.
/// - Input(s): from `f`.
/// - Output(s): from `f`.
pub fn pipe<N, X, F>(f: F) -> An<Chain<N, f64, X>>
where
    N: Size<f64>,
    N: Size<X>,
    X: AudioNode<Sample = f64>,
    X::Inputs: Size<f64>,
    X::Outputs: Size<f64>,
    F: Fn(i64) -> An<X>,
{
    super::prelude::pipe(f)
}

/// Chain together `N` similar nodes from fractional generator `f`.
/// The fractional generator is given values in the range 0...1.
/// - Input(s): from `f`.
/// - Output(s): from `f`.
pub fn pipef<N, X, F>(f: F) -> An<Chain<N, f64, X>>
where
    N: Size<f64>,
    N: Size<X>,
    X: AudioNode<Sample = f64>,
    X::Inputs: Size<f64>,
    X::Outputs: Size<f64>,
    F: Fn(f64) -> An<X>,
{
    super::prelude::pipef(f)
}

/// Split signal into N channels.
/// - Input 0: signal.
/// - Output(s): `N` copies of signal.
pub fn split<N>() -> An<Split<N, f64>>
where
    N: Size<f64>,
{
    super::prelude::split::<N, f64>()
}

/// Split `M` channels into `N` branches. The output has `N` * `M` channels.
/// - Input(s): `M`.
/// - Output(s): `N` * `M`. Each branch contains a copy of the input(s).
pub fn multisplit<M, N>() -> An<MultiSplit<M, N, f64>>
where
    M: Size<f64> + Mul<N>,
    N: Size<f64>,
    <M as Mul<N>>::Output: Size<f64>,
{
    super::prelude::multisplit::<M, N, f64>()
}

/// Average `N` channels into one. Inverse of `split`.
/// - Input(s): `N`.
/// - Output 0: average.
pub fn join<N>() -> An<Join<N, f64>>
where
    N: Size<f64>,
{
    super::prelude::join::<N, f64>()
}

/// Average `N` branches of `M` channels into one branch with `M` channels.
/// The input has `N` * `M` channels. Inverse of `multisplit::<M, N>`.
/// - Input(s): `N` * `M`.
/// - Output(s): `M`.
pub fn multijoin<M, N>() -> An<MultiJoin<M, N, f64>>
where
    M: Size<f64> + Mul<N>,
    N: Size<f64>,
    <M as Mul<N>>::Output: Size<f64>,
{
    super::prelude::multijoin::<M, N, f64>()
}

/// Stereo reverb.
/// `room_size` is in meters. An average room size is 10 meters.
/// `time` is approximate reverberation time to -60 dB in seconds.
/// - Input 0: left signal
/// - Input 1: right signal
/// - Output 0: reverberated left signal
/// - Output 1: reverberated right signal
///
/// ### Example: Add 20% Reverb
/// ```
/// use fundsp::hacker::*;
/// multipass() & 0.2 * reverb_stereo(10.0, 5.0);
/// ```
pub fn reverb_stereo(
    room_size: f64,
    time: f64,
) -> An<impl AudioNode<Sample = f64, Inputs = U2, Outputs = U2>> {
    super::prelude::reverb_stereo::<f64>(room_size, time)
}

/// Saw-like discrete summation formula oscillator.
/// - Input 0: frequency in Hz
/// - Input 1: roughness in 0...1 is the attenuation of successive partials.
/// - Output 0: DSF wave
pub fn dsf_saw() -> An<Dsf<f64, U2>> {
    An(Dsf::new(DEFAULT_SR, 1.0, 0.5))
}

/// Saw-like discrete summation formula oscillator.
/// Roughness in 0...1 is the attenuation of successive partials.
/// - Input 0: frequency in Hz
/// - Output 0: DSF wave
pub fn dsf_saw_r(roughness: f64) -> An<Dsf<f64, U1>> {
    An(Dsf::new(DEFAULT_SR, 1.0, roughness))
}

/// Square-like discrete summation formula oscillator.
/// - Input 0: frequency in Hz
/// - Input 1: roughness in 0...1 is the attenuation of successive partials.
/// - Output 0: DSF wave
pub fn dsf_square() -> An<Dsf<f64, U2>> {
    An(Dsf::new(DEFAULT_SR, 2.0, 0.5))
}

/// Square-like discrete summation formula oscillator.
/// Roughness in 0...1 is the attenuation of successive partials.
/// - Input 0: frequency in Hz
/// - Output 0: DSF wave
pub fn dsf_square_r(roughness: f64) -> An<Dsf<f64, U1>> {
    An(Dsf::new(DEFAULT_SR, 2.0, roughness))
}

/// Karplus-Strong plucked string oscillator with `frequency` in Hz.
/// High frequency damping is in 0...1.
/// Allocates: pluck buffer.
/// - Input 0: string excitation
/// - Output 0: oscillator output
///
/// ### Example
/// ```
/// use fundsp::hacker::*;
/// let node = zero() >> pluck(440.0, 0.5, 1.0);
/// ```
pub fn pluck(frequency: f64, gain_per_second: f64, high_frequency_damping: f64) -> An<Pluck<f64>> {
    An(Pluck::new(
        frequency,
        gain_per_second,
        high_frequency_damping,
    ))
}

/// Saw wavetable oscillator.
/// Allocates: global saw wavetable.
/// - Input 0: frequency in Hz
/// - Output 0: saw wave
pub fn saw() -> An<WaveSynth<'static, f64, U1>> {
    An(WaveSynth::new(DEFAULT_SR, &SAW_TABLE))
}

/// Square wavetable oscillator.
/// Allocates: global square wavetable.
/// - Input 0: frequency in Hz
/// - Output 0: square wave
pub fn square() -> An<WaveSynth<'static, f64, U1>> {
    An(WaveSynth::new(DEFAULT_SR, &SQUARE_TABLE))
}

/// Triangle wavetable oscillator.
/// Allocates: global triangle wavetable.
/// - Input 0: frequency in Hz
/// - Output 0: triangle wave
pub fn triangle() -> An<WaveSynth<'static, f64, U1>> {
    An(WaveSynth::new(DEFAULT_SR, &TRIANGLE_TABLE))
}

/// Organ wavetable oscillator. Emphasizes octave partials.
/// Allocates: global organ wavetable.
/// - Input 0: frequency in Hz
/// - Output 0: organ wave
pub fn organ() -> An<WaveSynth<'static, f64, U1>> {
    An(WaveSynth::new(DEFAULT_SR, &ORGAN_TABLE))
}

/// Soft saw wavetable oscillator.
/// Contains all partials, falls off like a triangle wave.
/// Allocates: global soft saw wavetable.
/// - Input 0: frequency in Hz
/// - Output 0: soft saw wave
pub fn soft_saw() -> An<WaveSynth<'static, f64, U1>> {
    An(WaveSynth::new(DEFAULT_SR, &SOFT_SAW_TABLE))
}

/// Hammond wavetable oscillator. Emphasizes first three partials.
/// Allocates: global Hammond wavetable.
/// - Input 0: frequency in Hz
/// - Output 0: Hammond wave
pub fn hammond() -> An<WaveSynth<'static, f64, U1>> {
    An(WaveSynth::new(DEFAULT_SR, &HAMMOND_TABLE))
}

/// Fixed saw wavetable oscillator at `f` Hz.
/// Allocates: global saw wavetable.
/// - Output 0: saw wave
pub fn saw_hz(f: f64) -> An<Pipe<f64, Constant<U1, f64>, WaveSynth<'static, f64, U1>>> {
    super::prelude::saw_hz(f)
}

/// Fixed square wavetable oscillator at `f` Hz.
/// Allocates: global square wavetable.
/// - Output 0: square wave
pub fn square_hz(f: f64) -> An<Pipe<f64, Constant<U1, f64>, WaveSynth<'static, f64, U1>>> {
    super::prelude::square_hz(f)
}

/// Fixed triangle wavetable oscillator at `f` Hz.
/// Allocates: global triangle wavetable.
/// - Output 0: triangle wave
pub fn triangle_hz(f: f64) -> An<Pipe<f64, Constant<U1, f64>, WaveSynth<'static, f64, U1>>> {
    super::prelude::triangle_hz(f)
}

/// Fixed organ wavetable oscillator at `f` Hz. Emphasizes octave partials.
/// Allocates: global organ wavetable.
/// - Output 0: organ wave
pub fn organ_hz(f: f64) -> An<Pipe<f64, Constant<U1, f64>, WaveSynth<'static, f64, U1>>> {
    constant(f) >> organ()
}

/// Fixed soft saw wavetable oscillator at `f` Hz.
/// Contains all partials, falls off like a triangle wave.
/// Allocates: global soft saw wavetable.
/// - Output 0: soft saw wave
pub fn soft_saw_hz(f: f64) -> An<Pipe<f64, Constant<U1, f64>, WaveSynth<'static, f64, U1>>> {
    constant(f) >> soft_saw()
}

/// Fixed Hammond wavetable oscillator at `f` Hz. Emphasizes first three partials.
/// Allocates: global Hammond wavetable.
/// - Output 0: Hammond wave
pub fn hammond_hz(f: f64) -> An<Pipe<f64, Constant<U1, f64>, WaveSynth<'static, f64, U1>>> {
    constant(f) >> hammond()
}

/// Lowpass filter.
/// - Input 0: audio
/// - Input 1: cutoff frequency (Hz)
/// - Input 2: Q
/// - Output 0: filtered audio
pub fn lowpass() -> An<Svf<f64, f64, LowpassMode<f64>>> {
    super::prelude::lowpass()
}

/// Lowpass filter with cutoff frequency `f` Hz with Q value `q`.
/// - Input 0: audio
/// - Output 0: filtered audio
pub fn lowpass_hz(f: f64, q: f64) -> An<FixedSvf<f64, f64, LowpassMode<f64>>> {
    super::prelude::lowpass_hz::<f64, f64>(f, q)
}

/// Lowpass filter with Q value `q`.
/// - Input 0: audio
/// - Input 1: cutoff frequency (Hz)
/// - Output 0: filtered audio
pub fn lowpass_q(
    q: f64,
) -> An<Pipe<f64, Stack<f64, MultiPass<U2, f64>, Constant<U1, f64>>, Svf<f64, f64, LowpassMode<f64>>>>
{
    super::prelude::lowpass_q::<f64, f64>(q)
}

/// Highpass filter.
/// - Input 0: audio
/// - Input 1: cutoff frequency (Hz)
/// - Input 2: Q
/// - Output 0: filtered audio
pub fn highpass() -> An<Svf<f64, f64, HighpassMode<f64>>> {
    super::prelude::highpass()
}

/// Highpass filter with cutoff frequency `f` Hz with Q value `q`.
/// - Input 0: audio
/// - Output 0: filtered audio
pub fn highpass_hz(f: f64, q: f64) -> An<FixedSvf<f64, f64, HighpassMode<f64>>> {
    super::prelude::highpass_hz::<f64, f64>(f, q)
}

/// Highpass filter with Q value `q`.
/// - Input 0: audio
/// - Input 1: cutoff frequency (Hz)
/// - Output 0: filtered audio
pub fn highpass_q(
    q: f64,
) -> An<
    Pipe<f64, Stack<f64, MultiPass<U2, f64>, Constant<U1, f64>>, Svf<f64, f64, HighpassMode<f64>>>,
> {
    super::prelude::highpass_q::<f64, f64>(q)
}

/// Bandpass filter.
/// - Input 0: audio
/// - Input 1: center frequency (Hz)
/// - Input 2: Q
/// - Output 0: filtered audio
pub fn bandpass() -> An<Svf<f64, f64, BandpassMode<f64>>> {
    super::prelude::bandpass()
}

/// Bandpass filter centered at `f` Hz with Q value `q`.
/// - Input 0: audio
/// - Output 0: filtered audio
pub fn bandpass_hz(f: f64, q: f64) -> An<FixedSvf<f64, f64, BandpassMode<f64>>> {
    super::prelude::bandpass_hz::<f64, f64>(f, q)
}

/// Bandpass filter with Q value `q`.
/// - Input 0: audio
/// - Input 1: center frequency (Hz)
/// - Output 0: filtered audio
pub fn bandpass_q(
    q: f64,
) -> An<
    Pipe<f64, Stack<f64, MultiPass<U2, f64>, Constant<U1, f64>>, Svf<f64, f64, BandpassMode<f64>>>,
> {
    super::prelude::bandpass_q::<f64, f64>(q)
}

/// Notch filter.
/// - Input 0: audio
/// - Input 1: center frequency (Hz)
/// - Input 2: Q
/// - Output 0: filtered audio
pub fn notch() -> An<Svf<f64, f64, NotchMode<f64>>> {
    super::prelude::notch()
}

/// Notch filter centered at `f` Hz with Q value `q`.
/// - Input 0: audio
/// - Output 0: filtered audio
pub fn notch_hz(f: f64, q: f64) -> An<FixedSvf<f64, f64, NotchMode<f64>>> {
    super::prelude::notch_hz::<f64, f64>(f, q)
}

/// Notch filter with Q value `q`.
/// - Input 0: audio
/// - Input 1: center frequency (Hz)
/// - Output 0: filtered audio
pub fn notch_q(
    q: f64,
) -> An<Pipe<f64, Stack<f64, MultiPass<U2, f64>, Constant<U1, f64>>, Svf<f64, f64, NotchMode<f64>>>>
{
    super::prelude::notch_q::<f64, f64>(q)
}

/// Peaking filter.
/// - Input 0: audio
/// - Input 1: center frequency (Hz)
/// - Input 2: Q
/// - Output 0: filtered audio
pub fn peak() -> An<Svf<f64, f64, PeakMode<f64>>> {
    super::prelude::peak()
}

/// Peaking filter centered at `f` Hz with Q value `q`.
/// - Input 0: audio
/// - Output 0: filtered audio
pub fn peak_hz(f: f64, q: f64) -> An<FixedSvf<f64, f64, PeakMode<f64>>> {
    super::prelude::peak_hz::<f64, f64>(f, q)
}

/// Peaking filter with Q value `q`.
/// - Input 0: audio
/// - Input 1: center frequency (Hz)
/// - Output 0: filtered audio
pub fn peak_q(
    q: f64,
) -> An<Pipe<f64, Stack<f64, MultiPass<U2, f64>, Constant<U1, f64>>, Svf<f64, f64, PeakMode<f64>>>>
{
    super::prelude::peak_q::<f64, f64>(q)
}

/// Allpass filter.
/// - Input 0: audio
/// - Input 1: center frequency (Hz)
/// - Input 2: Q
/// - Output 0: filtered audio
pub fn allpass() -> An<Svf<f64, f64, AllpassMode<f64>>> {
    super::prelude::allpass()
}

/// Allpass filter centered at `f` Hz with Q value `q`.
/// - Input 0: audio
/// - Output 0: filtered audio
pub fn allpass_hz(f: f64, q: f64) -> An<FixedSvf<f64, f64, AllpassMode<f64>>> {
    super::prelude::allpass_hz::<f64, f64>(f, q)
}

/// Allpass filter with Q value `q`.
/// - Input 0: audio
/// - Input 1: center frequency (Hz)
/// - Output 0: filtered audio
pub fn allpass_q(
    q: f64,
) -> An<Pipe<f64, Stack<f64, MultiPass<U2, f64>, Constant<U1, f64>>, Svf<f64, f64, AllpassMode<f64>>>>
{
    super::prelude::allpass_q::<f64, f64>(q)
}

/// Bell filter with adjustable gain.
/// - Input 0: audio
/// - Input 1: center frequency (Hz)
/// - Input 2: Q
/// - Input 3: amplitude gain
/// - Output 0: filtered audio
pub fn bell() -> An<Svf<f64, f64, BellMode<f64>>> {
    super::prelude::bell()
}

/// Bell filter centered at `f` Hz with Q value `q` and amplitude gain `gain`.
/// - Input 0: audio
/// - Output 0: filtered audio
pub fn bell_hz(f: f64, q: f64, gain: f64) -> An<FixedSvf<f64, f64, BellMode<f64>>> {
    super::prelude::bell_hz::<f64, f64>(f, q, gain)
}

/// Bell filter with Q value `q` and amplitude gain `gain`.
/// - Input 0: audio
/// - Input 1: center frequency
/// - Output 0: filtered audio
pub fn bell_q(
    q: f64,
    gain: f64,
) -> An<Pipe<f64, Stack<f64, MultiPass<U2, f64>, Constant<U2, f64>>, Svf<f64, f64, BellMode<f64>>>>
{
    super::prelude::bell_q::<f64, f64>(q, gain)
}

/// Low shelf filter with adjustable gain.
/// - Input 0: audio
/// - Input 1: center frequency (Hz)
/// - Input 2: Q
/// - Input 3: amplitude gain
/// - Output 0: filtered audio
pub fn lowshelf() -> An<Svf<f64, f64, LowshelfMode<f64>>> {
    super::prelude::lowshelf()
}

/// Low shelf filter centered at `f` Hz with Q value `q` and amplitude gain `gain`.
/// - Input 0: audio
/// - Output 0: filtered audio
pub fn lowshelf_hz(f: f64, q: f64, gain: f64) -> An<FixedSvf<f64, f64, LowshelfMode<f64>>> {
    super::prelude::lowshelf_hz::<f64, f64>(f, q, gain)
}

/// Low shelf filter with Q value `q` and amplitude gain `gain`.
/// - Input 0: audio
/// - Input 1: cutoff frequency
/// - Output 0: filtered audio
pub fn lowshelf_q(
    q: f64,
    gain: f64,
) -> An<
    Pipe<f64, Stack<f64, MultiPass<U2, f64>, Constant<U2, f64>>, Svf<f64, f64, LowshelfMode<f64>>>,
> {
    super::prelude::lowshelf_q::<f64, f64>(q, gain)
}

/// High shelf filter with adjustable gain.
/// - Input 0: audio
/// - Input 1: center frequency (Hz)
/// - Input 2: Q
/// - Input 3: amplitude gain
/// - Output 0: filtered audio
pub fn highshelf() -> An<Svf<f64, f64, HighshelfMode<f64>>> {
    super::prelude::highshelf()
}

/// High shelf filter centered at `cutoff` Hz with Q value `q` and amplitude gain `gain`.
/// - Input 0: audio
/// - Output 0: filtered audio
pub fn highshelf_hz(f: f64, q: f64, gain: f64) -> An<FixedSvf<f64, f64, HighshelfMode<f64>>> {
    super::prelude::highshelf_hz::<f64, f64>(f, q, gain)
}

/// High shelf filter with Q value `q` and amplitude gain `gain`.
/// - Input 0: audio
/// - Input 1: cutoff frequency
/// - Output 0: filtered audio
pub fn highshelf_q(
    q: f64,
    gain: f64,
) -> An<
    Pipe<f64, Stack<f64, MultiPass<U2, f64>, Constant<U2, f64>>, Svf<f64, f64, HighshelfMode<f64>>>,
> {
    super::prelude::highshelf_q::<f64, f64>(q, gain)
}

/// Resonant two-pole lowpass filter.
/// - Input 0: audio
/// - Input 1: cutoff frequency
/// - Input 2: Q
/// - Output 0: filtered audio
pub fn lowrez() -> An<Rez<f64, f64, U3>> {
    An(Rez::new(0.0, 440.0, 1.0))
}

/// Resonant two-pole lowpass filter with fixed cutoff frequency and Q.
/// - Input 0: audio
/// - Output 0: filtered audio
pub fn lowrez_hz(cutoff: f64, q: f64) -> An<Rez<f64, f64, U1>> {
    An(Rez::new(0.0, cutoff, q))
}

/// Resonant two-pole lowpass filter with fixed Q.
/// - Input 0: audio
/// - Input 1: cutoff frequency
/// - Output 0: filtered audio
pub fn lowrez_q(
    q: f64,
) -> An<Pipe<f64, Stack<f64, MultiPass<U2, f64>, Constant<U1, f64>>, Rez<f64, f64, U3>>> {
    (multipass::<U2>() | dc(q)) >> lowrez()
}

/// Resonant two-pole bandpass filter.
/// - Input 0: audio
/// - Input 1: center frequency
/// - Input 2: Q
/// - Output 0: filtered audio
pub fn bandrez() -> An<Rez<f64, f64, U3>> {
    An(Rez::new(1.0, 440.0, 1.0))
}

/// Resonant two-pole bandpass filter with fixed center frequency and Q.
/// - Input 0: audio
/// - Output 0: filtered audio
pub fn bandrez_hz(center: f64, q: f64) -> An<Rez<f64, f64, U1>> {
    An(Rez::new(1.0, center, q))
}

/// Resonant two-pole bandpass filter with fixed Q.
/// - Input 0: audio
/// - Input 1: center frequency
/// - Output 0: filtered audio
pub fn bandrez_q(
    q: f64,
) -> An<Pipe<f64, Stack<f64, MultiPass<U2, f64>, Constant<U1, f64>>, Rez<f64, f64, U3>>> {
    (multipass::<U2>() | dc(q)) >> bandrez()
}

/// Pulse wave oscillator.
/// - Input 0: frequency in Hz
/// - Input 1: pulse duty cycle in 0...1
/// - Output 0: pulse wave
pub fn pulse() -> An<super::prelude::PulseWave<f64>> {
    super::prelude::pulse()
}

/// Play back a channel of a Wave64.
/// Optional loop point is the index to jump to at the end of the wave.
/// - Output 0: wave
pub fn wave64(
    wave: &Arc<Wave64>,
    channel: usize,
    loop_point: Option<usize>,
) -> An<Wave64Player<f64>> {
    An(Wave64Player::new(
        wave,
        channel,
        0,
        wave.length(),
        loop_point,
    ))
}

/// Play back a channel of a Wave64 starting from sample `start_point`, inclusive,
/// and ending at sample `end_point`, exclusive.
/// Optional loop point is the index to jump to at the end point.
///
/// ### Example: One-Shot Playback
/// ```
/// use fundsp::hacker::*;
/// let wave = std::sync::Arc::new(Wave64::render(44100.0, 1.0, &mut (white())));
/// let player = wave64_at(&wave, 0, 0, wave.length(), None);
/// ```
/// - Output 0: wave
pub fn wave64_at(
    wave: &Arc<Wave64>,
    channel: usize,
    start_point: usize,
    end_point: usize,
    loop_point: Option<usize>,
) -> An<Wave64Player<f64>> {
    An(Wave64Player::new(
        wave,
        channel,
        start_point,
        end_point,
        loop_point,
    ))
}

/// Play back a channel of a Wave32.
/// Optional loop point is the index to jump to at the end of the wave.
/// - Output 0: wave
pub fn wave32(
    wave: &Arc<Wave32>,
    channel: usize,
    loop_point: Option<usize>,
) -> An<Wave32Player<f64>> {
    An(Wave32Player::new(
        wave,
        channel,
        0,
        wave.length(),
        loop_point,
    ))
}

/// Play back a channel of a Wave32 starting from sample `start_point`, inclusive,
/// and ending at sample `end_point`, exclusive.
/// Optional loop point is the index to jump to at the end.
///
/// ### Example: Looping Playback
/// ```
/// use fundsp::hacker::*;
/// let wave = std::sync::Arc::new(Wave32::render(44100.0, 1.0, &mut (fundsp::hacker32::white())));
/// let player = wave32_at(&wave, 0, 0, wave.length(), Some(0));
/// ```
/// - Output 0: wave
pub fn wave32_at(
    wave: &Arc<Wave32>,
    channel: usize,
    start_point: usize,
    end_point: usize,
    loop_point: Option<usize>,
) -> An<Wave32Player<f64>> {
    An(Wave32Player::new(
        wave,
        channel,
        start_point,
        end_point,
        loop_point,
    ))
}

/// Mono chorus, 5 voices. For stereo, stack two of these using different seed values.
/// `seed`: LFO seed.
/// `separation`: base voice separation in seconds (for example, 0.015).
/// `variation`: delay variation in seconds (for example, 0.005).
/// `mod_frequency`: delay modulation frequency (for example, 0.2).
/// - Input 0: audio.
/// - Output 0: chorused audio, including original signal.
///
/// ### Example: Chorused Saw Wave
/// ```
/// use fundsp::hacker::*;
/// saw_hz(110.0) >> chorus(0, 0.015, 0.005, 0.5);
/// ```
pub fn chorus(
    seed: i64,
    separation: f64,
    variation: f64,
    mod_frequency: f64,
) -> An<impl AudioNode<Sample = f64, Inputs = U1, Outputs = U1>> {
    super::prelude::chorus::<f64>(seed, separation, variation, mod_frequency)
}

/// Mono flanger.
/// `feedback_amount`: amount of feedback (for example, 0.9 or -0.9). Negative feedback inverts feedback phase.
/// `minimum_delay`: minimum delay in seconds (for example, 0.005).
/// `maximum_delay`: maximum delay in seconds (for example, 0.015).
/// ´delay_f´: Delay in `minimum_delay`...`maximum_delay` as a function of time. For example, `|t| lerp11(0.005, 0.015, sin_hz(0.1, t))`.
/// - Input 0: audio
/// - Output 0: flanged audio, including original signal
///
/// ### Example: Flanged Saw Wave
/// ```
/// use fundsp::hacker::*;
/// saw_hz(110.0) >> flanger(0.5, 0.005, 0.010, |t| lerp11(0.005, 0.010, sin_hz(0.1, t)));
/// ```
pub fn flanger(
    feedback_amount: f64,
    minimum_delay: f64,
    maximum_delay: f64,
    delay_f: impl Fn(f64) -> f64 + Clone + Send + Sync,
) -> An<impl AudioNode<Sample = f64, Inputs = U1, Outputs = U1>> {
    super::prelude::flanger::<f64, _>(feedback_amount, minimum_delay, maximum_delay, delay_f)
}

/// Mono phaser.
/// `feedback_amount`: amount of feedback (for example, 0.5). Negative feedback inverts feedback phase.
/// `phase_f`: allpass modulation value in 0...1 as function of time, for example `|t| sin_hz(0.1, t) * 0.5 + 0.5`.
/// - Input 0: audio
/// - Output 0: phased audio
///
/// ### Example: Phased Saw Wave
/// ```
/// use fundsp::hacker::*;
/// saw_hz(110.0) >> phaser(0.5, |t| sin_hz(0.1, t) * 0.5 + 0.5);
/// ```
pub fn phaser<X: Fn(f64) -> f64 + Clone + Send + Sync>(
    feedback_amount: f64,
    phase_f: X,
) -> An<impl AudioNode<Sample = f64, Inputs = U1, Outputs = U1>> {
    super::prelude::phaser::<f64, _>(feedback_amount, phase_f)
}

/// Shared float variable. Can be read from and written to from multiple threads.
///
/// ### Example: Add Chorus With Wetness Control
/// ```
/// use fundsp::hacker::*;
/// let wet = shared(0.2);
/// pass() & var(&wet) * chorus(0, 0.015, 0.005, 0.5);
/// ```
pub fn shared(value: f64) -> Shared<f64> {
    Shared::new(value)
}

/// Outputs the value of the shared variable.
/// - Output 0: value
///
/// ### Example: Add Chorus With Wetness Control
/// ```
/// use fundsp::hacker::*;
/// let wet = shared(0.2);
/// pass() & var(&wet) * chorus(0, 0.015, 0.005, 0.5);
/// ```
pub fn var(shared: &Shared<f64>) -> An<Var<f64>> {
    An(Var::new(shared))
}

/// Shared variable mapped through a function.
/// Outputs the value of the function, which may be scalar or tuple.
/// - Outputs: value
///
/// ### Example: Control Pitch In MIDI Semitones With Smoothing
/// ```
/// use fundsp::hacker::*;
/// let pitch = shared(69.0);
/// var_fn(&pitch, |x| midi_hz(x)) >> follow(0.01) >> saw();
/// ```
pub fn var_fn<F, R>(shared: &Shared<f64>, f: F) -> An<VarFn<f64, F, R>>
where
    F: Clone + Fn(f64) -> R + Send + Sync,
    R: ConstantFrame<Sample = f64>,
    R::Size: Size<f64>,
{
    An(VarFn::new(shared, f))
}

/// Timer node. A node with no inputs or outputs that maintains
/// current stream time in a shared variable.
/// It can be added to any node by stacking.
///
/// ### Example: Timer And LFO
/// ```
/// use fundsp::hacker::*;
/// let time = shared(0.0);
/// timer(&time) | lfo(|t: f64| 1.0 / (1.0 + t));
/// ```
pub fn timer(shared: &Shared<f64>) -> An<Timer<f64>> {
    An(Timer::new(DEFAULT_SR, shared))
}

/// Snoop node for sharing audio data with a frontend thread.
/// The latest samples buffer has room for at least `capacity` samples.
/// Returns (frontend, backend).
/// - Input 0: signal to snoop.
/// - Output 0: signal passed through.
pub fn snoop(capacity: usize) -> (Snoop<f64>, An<SnoopBackend<f64>>) {
    let (snoop, backend) = Snoop::new(capacity);
    (snoop, An(backend))
}
