FROM rust:1.86-slim AS builder

WORKDIR /usr/src/avail-tracker
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev build-essential \
    libclang-dev llvm-dev clang && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

COPY . .

RUN cargo build --release

FROM debian:stable-slim AS runner
WORKDIR /avail-tracker
ENV SERVER_ADDR="0.0.0.0"
ENV SERVER_PORT="8989"
ENV DB_PATH="/avail-tracker/data"
ENV VERBOSITY="debug"

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    libssl-dev \
    ca-certificates \
    curl && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* && \
    groupadd -r avail && \
    useradd -r -g avail -m -d /home/avail avail && \
    mkdir -p /avail-tracker/data && \
    chown -R avail:avail /avail-tracker

COPY --from=builder /usr/src/avail-tracker/target/release/avail-light-tracking-service /avail-tracker/

USER avail
EXPOSE ${SERVER_PORT}
VOLUME ["/avail-tracker/data"]

HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
    CMD curl -f http://${SERVER_ADDR}:${SERVER_PORT}/status || exit 1

ENTRYPOINT ["/bin/sh", "-c"]
CMD ["/avail-tracker/avail-light-tracking-service", "--server-addr ${SERVER_ADDR}", "--server-port ${SERVER_PORT}", "--db-path ${DB_PATH}", "--verbosity ${VERBOSITY}"]
