#!/usr/bin/env bash


cat ./input | xargs -I {} bash -c "\

  IN=\$(echo '{}' | tr ' ' '\n'); \

  PERM=\$(echo '{}');\
  for item in \$IN; do \
    LINE=\$(printf \"\$IN\" | grep -v \"\$item\"| tr '\n' ' ' ); \
    PERM=\"\$PERM\\n\$LINE\"; \
  done; \
  printf \"\$PERM\" | sed 's/[ \t]*$//' > temp/\$RANDOM.aoc
  "

FILES=$(find ./temp/*.aoc) 


for file in $FILES; do \
  cat $file | xargs -I {} bash -c "echo '{}' | tr ' ' '\n' | awk 'NR > 1 { if (sqrt((\$1 - prev)^2) > 0 && sqrt((\$1 - prev)^2) < 4) { if ((\$1 - prev) > 0) {print 1} else {print -1} } else { print 0 } } { prev = \$1 }' | uniq | wc -l" | grep -E '^[1-9]$' | wc -l >> temp/score
done;

cat temp/score | grep 1 -c

rm -r ./temp/*
