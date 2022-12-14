#!/bin/bash

year=2022

day=$(date +%-d)
dir=day$day

cargo new $dir
curl -b $(cat ~/aoc_cookie) "https://adventofcode.com/$year/day/$day/input" \
     > $dir/input
