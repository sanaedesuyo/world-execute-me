use std::io::Cursor;
use anyhow::{Context, Result};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use rodio::{Decoder, Sink};

#[derive(Debug)]
pub struct PlayStatus {
    current_timestamp: Duration,
    total_duration: Duration,
}

pub struct AudioPlayer {
    mp3_data: Vec<u8>,
    play_status: Arc<Mutex<PlayStatus>>,
}

impl PlayStatus {
    pub fn current(&self) -> Duration {
        self.current_timestamp
    }

    pub fn total_duration(&self) -> Duration {
        self.total_duration
    }

    pub fn set(&mut self, current: Duration) {
        self.current_timestamp = current;
    }
}

impl AudioPlayer {
    pub fn new(mp3_data: Vec<u8>) -> AudioPlayer {
        AudioPlayer {
            mp3_data,
            play_status: Arc::new(Mutex::new(PlayStatus {
                current_timestamp: Duration::from_secs_f32(0f32),
                total_duration: Duration::from_secs_f32(0f32),
            }))
        }
    }

    pub fn play(&self) -> Result<()> {
        let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
            .context("Failed to open default audio output stream")?;
        let sink = Sink::connect_new(&stream_handle.mixer());

        let cursor = Cursor::new(self.mp3_data.clone());
        let source = Decoder::new(cursor)
            .context("Failed to decode music file")?;

        let status = self.play_status.clone();

        sink.append(source);
        let start_time = Instant::now();
        let _ = thread::spawn(move || {
            loop {
                status.lock().unwrap().set(Instant::now().duration_since(start_time));
            }
        }).join().unwrap();

        Ok(())
    }

    pub fn get_status(&self) -> Arc<Mutex<PlayStatus>> {
        self.play_status.clone()
    }
}