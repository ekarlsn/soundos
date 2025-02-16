# Development


### Downaload speech syntesis files

```bash
wget https://huggingface.co/rhasspy/piper-voices/resolve/main/en/en_US/libritts_r/medium/en_US-libritts_r-medium.onnx
wget https://huggingface.co/rhasspy/piper-voices/resolve/main/en/en_US/libritts_r/medium/en_US-libritts_r-medium.onnx.json
```

### Serving Your App

Run the following command in the root of your project to start developing for linux:

```bash
dx serve --platform linux
```

# Linux problems

When running the app on linux, can't find sherpa c-api library.

export LD_LIBRARY_PATH=target/debug/


### Android problems

export PATH=$PATH:$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/

I Got some problems when doing "dx build --platform android". I just got "File not found". https://github.com/DioxusLabs/dioxus/issues/3487

Solved by:

mkdir $ANDROID_NDK_HOME/target/dx/sound-os/debug/android/app/app/src/main/jniLibs/arm64-v8a
cp ~/sdk/android/ndk/26.1.10909125/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib/aarch64-linux-android/libc++_shared.so target/dx/sound-os/debug/android/app/app/src/main/jniLibs/arm64-v8a

sherpa-rs-sys get's a build.rs compilation error because it can't find the out folder. Because it expects the profile name to be there.

dx build --platform android --profile dev

solves that
Maybe fix it upstream?
