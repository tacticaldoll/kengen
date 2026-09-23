# kengen-governance

Executable architectural governance for the Kengen workspace — the Tianheng constitution.

This crate is an internal gate, not a published library (`publish = false`). It depends only on
the [Tianheng](https://github.com/tacticaldoll/tianheng) composed adopter surface and holds the
workspace's dependency boundaries, the core's sans-I/O purity, the facade's re-exports-only shape,
workspace coverage, and the accepted constitution's generated projection, `AGENTS.kengen-law.md`.

Run it from the workspace root:

```sh
cargo run -p kengen-governance -- check --manifest-path Cargo.toml
```

Regenerate the projection after a deliberate, reviewed law change:

```sh
BLESS=1 cargo test -p kengen-governance law_projection_is_fresh
```

Part of [Kengen](https://github.com/tacticaldoll/kengen).

## License

Licensed under either of [Apache-2.0](https://github.com/tacticaldoll/kengen/blob/main/LICENSE-APACHE) or [MIT](https://github.com/tacticaldoll/kengen/blob/main/LICENSE-MIT), at your option.
