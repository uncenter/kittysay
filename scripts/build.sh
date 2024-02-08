#!/usr/bin/env bash

# https://github.com/ryanccn/nyoom/blob/0a0d471e476bda2c6b031e291c4192712d9fbe31/scripts/build.sh

set -eo pipefail

exec_print() {
    echo -e "\033[2m\$\033[0m $*"
    "$@"
}

apt_updated=""
apt_install() {
    if [ ! "$apt_updated" ]; then
        exec_print sudo apt-get update
        apt_updated="y"
    fi

    exec_print sudo apt-get install "$@"
}

if [[ -z "$TARGET" ]]; then
    echo -e "\033[31mNo TARGET provided! Exiting.\033[0m"
    exit 1
fi

export RUSTFLAGS="-C lto=fat -C embed-bitcode=yes -C strip=symbols -C codegen-units=1 -C opt-level=z"

if [[ "$TARGET" = "aarch64-unknown-linux-"* ]]; then
    apt_install gcc-aarch64-linux-gnu
    export RUSTFLAGS="$RUSTFLAGS -C linker=aarch64-linux-gnu-gcc"
fi
if [[ "$TARGET" = *"-linux-musl" ]]; then
    apt_install musl-dev musl-tools
    export RUSTFLAGS="$RUSTFLAGS -C target-feature=+crt-static"
fi
if [[ "$TARGET" = "aarch64-unknown-linux-musl" ]]; then
    apt_install clang llvm
    export CC_aarch64_unknown_linux_musl="clang"
    export AR_aarch64_unknown_linux_musl="llvm-ar"
    export RUSTFLAGS="$RUSTFLAGS -C link-self-contained=yes -C linker=rust-lld"
fi

echo -e "\033[2m>\033[0m RUSTFLAGS=\033[36m\"$RUSTFLAGS\"\033[0m"

exec_print cargo build -r --target "$TARGET" --locked

result_suffix=""
artifact_basename_suffix=""
[[ "$TARGET" == *"windows"* ]] && result_suffix=".exe"
[[ "$TARGET" == *"musl" ]] && artifact_basename_suffix="-static"

result_path="kittysay-$TARGET$artifact_basename_suffix$result_suffix"

exec_print cp "target/$TARGET/release/kittysay$result_suffix" "$result_path"

[ -f "$GITHUB_OUTPUT" ] && echo "path=$result_path" >> "$GITHUB_OUTPUT"
