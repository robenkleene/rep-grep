#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname "$0")" || exit 1

grep --line-number --with-filename bomp original.txt > grep.txt
diff --unified original.txt \
  <(sed s/bomp/ram/g original.txt) > patch.patch || true
sed -i '' 's/bomp/ram/g' grep.txt
sed -i '' '1s/.*/--- a\/original.txt/' patch.patch
sed -i '' '2s/.*/+++ b\/original.txt/' patch.patch
sed -i '' '3s/-1,2 +1,2/-1 +1/' patch.patch
sed -i '' '6d' patch.patch
wc -l < grep.txt | xargs > grep-count.txt
