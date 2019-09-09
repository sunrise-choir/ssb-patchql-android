curl https://sh.rustup.rs -sSf | sh -s -- -y
source $HOME/.cargo/env
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
export ORG_GRADLE_LOGGING_LEVEL=debug
echo $NDK_HOME
echo $NDK
echo $ORG_GRADLE_LOGGING_LEVEL
