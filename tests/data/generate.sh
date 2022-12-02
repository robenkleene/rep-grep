#!/usr/bin/env bash

set -euo pipefail

grep --line-number --with-filename Markdown markdown-syntax.md > markdown-grep.txt
diff --unified markdown-syntax.md \
  <(sed s/Markdown/Markup/g markdown-syntax.md) > markdown-markup.patch || true
wc -l < markdown-grep.txt | xargs > grep-count.txt
