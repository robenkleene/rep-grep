#!/usr/bin/env bash

set -euo pipefail

grep --line-number --with-filename bomp original.md > grep.txt
diff --unified original.md \
  <(sed s/bomp/ram/g original.md) > patch.patch || true
sed -i '' '1s/.*/--- a\/original.md/' patch.patch
sed -i '' '2s/.*/+++ b\/original.md/' patch.patch
wc -l < grep.txt | xargs > grep-count.txt
