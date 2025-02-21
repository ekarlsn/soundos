#!/usr/bin/env python3
import subprocess
import zipfile
import shutil
import os
from pathlib import Path

REPO_ROOT = Path(__file__).parent


def inject_shared_libraries(apk_path, libraries: list[str], output_apk_path):
    # Create a temporary directory to extract the APK contents
    temp_dir = "temp_apk"
    if os.path.exists(temp_dir):
        shutil.rmtree(temp_dir)
    os.makedirs(temp_dir)

    # Extract the APK contents
    with zipfile.ZipFile(apk_path, "r") as apk:
        apk.extractall(temp_dir)

    # Define the target directory for the shared libraries
    lib_dir = os.path.join(temp_dir, "lib", "arm64-v8a")
    if not os.path.exists(lib_dir):
        os.makedirs(lib_dir)

    # Copy the shared libraries to the target directory
    for lib in libraries:
        shutil.copy(lib, lib_dir)

    # Create a new APK file with the injected shared libraries
    with zipfile.ZipFile(output_apk_path, "w") as new_apk:
        for root, dirs, files in os.walk(temp_dir):
            for file in files:
                file_path = os.path.join(root, file)
                arcname = os.path.relpath(file_path, temp_dir)
                new_apk.write(file_path, arcname)

    # Clean up the temporary directory
    shutil.rmtree(temp_dir)

    print(f"Injected shared libraries into {output_apk_path}")


def zipalign(apk_path, output_apk_path):
    subprocess.run(["zipalign", "-p", "-v", "4", apk_path, output_apk_path])
    print(f"Zipaligned {apk_path} to {output_apk_path}")


def resign(apk_path):
    subprocess.run(
        [
            "apksigner",
            "sign",
            "--ks",
            "/home/ekarls66/.android/debug.keystore",
            "--ks-pass",
            "pass:android",
            "--key-pass",
            "pass:android",
            apk_path,
        ]
    )

    print(f"Resigned {apk_path}")


# Example usage
apk_path = (
    "target/dx/sound-os/debug/android/app/app/build/outputs/apk/debug/app-debug.apk"
)
cpp_lib_path = "/home/ekarls66/sdk/android/ndk/26.1.10909125/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib/aarch64-linux-android/libc++_shared.so"
injected_apk_path = "target/dx/sound-os/debug/android/app/app/build/outputs/apk/debug/app-debug-fixed.apk"
zipaligned_path = "target/dx/sound-os/debug/android/app/app/build/outputs/apk/debug/app-debug-fixed-aligned.apk"

inject_shared_libraries(apk_path, [cpp_lib_path], injected_apk_path)
zipalign(injected_apk_path, zipaligned_path)
resign(zipaligned_path)
