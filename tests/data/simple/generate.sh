#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname "$0")" || exit 1

# grep.txt
grep --line-number --with-filename bomp original.txt > grep.txt

# patch.patch
diff --unified original.txt \
  <(sed s/bomp/ram/g original.txt) > patch.patch || true
sed -i '' 's/bomp/ram/g' grep.txt
sed -i '' '1s/.*/--- a\/original.txt/' patch.patch
sed -i '' '2s/.*/+++ b\/original.txt/' patch.patch
sed -i '' '3s/-1,2 +1,2/-1 +1/' patch.patch
sed -i '' '6d' patch.patch
wc -l < grep.txt | xargs > grep-count.txt

# vimgrep.txt
rg --vimgrep bomp original.txt > vimgrep.txt
sed -i '' 's/bomp/ram/g' vimgrep.txt

# delete.patch
diff --unified original.txt \
  <(sed /bomp/d original.txt) > delete.patch || true
sed -i '' '1s/.*/--- a\/original.txt/' delete.patch
sed -i '' '2s/.*/+++ b\/original.txt/' delete.patch
sed -i '' '3s/-1,2 +1,2/-1 +1/' delete.patch
sed -i '' '5d' delete.patch
