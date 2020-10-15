ASSETS := $(shell yq r manifest.yaml assets.*.src)
ASSET_PATHS := $(addprefix assets/,$(ASSETS))
VERSION := $(shell yq r manifest.yaml version)
BTC_RPC_PROXY_SRC := $(shell find ./btc-rpc-proxy/src -name '*.rs') btc-rpc-proxy/Cargo.toml btc-rpc-proxy/Cargo.lock

.DELETE_ON_ERROR:

all: btc-rpc-proxy.s9pk

install: btc-rpc-proxy.s9pk
	appmgr install btc-rpc-proxy.s9pk

btc-rpc-proxy.s9pk: manifest.yaml config_spec.yaml config_rules.yaml image.tar instructions.md $(ASSET_PATHS)
	appmgr -vv pack $(shell pwd) -o btc-rpc-proxy.s9pk
	appmgr -vv verify btc-rpc-proxy.s9pk

image.tar: Dockerfile docker_entrypoint.sh btc-rpc-proxy/target/armv7-unknown-linux-musleabihf/btc-rpc-proxy
	DOCKER_CLI_EXPERIMENTAL=enabled docker buildx build --tag start9/btc-rpc-proxy --platform=linux/arm/v7 -o type=docker,dest=image.tar .

btc-rpc-proxy/target/armv7-unknown-linux-musleabihf/btc-rpc-proxy: $(BTC_RPC_PROXY_SRC)
	docker run --rm -it -v ~/.cargo/registry:/root/.cargo/registry -v "$(shell pwd)"/btc-rpc-proxy:/home/rust/src start9/rust-musl-cross:armv7-musleabihf cargo +beta build --release --features=start9 --no-default-features
	docker run --rm -it -v ~/.cargo/registry:/root/.cargo/registry -v "$(shell pwd)"/btc-rpc-proxy:/home/rust/src start9/rust-musl-cross:armv7-musleabihf musl-strip target/armv7-unknown-linux-musleabihf/release/btc-rpc-proxy
