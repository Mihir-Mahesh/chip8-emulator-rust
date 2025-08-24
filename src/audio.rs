use rodio::{OutputStream, Sink, Source};
use std::time::Duration;

pub struct Audio {
    _stream: OutputStream,
    sink: Sink,
    pub is_playing: bool
}

impl Audio {
    pub fn new() -> Self {
        let (_stream, handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&handle).unwrap();
        Audio { _stream, sink, is_playing: false }
    }

    pub fn play(&mut self) {
        self.is_playing = true;
        if self.sink.empty() {
            let source = rodio::source::SineWave::new(440.0).take_duration(Duration::from_secs(10));
            self.sink.append(source);
        }
        self.sink.play();
    }

    pub fn stop(&mut self) {
        self.is_playing = false;
        self.sink.stop();
    }
}
