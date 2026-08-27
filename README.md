# Velocity

A network diagnostics and speed testing utility written in Rust.

Velocity measures network latency and throughput using HTTP and ICMP tests, providing a simple overview of your connection's performance.

## Features

- HTTP latency testing
  - Minimum latency
  - Maximum latency
  - Average latency
  - Jitter
  - Failed requests

- ICMP latency testing
  - Minimum RTT
  - Maximum RTT
  - Average RTT
  - Jitter
  - Failed requests

- Download speed testing
  - Multiple payload sizes
  - Minimum speed
  - Maximum speed
  - Average speed
  - Failed requests

- Upload speed testing
  - Multiple payload sizes
  - Minimum speed
  - Maximum speed
  - Average speed
  - Failed requests

- Configurable test multiplier
- Uses Cloudflare's speed test endpoints
- Built with Rust

## Example

```text
VELOCITY V1
==================
   HTTP LATENCY
==================
max: 127.84
min: 38.21
avg: 54.63
jitter: 18.42
failure count: 0
==================
==================
   ICMP LATENCY
==================
max: 31.42
min: 26.18
avg: 27.04
jitter: 1.21
failure count: 0
==================
==================
  DOWNLOAD SPEED
==================
Max Speed: 116.52 Mbps
Min Speed: 89.20 Mbps
Avg Speed: 98.64 Mbps
Fail count: 0
==================
   UPLOAD SPEED
==================
Max Speed: 101.31 Mbps
Min Speed: 82.47 Mbps
Avg Speed: 94.72 Mbps
Fail count: 0
```

# How It Works

## Velocity performs several network tests using different techniques.

### HTTP Latency

Velocity sends HTTP HEAD requests to Cloudflare's speed test service and measures the time taken for each request.

The collected measurements are used to calculate:

- Minimum latency
- Maximum latency
- Average latency
- Jitter

### ICMP Latency

Velocity sends ICMP echo requests to 1.1.1.1 and measures the round-trip time of each packet.

### Download

Velocity downloads data from Cloudflare using different payload sizes:

| Payload | Tests |
|---------|-------|
| 1 KB | 5 |
| 1 MB | 4 |
| 10 MB | 3 |
| 25 MB | 2 |
| 50 MB | 2 |

The test can be repeated using the multiplier setting.

### Upload

Velocity generates a payload of the requested size and uploads it to Cloudflare's upload endpoint.

The upload and download measurements record the amount of data transferred and the time taken to calculate throughput in Mbps.

# Installation
## Requirements
- Rust
- Cargo
- Internet connection (duh!)

## Clone the repository:

```bash
git clone https://github.com/<your-username>/velocity.git
cd velocity
```

## Build and run:

```bash
cargo run --release
```

# Development

 Build the project:

```bash
cargo build
```

## Run in development mode:

```bash
cargo run
```

## Run the tests:

```bash
cargo test
```

## Check the project with Clippy:

```bash
cargo clippy
```

## Format the code:

```bash
cargo fmt
```

# Project Status

## Velocity is currently in early development!

The current version focuses on getting the core network measurements working. Results are not intended to be a perfect replacement for dedicated services such as Speedtest or Cloudflare's own speed test.

Planned improvements include:

 - Rate-limit handling
 - More reliable upload measurements
 - Configurable test parameters
 - Verbose output
 - Live terminal UI using Ratatui
 - Per-test statistics
 - Better error handling
 - More accurate latency/jitter measurements
 - Improved result aggregation
 - IPv4/IPv6 support

# Why?

This project started as a way to learn networking using C++, which eventually transformed into a way to learn Rust by rebuilding the project from the ground up and using it to explore the language's features and design.

Rather than using an existing speed-testing application, Velocity implements the measurements directly to explore concepts such as:

- Rust ownership and borrowing
- Generics and trait bounds
- Closures
- Error handling with Result
- Iterators
- HTTP networking
- ICMP networking
- Performance measurement
- Terminal user interfaces

# License
This project is licensed under the MIT License. See LICENSE for details.
