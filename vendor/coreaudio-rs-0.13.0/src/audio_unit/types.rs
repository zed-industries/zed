//! Core Audio's various const audio unit types identifiers represented as typesafe enums.
//!
//! Original documentation [here](https://developer.apple.com/library/prerelease/mac/documentation/AudioUnit/Reference/AUComponentServicesReference/index.html#//apple_ref/doc/constant_group/Audio_Unit_Types).

/// Represents the different kinds of Audio Units that are available.
///
/// Original documentation [here](https://developer.apple.com/library/prerelease/mac/documentation/AudioUnit/Reference/AUComponentServicesReference/index.html#//apple_ref/doc/constant_group/Audio_Unit_Types).
#[derive(Copy, Clone, Debug)]
pub enum Type {
    /// Provides input, output, or both input and output simultaneously.
    ///
    /// It can be used as the head of an audio unit processing graph.
    ///
    /// **Available** in OS X v10.2 and later.
    IO(IOType),
    /// An instrument unit can be used as a software musical instrument, such as a sampler or
    /// synthesizer.
    ///
    /// It responds to MIDI (Musical Instrument Digital Interface) control signals and can create
    /// notes.
    ///
    /// **Available** in OS X v10.2 and later.
    MusicDevice(MusicDeviceType),
    /// An effect unit that can respond to MIDI control messages, typically through a mapping of
    /// MIDI messages to parameters of the audio unit's DSP algorithm.
    ///
    /// **Available** in OS X v10.2 and later.
    MusicEffect,
    /// A format converter unit can transform audio formats, such as performing sample rate
    /// conversion.
    ///
    /// A format converter is also appropriate for deferred rendering and for effects such as
    /// varispeed.
    ///
    /// A format converter unit can ask for as much or as little audio input as it needs to produce
    /// a given output, while still completing its rendering within the time represented by the
    /// output buffer.
    ///
    /// For effect-like format converters, such as pitch shifters, it is common to provide both a
    /// real-time and an offline version. OS X, for example, includes Time-Pitch and Varispeed
    /// audio units in both real-time and offline versions.
    ///
    /// **Available** in OS X v10.2 and later.
    FormatConverter(FormatConverterType),
    /// An effect unit repeatedly processes a number of audio input samples to produce the same
    /// number of audio output samples.
    ///
    /// Most commonly, an effect unit has a single input and a single output.
    ///
    /// Some effects take side-chain inputs as well.
    ///
    /// Effect units can be run offline, such as to process a file without playing it, but are
    /// expected to run in real-time.
    ///
    /// **Available** in OS X v10.2 and later.
    Effect(EffectType),
    /// A mixer unit takes a number of input channels and mixes them to provide one or more output
    /// channels.
    ///
    /// For example, the **StereoMixer** **SubType** in OS X takes multiple mono or stereo inputs
    /// and produces a single stereo output.
    ///
    /// **Available** in OS X v10.2 and later.
    Mixer(MixerType),
    /// A panner unit is a specialised effect unit that distributes one or more channels in a
    /// single input to one or more channels in a single output.
    ///
    /// Panner units must support a set of standard audio unit parameters that specify panning
    /// coordinates.
    ///
    /// **Available** in OS X v10.3 and later.
    Panner,
    /// A generator unit provides audio output that has no audio input.
    ///
    /// This audio unit type is appropriate for a tone generator.
    ///
    /// Unlike an instrument unit, a generator unit does not have a control input.
    ///
    /// **Available** in OS X v10.3 and later.
    Generator(GeneratorType),
    /// An offline effect unit provides digital signal processing of a sort that cannot proceed in
    /// real-time.
    ///
    /// For example, level normalisation requires examination of an entire sound, beginning to end,
    /// before the normalisation factor can be calculated.
    ///
    /// As such, offline effect units also have a notion of a priming stage that can be performed
    /// before the actual rendering/processing phase is executed.
    ///
    /// **Available** in OS X v10.3 and later.
    OfflineEffect,
    /// FIXME: Could not find any documenation for this type - it seems it was added very recently
    /// (around 2013) and Apple's documentation doesn't seem to have updated to include it.
    MidiProcessor,
}

