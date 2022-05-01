#!/usr/bin/env bash
filename="/home/mura/Pictures/Sreenshots/$(date +'%Y.%m.%d-%H:%m:%S').png"
touch $filename
grim $filename
