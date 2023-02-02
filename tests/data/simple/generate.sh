#!/usr/bin/env bash

set -euo pipefail

grep --line-number --with-filename bomp original.txt > grep.txt
diff --unified original.txt \
  <(sed s/bomp/ram/g original.txt) > patch.patch || true
sed -i '' '1s/.*/--- a\/original.md/' patch.patch
sed -i '' '2s/.*/+++ b\/original.md/' patch.patch
wc -l < grep.txt | xargs > grep-count.txt