impl Type {
    /// Convert the `Type` to its associated `u32` for compatibility with original API.
    pub fn as_u32(&self) -> u32 {
        match *self {
            Type::IO(_) => 1635086197,
            Type::MusicDevice(_) => 1635085685,
            Type::MusicEffect => 1635085670,
            Type::FormatConverter(_) => 1635083875,
            Type::Effect(_) => 1635083896,
            Type::Mixer(_) => 1635085688,
            Type::Panner => 1635086446,
            Type::Generator(_) => 1635084142,
            Type::OfflineEffect => 1635086188,
            Type::MidiProcessor => 1635085673,
        }
    }

    /// Convert the `Type` to the const `u32` that is associated with its subtype.
    pub fn as_subtype_u32(&self) -> Option<u32> {
        match *self {
            Type::IO(ty) => Some(ty as u32),
            Type::MusicDevice(ty) => Some(ty as u32),
            Type::FormatConverter(ty) => Some(ty as u32),
            Type::Effect(ty) => Some(ty as u32),
            Type::Mixer(ty) => Some(ty as u32),
            Type::Generator(ty) => Some(ty as u32),
            _ => None,
        }
    }
}

impl From<EffectType> for Type {
    fn from(ty: EffectType) -> Self {
        Type::Effect(ty)
    }
}

impl From<FormatConverterType> for Type {
    fn from(ty: FormatConverterType) -> Self {
        Type::FormatConverter(ty)
    }
}

impl From<MixerType> for Type {
    fn from(ty: MixerType) -> Self {
        Type::Mixer(ty)
    }
}

impl From<GeneratorType> for Type {
    fn from(ty: GeneratorType) -> Self {
        Type::Generator(ty)
    }
}

impl From<MusicDeviceType> for Type {
    fn from(ty: MusicDeviceType) -> Self {
        Type::MusicDevice(ty)
    }
}

impl From<IOType> for Type {
    fn from(ty: IOType) -> Self {
        Type::IO(ty)
    }
}

/// Effect (digital signal processing) audio unit subtypes for audio units provided by Apple.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EffectType {
    /// An audio unit that enforces an upper dynamic limit on an audio signal.
    ///
    /// **Available** in OS X v10.2 and later.
    PeakLimiter = 1819112562,
    /// An audio unit that provides dynamic compression or expansion.
    ///
    /// **Available** in OS X v10.3 and later.
    DynamicsProcessor = 1684237680,
    /// An audio unit that passes frequencies below a specified cutoff frequency and blocks
    /// frequencies above that cutoff frequency.
    ///
    /// **Available** in OS X v10.2 and later.
    LowPassFilter = 1819304307,
    /// An audio unit that passes frequencies above a specified cutoff frequency and blocks
    /// frequencies below that cutoff frequency.
    ///
    /// **Available** in OS X v10.2 and later.
    HighPassFilter = 1752195443,
    /// An audio unit that passes frequencies between specified upper and lower cutoff frequencies,
    /// and blocks frequencies outside that band.
    ///
    /// **Available** in OS X v10.2 and later.
    BandPassFilter = 1651532147,
    /// An audio unit suitable for implementing a treble control in an audio playback or recording
    /// system.
    ///
    /// **Available** in OS X v10.2 and later.
    HighShelfFilter = 1752393830,
    /// An audio unit suitable for implementing a bass control in an audio playback or recording
    /// system.
    ///
    /// **Available** in OS X v10.2 and later.
    LowShelfFilter = 1819502694,
    /// An audio unit that provides a filter whose center frequency, boost/cut level, and Q can be
    /// adjusted.
    ///
    /// **Available** in OS X v10.2 and later.
    ParametricEQ = 1886217585,
    /// An audio unit that provides a distortion effect.
    ///
    /// **Available** in OS X v10.5 and later.
    Distortion = 1684632436,
    /// An audio unit that introduces a time delay to a signal.
    ///
    /// **Available** in OS X v10.2 and later.
    Delay = 1684368505,
    /// An audio unit that provides a time delay for a specified number of samples.
    ///
    /// **Available** in OS X v10.4 and later.
    SampleDelay = 1935961209,
    /// An audio unit that provides a 10- or 31-band graphic equalizer.
    ///
    /// Available in OS X v10.2 and later.
    GraphicEQ = 1735550321,
    /// An audio unit that provides four-bands of dynamic compression or expansion.
    ///
    /// **Available** in OS X v10.3 and later.
    MultiBandCompressor = 1835232624,
    /// An audio unit that provides a reverberation effect that can be used to simulate a variety
    /// of acoustic spaces.
    ///
    /// **Available** in OS X v10.2 and later.
    MatrixReverb = 1836213622,
    /// An audio unit for modifying the pitch of a signal.
    ///
    /// **Available** in OS X v10.4 and later.
    Pitch = 1953329268,
    /// An audio unit that provides a combination of five filters: low-frequency, three
    /// mid-frequencies, and high-frequency.
    ///
    /// **Available** in OS X v10.4 and later.
    AUFilter = 1718185076,
    /// An audio unit for use in conjunction with a kAudioUnitSubType_NetReceive audio unit for
    /// sending audio across a network or from one application to another.
    ///
    /// **Available** in OS X v10.4 and later.
    NetSend = 1853058660,
    /// An audio unit that detects gaps between segments of speech and fills the gaps with a short
    /// tone, simulating the sound of a walkie-talkie communication device.
    ///
    /// **Available** in OS X v10.5 and later.
    RogerBeep = 1919903602,
    /// A multi-band equalizer with specifiable filter type for each band.
    ///
    /// **Available** in OS X v10.9 and later.
    NBandEQ = 1851942257,
}

