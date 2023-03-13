#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname "$0")" || exit 1

grep --line-number --with-filename change 1.txt 2.txt 3.txt > grep.txt
diff --unified 1.txt \
  <(sed s/changes/altered/g 1.txt) > patch.patch || true
diff --unified 2.txt \
  <(sed s/changes/altered/g 2.txt) >> patch.patch || true
diff --unified 3.txt \
  <(sed s/changes/altered/g 3.txt) >> patch.patch || true
sed -i '' '1s/.*/--- a\/1.txt/' patch.patch
sed -i '' '2s/.*/+++ b\/1.txt/' patch.patch
sed -i '' '7s/.*/--- a\/2.txt/' patch.patch
sed -i '' '8s/.*/+++ b\/2.txt/' patch.patch
sed -i '' '16s/.*/--- a\/3.txt/' patch.patch
sed -i '' '17s/.*/+++ b\/3.txt/' patch.patch

# newline messages
line_fix='7i\
\\ No newline at end of file
'
sed -i '' "${line_fix}" patch.patch
line_fix='16i\
\\ No newline at end of file
'
sed -i '' "${line_fix}" patch.patch
line_fix='18i\
\\ No newline at end of file
'
sed -i '' "${line_fix}" patch.patch
line_fix='$a\
\\ No newline at end of file
'
sed -i '' "${line_fix}" patch.patch

wc -l < 1.txt | xargs > 1-count.txt
wc -l < 2.txt | xargs > 2-count.txt
wc -l < 3.txt | xargs > 3-count.txt
