#!/bin/bash

# for module in $(find ./src/api/* -type d)
# do
#    printf "    $module - "
#    awk '/pub struct/ { print $3 }' $module/mod.rs
# done | tr -d ';' | sed "s/.\/src//g" > ./banner_listen.txt


mkdir -p ./dist
PACKAGE_NAME=$(awk '/^name/{print $3}' Cargo.toml | tr -d '"')

if [ X"$1" == X"-d" ]; then
    cargo build --package ${PACKAGE_NAME} --target $(rustup target list | awk '/musl.*installed/{print $1}')
    strip ./target/x86_64-unknown-linux-musl/debug/${PACKAGE_NAME}
    cp -v ./target/x86_64-unknown-linux-musl/debug/${PACKAGE_NAME} ./dist/${PACKAGE_NAME}

else
    cargo build --release --package ${PACKAGE_NAME} --target $(rustup target list | awk '/musl.*installed/{print $1}')
    strip ./target/x86_64-unknown-linux-musl/release/${PACKAGE_NAME}
    cp -v ./target/x86_64-unknown-linux-musl/release/${PACKAGE_NAME} ./dist/${PACKAGE_NAME}

fi

