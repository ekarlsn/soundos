# Default task that lists the options
default:
    @just --list

android:
    #!/usr/bin/env bash
    set -euxo pipefail

    export PATH=${ANDROID_NDK_HOME}/toolchains/llvm/prebuilt/linux-x86_64/bin/:${ANDROID_HOME}/build-tools/33.0.1/:$PATH
    export SHERPA_LIB_PATH=/home/ekarls66/repos/soundos/sherpa-onnx-v1.10.28-linux-x64-static

    rm -rf target/dx/sound-os/
    dx build --platform android --profile dev
    ./inject_shared_libs_into_apk.py
    adb install target/dx/sound-os/debug/android/app/app/build/outputs/apk/debug/app-debug-fixed-aligned.apk

linux:
    #!/usr/bin/env bash
    set -euxo pipefail

    export LD_LIBRARY_PATH=target/debug

    dx build --platform linux
