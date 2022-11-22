#!/bin/bash

curl --silent btc-rpc-proxy.embassy:8332 &>/dev/null
RES=$?
if test "$RES" != 0; then
    echo "RPC interface is unreachable" >&2
    exit 1
fi
