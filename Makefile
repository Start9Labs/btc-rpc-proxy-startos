ASSETS := $(shell yq r manifest.yaml assets.*.src)
ASSET_PATHS := $(addprefix assets/,$(ASSETS))
VERSION := $(shell yq r manifest.yaml version)

.DELETE_ON_ERROR:

all: btc-rpc-proxy.s9pk

install: btc-rpc-proxy.s9pk
	appmgr install btc-rpc-proxy.s9pk

btc-rpc-proxy.s9pk: manifest.yaml config_spec.yaml config_rules.yaml image.tar instructions.md $(ASSET_PATHS)
	appmgr -vv pack $(shell pwd) -o btc-rpc-proxy.s9pk
	appmgr -vv verify btc-rpc-proxy.s9pk

image.tar: Dockerfile docker_entrypoint.sh
	DOCKER_CLI_EXPERIMENTAL=enabled docker buildx build --tag start9/btc-rpc-proxy --platform=linux/arm/v7 -o type=docker,dest=image.tar .

