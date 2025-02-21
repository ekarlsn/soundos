#!/bin/sh

mkdir -p target/dx/sound-os/debug/android/app/app/src/main/jniLibs/arm64-v8a
cp ~/sdk/android/ndk/26.1.10909125/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib/aarch64-linux-android/libc++_shared.so \
    target/dx/sound-os/debug/android/app/app/src/main/jniLibs/arm64-v8a
