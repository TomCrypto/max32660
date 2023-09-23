#!/usr/bin/env sh

svd2rust --target cortex-m --strict -i svd/MAX32660.xml && rm -rf src && form -i lib.rs -o src/ && rm lib.rs && sed -i '1i#![allow(clippy::all)]' src/lib.rs && cargo fmt
