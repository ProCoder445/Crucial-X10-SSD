#!/bin/bash

normal_dir="$HOME/Crucial X10"
SSD="/Volumes/Crucial X10"

echo "Trying to make the Crucial X10 dir..."

mkdir -p "$normal_dir" || echo "Could not make the Crucial X10 dir in ~";

[[ -d "$SSD" ]] || (echo "SSD not found" && exit 0);

rsync -a "$normal_dir/" "$SSD/"
