#!/usr/bin/env bash

echo "running on nestest100"

rm mylog.log
cargo run >> mylog.log
icdiff mylog.log ./test-roms/nestest/nestest100.log
