# Kengen Tianheng Law Projection

This file is generated from `constitution()` in `crates/kengen-governance/src/main.rs`.
The Rust declaration is authoritative; do not edit the projection by hand.
Regenerate it with `BLESS=1 cargo test -p kengen-governance law_projection_is_fresh`.

# Constitution: kengen

## Static boundaries

### `kengen-contract` (crate)

> kengen-contract is the adjudication core and a leaf of the discipline: it declares no normal dependency, so the verdict lattice pulls in no framework, runtime, or policy source.

- **rule**: restrict dependencies to (only: )
- **kind**: crate · **severity**: enforce

### `kengen-governance` (crate)

> the governance gate must stay independent of the workspace graph it judges: its normal dependencies are Tianheng's composed adopter surface alone, never an individual governance instrument or a workspace crate under judgment.

- **rule**: restrict dependencies to (only: tianheng)
- **kind**: crate · **severity**: enforce

### `kengen` (crate)

> kengen is the curated published entrypoint: its normal dependencies are kengen-contract alone, never a policy source, enforcement point, runtime, or external framework.

- **rule**: restrict dependencies to (only: kengen-contract)
- **kind**: crate · **severity**: enforce

### `kengen-contract::crate` (module)

> kengen-contract's library makes no inline `std::time` `now` call and declares no public `async fn`: time and asynchronous driving belong to the consumer that composes it. Coverage is partial by nature (a clock read through a method on a value, such as `Instant::elapsed`, a `now` path taken as a value rather than called, and a public `fn` returning `impl Future` are invisible to a source scan), so this tooth complements review rather than replacing it.

- **rule**: inline symbol path confined to module (confined_prefix: std::time; ending_with: now)
- **kind**: module · **severity**: enforce · **crate**: kengen-contract

### `kengen-contract::crate` (module)

> kengen-contract's library makes no inline call into std::io/fs/net/process, because loading policy, storing it, and enforcing a verdict are the consumer's. Coverage is partial by nature (macro-expanded I/O such as println!, a method called on an I/O value such as `write_all` on a `std::io::Write` parameter, and an I/O path taken as a value are invisible to a source scan), so this tooth complements review rather than replacing it.

- **rule**: inline symbol path confined to module (confined_prefix: std::io)
- **kind**: module · **severity**: enforce · **crate**: kengen-contract

### `kengen-contract::crate` (module)

> kengen-contract's library makes no inline call into std::io/fs/net/process, because loading policy, storing it, and enforcing a verdict are the consumer's. Coverage is partial by nature (macro-expanded I/O such as println!, a method called on an I/O value such as `write_all` on a `std::io::Write` parameter, and an I/O path taken as a value are invisible to a source scan), so this tooth complements review rather than replacing it.

- **rule**: inline symbol path confined to module (confined_prefix: std::fs)
- **kind**: module · **severity**: enforce · **crate**: kengen-contract

### `kengen-contract::crate` (module)

> kengen-contract's library makes no inline call into std::io/fs/net/process, because loading policy, storing it, and enforcing a verdict are the consumer's. Coverage is partial by nature (macro-expanded I/O such as println!, a method called on an I/O value such as `write_all` on a `std::io::Write` parameter, and an I/O path taken as a value are invisible to a source scan), so this tooth complements review rather than replacing it.

- **rule**: inline symbol path confined to module (confined_prefix: std::net)
- **kind**: module · **severity**: enforce · **crate**: kengen-contract

### `kengen-contract::crate` (module)

> kengen-contract's library makes no inline call into std::io/fs/net/process, because loading policy, storing it, and enforcing a verdict are the consumer's. Coverage is partial by nature (macro-expanded I/O such as println!, a method called on an I/O value such as `write_all` on a `std::io::Write` parameter, and an I/O path taken as a value are invisible to a source scan), so this tooth complements review rather than replacing it.

- **rule**: inline symbol path confined to module (confined_prefix: std::process)
- **kind**: module · **severity**: enforce · **crate**: kengen-contract

## Async-exposure boundaries

### `kengen-contract::crate` (semantic)

> kengen-contract's library makes no inline `std::time` `now` call and declares no public `async fn`: time and asynchronous driving belong to the consumer that composes it. Coverage is partial by nature (a clock read through a method on a value, such as `Instant::elapsed`, a `now` path taken as a value rather than called, and a public `fn` returning `impl Future` are invisible to a source scan), so this tooth complements review rather than replacing it.

- **rule**: must not expose async fn (including_submodules: true; scan_depth: subtree)
- **kind**: semantic · **severity**: enforce · **crate**: kengen-contract
