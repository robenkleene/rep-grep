#!/usr/bin/env bash

set -euo pipefail

grep --line-number --with-filename Markdown markdown-syntax.md > grep.txt
diff --unified markdown-syntax.md <(sed s/Markdown/Markup/g markdown-syntax.md) > markdown-markup.patch || true
