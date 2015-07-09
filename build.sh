#!/bin/bash
set -x
cd _snakemeter && cargo build
cp target/debug/lib_snakemeter.dylib _snakemeter.so