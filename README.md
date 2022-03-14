# Rust Audio Codecs

Bindings and a friendly wrapper for a selection of high quality audio
encoders that are useful for streaming music via the internet and HTTP.

## Use:

By default no codecs are included and no native libraries are built.

To select codecs use the `--features` flag or the `features = []`
list when declaring your dependency on this crate via Cargo.

Building encoder natives through `cargo build` is also supported
with a feature flag. If you have cloned this crate locally and
wish to build the natives please make sure you have activated the
required vendor submodules in your local git repository.
