# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v1.0.0 (2024-12-17)

### Chore

 - <csr-id-e6b2eae0a771af67266e7e72df01b7a97b07c8bd/> publish only source code files

### Bug Fixes

 - <csr-id-9d6346a1f99a127b6d45f0adc2b38bacdbd6e32c/> ci

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
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

