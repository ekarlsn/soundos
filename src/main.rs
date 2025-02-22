use dioxus::prelude::*;

use components::Hero;

mod components;
mod file_cache;
mod menu;
mod pods;
mod sound;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

// For the speech synthesis onnx files
// wget https://huggingface.co/rhasspy/piper-voices/resolve/main/en/en_US/libritts_r/medium/en_US-libritts_r-medium.onnx
// wget https://huggingface.co/rhasspy/piper-voices/resolve/main/en/en_US/libritts_r/medium/en_US-libritts_r-medium.onnx.json

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️
    let mut has_tts_data = use_signal(|| false);

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }


        if has_tts_data() {
            Hero {}
        } else {
            "Loading tts data..."
        }

    }
}

async fn download_tts() {
    // wget https://github.com/k2-fsa/sherpa-onnx/releases/download/tts-models/matcha-icefall-en_US-ljspeech.tar.bz2
    // wget https://github.com/k2-fsa/sherpa-onnx/releases/download/vocoder-models/hifigan_v2.onnx
    // tar xvf matcha-icefall-en_US-ljspeech.tar.bz2
    // rm matcha-icefall-en_US-ljspeech.tar.bz2
    // reqwest::get("https://github.com/k2-fsa/sherpa-onnx/releases/download/tts-models/matcha-icefall-en_US-ljspeech.tar.bz2").await.unwrap();


    // let sound_file =  reqwest::get("https://github.com/k2-fsa/sherpa-onnx/releases/download/vocoder-models/hifigan_v2.onnx").await.unwrap();
    // let filename = file_cache::create_file(
    //     pod_name.to_string(),
    //     episode_name.to_string(),
    //     &sound_file.bytes().await.unwrap(),
    // );
    // episode.file_path = Some(filename);
}
