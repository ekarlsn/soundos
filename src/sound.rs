use std::{io::BufReader, path::Path};

use rodio::buffer::SamplesBuffer;

pub struct SoundHandle {
    // Speech
    speech_stream: rodio::OutputStream,
    speech_handle: rodio::OutputStreamHandle,
    speech_sink: rodio::Sink,

    tts: sherpa_rs::tts::MatchaTts,

    // Music
    music_stream: rodio::OutputStream,
    music_handle: rodio::OutputStreamHandle,
    pub music_sink: rodio::Sink,
}

impl SoundHandle {
    pub fn new() -> Self {
        let tts = {
            let config = sherpa_rs::tts::MatchaTtsConfig {
                acoustic_model: "./matcha-icefall-en_US-ljspeech/model-steps-3.onnx".into(),
                vocoder: "./hifigan_v2.onnx".into(),
                tokens: "./matcha-icefall-en_US-ljspeech/tokens.txt".into(),
                data_dir: "./matcha-icefall-en_US-ljspeech/espeak-ng-data".into(),
                ..Default::default()
            };
            sherpa_rs::tts::MatchaTts::new(config)
        };

        let (speech_stream, speech_handle) = rodio::OutputStream::try_default().unwrap();
        let speech_sink = rodio::Sink::try_new(&speech_handle).unwrap();

        let (music_stream, music_handle) = rodio::OutputStream::try_default().unwrap();
        let music_sink = rodio::Sink::try_new(&music_handle).unwrap();

        SoundHandle {
            speech_stream,
            speech_handle,
            speech_sink,
            tts,
            music_stream,
            music_handle,
            music_sink,
        }
    }

    pub fn say(&mut self, text: &str) {
        println!("Saying {text}");

        let (samples, sample_rate) = {
            let audio = self.tts.create(text, 0, 1.5).unwrap();

            (audio.samples, audio.sample_rate)
        };

        let buf = SamplesBuffer::new(1, sample_rate, samples);
        self.speech_sink.clear();
        self.speech_sink.append(buf);
        self.speech_sink.play();
    }

    pub fn play_music(&mut self, source: BufReader<std::fs::File>) {
        let source = rodio::Decoder::new(source).unwrap();
        self.music_sink.clear();
        self.music_sink.append(source);
        self.music_sink.play();
    }

    pub fn resume_music(&mut self) {
        self.music_sink.play();
    }

    pub fn pause_music(&mut self) {
        self.music_sink.pause();
    }
}
