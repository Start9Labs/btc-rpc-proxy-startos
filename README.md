# Wrapper for Bitcoin Core RPC Proxy

This project wraps [BTC RPC Proxy](https://github.com/Kixunil/btc-rpc-proxy) for EmbassyOS. BTC RPC Proxy enables a finer-grained permission management for bitcoin and fetching of requested blocks for a pruned node.

## Dependencies

- [docker](https://docs.docker.com/get-docker)
- [docker-buildx](https://docs.docker.com/buildx/working-with-buildx/)
- [yq](https://mikefarah.gitbook.io/yq)
- [embassy-sdk](https://github.com/Start9Labs/embassy-os/blob/master/backend/install-sdk.sh)
- [make](https://www.gnu.org/software/make/)

## Cloning

Clone the project locally. Note the submodule link to the original project(s). 

```
git clone git@github.com:Start9Labs/btc-rpc-proxy-wrapper.git
cd btc-rpc-proxy-wrapper
git submodule update --init

```

## Building

```
make
```

## Installing (on Embassy)

```
scp btc-rpc-proxy.s9pk root@embassy-<id>.local:/embassy-data/package-data/tmp # Copy S9PK to the external disk. Make sure to create the directory if it doesn't already exist
ssh root@embassy-<id>.local
embassy-cli auth login
embassy-cli package install /embassy-data/pacakge-data/tmp/btc-rpc-proxy.s9pk # Install the sideloaded package
```
