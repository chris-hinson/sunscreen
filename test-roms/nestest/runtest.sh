#!/usr/bin/env bash

echo "running on nestest200"

rm mylog.log
cargo run >> mylog.log
icdiff mylog.log ./test-roms/nestest/nestest200.log
