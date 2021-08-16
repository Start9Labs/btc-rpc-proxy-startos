# Build stage for compiled artifacts
FROM alpine:latest

RUN apk update
RUN apk add tini

ADD ./configurator/target/aarch64-unknown-linux-musl/release/btc-rpc-proxy /usr/local/bin/btc-rpc-proxy
RUN chmod a+x /usr/local/bin/btc-rpc-proxy
ADD ./docker_entrypoint.sh /usr/local/bin/docker_entrypoint.sh
RUN chmod a+x /usr/local/bin/docker_entrypoint.sh

EXPOSE 8332

ENTRYPOINT ["/usr/local/bin/docker_entrypoint.sh"]
