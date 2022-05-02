#!/usr/bin/env bash


rm mylog.log
cargo run >> mylog.log
./fuck-you-diff mylog.log ./test-roms/nestest-redux/nestest_cpu_relined20.log
