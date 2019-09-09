$ANDROID_HOME/tools/bin/sdkmanager --install "lldb;3.1" "ndk;20.0.5594570" "cmake;3.10.2.4988404" && \
curl https://sh.rustup.rs -sSf | sh -s -- -y && \
source $HOME/.cargo/env && \
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
