FROM rust:alpine as rust-build
RUN apk add musl-dev
WORKDIR /rust
ADD bollard-stubs /rust/bollard-stubs
COPY Cargo.* /rust/
COPY src /rust/src
RUN cargo build --release

FROM alpine
ARG S6_OVERLAY_VERSION=3.0.0.2-2
ADD https://github.com/just-containers/s6-overlay/releases/download/v${S6_OVERLAY_VERSION}/s6-overlay-noarch-${S6_OVERLAY_VERSION}.tar.xz /tmp
ADD https://github.com/just-containers/s6-overlay/releases/download/v${S6_OVERLAY_VERSION}/s6-overlay-x86_64-${S6_OVERLAY_VERSION}.tar.xz /tmp
RUN \
  tar -C / -Jxpf /tmp/s6-overlay-noarch-${S6_OVERLAY_VERSION}.tar.xz && \
  tar -C / -Jxpf /tmp/s6-overlay-x86_64-${S6_OVERLAY_VERSION}.tar.xz && \
  rm /tmp/*.tar.xz && \
  apk add --no-cache coredns

COPY --from=rust-build /rust/target/*/localns /bin/localns
COPY etc /etc/

ENV LOCALNS_CONFIG=/etc/localns/config.yaml
EXPOSE 53/udp
EXPOSE 53/tcp

ENTRYPOINT ["/init"]
