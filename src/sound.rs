use std::{io::BufReader, path::Path};

use rodio::{buffer::SamplesBuffer, Source};

pub struct SoundHandle {
    // Speech
    speech_stream: rodio::OutputStream,
    speech_handle: rodio::OutputStreamHandle,
    speech_sink: rodio::Sink,

    #[cfg(feature = "desktop")]
    synth: piper_rs::synth::PiperSpeechSynthesizer,

    // Music
    music_stream: rodio::OutputStream,
    music_handle: rodio::OutputStreamHandle,
    pub music_sink: rodio::Sink,
}

impl SoundHandle {
    pub fn new() -> Self {
        #[cfg(feature = "desktop")]
        let synth = {
            let config_path = "en_US-libritts_r-medium.onnx.json";
            let model = piper_rs::from_config_path(Path::new(config_path)).unwrap();
            piper_rs::synth::PiperSpeechSynthesizer::new(model).unwrap()
        };

        let (speech_stream, speech_handle) = rodio::OutputStream::try_default().unwrap();
        let speech_sink = rodio::Sink::try_new(&speech_handle).unwrap();

        let (music_stream, music_handle) = rodio::OutputStream::try_default().unwrap();
        let music_sink = rodio::Sink::try_new(&music_handle).unwrap();

        SoundHandle {
            speech_stream,
            speech_handle,
            speech_sink,
            #[cfg(feature = "desktop")]
            synth,
            music_stream,
            music_handle,
            music_sink,
        }
    }

    pub fn say(&mut self, text: &str) {
        println!("Saying {text}");

        #[cfg(feature = "desktop")]
        let samples: Vec<f32> = {
            let mut samples: Vec<f32> = Vec::new();
            let audio = self
                .synth
                .synthesize_parallel(text.to_string(), None)
                .unwrap();
            for result in audio {
                samples.append(&mut result.unwrap().into_vec());
            }
            samples
        };

        #[cfg(feature = "mobile")]
        let samples: Vec<f32> = Vec::new();

        let buf = SamplesBuffer::new(1, 22050, samples);
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
