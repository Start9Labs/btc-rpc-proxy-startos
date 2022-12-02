BTC_RPC_PROXY_SRC := $(shell find ./btc-rpc-proxy/src -name '*.rs') btc-rpc-proxy/Cargo.toml btc-rpc-proxy/Cargo.lock
CONFIGURATOR_SRC := $(shell find ./configurator/src -name '*.rs') configurator/Cargo.toml configurator/Cargo.lock
PKG_VERSION := $(shell yq e ".version" manifest.yaml)
PKG_ID := $(shell yq e ".id" manifest.yaml)
SCRIPTS_SRC := $(shell find ./scripts -name '*.ts')
S9PK_PATH=$(shell find . -name btc-rpc-proxy.s9pk -print)

.DELETE_ON_ERROR:

all: verify

clean:
	rm -f $(PKG_ID).s9pk
	rm -f docker-images/*.tar
	rm -f scripts/*.js

verify: btc-rpc-proxy.s9pk $(S9PK_PATH)
	embassy-sdk verify s9pk $(S9PK_PATH)

$(PKG_ID).s9pk: manifest.yaml docker-images/aarch64.tar docker-images/x86_64.tar instructions.md LICENSE icon.png scripts/embassy.js
	embassy-sdk pack

docker-images/aarch64.tar: Dockerfile docker_entrypoint.sh check-rpc.sh configurator/target/aarch64-unknown-linux-musl/btc-rpc-proxy
	mkdir -p docker-images
	docker buildx build --tag start9/$(PKG_ID)/main:$(PKG_VERSION) --build-arg BITCOIN_VERSION=$(VERSION_CORE) --build-arg ARCH=aarch64 --build-arg PLATFORM=arm64 --platform=linux/arm64 -o type=docker,dest=docker-images/aarch64.tar .

docker-images/x86_64.tar: Dockerfile docker_entrypoint.sh check-rpc.sh configurator/target/x86_64-unknown-linux-musl/btc-rpc-proxy
	mkdir -p docker-images
	docker buildx build --tag start9/$(PKG_ID)/main:$(PKG_VERSION) --build-arg BITCOIN_VERSION=$(VERSION_CORE) --build-arg ARCH=x86_64 --build-arg PLATFORM=amd64 --platform=linux/amd64 -o type=docker,dest=docker-images/x86_64.tar .

configurator/target/aarch64-unknown-linux-musl/btc-rpc-proxy: $(BTC_RPC_PROXY_SRC) $(CONFIGURATOR_SRC)
	docker run --rm -v ~/.cargo/registry:/root/.cargo/registry -v "$(shell pwd)":/home/rust/src start9/rust-musl-cross:aarch64-musl sh -c "cd configurator && cargo +beta build --release"

configurator/target/x86_64-unknown-linux-musl/btc-rpc-proxy: $(BTC_RPC_PROXY_SRC) $(CONFIGURATOR_SRC)
	docker run --rm -v ~/.cargo/registry:/root/.cargo/registry -v "$(shell pwd)":/home/rust/src start9/rust-musl-cross:x86_64-musl sh -c "cd configurator && cargo +beta build --release"

scripts/embassy.js: $(SCRIPTS_SRC)
	deno bundle scripts/embassy.ts scripts/embassy.js
