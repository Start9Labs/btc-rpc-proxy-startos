#!/bin/bash
set -e
yq e -i '.bitcoind = {"user":null,"password":null}' /root/start9/config.yaml
echo -n '{"configured":false}'