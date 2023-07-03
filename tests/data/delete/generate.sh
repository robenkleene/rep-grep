#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname "$0")" || exit 1

grep --line-number --with-filename delete 1.txt 2.txt 3.txt > grep.txt
diff --unified 1.txt \
  <(sed /delete/d 1.txt) > patch.patch || true
diff --unified 2.txt \
  <(sed /delete/d 2.txt) >> patch.patch || true
diff --unified 3.txt \
  <(sed /delete/d 3.txt) >> patch.patch || true
sed -i '' '1s/.*/--- a\/1.txt/' patch.patch
sed -i '' '2s/.*/+++ b\/1.txt/' patch.patch
sed -i '' '6s/.*/--- a\/2.txt/' patch.patch
sed -i '' '7s/.*/+++ b\/2.txt/' patch.patch
sed -i '' '13s/.*/--- a\/3.txt/' patch.patch
sed -i '' '14s/.*/+++ b\/3.txt/' patch.patch
