#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname "$0")" || exit 1

# markdown-to-markup-grep.txt
grep --line-number --with-filename Markdown markdown-syntax.md > markdown-to-markup-grep.txt

# markdown-markup.patch
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

# markdown-to-markup-grep.txt
sed -i '' 's/Markdown/Markup/g' markdown-to-markup-grep.txt
wc -l < markdown-to-markup-grep.txt | xargs > grep-count.txt

# markdown-to-markup-vimgrep.txt
rg --vimgrep Markdown markdown-syntax.md > markdown-to-markup-vimgrep.txt
sed -i '' 's/Markdown/Markup/g' markdown-to-markup-vimgrep.txt

# delete.patch
diff --unified markdown-syntax.md \
  <(sed /Markdown/d markdown-syntax.md) > delete.patch || true
sed -i '' '1s/.*/--- a\/markdown-syntax.md/' delete.patch
sed -i '' '2s/.*/+++ b\/markdown-syntax.md/' delete.patch
sed -i '' '242s/.*/@@ -458,10 +406,8 @@\n/' delete.patch
sed -i '' '244d' delete.patch
sed -i '' '253i\
@@ -468,7 +414,6 @@\
' delete.patch
sed -i '' '379s/.*/@@ -872,8 +799,6 @@\n/' delete.patch
sed -i '' '380d' delete.patch
sed -i '' '388s/.*/@@ -880,7 +805,6 @@\n/' delete.patch
