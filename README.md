# Repro Case for [rust-lang/rust#95825](https://github.com/rust-lang/rust/issues/95825)

## Results Observed on My Laptop

* MacBook Pro (16-inch, 2019)
* macOS Monterey version 12.1 (21C52)
* Processor: 2.3 GHz 8-Core Intel Core i9
* Memory: 16 GB 2667 MHz DDR4

`rustc --version --verbose:`

```
rustc 1.60.0 (7737e0b5c 2022-04-04)
binary: rustc
commit-hash: 7737e0b5c4103216d6fd8cf941b7ab9bdbaace7c
commit-date: 2022-04-04
host: x86_64-apple-darwin
release: 1.60.0
LLVM version: 14.0.0
```

## Observations and Theories

I _think_ the key feature in this bug is that there are a lot of dependencies.

When watching `top` output while running the "with coverage" case below, I noticed a _lot_ of CPU activity in `rustc` _while_ the `bin` project was running its unit tests. I suspect there's a race condition between running the `bin` unit tests and the `lib` doc tests that does not occur with code coverage disabled.

## Running Without Code Coverage

```sh
$ cargo test
[build progress omitted...]
     Running unittests (target/debug/deps/bin-baa2cdc9756c282a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration.rs (target/debug/deps/integration-8fafe3abc3bd36c4)

running 1 test
test bin_errors_out ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 33.58s

     Running unittests (target/debug/deps/lib-1e31e0e54b9c0c1d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests lib

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Running With Code Coverage

```sh
$ RUSTFLAGS="-C instrument-coverage" cargo test --all-features 
[build progress omitted...]
     Running unittests (target/debug/deps/bin-8f115d9d2990e87c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration.rs (target/debug/deps/integration-a5cfd02c9977acc5)

running 1 test
test bin_errors_out ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 52.23s

     Running unittests (target/debug/deps/lib-4d09b9d33c89fcfa)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests lib
error[E0460]: found possibly newer version of crate `cfg_if` which `log` depends on
 --> (path redacted)/cov-bug-repro/lib/src/time_it.rs:1:5
  |
1 | use log::info;
  |     ^^^
  |
  = note: perhaps that crate needs to be recompiled?
  = note: the following crate versions were found:
          crate `cfg_if`: (path redacted)/cov-bug-repro/target/debug/deps/libcfg_if-7e74d31581507915.rmeta
          crate `cfg_if`: (path redacted)/cov-bug-repro/target/debug/deps/libcfg_if-7e74d31581507915.rlib
          crate `cfg_if`: (path redacted)/cov-bug-repro/target/debug/deps/libcfg_if-3cd7bda968e519ae.rmeta
          crate `cfg_if`: (path redacted)/.rustup/toolchains/1.60.0-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/libcfg_if-100dc4191a6287d7.rlib
          crate `log`: (path redacted)/cov-bug-repro/target/debug/deps/liblog-89e0b97aa80e736c.rlib

error[E0463]: can't find crate for `serde_derive` which `serde` depends on
 --> (path redacted)/cov-bug-repro/lib/src/cbor_types.rs:1:5
  |
1 | use serde::de::{Deserialize, Deserializer};
  |     ^^^^^ can't find crate

error[E0463]: can't find crate for `serde_derive` which `serde` depends on
 --> (path redacted)/cov-bug-repro/lib/src/cbor_types.rs:2:5
  |
2 | use serde::ser::{Serialize, Serializer};
  |     ^^^^^ can't find crate

error[E0463]: can't find crate for `serde_derive` which `serde_bytes` depends on
 --> (path redacted)/cov-bug-repro/lib/src/cbor_types.rs:3:5
  |
3 | use serde_bytes::ByteBuf;
  |     ^^^^^^^^^^^ can't find crate

error[E0463]: can't find crate for `serde_derive` which `serde_cbor` depends on
 --> (path redacted)/cov-bug-repro/lib/src/cbor_types.rs:4:5
  |
4 | use serde_cbor::tags::Tagged;
  |     ^^^^^^^^^^ can't find crate

error: cannot find macro `info` in this scope
  --> (path redacted)/cov-bug-repro/lib/src/time_it.rs:19:9
   |
19 |         info!("timing for {}: {:.2?}", self.label, self.start.elapsed());
   |         ^^^^

error[E0463]: can't find crate for `serde_derive` which `serde` depends on
  --> (path redacted)/cov-bug-repro/lib/src/cbor_types.rs:21:28
   |
21 |             Some(_) => Err(serde::de::Error::custom("unexpected tag")),
   |                            ^^^^^ can't find crate

error[E0463]: can't find crate for `serde_derive` which `serde` depends on
  --> (path redacted)/cov-bug-repro/lib/src/cbor_types.rs:47:28
   |
47 |             Some(_) => Err(serde::de::Error::custom("unexpected tag")),
   |                            ^^^^^ can't find crate

error[E0463]: can't find crate for `serde_derive` which `serde` depends on
  --> (path redacted)/cov-bug-repro/lib/src/cbor_types.rs:72:28
   |
72 |             Some(_) => Err(serde::de::Error::custom("unexpected tag")),
   |                            ^^^^^ can't find crate

error[E0405]: cannot find trait `Serialize` in this scope
  --> (path redacted)/cov-bug-repro/lib/src/cbor_types.rs:10:6
   |
10 | impl Serialize for DateT {
   |      ^^^^^^^^^ not found in this scope

error[E0405]: cannot find trait `Serializer` in this scope
  --> (path redacted)/cov-bug-repro/lib/src/cbor_types.rs:11:21
   |
11 |     fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
   |                     ^^^^^^^^^^ not found in this scope

error[E0405]: cannot find trait `Deserialize` in this scope
  --> (path redacted)/cov-bug-repro/lib/src/cbor_types.rs:16:11
   |
16 | impl<'de> Deserialize<'de> for DateT {
   |           ^^^^^^^^^^^ not found in this scope

error[E0405]: cannot find trait `Deserializer` in this scope
  --> (path redacted)/cov-bug-repro/lib/src/cbor_types.rs:17:23
   |
17 |     fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
   |                       ^^^^^^^^^^^^ not found in this scope

error[E0405]: cannot find trait `Serialize` in this scope
  --> (path redacted)/cov-bug-repro/lib/src/cbor_types.rs:35:6
   |
35 | impl Serialize for UriT {
   |      ^^^^^^^^^ not found in this scope

error[E0405]: cannot find trait `Serializer` in this scope
  --> (path redacted)/cov-bug-repro/lib/src/cbor_types.rs:36:21
   |
36 |     fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
   |                     ^^^^^^^^^^ not found in this scope

error[E0405]: cannot find trait `Deserialize` in this scope
  --> (path redacted)/cov-bug-repro/lib/src/cbor_types.rs:41:11
   |
41 | impl<'de> Deserialize<'de> for UriT {
   |           ^^^^^^^^^^^ not found in this scope

error[E0405]: cannot find trait `Deserializer` in this scope
  --> (path redacted)/cov-bug-repro/lib/src/cbor_types.rs:42:23
   |
42 |     fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
   |                       ^^^^^^^^^^^^ not found in this scope

error[E0405]: cannot find trait `Serialize` in this scope
  --> (path redacted)/cov-bug-repro/lib/src/cbor_types.rs:61:6
   |
61 | impl Serialize for BytesT {
   |      ^^^^^^^^^ not found in this scope

error[E0405]: cannot find trait `Serializer` in this scope
  --> (path redacted)/cov-bug-repro/lib/src/cbor_types.rs:62:21
   |
62 |     fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
   |                     ^^^^^^^^^^ not found in this scope

error[E0405]: cannot find trait `Deserialize` in this scope
  --> (path redacted)/cov-bug-repro/lib/src/cbor_types.rs:67:11
   |
67 | impl<'de> Deserialize<'de> for BytesT {
   |           ^^^^^^^^^^^ not found in this scope

error[E0405]: cannot find trait `Deserializer` in this scope
  --> (path redacted)/cov-bug-repro/lib/src/cbor_types.rs:68:23
   |
68 |     fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
   |                       ^^^^^^^^^^^^ not found in this scope

error: aborting due to 21 previous errors

Some errors have detailed explanations: E0405, E0463.
For more information about an error, try `rustc --explain E0405`.
error: test failed, to rerun pass '--doc'
```
