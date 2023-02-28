#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname "$0")" || exit 1

grep --line-number --with-filename Markdown markdown-syntax.md > markdown-to-markup-grep.txt
diff --unified markdown-syntax.md \
  <(sed s/Markdown/Markup/g markdown-syntax.md) > markdown-markup.patch || true
sed -i '' '1s/.*/--- a\/markdown-syntax.md/' markdown-markup.patch
sed -i '' '2s/.*/+++ b\/markdown-syntax.md/' markdown-markup.patch
wc -l < markdown-grep.txt | xargs > grep-count.txt
