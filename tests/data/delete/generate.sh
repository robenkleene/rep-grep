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
sed -i.bak '1s#.*#--- a/1.txt#' patch.patch
sed -i.bak '2s#.*#+++ b/1.txt#' patch.patch
sed -i.bak '6s#.*#--- a/2.txt#' patch.patch
sed -i.bak '7s#.*#+++ b/2.txt#' patch.patch
sed -i.bak '13s#.*#--- a/3.txt#' patch.patch
sed -i.bak '14s#.*#+++ b/3.txt#' patch.patch

# newline messages
line_fix='6i\
\\ No newline at end of file
'
sed -i.bak "${line_fix}" patch.patch
line_fix='14i\
\\ No newline at end of file
'
sed -i.bak "${line_fix}" patch.patch
line_fix='$a\
\\ No newline at end of file
'
sed -i.bak "${line_fix}" patch.patch
rm patch.patch.bak
