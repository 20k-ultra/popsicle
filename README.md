# Popsicle

<img align="right" src="icon.png" height="150px" alt="the deno mascot dinosaur standing in the rain">

Deploy and run PoP (Points of Presence) profilers around the world to see what latency your users in that region experience.

### Features

- [Profiler](./profiler/)
  - Benchmarks time to establish tcp connection, tls handshake, and recieve first byte.
  - Supports concurrent requests to PoP node
- [CLI](./cli/)
  - Query deployed regions to profile PoPs
  - Output results in JSON, flamegraph, or plotted percentiles
- [Automated Deploy](./deploy/)
  - Deploy profilers to several regions with 1 command using Terraform
  - With anycast networks you must run the profiler in regions you want to test
  - Don't pay anything for having profiler's available at several locations

### Usage

First Deploy atleast 1 profiler to a region than you can run the cli like so:

```
popsicle --region us-east-1.xgho.st --concurrency 10
```

This command will ask a profiler in us-east-1 region with 10 profling requests to the PoP closest to that region. This effectively tells you what users in this region would experience for latency.

### Deploy

```
terraform deploy
```
