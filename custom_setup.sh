apt-get install build-essential cmake
curl https://sh.rustup.rs -sSf | sh -s -- -y
source $HOME/.cargo/env
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
