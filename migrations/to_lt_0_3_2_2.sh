#!/bin/bash
set -e
yq e -i '.bitcoind.type = "internal"' /root/start9/config.yaml
echo -n '{"configured":true}'