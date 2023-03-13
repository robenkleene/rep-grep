#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname "$0")" || exit 1

grep --line-number --with-filename Markdown markdown-syntax.md > markdown-to-markup-grep.txt
diff --unified markdown-syntax.md \
  <(sed s/Markdown/Markup/g markdown-syntax.md) > markdown-markup.patch || true
sed -i '' 's/^ $//' markdown-markup.patch
sed -i '' '1s/.*/--- a\/markdown-syntax.md/' markdown-markup.patch
sed -i '' '2s/.*/+++ b\/markdown-syntax.md/' markdown-markup.patch

# Fix a couple of random diff differences
sed -i '' '307s/.*/@@ -468,7 +468,7 @@\n/' markdown-markup.patch
sed -i '' '294s/.*/@@ -458,10 +458,10 @@/' markdown-markup.patch
sed -i '' '452s/.*/@@ -872,8 +872,8 @@/' markdown-markup.patch
sed -i '' '463s/.*/@@ -880,7 +880,7 @@\n/' markdown-markup.patch

sed -i '' 's/Markdown/Markup/g' markdown-to-markup-grep.txt
wc -l < markdown-to-markup-grep.txt | xargs > grep-count.txt
