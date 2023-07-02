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
