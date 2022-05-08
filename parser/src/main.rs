use std::{io::{self, prelude::*}, fs::OpenOptions};
use midly::TrackEventKind;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let filename = match std::env::args().nth(1) {
        Some(filename) => filename,
        None => {
            eprintln!("Usage: {} [filename]", std::env::args().nth(0).as_deref().unwrap_or("cargo run"));
            std::process::exit(1);
        }
    };

    let mut stdout = OpenOptions::new().write(true).open("/dev/stdout").unwrap();

    let mut file = std::fs::File::open(&filename).expect("Failed to open file");
    let file_length = file.seek(io::SeekFrom::End(0));
    file.rewind();

    let mut data = Vec::with_capacity(file_length.unwrap_or(0).try_into().unwrap_or(0));

    file.read_to_end(&mut data);

    let smf = midly::Smf::parse(&data).unwrap();

    let mut midi_stream = parser::MidiEventStream::new(&smf);

    // tokio::pin!(midi_stream);

    while let Some(msg) = midi_stream.next().await {
        // match msg {
        //     TrackEventKind::Midi { channel, message } => match message {
        //         midly::MidiMessage::NoteOff { key, vel } => todo!(),
        //         midly::MidiMessage::NoteOn { key, vel } => todo!(),
        //         midly::MidiMessage::Aftertouch { key, vel } => todo!(),
        //         midly::MidiMessage::Controller { controller, value } => todo!(),
        //         midly::MidiMessage::ProgramChange { program } => todo!(),
        //         midly::MidiMessage::ChannelAftertouch { vel } => todo!(),
        //         midly::MidiMessage::PitchBend { bend } => todo!(),
        //     }
        //     msg => {
        //         eprintln!("TODO: {msg:?}")
        //     }
        // }
        if let Some(msg) = msg.as_live_event() {
            msg.write_std(&mut stdout).unwrap();
        }
    }

    // dbg!(smf);

}
