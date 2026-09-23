# Kengen

Kengen (権限) is a **sans-I/O access-policy adjudicator**. Given the verdicts a set of policy rules
have reached about a request, it returns the single decision — **Allow**, **Ask** (an authority
must confirm), or **Deny** — by folding them over the verdict lattice (`Allow < Ask < Deny`, most
restrictive wins).

It owns the *combination mechanism* and nothing else. It does not read rules, store or fetch
policy, hold sessions, mint tokens, act on a decision, or perform any I/O. Consumers bring the
rules, the content, and the enforcement; Kengen brings the decision lattice.

```rust
use kengen::{Verdict, adjudicate};

// The consumer evaluated its own rules to these verdicts, with an allow-by-default stance:
let decision = adjudicate([Verdict::Allow, Verdict::Ask, Verdict::Allow], Verdict::Allow);
assert_eq!(decision, Verdict::Ask); // Ask escalates over Allow; nothing downgrades it
```

## Scope

Kengen owns the verdict-lattice combination mechanism and nothing else (see "What Kengen owns"
below). It is built **bet-first**: behaviour grows only as a consumer forces it, so the surface
stays deliberately minimal. The plan and deferrals live in `BACKLOG.md`; `CHANGELOG.md` records what
each release adds.

## What Kengen owns — and what it does not

- **Owns**: the `Verdict` lattice and its combination (commutative, associative, idempotent; a
  `Deny` always dominates, an `Ask` always escalates over `Allow`).
- **Does not own**: which rules exist, what each rule decides, the default stance, where policy
  lives, or what happens after a verdict. Those are the consumer's — forever.

## Architecture

- [`kengen-contract`](crates/kengen-contract) — the isolated core: the `Verdict` lattice,
  `combine`, and `adjudicate`.
- [`kengen`](crates/kengen) — the curated entrypoint you depend on; it re-exports the core.
- [`kengen-governance`](crates/kengen-governance) — the unpublished Tianheng gate; its accepted
  law is projected into `AGENTS.kengen-law.md`.
- `PROJECT.md` — purpose, the invariants to protect, non-goals.
- `BACKLOG.md` — the bet, the phased plan, and the dependency stance.
- `docs/domain-language.md` — the naming worldview and its guard.
- `openspec/specs/` — shipped requirements.

## Contributing

`AGENTS.md` is the contributor and agent guide, including the Definition of Done;
`docs/development-flow.md` is the short checklist.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT), at your option.
