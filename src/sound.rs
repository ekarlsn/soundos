use std::path::Path;

use piper_rs::synth::PiperSpeechSynthesizer;
use rodio::buffer::SamplesBuffer;

pub struct SoundHandle {
    // Speech
    speech_stream: rodio::OutputStream,
    speech_handle: rodio::OutputStreamHandle,
    speech_sink: rodio::Sink,
    synth: PiperSpeechSynthesizer,

    // Music
    music_stream: rodio::OutputStream,
    music_handle: rodio::OutputStreamHandle,
    music_sink: rodio::Sink,
}

impl SoundHandle {
    pub fn new() -> Self {
        let config_path = "en_US-libritts_r-medium.onnx.json";
        let model = piper_rs::from_config_path(Path::new(config_path)).unwrap();
        let synth = PiperSpeechSynthesizer::new(model).unwrap();

        let (speech_stream, speech_handle) = rodio::OutputStream::try_default().unwrap();
        let speech_sink = rodio::Sink::try_new(&speech_handle).unwrap();

        let (music_stream, music_handle) = rodio::OutputStream::try_default().unwrap();
        let music_sink = rodio::Sink::try_new(&music_handle).unwrap();

        SoundHandle {
            speech_stream,
            speech_handle,
            speech_sink,
            synth,
            music_stream,
            music_handle,
            music_sink,
        }
    }

    pub fn say(&mut self, text: &str) {
        println!("Trying to speak");

        let mut samples: Vec<f32> = Vec::new();
        let audio = self
            .synth
            .synthesize_parallel(text.to_string(), None)
            .unwrap();
        for result in audio {
            samples.append(&mut result.unwrap().into_vec());
        }

        let buf = SamplesBuffer::new(1, 22050, samples);
        self.speech_sink.append(buf);

        println!("Done speaking");
    }
}
