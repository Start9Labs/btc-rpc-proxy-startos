#!/bin/bash
set -e
yq e -i 'del(.bitcoind)' /root/start9/config.yaml
echo -n '{"configured":false}'