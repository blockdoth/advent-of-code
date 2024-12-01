#!/usr/bin/env bash

left=$( cat ./input | awk '{print $1}')
right=$(cat ./input | awk '{print $2}')

count=$(echo "$left" | xargs -I {} bash -c "echo \"$right\" | grep -o \"{}\" | wc -l")

paste <(echo "$left") <(echo "$count") | awk '{s += ($1 * $2)} END {print s}'

