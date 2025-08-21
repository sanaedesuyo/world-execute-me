use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration};
use crate::stream_controller::audio::PlayStatus;

pub struct DisplayInfo {
    start_time: Duration,
    event: Arc<dyn Fn() + Send + Sync>,
}

impl DisplayInfo {
    pub fn new(min: u64, secs: u64, millis: u64, event: Arc<dyn Fn() + Send + Sync>) -> Self {
        let start_time = Duration::from_secs(min * 60 + secs) + Duration::from_millis(millis);
        
        DisplayInfo { start_time, event }
    }
}

pub struct Displayer {
    sequence: Vec<DisplayInfo>,
    start_time: Vec<Duration>,
    next_index: usize,
}

impl Displayer {
    pub fn new(sequence: Vec<DisplayInfo>) -> Self {
        let start_time = sequence.iter().map(|info| {
            info.start_time.clone()
        }).collect::<Vec<Duration>>();

        Displayer {
            sequence,
            start_time,
            next_index: 0,
        }
    }

    /// update the display status at each call.
    pub fn update(&mut self, current_status: Arc<Mutex<PlayStatus>>) {
        if self.next_index >= self.sequence.len() {
            return;
        }

        if self.start_time[self.next_index] <= current_status.lock().unwrap().current() {
            let event = self.sequence[self.next_index].event.clone();

            thread::spawn(move || {
                event();
            }).join().unwrap();
            
            self.next_index += 1;
        }
    }
}