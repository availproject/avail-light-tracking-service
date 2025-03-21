# Avail Light Tracking Service

## Introduction

Avail Light Tracking Service is used for tracking fleets of Avail Light Clients (LC).
If enabled by the Avail LCs they send a periodic heartbeat message on the `/ping` endpoint with a signed payload containing LC relevant data - public multi-address, peerID and the latest block number.

The Avail Tracking Service verifies the incoming ping using the SS58 encoded LCs public key and stores it locally in RocksDB. Tracking Service data is persisted across restarts (WIP).

## ⚠️ Warning

**This is an alpha version of the Avail Light Tracking Service. Additional features are currently in development, and bugs are to be expected. Use in production environments is not recommended at this stage.**

## Setup

Build the tracker from source:

```sh
git clone git@github.com:availproject/avail-light-tracking-service.git
cd avail-light-tracking-service
cargo build --release
```

Run the service:

```sh
./target/release/avail-light-tracking-service --verbosity trace
```

For more CLI options run:

```sh
./target/release/avail-light-tracking-service --help
```

## Enabling tracking on the Avail Light Client

Run the Avail Light Client directly with the following flags:

```sh
--tracking-service-enable --tracking-service-address "http://127.0.0.1:8989"
```

Or, run the `availup` tool with the following flags:

```sh
--tracking_service y --tracking_service_address "http://127.0.0.1:8989"
```

The Avail Light Client ss58 address (used for querying LC data) can be found in the logs with the following line:

`2025-03-20T15:39:18.698513Z  INFO avail_light_client: Avail ss58 address: 5GELNf6s8m7cK6qsUysmpPGPwLp6tUGFCTZ3D64PdZ5cwumA, public key: b84911dead0b1bf7b212da759fce35d25cc0ac84c4af0faf69b10971a514df4c`

Or, in the `identity.toml` file in the LC working directory

## API endpoints

### GET /client-info/{public_key}

Retrieves information about a light client identified by its public key (ss58 encoded format).

### POST /ping

Endpoint for Avail Light Client to deliver its ping payload.

## Notes

- LC Tracker uses RocksDB for data storage and persistence across sessions. Default database location is in `./ping_db`.
- Though the Tracking Service data is persisted across restarts, the eventual gaps are not accounted for - work in progress. The service delivers just the first and last seen data, without any details of the frequency of heartbeats in between.
