#!/usr/bin/env bash

grep -oE 'mul\([0-9]+,[0-9]+\)' ./input | sed -E 's/mul\(([0-9]+),([0-9]+)\)/\1*\2/' | paste -s -d + | bc

