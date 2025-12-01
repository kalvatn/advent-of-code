#!/bin/sh

cargo build --release

for i in $(seq -w 1 12); do
  module="aoc-2025-day-$i"
  if [ -f ./target/release/$module ]; then
    echo "$module"
    ./target/release/$module $module/input
    echo
  fi
done
