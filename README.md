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

#### Linker error

export PATH=$PATH:$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/

#### File not found

I Got some problems when doing "dx build --platform android". I just got "File not found". https://github.com/DioxusLabs/dioxus/issues/3487

Solved by:

```toml
oboe = { version = "0.6.1", features = [
    "java-interface",
    "jni",
    "ndk",
    "ndk-context",
    "shared-stdcxx",
], optional = true }
```

and

mkdir target/dx/sound-os/debug/android/app/app/src/main/jniLibs/arm64-v8a
cp ~/sdk/android/ndk/26.1.10909125/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib/aarch64-linux-android/libc++_shared.so target/dx/sound-os/debug/android/app/app/src/main/jniLibs/arm64-v8a

#### Shared library at runtime

./inject_shared_libs_into_apk.py

zipalign -f -p -v 4 target/dx/sound-os/debug/android/app/app/build/outputs/apk/debug/app-debug-fixed.apk target/dx/sound-os/debug/android/app/app/build/outputs/apk/debug/app-debug-fixed-2.apk
apksigner sign --ks ~/.android/debug.keystore target/dx/sound-os/debug/android/app/app/build/outputs/apk/debug/app-debug-fixed-2.apk

#### Sherpa-rs build.rs

sherpa-rs-sys get's a build.rs compilation error because it can't find the out folder. Because it expects the profile name to be there.

dx build --platform android --profile dev

solves that
Maybe fix it upstream?
