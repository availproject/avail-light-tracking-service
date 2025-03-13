# Avail Light Tracking Service

## Introduction

Avail Light Tracking Service is used for tracking fleets of Avail Light Clients (LC). 
If enabled by the Avail LCs they send a periodic heartbeat message on the `/ping` endpoint with a signed payload containing LC relevant data - public multi-address, peerID and the latest block number.

The Avail Tracking Service verifies the incoming ping using the SS58 encoded LCs public key and stores it locally in RocksDB. Tracking Service data is persisted across restarts (WIP).

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


## API endpoints

`/ping` - Avail Light Client delivers its ping payload using a POST method to this endpoint

`/client-info/{public_key}` - GET request to this endpoint returns information about the light client with the provided public key


## Notes

- LC Tracker uses RocksDB for data storage and persistence across sessions. Default database location is in `./ping_db`.
- Though the Tracking Service data is persisted across restarts, the eventual gaps are not accounted for - work in progress. The service delivers just the first and last seen data, without any details of the frequency of heartbeats in between.



