#!/bin/bash

DURATION=$(</dev/stdin)
if (($DURATION <= 5000 )); then
    exit 60
else
    curl --silent btc-rpc-proxy.embassy:8332 &>/dev/null
    RES=$?
    if test "$RES" != 0; then
        echo "RPC interface is unreachable" >&2
        exit 1
    fi
fi
