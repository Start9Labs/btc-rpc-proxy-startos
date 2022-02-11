ASSET_PATHS := $(shell find ./assets/*)
VERSION := $(shell toml get btc-rpc-proxy/Cargo.toml package.version)
EMVER := $(shell yq e ".version" manifest.yaml)
BTC_RPC_PROXY_SRC := $(shell find ./btc-rpc-proxy/src -name '*.rs') btc-rpc-proxy/Cargo.toml btc-rpc-proxy/Cargo.lock
CONFIGURATOR_SRC := $(shell find ./configurator/src -name '*.rs') configurator/Cargo.toml configurator/Cargo.lock
S9PK_PATH=$(shell find . -name btc-rpc-proxy.s9pk -print)

.DELETE_ON_ERROR:

all: verify

verify: btc-rpc-proxy.s9pk $(S9PK_PATH)
	embassy-sdk verify s9pk $(S9PK_PATH)

btc-rpc-proxy.s9pk: manifest.yaml image.tar instructions.md LICENSE icon.png $(ASSET_PATHS)
	embassy-sdk pack

image.tar: Dockerfile docker_entrypoint.sh check-rpc.sh configurator/target/aarch64-unknown-linux-musl/btc-rpc-proxy
	DOCKER_CLI_EXPERIMENTAL=enabled docker buildx build --tag start9/btc-rpc-proxy/main:${EMVER} --platform=linux/arm64/v8 -o type=docker,dest=image.tar .

configurator/target/aarch64-unknown-linux-musl/btc-rpc-proxy: $(BTC_RPC_PROXY_SRC) $(CONFIGURATOR_SRC)
	docker run --rm -v ~/.cargo/registry:/root/.cargo/registry -v "$(shell pwd)":/home/rust/src start9/rust-musl-cross:aarch64-musl sh -c "cd configurator && cargo +beta build --release"
	docker run --rm -v ~/.cargo/registry:/root/.cargo/registry -v "$(shell pwd)":/home/rust/src start9/rust-musl-cross:aarch64-musl musl-strip configurator/target/aarch64-unknown-linux-musl/release/btc-rpc-proxy

# manifest.yaml: btc-rpc-proxy/Cargo.toml
# 	yq eval -i '.version = $(VERSION)' manifest.yaml
