#!/usr/bin/env bash

cat ./input | xargs -I {} bash -c "echo '{}' | tr ' ' '\n' | awk 'NR > 1 { if (sqrt((\$1 - prev)^2) > 0 && sqrt((\$1 - prev)^2) < 4) { if ((\$1 - prev) > 0) {print 1} else {print -1} } else { print 0 } } { prev = \$1 }' | uniq | wc -l" | grep 1 -c

