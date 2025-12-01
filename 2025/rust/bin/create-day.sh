#!/bin/sh

if [ -z "$1" ] || [ "$1" -lt 0 ] || [ "$1" -gt 12 ]; then
  echo "usage: ./$0 <1-25>"
  exit 1
fi

day="$(printf '%02d' $1)"
module="aoc-2025-day-$day"
template="aoc-2025-day-template"

cargo new "$module"
cp -Rv aoc-2025-day-template/* "$module"
cd $module
sed -i "s/template/$day/g" Cargo.toml
