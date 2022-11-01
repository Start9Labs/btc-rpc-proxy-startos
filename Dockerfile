# Build stage for compiled artifacts
FROM alpine:latest

RUN apk update
RUN apk add bash curl tini
RUN wget https://github.com/mikefarah/yq/releases/download/v4.12.2/yq_linux_arm.tar.gz -O - |\
    tar xz && mv yq_linux_arm /usr/bin/yq

ADD ./configurator/target/aarch64-unknown-linux-musl/release/btc-rpc-proxy /usr/local/bin/btc-rpc-proxy
RUN chmod a+x /usr/local/bin/btc-rpc-proxy
ADD ./docker_entrypoint.sh /usr/local/bin/docker_entrypoint.sh
RUN chmod a+x /usr/local/bin/docker_entrypoint.sh
ADD ./check-rpc.sh /usr/local/bin/check-rpc.sh
RUN chmod a+x /usr/local/bin/check-rpc.sh

EXPOSE 8332

ENTRYPOINT ["/usr/local/bin/docker_entrypoint.sh"]
