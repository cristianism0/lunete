#!/bin/sh
set -e
BIN="${1:-target/debug/lunete}"
setcap cap_dac_read_search+ep "$BIN"
echo "capability aplicada em $BIN"
