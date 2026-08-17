# AWS Libcrypto for Rust ({{crate}})

[![Crates.io](https://img.shields.io/crates/v/aws-lc-rs.svg)](https://crates.io/crates/aws-lc-rs)
[![GitHub](https://img.shields.io/badge/GitHub-aws%2Faws--lc--rs-blue)](https://github.com/aws/aws-lc-rs)

{{readme}}

### Contributor Quickstart for Amazon Linux 2023

For those who would like to contribute to our project or build it directly from our repository,
a few more packages may be needed. The listing below shows the steps needed for you to begin
building and testing our project locally.

```shell
# Install dependencies needed for build and testing
sudo yum install -y cmake3 clang git clang-libs golang openssl-devel perl-FindBin

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"

# Clone and initialize a local repository
git clone https://github.com/aws/aws-lc-rs.git
cd aws-lc-rs
git submodule update --init --recursive

# Build and test the project
cargo test

```

## Questions, Feedback and Contributing

* [Submit an non-security Bug/Issue/Request](https://github.com/aws/aws-lc-rs/issues/new/choose)
* [API documentation](https://docs.rs/aws-lc-rs/)
* [Fork our repo](https://github.com/aws/aws-lc-rs/fork)

We use [GitHub Issues](https://github.com/aws/aws-lc-rs/issues/new/choose) for managing feature requests, bug
reports, or questions about aws-lc-rs API usage.

Otherwise, if you think you might have found a security impacting issue, please instead
follow our *Security Notification Process* below.

## Security Notification Process

If you discover a potential security issue in *AWS-LC* or *aws-lc-rs*, we ask that you notify AWS
Security via our
[vulnerability reporting page](https://aws.amazon.com/security/vulnerability-reporting/).
Please do **not** create a public GitHub issue.

If you package or distribute *aws-lc-rs*, or use *aws-lc-rs* as part of a large multi-user service,
you may be eligible for pre-notification of future *aws-lc-rs* releases.
Please contact aws-lc-pre-notifications@amazon.com.

## License

This library is licensed under the Apache-2.0 or the ISC License.
