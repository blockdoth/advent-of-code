#!/usr/bin/env bash

paste <(cat ./input | awk '{print $1}'| sort) <(cat ./input | awk '{print $2}'| sort) | awk '{s += sqrt(($1 - $2)^2)} END {print s}'

