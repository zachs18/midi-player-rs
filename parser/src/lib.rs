use std::{pin::Pin, future::Future, task::{Context, Poll}, iter::Peekable, time::Duration};

use futures_core::ready;
use midly::{Smf, TrackEvent};
use tokio::time::Sleep;

pub struct MidiEventStream<'a> {
    sleep: Option<Pin<Box<Sleep>>>,
    todo: &'a Smf<'a>,
    // Ticks until next event, remaining events.
    // Use .remove() when the iterator is empty
    progress: Vec<(u32, Peekable<std::slice::Iter<'a, TrackEvent<'a>>>)>,
}

impl<'a> MidiEventStream<'a> {
    pub fn new(midi: &'a Smf<'a>) -> Self {

        let mut progress = midi.tracks.iter().flat_map(
            |track| {
                let first = track.get(0)?;
                let ticks: u32 = first.delta.into();
                Some((ticks, track.iter().peekable()))
            }
        ).collect();

        Self {
            sleep: None,
            todo: midi,
            progress,
        }
    }

    fn ticks_to_time(&self, ticks: u32) -> Duration {
        // TODO: handle tempo and tempo changes
        Duration::from_millis(u64::from(ticks) * 1)
    }
}

impl<'a> futures_core::stream::Stream for MidiEventStream<'a> {
    type Item = midly::TrackEventKind<'a>;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>
    ) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        let progress = &mut this.progress;
        // If there are no tracks left, return None
        if progress.len() == 0 {
            return Poll::Ready(None);
        }

        // If any event can be returned now, do so
        for i in 0..progress.len() {
            if i >= progress.len() { break; }
            let (ticks, events) = &mut progress[i];
            if *ticks == 0 {
                // Return this event
                let event = events.next().expect("empty track should have been removed already");

                match events.peek() {
                    Some(a) => {
                        // Set the new event time
                        *ticks = a.delta.into();
                    },
                    None => {
                        // If this track has no more events, remove it
                        progress.remove(i);
                    },
                }
                return Poll::Ready(Some(event.kind))
            }
        }

        // Else, find the lowest time, subtract it from all times, and sleep
        let lowest_ticks = progress
            .iter()
            .map(|(ticks, _)| *ticks)
            .min().expect("progress is not empty");
        
        for (ticks, _) in progress {
            *ticks -= lowest_ticks;
        }

        let duration = this.ticks_to_time(lowest_ticks);
        // dbg!(duration);

        this.sleep = Some(Box::pin(tokio::time::sleep(duration)));
        let sleep = this.sleep.as_mut().unwrap();

        ready!(sleep.as_mut().poll(cx));

        todo!("handle case where sleep finished before polling (probably by putting this whole function in a loop {{}})")
    }
}