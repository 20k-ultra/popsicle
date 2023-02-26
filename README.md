# Popsicle

<img align="right" src="icon.png" height="150px" alt="PoPs in the cloud - Generated by DALL-E">

Deploy and run PoP (Points of Presence) profilers around the world to see what latency your users in that region experience.

### Features

- [Profiler](./profiler/)
  - Asynchronous web server profiling library that supports concurrent requests.
  - Benchmarks time to resolve DNS, establish tcp connection, perform tls handshake, and recieve first byte.
- [Lambda Function](./lambda-function/)
  - Build and package Rust Lambda HTTP service to expose profiler
  - Deploy to several regions with 1 command using Terraform
  - Don't pay anything for having profiler's available at several locations
- [CLI](./cli/)
  - Query deployed regions to profile PoPs
  - Output results in JSON, flamegraph, or plotted percentiles

### Setup

This repo contains a Rust crate (Profiler), Rust HTTP service for AWS Lambda runtime, and a CLI.

To starting using Popsicle, you need to deploy the HTTP service to AWS Lambda regions so you can than query them. Therefore, start by looking at the [lambda-function](./lambda-function/#deploying) docs.

Once that is done you can proceed to usage.

### CLI Installaion

```
# Clone this repo
git clone git@github.com:20k-ultra/popsicle.git

# Build and install CLI binary
cargo install --path cli

```

### Example Usage

First Deploy atleast 1 profiler to a region than you can run the cli like so:

```
popsicle-cli --region us-east-1.xgho.st --concurrency 10
```

This command will ask a profiler in us-east-1 region with 10 profling requests to the PoP closest to that region. This effectively tells you what users in this region would experience for latency.