/// Audio data format converter audio unit subtypes for **AudioUnit**s provided by Apple.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FormatConverterType {
    /// An audio unit that uses an audio converter to do linear PCM conversions, such as changes to
    /// sample rate, bit depth, or interleaving.
    ///
    /// **Available** in OS X v10.2 and later.
    AUConverter = 1668247158,
    /// An audio unit that can be used to have independent control of both playback rate and pitch.
    ///
    /// In OS X it provides a generic view, so it can be used in both a UI and programmatic
    /// context.
    ///
    /// It also comes in an offline version for processing audio files.
    ///
    /// **Available** in OS X v10.7 and later.
    NewTimePitch = 1853191280,
    /// An audio unit that can provide independent control of playback rate and pitch. This subtype
    /// provides a generic view, making it suitable for UI and programmatic context. OS X provides
    /// realtime and offline audio units of this subtype.
    ///
    /// **Available** in OS X v10.3 and later.
    TimePitch = 1953329268,
    /// An audio unit that acquires audio input from a separate thread than the thread on which its
    /// render method is called.
    ///
    /// You can use this subtype to introduce multiple threads into an audio unit processing graph.
    ///
    /// There is a delay, equal to the buffer size, introduced between the audio input and output.
    ///
    /// **Available** in OS X v10.4 and later.
    DeferredRenderer = 1684366962,
    /// An audio unit with one input bus and two output buses. The audio unit duplicates the input
    /// signal to each of its two output buses.
    ///
    /// **Available** in OS X v10.4 and later.
    Splitter = 1936747636,
    /// An audio unit with two input buses and one output bus. The audio unit merges the two input
    /// signals to the single output.
    ///
    /// **Available** in OS X v10.4 and later.
    Merger = 1835364967,
    /// An audio unit that can control playback rate. As the playback rate increases, so does
    /// pitch.
    ///
    /// This subtype provides a generic view, making it suitable for UI and programmatic context.
    ///
    /// OS X provides realtime and offline audio units of this subtype.
    ///
    /// **Available** in OS X v10.3 and later.
    Varispeed = 1986097769,
    /// **Available** in OS X v10.9 and later.
    AUiPodTimeOther = 1768977519,
}

/// Audio mixing **AudioUnit** subtypes for **AudioUnit**s provided by Apple.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MixerType {
    /// An audio unit that can have any number of input buses, with any number of channels on each
    /// input bus, and one output bus.
    ///
    /// In OS X, the output bus can have any number of channels.
    ///
    /// In iPhone OS, the output bus always has two channels.
    ///
    /// **Available** in OS X v10.5 and later.
    MultiChannelMixer = 1835232632,
    /// An audio unit that can have any number of input buses, each of which is mono or stereo, and
    /// one stereo output bus.
    ///
    /// **Available** in OS X v10.2 and later.
    StereoMixer = 1936554098,
    /// An audio unit that can have any number of input buses and one output bus.
    ///
    /// Each input bus can be mono, in which case it can be panned using 3D coordinates and
    /// parameters.
    ///
    /// Stereo input buses pass directly through to the output.
    ///
    /// Four-channel ambisonic inputs are rendered to the output configuration.
    ///
    /// The single output bus can be configured with 2, 4, 5, 6, 7 or 8 channels.
    ///
    /// **Available** in OS X v10.3 and later.
    ///
    /// **Deprecated** in OS X v10.10.
    Mixer3D = 862219640,
    /// An audio unit that can have any number of input and output buses with any number of
    /// channels on each bus.
    ///
    /// You configure the mix using a matrix of channels with a separate input level control for
    /// each channel.
    ///
    /// The audio unit also provides individual level control for each
    /// input-channel-to-output-channel combination, as well as level control for each output
    /// channel.
    ///
    /// Finally, the audio unit provides a global level control for the matrix as a whole.
    ///
    /// **Available** in OS X v10.3 and later.
    MatrixMixer = 1836608888,
}

