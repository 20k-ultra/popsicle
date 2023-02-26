# Lambda Function

This HTTP interface accepts 2 query parameters `domain` and `concurrency`. Returns `Vec<Benchmark>` serialized JSON.

Deplyoment can be automatically deployed to several regions with the included [Terraform](https://github.com/hashicorp/terraform) files. See [Deploying](#deploying).

### Example Usage

Install [cargo-lambda](https://www.cargo-lambda.info/guide/installation.html) for a local Lambda runner.

```
# See other installation methods if you don't want to do it this way
pip3 install cargo-lambda

# Start a local runner
cargo lambda watch
```

Now you can query the local running instance:

```bash
~$ curl -X POST "localhost:9000/lambda-url/bootstrap/?domain=xgho.st&concurrency=3" | jq
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100   642  100   642    0     0   1716      0 --:--:-- --:--:-- --:--:--  1716
[
  {
    "dns_resolution": {
      "secs": 0,
      "nanos": 239227916
    },
    "tcp_handshake": {
      "secs": 0,
      "nanos": 21278493
    },
    "tls_handshake": {
      "secs": 0,
      "nanos": 49971361
    },
    "first_byte": {
      "secs": 0,
      "nanos": 28711364
    },
    "total": {
      "secs": 0,
      "nanos": 339189134
    }
  },
  {
    "dns_resolution": {
      "secs": 0,
      "nanos": 101273355
    },
    "tcp_handshake": {
      "secs": 0,
      "nanos": 15834533
    },
    "tls_handshake": {
      "secs": 0,
      "nanos": 84542919
    },
    "first_byte": {
      "secs": 0,
      "nanos": 34033797
    },
    "total": {
      "secs": 0,
      "nanos": 235684604
    }
  },
  {
    "dns_resolution": {
      "secs": 0,
      "nanos": 27276411
    },
    "tcp_handshake": {
      "secs": 0,
      "nanos": 19636259
    },
    "tls_handshake": {
      "secs": 0,
      "nanos": 46505330
    },
    "first_byte": {
      "secs": 0,
      "nanos": 76646123
    },
    "total": {
      "secs": 0,
      "nanos": 170064123
    }
  }
]
```

### Deploying

```bash
# Install cross for easier builds
cargo install cross

# Make a build targeting x86_64-unknown-linux-gnu
cross build --target x86_64-unknown-linux-gnu --release

# Create payload to upload
zip -r9 -j bootstrap.zip ./target/x86_64-unknown-linux-gnu/release/bootstrap

# Run terraform
terraform init
terraform apply
```

Test one of the endpoints like so:

```bash
$ curl -X POST "https://elex62ivj52ttxthgacylfksey0jkjkl.lambda-url.us-east-1.on.aws?domain=xgho.st&concurrency=3"
```
