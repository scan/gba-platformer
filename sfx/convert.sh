#!/bin/bash

for f in *.ogg; do
    ffmpeg -y -i "$f" "${f%.ogg}.wav"
done
