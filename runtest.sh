#!/usr/bin/env bash


rm mylog.log
cargo run >> mylog.log
./fuck-you-diff mylog.log ./test-roms/nestest/nestest1k.log
