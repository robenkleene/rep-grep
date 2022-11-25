#!/usr/bin/env bash

set -euo pipefail

grep --line-number --with-filename Markdown markdown-syntax.md > grep.txt
diff markdown-syntax.md <(sed s/Markdown/Markup/g markdown-syntax.md)
