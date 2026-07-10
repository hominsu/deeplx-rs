<div id="top"></div>

<!-- PROJECT SHIELDS -->
<p align="center">
<a href="https://github.com/hominsu/deeplx-rs/graphs/contributors"><img src="https://img.shields.io/github/contributors/hominsu/deeplx-rs.svg?style=for-the-badge" alt="Contributors"></a>
<a href="https://github.com/hominsu/deeplx-rs/network/members"><img src="https://img.shields.io/github/forks/hominsu/deeplx-rs.svg?style=for-the-badge" alt="Forks"></a>
<a href="https://github.com/hominsu/deeplx-rs/stargazers"><img src="https://img.shields.io/github/stars/hominsu/deeplx-rs.svg?style=for-the-badge" alt="Stargazers"></a>
<a href="https://github.com/hominsu/deeplx-rs/issues"><img src="https://img.shields.io/github/issues/hominsu/deeplx-rs.svg?style=for-the-badge" alt="Issues"></a>
<a href="https://github.com/hominsu/deeplx-rs/blob/master/LICENSE"><img src="https://img.shields.io/github/license/hominsu/deeplx-rs.svg?style=for-the-badge" alt="License"></a>
<a href="https://github.com/hominsu/deeplx-rs/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/hominsu/deeplx-rs/ci.yml?branch=main&style=for-the-badge" alt="Build"></a>
</p>


<!-- PROJECT LOGO -->
<br/>
<div align="center">
<h3 align="center">deeplx-rs</h3>
  <p align="center">
    A Rust package for unlimited DeepL translation
    <br/>
    <br/>
    <a href="#features">Features</a>
    ·
    <a href="#usage">Usage</a>
    ·
    <a href="#Integration">Integration</a>
    ·
    <a href="#Reference">Reference</a>
    ·
    <a href="#license">License</a>
  </p>
</div>

## Features

- **DeepL Chrome extension oneshot transport**.
- **Typed library API** with `Client`, `TranslateRequest`, `Auth`, and strong errors.
- **Optional server binary** behind the `server` feature.
- **Proxy support** on native targets.

## Usage

### Docker Deployment

1. Clone the repository:
    ```shell
    git clone https://github.com/hominsu/deeplx-rs.git
    ```
2. Start the container:
    ```shell
    cd deploy
    docker-compose up -d
    ```

### Install with Cargo

```shell
cargo install deeplx --features server,mimalloc
```

## Integration

### Installation

Add `deeplx` to your `Cargo.toml`:

```toml
[dependencies]
deeplx = { version = "3", default-features = false }
```

### Configuration

deeplx is configured through `Client::builder()`:

```rust
use std::time::Duration;

use deeplx::{Auth, Client};

let client = Client::builder()
    .auth(Auth::Anonymous)
    .timeout(Duration::from_secs(20))
    .proxy("http://127.0.0.1:7890".parse().unwrap())
    .build()?;
```

For Pro requests, use a Bearer token:

```rust
use deeplx::{Auth, Client};
use secrecy::SecretString;

let client = Client::builder()
    .auth(Auth::Bearer(SecretString::from("token".to_string())))
    .build()?;
```

### Usage

Below is an example using tokio for async execution:

```rust
use deeplx::{Auth, Client, SourceLang, TargetLang, TranslateRequest};

#[tokio::main]
async fn main() -> Result<(), deeplx::Error> {
    let client = Client::builder().auth(Auth::Anonymous).build()?;
    let request = TranslateRequest::builder()
        .text("Hello, world!")?
        .source(SourceLang::Auto)
        .target(TargetLang::parse("ZH-HANS")?)
        .build()?;

    let response = client.translate(request).await?;
    println!("{}", response.translations[0].text);

    Ok(())
}
```

### Server

The HTTP server is optional:

```shell
cargo run --features server,mimalloc -- run --conf ./configs
```

`/translate` uses anonymous oneshot requests. `/v1/translate` accepts
`Authorization: Bearer <token>` for Pro requests, with temporary compatibility
for `Cookie: dl_session=<token>`.

## Reference

- [OwO-Network/DeepLX](https://github.com/OwO-Network/DeepLX)

## License

Distributed under the MIT License. See `LICENSE` for more information.
