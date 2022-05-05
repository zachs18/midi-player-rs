use super::{Instrument, Envelope};

macro_rules! make_instrument {
    ($name:ident: [$($amps:expr),*], $envelope:expr) => {
        const $name: Instrument = Instrument {
            amplitudes: &[$($amps),*],
            full_amplitude: $($amps +)* 0.0,
            envelope: $envelope,
        };
    };
}

impl Envelope {
    pub const DEFAULT_ENVELOPE: Envelope = Envelope {
        attack: 44,
        decay: 22050, // 0.5s
        sustain: 0.5,
        release: 4410, // 0.1s
    };
}

impl Instrument {
    make_instrument!{
        VIOLIN:
        [1.0, 0.6, 0.6, 0.7, 0.4, 0.2, 0.4, 0.1],
        Envelope {
            attack: 1,
            decay: 17640,
            sustain: 0.8,
            release: 4410,
        }
    }
    make_instrument!{
        PIANO:
        [1.0, 1.0, 0.1, 0.2, 0.2],
        Envelope::DEFAULT_ENVELOPE
    }
    make_instrument!{
        GUITAR:
        [1.0, 0.7, 0.3, 0.4, 0.4, 0.2, 0.4, 0.1],
        Envelope {
            attack: 1,
            decay: 44100,
            sustain: 0.2,
            release: 4410,
        }
    }
    make_instrument!{
        FLUTE:
        [1.0, 1.0, 0.1, 0.2, 0.2],
        Envelope::DEFAULT_ENVELOPE
    }
    make_instrument!{
        RECORDER:
        [1.0, 0.8, 1.0, 0.2, 0.2],
        Envelope::DEFAULT_ENVELOPE
    }
    make_instrument!{
        PIZZICATO_STRINGS:
        [1.0, 0.7, 0.3, 0.4, 0.4, 0.2, 0.4, 0.1],
        Envelope {
            attack: 1,
            decay: 17640, // 0.4s
            sustain: 0.0,
            release: 1,
        }
    }
    make_instrument!{
        SQUARE_SYNTH:
        [1.0, 0.0, 1.0 / 3.0, 0.0, 1.0 / 5.0, 0.0, 1.0 / 7.0],
        Envelope::DEFAULT_ENVELOPE
    }
    make_instrument!{
        SAW_SYNTH:
        [1.0, -1.0 / 2.0, 1.0 / 3.0, -1.0 / 4.0, 1.0 / 5.0, -1.0 / 6.0, 1.0 / 7.0],
        Envelope::DEFAULT_ENVELOPE
    }

    make_instrument!{
        TODO:
        [1.0, 1.0, 0.1, 0.2, 0.2],
        Envelope {
            attack: 44,
            decay: 22050,
            sustain: 0.5,
            release: 4410,
        }
    }
}

