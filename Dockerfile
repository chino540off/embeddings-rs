FROM nvidia/cuda:12.8.0-devel-ubuntu24.04 AS builder

ARG RUST_VERSION

WORKDIR /src

RUN apt update && \
    apt install -y \
      curl \
      libssl-dev \
    && \
    rm -rf /var/lib/apt/lists/* \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
      --default-toolchain $RUST_VERSION \
      --component cargo,rustc,rust-std

COPY . .
ENV CUDA_COMPUTE_CAP=75
RUN . /root/.cargo/env && \
    apt install -y \
      pkg-config \
    && \
    cargo install --path embeddings-server --features cuda

FROM nvidia/cuda:12.8.0-runtime-ubuntu24.04
#RUN apt-get update && \
#    apt-get install -y \
#      extra-runtime-dependencies \
#    && \
#    rm -rf /var/lib/apt/lists/*

COPY --from=builder /root/.cargo/bin/embeddings-server /usr/local/bin/embeddings-server
ENTRYPOINT ["embeddings-server"]
