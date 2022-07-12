#!/bin/bash
set -e
yq e -i '.bitcoind = {}' /root/start9/config.yaml
echo -n '{"configured":true}'