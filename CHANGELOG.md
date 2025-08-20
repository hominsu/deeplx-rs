# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v2.0.0 (2025-08-20)

### Bug Fixes

 - <csr-id-919af091e304ec98220aec60118b5d12ab4a9e7b/> lint
 - <csr-id-ce47124491b862d767966d2abca5ad16296ff0a3/> Correct the arguments in translate function calls
   This commit adjusts the calls to the `translate` function within the test suite to match its correct signature.

### New Features (BREAKING)

 - <csr-id-fec0ad2ac7ad993614fe91494e7ba8b82038bb36/> remove tag_handling parameter

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 2 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Lint ([`919af09`](https://github.com/hominsu/deeplx-rs/commit/919af091e304ec98220aec60118b5d12ab4a9e7b))
    - Correct the arguments in translate function calls ([`ce47124`](https://github.com/hominsu/deeplx-rs/commit/ce47124491b862d767966d2abca5ad16296ff0a3))
    - Remove tag_handling parameter ([`fec0ad2`](https://github.com/hominsu/deeplx-rs/commit/fec0ad2ac7ad993614fe91494e7ba8b82038bb36))
</details>

## v1.4.2 (2025-08-17)

<csr-id-8ebefeb492c48a11597d2719aba36fabeb81e495/>



### Refactor

 - <csr-id-8ebefeb492c48a11597d2719aba36fabeb81e495/> simplify logic

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 166 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release deeplx v1.4.2 ([`39486cd`](https://github.com/hominsu/deeplx-rs/commit/39486cd4265becd61bac9143fd4ebb8a802b7b09))
    - Release deeplx v1.4.2 ([`428d168`](https://github.com/hominsu/deeplx-rs/commit/428d168e00cef09475b2174504202335fa8c91bd))
    - Simplify logic ([`8ebefeb`](https://github.com/hominsu/deeplx-rs/commit/8ebefeb492c48a11597d2719aba36fabeb81e495))
</details>

## v1.4.1 (2025-03-04)

<csr-id-57c8fa6c4ed0d69793e93d13478a887c04aacecc/>
<csr-id-6729abfa5087ec12773c28b623979187150ea525/>
<csr-id-2ded061aac5994fc3151bd743cebd82a32ce69d0/>
<csr-id-1feec80534d9ab29f3c19d5f7c0bf0371bf08a0b/>
<csr-id-08c478d51f141ebaf86d3d2165c7a6fa499b6fb6/>
<csr-id-4006f8255253b1eea42e31530e20d0fec4a828ce/>


<csr-id-6729abfa5087ec12773c28b623979187150ea525/>
<csr-id-2ded061aac5994fc3151bd743cebd82a32ce69d0/>
<csr-id-1feec80534d9ab29f3c19d5f7c0bf0371bf08a0b/>
<csr-id-08c478d51f141ebaf86d3d2165c7a6fa499b6fb6/>
<csr-id-4006f8255253b1eea42e31530e20d0fec4a828ce/>

### New Features

 - <csr-id-529be488eed096332bed223ebf04e82d0bc0a1dc/> release v1.4.1
 - <csr-id-dcf143f805f4d5bfba740d3174725ab88bd3bba0/> set worker threads to available parallelism

### Bug Fixes

 - <csr-id-fd30fd8117fed6c4faefd07bdef60c9700e6b548/> ci
 - <csr-id-fc6b8a860e001ec0ab4c20036307c80594aa6c42/> lint

### Other

 - <csr-id-57c8fa6c4ed0d69793e93d13478a887c04aacecc/> use rust-tls instead of openssl
 - <csr-id-6729abfa5087ec12773c28b623979187150ea525/> add libssl-dev
 - <csr-id-2ded061aac5994fc3151bd743cebd82a32ce69d0/> enable mimalloc as global alloc
 - <csr-id-1feec80534d9ab29f3c19d5f7c0bf0371bf08a0b/> use QEMU
 - <csr-id-08c478d51f141ebaf86d3d2165c7a6fa499b6fb6/> remove sccache and cargo-chef
 - <csr-id-4006f8255253b1eea42e31530e20d0fec4a828ce/> automate build and release
   - Add a build job to build the project for multiple targets using `cargo-zigbuild`
   - Upload the built binaries to GitHub Release
   - Add a job to publish the crate to crates.io

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release over the course of 21 calendar days.
 - 21 days passed between releases.
 - 10 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Use rust-tls instead of openssl ([`57c8fa6`](https://github.com/hominsu/deeplx-rs/commit/57c8fa6c4ed0d69793e93d13478a887c04aacecc))
    - Add libssl-dev ([`6729abf`](https://github.com/hominsu/deeplx-rs/commit/6729abfa5087ec12773c28b623979187150ea525))
    - Enable mimalloc as global alloc ([`2ded061`](https://github.com/hominsu/deeplx-rs/commit/2ded061aac5994fc3151bd743cebd82a32ce69d0))
    - Use QEMU ([`1feec80`](https://github.com/hominsu/deeplx-rs/commit/1feec80534d9ab29f3c19d5f7c0bf0371bf08a0b))
    - Remove sccache and cargo-chef ([`08c478d`](https://github.com/hominsu/deeplx-rs/commit/08c478d51f141ebaf86d3d2165c7a6fa499b6fb6))
    - Ci ([`fd30fd8`](https://github.com/hominsu/deeplx-rs/commit/fd30fd8117fed6c4faefd07bdef60c9700e6b548))
    - Release v1.4.1 ([`529be48`](https://github.com/hominsu/deeplx-rs/commit/529be488eed096332bed223ebf04e82d0bc0a1dc))
    - Set worker threads to available parallelism ([`dcf143f`](https://github.com/hominsu/deeplx-rs/commit/dcf143f805f4d5bfba740d3174725ab88bd3bba0))
    - Automate build and release ([`4006f82`](https://github.com/hominsu/deeplx-rs/commit/4006f8255253b1eea42e31530e20d0fec4a828ce))
    - Lint ([`fc6b8a8`](https://github.com/hominsu/deeplx-rs/commit/fc6b8a860e001ec0ab4c20036307c80594aa6c42))
</details>

## v1.4.0 (2025-02-10)

<csr-id-a29a32db11450ef4c22d20c44abbd778d8ead8c0/>
<csr-id-aa06ce866e21614f5361e4492c4c0e9b023fa83e/>


<csr-id-aa06ce866e21614f5361e4492c4c0e9b023fa83e/>

### Chore

 - <csr-id-a29a32db11450ef4c22d20c44abbd778d8ead8c0/> enable impersonation feature in docker

### New Features

 - <csr-id-ef26b4be967c454283df9dd773997163cb1b75bf/> bump rand crate

### Refactor

 - <csr-id-aa06ce866e21614f5361e4492c4c0e9b023fa83e/> proxy configuration for improved flexibility
   - Store the proxy configuration in the `DeepLX` struct.
   - Create a `reqwest` client inside the `translate` function, instead of inside the `new` function.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 8 calendar days.
 - 8 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release deeplx v1.4.0 ([`b4449ec`](https://github.com/hominsu/deeplx-rs/commit/b4449ec7bc28d04057aa07a3077cb462b5ffb61e))
    - Enable impersonation feature in docker ([`a29a32d`](https://github.com/hominsu/deeplx-rs/commit/a29a32db11450ef4c22d20c44abbd778d8ead8c0))
    - Proxy configuration for improved flexibility ([`aa06ce8`](https://github.com/hominsu/deeplx-rs/commit/aa06ce866e21614f5361e4492c4c0e9b023fa83e))
    - Bump rand crate ([`ef26b4b`](https://github.com/hominsu/deeplx-rs/commit/ef26b4be967c454283df9dd773997163cb1b75bf))
</details>

## v1.3.3 (2025-02-02)

<csr-id-59f9eb1252117f0f32704caeaf9e60750a1bf5b8/>
<csr-id-dd4fd71c6cf091749547a06c5b22584ebcc461be/>


<csr-id-dd4fd71c6cf091749547a06c5b22584ebcc461be/>

### Chore

 - <csr-id-59f9eb1252117f0f32704caeaf9e60750a1bf5b8/> docker authentication

### Bug Fixes

 - <csr-id-3b56ba3474ba43dcd3f02b8a5d0e910a90163457/> lint

### Refactor

 - <csr-id-dd4fd71c6cf091749547a06c5b22584ebcc461be/> server to use SocketAddr config
   - Move the server run logic to a separate module
   - Change the config `bind` field to be a `SocketAddr` instead of a String

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 9 calendar days.
 - 9 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release deeplx v1.3.3 ([`7038f1b`](https://github.com/hominsu/deeplx-rs/commit/7038f1b252d451e3f05ed28c80c8180e07a571f7))
    - Lint ([`3b56ba3`](https://github.com/hominsu/deeplx-rs/commit/3b56ba3474ba43dcd3f02b8a5d0e910a90163457))
    - Server to use SocketAddr config ([`dd4fd71`](https://github.com/hominsu/deeplx-rs/commit/dd4fd71c6cf091749547a06c5b22584ebcc461be))
    - Docker authentication ([`59f9eb1`](https://github.com/hominsu/deeplx-rs/commit/59f9eb1252117f0f32704caeaf9e60750a1bf5b8))
</details>

## v1.3.2 (2025-01-24)

<csr-id-444652ceefccb3d6ff17ed9f2156b57cac3e126e/>



### Bug Fixes

 - <csr-id-cfcdb4c4003975b5a76038eb622c9fda5fc724ea/> request library imports

### Other

 - <csr-id-444652ceefccb3d6ff17ed9f2156b57cac3e126e/> configure docker buildx cloud driver

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release deeplx v1.3.2 ([`3e9a907`](https://github.com/hominsu/deeplx-rs/commit/3e9a9078dc13255b0aca908c5f6648dc14bee7c4))
    - Configure docker buildx cloud driver ([`444652c`](https://github.com/hominsu/deeplx-rs/commit/444652ceefccb3d6ff17ed9f2156b57cac3e126e))
    - Request library imports ([`cfcdb4c`](https://github.com/hominsu/deeplx-rs/commit/cfcdb4c4003975b5a76038eb622c9fda5fc724ea))
</details>

## v1.3.1 (2025-01-24)

<csr-id-76648191c8dafc1ed32771c10d2aa42405ce07fa/>
<csr-id-543411c7892a9dcfc926f538faa17fac07865a0e/>
<csr-id-a90df3140f0a6c20f285c04598c26bba17233215/>
<csr-id-bed536265ea643fcd1f7654144578667f0ec1f7e/>
<csr-id-f336482cc4d3432c7e08b52a61b6c404ac5bcea0/>
<csr-id-49c857123034ad725d5beada036a00662201f956/>
<csr-id-3eaa33b7fcccde9d3d8de26150c51c8dcb85f8f3/>
<csr-id-b33d7898fc127e31ff87e25e5b5911f0c26c8a82/>


<csr-id-543411c7892a9dcfc926f538faa17fac07865a0e/>
<csr-id-a90df3140f0a6c20f285c04598c26bba17233215/>
<csr-id-bed536265ea643fcd1f7654144578667f0ec1f7e/>
<csr-id-f336482cc4d3432c7e08b52a61b6c404ac5bcea0/>
<csr-id-49c857123034ad725d5beada036a00662201f956/>
<csr-id-3eaa33b7fcccde9d3d8de26150c51c8dcb85f8f3/>
<csr-id-b33d7898fc127e31ff87e25e5b5911f0c26c8a82/>


<csr-id-a90df3140f0a6c20f285c04598c26bba17233215/>
<csr-id-bed536265ea643fcd1f7654144578667f0ec1f7e/>
<csr-id-f336482cc4d3432c7e08b52a61b6c404ac5bcea0/>
<csr-id-49c857123034ad725d5beada036a00662201f956/>
<csr-id-3eaa33b7fcccde9d3d8de26150c51c8dcb85f8f3/>
<csr-id-b33d7898fc127e31ff87e25e5b5911f0c26c8a82/>

### Documentation

<csr-id-a98ad40f33e7b18671eb4da040c03df4981e0017/>

 - <csr-id-12c0470aaea7a326ae406fa17bac155e48479fa2/> update install instructions and Dockerfile
   - Remove the `server` feature flag from the install command in README

### New Features

 - <csr-id-e9201998503fa80e880efd2c9f0b547b2b3787de/> enhance translation robustness and error handling
   - Improve error handling for language detection.

### Bug Fixes

 - <csr-id-ce7dde6b67a85921f99478c292b9fc457fef00fb/> new line

### Refactor

 - <csr-id-76648191c8dafc1ed32771c10d2aa42405ce07fa/> improve server shutdown and config handling
 - <csr-id-543411c7892a9dcfc926f538faa17fac07865a0e/> Improve CLI and error handling
   - Updated Cargo.toml: Added `thiserror` and `clap` dependencies, removed `argh`.
   - Updated config.toml: Added `debug`, `bind` and `concurrent` fields, changed `addr` to `bind`.
   - Updated Dockerfile: Changed the CMD to use the `run` subcommand.
   - Updated main.rs: Refactored the main function to use clap for command-line argument parsing, added subcommands and changed the server setup.
   - Updated server/conf/mod.rs: Added default values for config fields.
   - Updated server/mod.rs: Modified the server setup to use clap and to handle the subcommand `run`.
   - Updated server/routes/middleware.rs: Modified the middleware to handle optional authentication.
 - <csr-id-a90df3140f0a6c20f285c04598c26bba17233215/> Improve `split_text` error handling and response
   - Refactor `split_text` to directly return a `SplitTextResponse` instead of bytes.
   - Adjust the code to handle the response from `make_request` consistently.
 - <csr-id-bed536265ea643fcd1f7654144578667f0ec1f7e/> enhance memory management
   - Added support for multiple memory allocators: mimalloc, rpmalloc, snmalloc, and tikv-jemallocator.
   - Removed `mimalloc` as the default global allocator.
 - <csr-id-f336482cc4d3432c7e08b52a61b6c404ac5bcea0/> improve error handling and middleware
   - Rename `InternalServerError` error variant to `InternalServer`
   - Remove unnecessary `Future` trait bound in `from_request_parts` functions
   - Simplify error handling in middleware and state extraction
   - Remove unnecessary `async` block in `from_request_parts` function in `middleware.rs`
   - Use `or` instead of `or_else` for optional token handling.
 - <csr-id-49c857123034ad725d5beada036a00662201f956/> improve CI/CD workflows and Docker build
 - <csr-id-3eaa33b7fcccde9d3d8de26150c51c8dcb85f8f3/> upgrade `axum` to 0.8 and remove `async-trait`
   - Change `RequireAuth` and `AppState` middleware to return a `Future`.
   - Change `translate` function to return a `Future`.
 - <csr-id-b33d7898fc127e31ff87e25e5b5911f0c26c8a82/> cli argument parsing
   - Replace `clap` with `argh` for command line parsing, reduced about 300
     KB of server.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release over the course of 31 calendar days.
 - 31 days passed between releases.
 - 12 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release deeplx v1.3.1 ([`11d34a9`](https://github.com/hominsu/deeplx-rs/commit/11d34a93fc94dbf47aa4fc187d500d4cc6d643e3))
    - Update install instructions and Dockerfile ([`12c0470`](https://github.com/hominsu/deeplx-rs/commit/12c0470aaea7a326ae406fa17bac155e48479fa2))
    - Improve server shutdown and config handling ([`7664819`](https://github.com/hominsu/deeplx-rs/commit/76648191c8dafc1ed32771c10d2aa42405ce07fa))
    - Improve CLI and error handling ([`543411c`](https://github.com/hominsu/deeplx-rs/commit/543411c7892a9dcfc926f538faa17fac07865a0e))
    - Improve `split_text` error handling and response ([`a90df31`](https://github.com/hominsu/deeplx-rs/commit/a90df3140f0a6c20f285c04598c26bba17233215))
    - New line ([`ce7dde6`](https://github.com/hominsu/deeplx-rs/commit/ce7dde6b67a85921f99478c292b9fc457fef00fb))
    - Enhance translation robustness and error handling ([`e920199`](https://github.com/hominsu/deeplx-rs/commit/e9201998503fa80e880efd2c9f0b547b2b3787de))
    - Enhance memory management ([`bed5362`](https://github.com/hominsu/deeplx-rs/commit/bed536265ea643fcd1f7654144578667f0ec1f7e))
    - Improve error handling and middleware ([`f336482`](https://github.com/hominsu/deeplx-rs/commit/f336482cc4d3432c7e08b52a61b6c404ac5bcea0))
    - Improve CI/CD workflows and Docker build ([`49c8571`](https://github.com/hominsu/deeplx-rs/commit/49c857123034ad725d5beada036a00662201f956))
    - Upgrade `axum` to 0.8 and remove `async-trait` ([`3eaa33b`](https://github.com/hominsu/deeplx-rs/commit/3eaa33b7fcccde9d3d8de26150c51c8dcb85f8f3))
    - Update README.md ([`a98ad40`](https://github.com/hominsu/deeplx-rs/commit/a98ad40f33e7b18671eb4da040c03df4981e0017))
    - Cli argument parsing ([`b33d789`](https://github.com/hominsu/deeplx-rs/commit/b33d7898fc127e31ff87e25e5b5911f0c26c8a82))
</details>

<csr-unknown>

<csr-unknown/>

## v1.2.2 (2024-12-23)

<csr-id-d8bf36c9ca8a40f627c39d9d6c6d6efad90bffe3/>



### Other

 - <csr-id-d8bf36c9ca8a40f627c39d9d6c6d6efad90bffe3/> significantly reduced the size of the build artifacts
   - Add configuration for release mode optimization, including setting `opt-level` to 3, enabling `strip` and `lto`, setting `codegen-units` to 1, and setting `panic` to abort.
   - Remove the `impersonate` feature flag in Dockerfile, since it's not a stable and usable crate.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 day passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release deeplx v1.2.2 ([`f4d3038`](https://github.com/hominsu/deeplx-rs/commit/f4d30383c49311b49ea2db916330f26332aa607e))
    - Significantly reduced the size of the build artifacts ([`d8bf36c`](https://github.com/hominsu/deeplx-rs/commit/d8bf36c9ca8a40f627c39d9d6c6d6efad90bffe3))
</details>

## v1.2.1 (2024-12-22)

<csr-id-294dbe6087abdbe77970bda1a16d37789e0fde26/>



### Documentation

 - <csr-id-c6af5ff4676e9c6cf922db73b0a5edc291689853/> update README.md

### Refactor

 - <csr-id-294dbe6087abdbe77970bda1a16d37789e0fde26/> remove generic types from router function

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 3 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release deeplx v1.2.1 ([`2fbc750`](https://github.com/hominsu/deeplx-rs/commit/2fbc7501e95f25e3b678b1b3af09ca8e835f53b6))
    - Update README.md ([`c6af5ff`](https://github.com/hominsu/deeplx-rs/commit/c6af5ff4676e9c6cf922db73b0a5edc291689853))
    - Remove generic types from router function ([`294dbe6`](https://github.com/hominsu/deeplx-rs/commit/294dbe6087abdbe77970bda1a16d37789e0fde26))
</details>

## v1.2.0 (2024-12-18)

<csr-id-fe43461a5359031e43235f20d1bca666e3455456/>



### Chore

 - <csr-id-fe43461a5359031e43235f20d1bca666e3455456/> update dependencies

### Documentation

 - <csr-id-c26c68b2809acf6802975889cf3e0611a5459e32/> update dependencies

### New Features

<csr-id-e30086552556bdbb0dba50869a7ea6f8e7cf120f/>

 - <csr-id-934875cd2a645e715e82ae649b4afcde5b4de702/> configure proxy
   - Change the `DeepLX` struct to accept a proxy configuration.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release deeplx v1.2.0 ([`4e0f1cf`](https://github.com/hominsu/deeplx-rs/commit/4e0f1cfbd98a09b126f788a28454246e6af88c96))
    - Configure proxy ([`934875c`](https://github.com/hominsu/deeplx-rs/commit/934875cd2a645e715e82ae649b4afcde5b4de702))
    - Improve token handling in authentication ([`e300865`](https://github.com/hominsu/deeplx-rs/commit/e30086552556bdbb0dba50869a7ea6f8e7cf120f))
    - Update dependencies ([`fe43461`](https://github.com/hominsu/deeplx-rs/commit/fe43461a5359031e43235f20d1bca666e3455456))
    - Update dependencies ([`c26c68b`](https://github.com/hominsu/deeplx-rs/commit/c26c68b2809acf6802975889cf3e0611a5459e32))
</details>

<csr-unknown>

<csr-unknown/>

## v1.1.0 (2024-12-17)

### Performance

 - <csr-id-cd332c0ad5fe97e60982d3b5af5684c4cf7421b6/> enable memory allocation with `mimalloc`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release deeplx v1.1.0 ([`87a16c8`](https://github.com/hominsu/deeplx-rs/commit/87a16c86170f1c3b1fb4503426d27fe15afb9998))
    - Enable memory allocation with `mimalloc` ([`cd332c0`](https://github.com/hominsu/deeplx-rs/commit/cd332c0ad5fe97e60982d3b5af5684c4cf7421b6))
</details>

## v1.0.0 (2024-12-17)

<csr-id-e6b2eae0a771af67266e7e72df01b7a97b07c8bd/>



### Chore

 - <csr-id-e6b2eae0a771af67266e7e72df01b7a97b07c8bd/> publish only source code files

### Bug Fixes

 - <csr-id-9d6346a1f99a127b6d45f0adc2b38bacdbd6e32c/> ci

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release deeplx v1.0.0 ([`2758be2`](https://github.com/hominsu/deeplx-rs/commit/2758be2fd62c3a719cbbb7953d1653d06e9f0d34))
    - Publish only source code files ([`e6b2eae`](https://github.com/hominsu/deeplx-rs/commit/e6b2eae0a771af67266e7e72df01b7a97b07c8bd))
    - Ci ([`9d6346a`](https://github.com/hominsu/deeplx-rs/commit/9d6346a1f99a127b6d45f0adc2b38bacdbd6e32c))
</details>

## v0.1.10 (2024-12-16)

### Bug Fixes

 - <csr-id-a60000d820efe6c5bbbdb7bb25a1fbbd63fede4b/> sscache env

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release deeplx v0.1.10 ([`76d6b49`](https://github.com/hominsu/deeplx-rs/commit/76d6b49c56131779c32253a1d78f9d2138d5e314))
    - Sscache env ([`a60000d`](https://github.com/hominsu/deeplx-rs/commit/a60000d820efe6c5bbbdb7bb25a1fbbd63fede4b))
</details>

## v0.1.9 (2024-12-16)

<csr-id-e24b2e2072bc86fe55bb04090321142c5d60c1a5/>
<csr-id-5e45fd5e2f6c935a67e6ffefc193dc9a485c4f3c/>
<csr-id-6766cf0f59375773b36d91a350c601d7c084dbce/>


<csr-id-5e45fd5e2f6c935a67e6ffefc193dc9a485c4f3c/>
<csr-id-6766cf0f59375773b36d91a350c601d7c084dbce/>


<csr-id-6766cf0f59375773b36d91a350c601d7c084dbce/>

### Documentation

 - <csr-id-7e6b4e5cc272386b2b5496d9b95390cff48f1aba/> update README.md

### New Features

 - <csr-id-5530c6406e6da5ca37c0eab55a3584c36bdd7bab/> add server

### Bug Fixes

 - <csr-id-1ddef14571c282bdd96905b2b53187e90d5bf1a9/> enbale `server` feature for cargo install

### Other

 - <csr-id-e24b2e2072bc86fe55bb04090321142c5d60c1a5/> automate docker image releases
 - <csr-id-5e45fd5e2f6c935a67e6ffefc193dc9a485c4f3c/> add docker support
   - add `Dockerfile` for building the application
   - add `docker-bake.hcl` for defining multi-platform builds
   - add `docker-compose.yml`

### Refactor

 - <csr-id-6766cf0f59375773b36d91a350c601d7c084dbce/> restructure codebase into skeleton module
   - Move the modules `data`, `translate` and `utils` into the new `skeleton` module
   - Update `rquest` dependency version from `0.32.1` to `0.33`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release.
 - 1 day passed between releases.
 - 6 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release deeplx v0.1.9 ([`e956f93`](https://github.com/hominsu/deeplx-rs/commit/e956f9344f8f23ba5c3d484193b4c39caf7116da))
    - Update README.md ([`7e6b4e5`](https://github.com/hominsu/deeplx-rs/commit/7e6b4e5cc272386b2b5496d9b95390cff48f1aba))
    - Enbale `server` feature for cargo install ([`1ddef14`](https://github.com/hominsu/deeplx-rs/commit/1ddef14571c282bdd96905b2b53187e90d5bf1a9))
    - Automate docker image releases ([`e24b2e2`](https://github.com/hominsu/deeplx-rs/commit/e24b2e2072bc86fe55bb04090321142c5d60c1a5))
    - Add docker support ([`5e45fd5`](https://github.com/hominsu/deeplx-rs/commit/5e45fd5e2f6c935a67e6ffefc193dc9a485c4f3c))
    - Add server ([`5530c64`](https://github.com/hominsu/deeplx-rs/commit/5530c6406e6da5ca37c0eab55a3584c36bdd7bab))
    - Restructure codebase into skeleton module ([`6766cf0`](https://github.com/hominsu/deeplx-rs/commit/6766cf0f59375773b36d91a350c601d7c084dbce))
</details>

## v0.1.8 (2024-12-15)

### Bug Fixes

 - <csr-id-063377c4b6537a885b3bdc2492fce7ab3f2cdcf6/> add gzip support
   - Use `bytes::Bytes` instead of `Vec<u8>` for http responses.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release deeplx v0.1.8 ([`2b63521`](https://github.com/hominsu/deeplx-rs/commit/2b63521cb0fabb73e26572ee69eb3641e5dc512f))
    - Add gzip support ([`063377c`](https://github.com/hominsu/deeplx-rs/commit/063377c4b6537a885b3bdc2492fce7ab3f2cdcf6))
</details>

## v0.1.7 (2024-12-15)

### Documentation

 - <csr-id-da8263e7a83aaefa060523f0fed06dcda32b3820/> update REMEAD.md

### New Features

 - <csr-id-e2b6263d64d697875cc4333b5cea321120140ca9/> add rquest for impersonation support

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 7 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release deeplx v0.1.7 ([`13acb8f`](https://github.com/hominsu/deeplx-rs/commit/13acb8f598c4c4569771a117558026010220e389))
    - Update REMEAD.md ([`da8263e`](https://github.com/hominsu/deeplx-rs/commit/da8263e7a83aaefa060523f0fed06dcda32b3820))
    - Add rquest for impersonation support ([`e2b6263`](https://github.com/hominsu/deeplx-rs/commit/e2b6263d64d697875cc4333b5cea321120140ca9))
</details>

## v0.1.6 (2024-12-07)

### Bug Fixes

 - <csr-id-4cb9afa234f7e1bdacda61309804afd6e2fc7a7f/> error in `DeepLX::new`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release deeplx v0.1.6 ([`9b65b37`](https://github.com/hominsu/deeplx-rs/commit/9b65b37a1ae48dea3fe54a5e999ca0b684deb73e))
    - Error in `DeepLX::new` ([`4cb9afa`](https://github.com/hominsu/deeplx-rs/commit/4cb9afa234f7e1bdacda61309804afd6e2fc7a7f))
</details>

## v0.1.5 (2024-12-07)

### New Features

 - <csr-id-52b2c96cf1a4241489795108b651d8f080fca5e6/> add `Config` to configure `DeepLX`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release deeplx v0.1.5 ([`a626ba8`](https://github.com/hominsu/deeplx-rs/commit/a626ba85a4ed174a459e69dec0aef12257553c9c))
    - Add `Config` to configure `DeepLX` ([`52b2c96`](https://github.com/hominsu/deeplx-rs/commit/52b2c96cf1a4241489795108b651d8f080fca5e6))
</details>

## v0.1.4 (2024-12-07)

### New Features

 - <csr-id-7c437e823e0e1845b60c2e99fb53685e790d9839/> use proxy as default feature

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release deeplx v0.1.4 ([`edaa15f`](https://github.com/hominsu/deeplx-rs/commit/edaa15f36576d5200d13c8d73608a269ba3ad6ad))
    - Use proxy as default feature ([`7c437e8`](https://github.com/hominsu/deeplx-rs/commit/7c437e823e0e1845b60c2e99fb53685e790d9839))
</details>

## v0.1.3 (2024-12-06)

<csr-id-68e63558cfaaa04ab33970aa26afa90f9a7712d4/>



### Refactor

 - <csr-id-68e63558cfaaa04ab33970aa26afa90f9a7712d4/> String is enough for this case

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release deeplx v0.1.3 ([`acc051e`](https://github.com/hominsu/deeplx-rs/commit/acc051e8c6083b1c6e6d366758dc9d7b998524a5))
    - String is enough for this case ([`68e6355`](https://github.com/hominsu/deeplx-rs/commit/68e63558cfaaa04ab33970aa26afa90f9a7712d4))
</details>

## v0.1.2 (2024-12-06)

<csr-id-00ce8653b58562bfe39901889a542c3faae15e8f/>



### Refactor

 - <csr-id-00ce8653b58562bfe39901889a542c3faae15e8f/> serialize data into struct

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release deeplx v0.1.2 ([`6118c58`](https://github.com/hominsu/deeplx-rs/commit/6118c58ac4a7028f9b28460dc5d70114cea5a394))
    - Serialize data into struct ([`00ce865`](https://github.com/hominsu/deeplx-rs/commit/00ce8653b58562bfe39901889a542c3faae15e8f))
</details>

## v0.1.1 (2024-12-06)

<csr-id-7fa3532f7169fecc232d0957c41d2442750bc7d2/>



### Chore

 - <csr-id-7fa3532f7169fecc232d0957c41d2442750bc7d2/> more metadate

### Documentation

 - <csr-id-be1543807e28738be3fc0891a2c72aee6c23bea4/> add more docs

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release deeplx v0.1.1 ([`481e1de`](https://github.com/hominsu/deeplx-rs/commit/481e1de2a77af922d79d391fe2c1eadde96756c7))
    - More metadate ([`7fa3532`](https://github.com/hominsu/deeplx-rs/commit/7fa3532f7169fecc232d0957c41d2442750bc7d2))
    - Add more docs ([`be15438`](https://github.com/hominsu/deeplx-rs/commit/be1543807e28738be3fc0891a2c72aee6c23bea4))
</details>

## v0.1.0 (2024-12-05)

<csr-id-ae454951912143bc5b5bafcfd0fcffeab44489ea/>
<csr-id-05f1555739794356cf66c3ed95db99df3e103917/>
<csr-id-b080ada977b0192042e6a3a4cb07c79f2f5c0060/>
<csr-id-904ed1645a6ec5cefd2cafa6531dec6f7620adbc/>
<csr-id-9a3d3c6545403905d92c5e3ae52eb16edaba0813/>
<csr-id-781134495786432752fa191a79a219f42ea036db/>


<csr-id-05f1555739794356cf66c3ed95db99df3e103917/>
<csr-id-b080ada977b0192042e6a3a4cb07c79f2f5c0060/>
<csr-id-904ed1645a6ec5cefd2cafa6531dec6f7620adbc/>
<csr-id-9a3d3c6545403905d92c5e3ae52eb16edaba0813/>
<csr-id-781134495786432752fa191a79a219f42ea036db/>


<csr-id-b080ada977b0192042e6a3a4cb07c79f2f5c0060/>
<csr-id-904ed1645a6ec5cefd2cafa6531dec6f7620adbc/>
<csr-id-9a3d3c6545403905d92c5e3ae52eb16edaba0813/>
<csr-id-781134495786432752fa191a79a219f42ea036db/>

### Chore

 - <csr-id-ae454951912143bc5b5bafcfd0fcffeab44489ea/> exclude shadow files
 - <csr-id-05f1555739794356cf66c3ed95db99df3e103917/> rename
 - <csr-id-b080ada977b0192042e6a3a4cb07c79f2f5c0060/> add LICENSE

### Chore

 - <csr-id-781134495786432752fa191a79a219f42ea036db/> relaese deeplx v0.1.0

### Documentation

 - <csr-id-b753522c8c3c3af3f59c9809887572c855bb6a73/> update README.md

### New Features

 - <csr-id-a137956ed8c76412e4276851290d4f4704849aab/> initial commit

### Other

 - <csr-id-904ed1645a6ec5cefd2cafa6531dec6f7620adbc/> add action to check, test code

### Style

 - <csr-id-9a3d3c6545403905d92c5e3ae52eb16edaba0813/> fix lint

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release.
 - 8 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release deeplx v0.1.0 ([`6917ee1`](https://github.com/hominsu/deeplx-rs/commit/6917ee1a91f243c217e2738857dd2cc34eaf1922))
    - Relaese deeplx v0.1.0 ([`7811344`](https://github.com/hominsu/deeplx-rs/commit/781134495786432752fa191a79a219f42ea036db))
    - Exclude shadow files ([`ae45495`](https://github.com/hominsu/deeplx-rs/commit/ae454951912143bc5b5bafcfd0fcffeab44489ea))
    - Rename ([`05f1555`](https://github.com/hominsu/deeplx-rs/commit/05f1555739794356cf66c3ed95db99df3e103917))
    - Add action to check, test code ([`904ed16`](https://github.com/hominsu/deeplx-rs/commit/904ed1645a6ec5cefd2cafa6531dec6f7620adbc))
    - Fix lint ([`9a3d3c6`](https://github.com/hominsu/deeplx-rs/commit/9a3d3c6545403905d92c5e3ae52eb16edaba0813))
    - Update README.md ([`b753522`](https://github.com/hominsu/deeplx-rs/commit/b753522c8c3c3af3f59c9809887572c855bb6a73))
    - Add LICENSE ([`b080ada`](https://github.com/hominsu/deeplx-rs/commit/b080ada977b0192042e6a3a4cb07c79f2f5c0060))
    - Initial commit ([`a137956`](https://github.com/hominsu/deeplx-rs/commit/a137956ed8c76412e4276851290d4f4704849aab))
</details>

