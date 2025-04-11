FROM rust:1.86-slim AS builder

WORKDIR /usr/src/avail-tracker

# Install dependencies required for building
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev git build-essential \
    libclang-dev llvm-dev clang && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

RUN git clone https://github.com/availproject/avail-light-tracking-service .

RUN cargo build --release

# Runtime stage
FROM debian:stable-slim

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    libssl-dev \
    ca-certificates \
    curl && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

RUN groupadd -r avail && useradd -r -g avail -m -d /home/avail avail

WORKDIR /avail-tracker

COPY --from=builder /usr/src/avail-tracker/target/release/avail-light-tracking-service /avail-tracker/

RUN mkdir -p /avail-tracker/data && \
    chown -R avail:avail /avail-tracker

ENV SERVER_ADDR="0.0.0.0"
ENV SERVER_PORT="8989"
ENV DB_PATH="/avail-tracker/data"
ENV VERBOSITY="debug"

USER avail

EXPOSE ${SERVER_PORT}

VOLUME ["/avail-tracker/data"]

HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
    CMD curl -f http://${SERVER_ADDR}:${SERVER_PORT}/ || exit 1

ENTRYPOINT ["/bin/sh", "-c", "/avail-tracker/avail-light-tracking-service \
    --server-addr ${SERVER_ADDR} \
    --server-port ${SERVER_PORT} \
    --db-path ${DB_PATH} \
    --verbosity ${VERBOSITY}"]