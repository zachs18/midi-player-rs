use std::{time::{Duration, Instant}, io::Read, os::unix::prelude::AsRawFd};

use psimple::Simple;
use pulse::sample::CHANNELS_MAX;

mod sys {
    extern "C" {
        pub(super) fn opt_sin(x: f64) -> f64;
    }
}

fn opt_sin(x: f64) -> f64 {
    unsafe { sys::opt_sin(x) }
}

const NOTE_COUNT: usize = 8;
const CHANNEL_COUNT: usize = 16;
const RATE: u32 = 44100;
const SAMPLE_DT: f64 = 1.0 / RATE as f64;
const BUFSIZE: usize = 128;

#[derive(Debug, Default, Clone, Copy)]
struct Note {
    note: u8,
    amp: u32,
    freq: f64,
    /// Zero indicates this not is not in use
    /// Positive value
    /// Negative value 
    sample_time: i64,
    instrument: &'static Instrument,
}

#[derive(Debug, Clone, Copy)]
struct Envelope {
    attack: i64,
    decay: i64,
    sustain: f64,
    release: i64,
}
#[derive(Debug, Clone, Copy)]
struct Instrument {
    amplitudes: &'static [f64],
    full_amplitude: f64,
    envelope: Envelope,
}

impl Default for &'static Instrument {
    fn default() -> Self {
        &Instrument {
            amplitudes: &[1.0, 0.6, 0.6, 0.7, 0.4, 0.2, 0.4, 0.1],
            full_amplitude: 1.0 + 0.6 + 0.6 + 0.7 + 0.4 + 0.2 + 0.4 + 0.1,
            envelope: Envelope {
                attack: 1,
                decay: 17640,
                sustain: 0.8,
                release: 4410,
            },
        }
    }
}

fn poll_in(input: &impl AsRawFd) -> bool {
    let mut pollfd = libc::pollfd {
        fd: input.as_raw_fd(),
        events: libc::POLLIN,
        revents: 0,
    };
    let ret = unsafe {
        libc::poll(&mut pollfd, 1, 0)
    };
    pollfd.revents & libc::POLLIN != 0
}

fn main() {
    let sample_spec = pulse::sample::Spec {
        format: pulse::sample::Format::S16le,
        rate: RATE,
        channels: 1,
    };
    
    let s = Simple::new(
        None,
        "midi player",
        pulse::stream::Direction::Playback,
        None,
        "Music",
        &sample_spec,
        None,
        None,
    ).expect("Failed to connect to pulseaudio.");

    // let mut data = vec![0i16; 44100];
    // for (i, w) in data.iter_mut().enumerate() {
    //     // *b = ((((i as f64 / 4410.0) * 440.0 / std::f64::consts::PI).sin() * 12800.0) as u16);
    //     let wav = (2.0 * std::f64::consts::PI * SAMPLE_DT * (i as f64) * 440.0).sin();
    //     *w = (wav * 32767.0) as i16;
    // }
    // println!("a");
    // s.write(bytemuck::cast_slice(&data)).expect("Failed to write audio data");
    // println!("b");
    // s.write(bytemuck::cast_slice(&data)).expect("Failed to write audio data");
    // println!("c");

    let mut notes = [<[Note; NOTE_COUNT]>::default(); CHANNEL_COUNT];

    // let poller = polling::Poller::new().expect("Failed to create poller.");
    let mut stdin = std::io::stdin();
    // let mut stdin = stdin.lock(); // StdinLock impls BufRead, is that a problem?
    // let interest = Event::readable(0);
    // poller.add(&stdin, interest).expect("Failed to add stdin to poller.");

    let mut buf = vec![0i16; BUFSIZE];

    // let mut events = Vec::with_capacity(1);
    let mut running = true;
    'main_loop: while running {
        let mut read_start = std::time::Instant::now();
        'read_loop: while running && poll_in(&stdin) {
            // poller.modify(&stdin, interest).expect("Failed to add stdin to poller.");
            // events.clear();
            let mut buf = [0u8; 3];
            let bytes_read = stdin.read(&mut buf[..1]).expect("Failed to read");
            if bytes_read == 0 {
                // EOF
                running = false;
                break 'read_loop;
            }
            match buf[0] {
                0x80..=0x8f | 0x90..=0x9f => {
                    // Note-off or Note-on 
                    let bytes_read = stdin.read(&mut buf[1..3]).expect("Failed to read");
                    if bytes_read != 2 {
                        // EOF
                        running = false;
                        break 'read_loop;
                    }
                    let channel = (buf[0] & 0x0f) as usize;
                    let note = buf[1];
                    let velocity = buf[2];
                    let off = matches!(buf[0], 0x80..=0x8f) || velocity == 0;
                    // println!("channel {channel} {on}, {note}, {velocity}", on = if off {"off"} else {"on"});

                    for note_info in &mut notes[channel] {
                        if off && note_info.note == note && note_info.sample_time > 0 {
                            note_info.sample_time = -note_info.instrument.envelope.release;
                            break;
                        } else if !off && note_info.sample_time == 0 {
                            note_info.sample_time = 1;
                            note_info.note = note;
                            note_info.amp = velocity as u32 * 8192 / 0xff;
                            note_info.freq = 440.0 * (2.0f64).powf((note as f64 - 69.0) as f64 / 12.0);
                            break;
                        }
                    }
                    // dbg!("AAA");
                }
                b => {dbg!(b);},
            };
        }
        let read_end = std::time::Instant::now();
        let read_took = read_end - read_start;
        if read_took > Duration::from_secs_f64(BUFSIZE as f64 * 2.0 * SAMPLE_DT) {
            println!("Reading took too long! {read_took:?}");
        }
        
        let mut ampsum = 0;
        for note in notes.iter().flatten() {
            if note.sample_time != 0 {
                ampsum += note.amp; // TODO: envelope
            }
        }

        let start = std::time::Instant::now();
        for i in 0..BUFSIZE {
        // for i in 0..0 {
            let mut wav = 0.0;

            for channel in notes.iter_mut() {
                for note in channel {
                    if note.sample_time == 0 {
                        continue;
                    }
                    note.sample_time += 1;
                    let mut wava = 0.0;
                    for (i, amp) in note.instrument.amplitudes.iter().copied().enumerate() {
                        let parameter = (i + 1) as f64 * std::f64::consts::TAU * SAMPLE_DT * note.sample_time as f64 * note.freq;
                        // wava += amp * parameter.sin();
                        wava += amp * opt_sin(parameter);
                    }
                    wav += note.amp as f64 * wava;
                }
            }
            buf[i] = if ampsum > 32767 {
                wav * (32767.0 / ampsum as f64)
            } else {
                wav
            } as i16;
        }
        let end = std::time::Instant::now();
        let took = end - start;
        if took > Duration::from_secs_f64(BUFSIZE as f64 * 2.0 * SAMPLE_DT) {
            println!("Generating took too long! {took:?}");
        }

        // write data to pulse
        let start = std::time::Instant::now();
        s.write(bytemuck::cast_slice(&buf)).expect("Failed to write audio data");
        // println!("written!");
        // println!("{:?}", &buf[..100]);
        let end = std::time::Instant::now();
        let took = end - start;
        if took > Duration::from_secs_f64(BUFSIZE as f64 * 2.0 * SAMPLE_DT) {
            // println!("Writing took too long! {took:?}");
        }

        let took = end - read_start;
        let expected = Duration::from_secs_f64(BUFSIZE as f64 * 2.0 * SAMPLE_DT);
        if took > expected {
            // println!("Segment took too long! {took:?}, expected: {expected:?}");
        }
    }
    
}
