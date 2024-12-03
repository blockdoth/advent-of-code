#!/usr/bin/env bash

cat <(echo "do()") ./input  | grep -oE "mul\([0-9]+,[0-9]+\)|do\(\)|don't\(\)" | grep -Poz "do\(\)(.*?\n)*?don't\(\)"| grep -aoE 'mul\([0-9]+,[0-9]+\)' | sed -E 's/mul\(([0-9]+),([0-9]+)\)/\1*\2/' | paste -s -d + | bc

