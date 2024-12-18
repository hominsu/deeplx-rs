<div id="top"></div>

<!-- PROJECT SHIELDS -->
<p align="center">
<a href="https://github.com/hominsu/deeplx-rs/graphs/contributors"><img src="https://img.shields.io/github/contributors/hominsu/deeplx-rs.svg?style=for-the-badge" alt="Contributors"></a>
<a href="https://github.com/hominsu/deeplx-rs/network/members"><img src="https://img.shields.io/github/forks/hominsu/deeplx-rs.svg?style=for-the-badge" alt="Forks"></a>
<a href="https://github.com/hominsu/deeplx-rs/stargazers"><img src="https://img.shields.io/github/stars/hominsu/deeplx-rs.svg?style=for-the-badge" alt="Stargazers"></a>
<a href="https://github.com/hominsu/deeplx-rs/issues"><img src="https://img.shields.io/github/issues/hominsu/deeplx-rs.svg?style=for-the-badge" alt="Issues"></a>
<a href="https://github.com/hominsu/deeplx-rs/blob/master/LICENSE"><img src="https://img.shields.io/github/license/hominsu/deeplx-rs.svg?style=for-the-badge" alt="License"></a>
<a href="https://github.com/hominsu/deeplx-rs/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/hominsu/deeplx-rs/ci.yml?branch=main&style=for-the-badge" alt="Build"></a>
<a href="https://app.fossa.com/projects/git%2Bgithub.com%2Fhominsu%2Fdeeplx-rs?ref=badge_shield" alt="FOSSA Status"><img src="https://app.fossa.com/api/projects/git%2Bgithub.com%2Fhominsu%2Fdeeplx-rs.svg?type=shield"/></a>
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
    <a href="#license">License</a>
  </p>
</div>


[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fhominsu%2Fdeeplx-rs.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2Fhominsu%2Fdeeplx-rs?ref=badge_large)

## Features

- **Docker support** for easy deployment.
- **Proxy support** by default (can be disabled via features).
- **Optional impersonation** using the `impersonate` feature to mimic browser settings.

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
cargo install --features=impersonate,server
```

## Integration

### Installation

Add `deeplx` to your `Cargo.toml`:

```toml
[dependencies]
deeplx = "1"
```

By default, deeplx includes proxy support. If you do not need proxy support, disable the default features:

```toml
[dependencies]
deeplx = { version = "1", default-features = false }
```

If you want to enable the impersonate feature to mimic browser headers, TLS settings, etc.:

```toml
[dependencies]
deeplx = { version = "1", features = ["impersonate"] }
```

### Configuration

deeplx is configured via the Config struct. You can specify options such as proxy, timeout, and more. For example:

```rust
use deeplx::{Config, DeepLX};

let translator = DeepLX::new(Config {
proxy: Some("http://pro.xy".to_string()),
..Default::default ()
});
```

If you have disabled the proxy feature, you can simply omit the proxy field:

```rust
use deeplx::{Config, DeepLX};

let translator = DeepLX::new(Config::default ());
```

### Usage

Below is an example using tokio for async execution:

```rust
use deeplx::{Config, DeepLX};

#[tokio::main]
async fn main() {
    let translator = DeepLX::new(Config {
        proxy: Some("http://pro.xy".to_string()),
        ..Default::default()
    });
    // Or without proxy:
    // let translator = DeepLX::new(Config::default());

    match translator.translate("auto", "zh", "Hello, world!", None, None).await {
        Ok(res) => println!("Translated: {}", res.data),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

## License

Distributed under the MIT License. See `LICENSE` for more information.