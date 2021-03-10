ASSETS := $(shell yq e '.assets.[].src' manifest.yaml)
ASSET_PATHS := $(addprefix assets/,$(ASSETS))
VERSION := $(shell toml get btc-rpc-proxy/Cargo.toml package.version)
BTC_RPC_PROXY_SRC := $(shell find ./btc-rpc-proxy/src -name '*.rs') btc-rpc-proxy/Cargo.toml btc-rpc-proxy/Cargo.lock
CONFIGURATOR_SRC := $(shell find ./configurator/src -name '*.rs') configurator/Cargo.toml configurator/Cargo.lock

.DELETE_ON_ERROR:

all: btc-rpc-proxy.s9pk

install: btc-rpc-proxy.s9pk
	appmgr install btc-rpc-proxy.s9pk

btc-rpc-proxy.s9pk: manifest.yaml config_spec.yaml config_rules.yaml image.tar instructions.md $(ASSET_PATHS)
	appmgr -vv pack $(shell pwd) -o btc-rpc-proxy.s9pk
	appmgr -vv verify btc-rpc-proxy.s9pk

image.tar: Dockerfile docker_entrypoint.sh configurator/target/armv7-unknown-linux-musleabihf/btc-rpc-proxy
	DOCKER_CLI_EXPERIMENTAL=enabled docker buildx build --tag start9/btc-rpc-proxy --platform=linux/arm/v7 -o type=docker,dest=image.tar .

configurator/target/armv7-unknown-linux-musleabihf/btc-rpc-proxy: $(BTC_RPC_PROXY_SRC) $(CONFIGURATOR_SRC)
	docker run --rm -v ~/.cargo/registry:/root/.cargo/registry -v "$(shell pwd)":/home/rust/src start9/rust-musl-cross:armv7-musleabihf sh -c "cd configurator && cargo +beta build --release"
	docker run --rm -v ~/.cargo/registry:/root/.cargo/registry -v "$(shell pwd)":/home/rust/src start9/rust-musl-cross:armv7-musleabihf musl-strip configurator/target/armv7-unknown-linux-musleabihf/release/btc-rpc-proxy

manifest.yaml: btc-rpc-proxy/Cargo.toml
	yq eval -i '.version = $(VERSION)' manifest.yaml
