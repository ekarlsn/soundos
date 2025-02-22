use std::io::Read;

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
    let fut = use_resource(move || async move { download_tts().await });

    let tts_loaded = fut.read_unchecked().as_ref().is_some();

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }


        if tts_loaded {
            Hero {},
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

    if file_cache::file_exists("tts_model".to_owned(), "hifigan.onnx".to_owned()) {
        println!("hifigan.onnx already exists");
    } else {
        let onnx_file = reqwest::get(
            "https://github.com/k2-fsa/sherpa-onnx/releases/download/vocoder-models/hifigan_v2.onnx",
        )
        .await
        .unwrap();
        let _filename = file_cache::create_file(
            "tts_model".to_owned(),
            "hifigan.onnx".to_owned(),
            &onnx_file.bytes().await.unwrap(),
        );
    }

    if file_cache::file_exists(
        "tts_model".to_owned(),
        "matcha-icefall-en_US-ljspeech".to_owned(),
    ) {
        println!("Unpacked tts model already exists, we're good");
    } else {
        let tar_file = reqwest::get(
                        "https://github.com/k2-fsa/sherpa-onnx/releases/download/tts-models/matcha-icefall-en_US-ljspeech.tar.bz2",
                    )
                    .await
                    .unwrap();
        let tar_bytes = &tar_file.bytes().await.unwrap();

        // Extract the tar.bz2 file
        let tar_cursor = std::io::Cursor::new(tar_bytes);
        let bz_decoder = bzip2::read::BzDecoder::new(tar_cursor);
        let mut archive = tar::Archive::new(bz_decoder);

        // Create the output directory if it doesn't exist
        let output_dir = file_cache::make_namespace("tts_model");

        // Unpack the archive
        archive.unpack(output_dir).unwrap();
    }
}