pub(super) static INSTRUMENTS: &'static [Instrument] = &[
    // Piano
    Instrument::PIANO, // 0
    Instrument::TODO, // 1
    Instrument::TODO, // 2
    Instrument::TODO, // 3
    Instrument::TODO, // 4
    Instrument::TODO, // 5
    Instrument::TODO, // 6
    Instrument::TODO, // 7

    // Chromatic Percussion
    Instrument::TODO, // 8
    Instrument::TODO, // 9
    Instrument::TODO, // 10
    Instrument::TODO, // 11
    Instrument::TODO, // 12
    Instrument::TODO, // 13
    Instrument::TODO, // 14
    Instrument::TODO, // 15

    // Organ
    Instrument::TODO, // 16
    Instrument::TODO, // 17
    Instrument::TODO, // 18
    Instrument::TODO, // 19
    Instrument::TODO, // 20
    Instrument::TODO, // 21
    Instrument::TODO, // 22
    Instrument::TODO, // 23

    // Guitar
    Instrument::GUITAR, // 24
    Instrument::TODO, // 25
    Instrument::TODO, // 26
    Instrument::TODO, // 27
    Instrument::TODO, // 28
    Instrument::TODO, // 29
    Instrument::TODO, // 30
    Instrument::TODO, // 31

    // Bass
    Instrument::TODO, // 32
    Instrument::TODO, // 33
    Instrument::TODO, // 34
    Instrument::TODO, // 35
    Instrument::TODO, // 36
    Instrument::TODO, // 37
    Instrument::TODO, // 38
    Instrument::TODO, // 39

    // Strings
    Instrument::VIOLIN, // 40
    Instrument::VIOLIN, // 41
    Instrument::VIOLIN, // 42
    Instrument::VIOLIN, // 43
    Instrument::VIOLIN, // 44
    Instrument::PIZZICATO_STRINGS, // 45
    Instrument::TODO, // 46
    Instrument::TODO, // 47

    // Ensemble
    Instrument::TODO, // 48
    Instrument::TODO, // 49
    Instrument::TODO, // 50
    Instrument::TODO, // 51
    Instrument::TODO, // 52
    Instrument::TODO, // 53
    Instrument::TODO, // 54
    Instrument::TODO, // 55

    // Brass
    Instrument::TODO, // 56
    Instrument::TODO, // 57
    Instrument::TODO, // 58
    Instrument::TODO, // 59
    Instrument::TODO, // 60
    Instrument::TODO, // 61
    Instrument::TODO, // 62
    Instrument::TODO, // 63

    // Reed
    Instrument::TODO, // 64
    Instrument::TODO, // 65
    Instrument::TODO, // 66
    Instrument::TODO, // 67
    Instrument::TODO, // 68
    Instrument::TODO, // 69
    Instrument::TODO, // 70
    Instrument::TODO, // 71

    // Pipe
    Instrument::TODO, // 72
    Instrument::FLUTE, // 73
    Instrument::RECORDER, // 74
    Instrument::TODO, // 75
    Instrument::TODO, // 76
    Instrument::TODO, // 77
    Instrument::TODO, // 78
    Instrument::TODO, // 79

    // Synth Lead
    Instrument::SQUARE_SYNTH, // 80
    Instrument::SAW_SYNTH, // 81
    Instrument::TODO, // 82
    Instrument::TODO, // 83
    Instrument::TODO, // 84
    Instrument::TODO, // 85
    Instrument::TODO, // 86
    Instrument::TODO, // 87

    // Synth Pad
    Instrument::TODO, // 88
    Instrument::TODO, // 89
    Instrument::TODO, // 90
    Instrument::TODO, // 91
    Instrument::TODO, // 92
    Instrument::TODO, // 93
    Instrument::TODO, // 94
    Instrument::TODO, // 95

    // Synth Effects
    Instrument::TODO, // 96
    Instrument::TODO, // 97
    Instrument::TODO, // 98
    Instrument::TODO, // 99
    Instrument::TODO, // 100
    Instrument::TODO, // 101
    Instrument::TODO, // 102
    Instrument::TODO, // 103

    // Ethnic
    Instrument::TODO, // 104
    Instrument::TODO, // 105
    Instrument::TODO, // 106
    Instrument::TODO, // 107
    Instrument::TODO, // 108
    Instrument::TODO, // 109
    Instrument::TODO, // 110
    Instrument::TODO, // 111

    // Percussive
    Instrument::TODO, // 112
    Instrument::TODO, // 113
    Instrument::TODO, // 114
    Instrument::TODO, // 115
    Instrument::TODO, // 116
    Instrument::TODO, // 117
    Instrument::TODO, // 118
    Instrument::TODO, // 119

    // Sound Effects
    Instrument::TODO, // 120
    Instrument::TODO, // 121
    Instrument::TODO, // 122
    Instrument::TODO, // 123
    Instrument::TODO, // 124
    Instrument::TODO, // 125
    Instrument::TODO, // 126
    Instrument::TODO, // 127
];