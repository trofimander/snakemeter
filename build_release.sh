#!/bin/bash
set -x
cd _snakemeter && cargo build --release
cp target/release/lib_snakemeter.dylib ../_snakemeter.so