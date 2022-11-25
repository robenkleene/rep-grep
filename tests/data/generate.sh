#!/usr/bin/env bash

set -euo pipefail

grep --line-number --with-filename Markdown markdown-syntax.md > grep.txt