/// Audio units that serve as sound sources.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GeneratorType {
    /// A generator unit that can be used to schedule slices of audio to be played at specified
    /// times.
    ///
    /// The audio is scheduled using the time stamps for the render operation and can be scheduled
    /// from any thread.
    ///
    /// **Available** in OS X v10.4 and later.
    ScheduledSoundPlayer = 1936945260,
    /// A generator unit that is used to play a file. In OS X it presents a custom UI so can be
    /// used in a UI context as well as in a programmatic context.
    ///
    /// **Available** in OS X v10.4 and later.
    AudioFilePlayer = 1634103404,
}

/// Audio units that can be played as musical instruments via MIDI control.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MusicDeviceType {
    /// A multitimbral instrument unit that can use sample banks in either DLS or SoundFont
    /// formats.
    ///
    /// It fully supports GM-MIDI and the basic extensions of GS-MIDI
    ///
    /// **Available** in OS X v10.2 and later.
    DLSSynth = 1684828960,
    /// A monotimbral instrument unit that functions a a sampler-synthesizer and supports full
    /// interactive editing of its state.
    ///
    /// **Available** in OS X v10.7 and later.
    Sampler = 1935764848,
}

/// Input/output **AudioUnit** subtypes for **AudioUnit**s provided by Apple.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum IOType {
    /// An audio unit that responds to start/stop calls and provides basic services for converting
    /// to and from linear PCM formats.
    ///
    /// Use this audio unit when sending the output of an audio processing graph to your
    /// application rather than to the output audio hardware. You would typically use the Generic
    /// Output unit for offline audio processing. Just like the other I/O units, the Generic Output
    /// unit incorporates a Format Converter unit. This lets the Generic Output unit perform format
    /// conversion between the stream format used in an audio processing graph and the format you
    /// want.
    ///
    /// You can also use a Generic Output unit as the final node in a subgraph that you place into
    /// a parent audio processing graph.
    ///
    /// **Available** in OS X v10.2 and later.
    GenericOutput = 1734700658,
    /// An audio unit that can provides input/output connection to an a specified audio device.
    ///
    /// Bus 0 provides output to the audio device and bus 1 accepts input from the audio device.
    ///
    /// **Available** in OS X v10.2 and later.
    HalOutput = 1634230636,
    /// A specialized **HalOutput** audio unit that connects to the user’s selected default device
    /// in Sound Preferences.
    ///
    /// **Available** in OS X v10.2 and later.
    DefaultOutput = 1684366880,
    /// A specialized **HalOutput** audio unit that connects to the user’s selected device for
    /// sound effects, alerts, and other user-interface sounds.
    ///
    /// **Available** in OS X v10.2 and later.
    SystemOutput = 1937339168,
    /// An audio unit that interfaces to the audio inputs and outputs of iPhone OS devices and
    /// provides voice processing features.
    ///
    /// Bus 0 provides output to hardware and bus 1 accepts input from hardware.
    ///
    /// See the [Voice-Processing I/O Audio Unit
    /// Properties](https://developer.apple.com/library/prerelease/mac/documentation/AudioUnit/Reference/AudioUnitPropertiesReference/index.html#//apple_ref/doc/constant_group/Voice_Processing_I_O_Audio_Unit_Properties)
    /// enumeration for the identifiers for this audio unit’s properties.
    ///
    /// **Available** in OS X v10.7 and later.
    VoiceProcessingIO = 1987078511,
    /// Connects to device hardware for input, output, or simultaneous input and output.
    /// Use it for playback, recording, or low-latency simultaneous input and output where echo
    /// cancellation is not needed.
    ///
    /// See <https://developer.apple.com/library/content/documentation/MusicAudio/Conceptual/AudioUnitHostingGuide_iOS/UsingSpecificAudioUnits/UsingSpecificAudioUnits.html>
    /// **Available** in iOS.
    RemoteIO = 1919512419,
}
