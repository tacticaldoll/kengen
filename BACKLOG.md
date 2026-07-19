# Backlog & Deferred Decisions

Records the plan, the bet, and deferred decisions so the repo can drive its own development.
Shipped truth lives in `openspec/specs/`; active proposed truth in `openspec/changes/`.

## Current Baseline

Project shape established: the sans-I/O core (`crates/kengen`) with the `Verdict` lattice
(`Allow`/`Ask`/`Deny`), `combine`, and `adjudicate`; the governance surface (`PROJECT.md`,
`AGENTS.md`, naming worldview + guard, `deny.toml`, DoD); and OpenSpec scaffolding. Behaviour is
built bet-first (below).

## The Bet (gates everything)

The family thesis is that a genuinely useful adjudicator can be a **thin, sans-I/O core** — the
verdict lattice — with everything domain-specific (rules, content, enforcement, I/O) left to the
consumer. Kengen is a test of that thesis for access policy. Before scaling, prove — cheaply —
that the mechanism/content split carries its weight:

- a real consumer supplies **per-rule verdicts** (its own rule evaluation) and a **default
  stance**, and Kengen folds them into one `Allow`/`Ask`/`Deny` with acceptable friction;
- success = the consumer only *classifies* (evaluates its rules to verdicts) and *disposes* (acts
  on the decision); Kengen does the combination and nothing else. If Kengen ends up needing to
  read rules, hold state, do I/O, or enforce, the bet is failing and the design must be revisited.

Only after the bet holds do the phases below get built.

## Family dependency stance

Kengen depends on **nothing** — it is a pure leaf of the discipline. Consumers compose it; nothing
composes into its core. This repo is **sibling-blind**: it names no other product, and which
products it is composed with is the consumer's knowledge, not Kengen's.

## Phased plan (after the bet)

1. **Explanation**: a verdict that can carry *why* (which rule(s) drove `Deny`/`Ask`) — as an
   optional, still-sans-I/O return, so a consumer can render/audit the reason. Adjudicate stays a
   pure function.
2. **Obligation on `Ask`**: whether an `Ask` may carry a caller-typed obligation (what must be
   confirmed) without Kengen interpreting it — mechanism only.
3. **Richer lattices (only if forced)**: verdicts beyond the three-point lattice must be forced by
   a real consumer need, never guessed; the three-point lattice is the least-commitment default.

## Open questions (with recommended defaults)

- **Default stance**: caller-supplied per adjudication (recommended) vs. a fixed secure default.
  Recommendation: caller-supplied — the default is policy content, not mechanism.
- **`Ask` semantics**: does `Ask` mean "escalate to an authority" universally, or is the authority
  the consumer's concern? Recommendation: Kengen defines only the *lattice position* of `Ask`
  (between `Allow` and `Deny`); who is asked is the consumer's.
- **Explanation shape**: attach provenance to the `Verdict` vs. return a separate structure.
  Deferred to the phase-1 consumer that forces it.

## Deferred decisions

- **Executable governance as a `kengen-governance` crate**: the family ships architecture as an
  executable Tianheng/guibiao `*-governance` crate (see `suunta-governance`, `shaahid-governance`).
  Kengen currently enforces its one architectural axiom — the naming worldview — through
  `scripts/naming-guard.sh` (run in the Definition of Done and in CI). Promoting that to a
  `kengen-governance` crate that reacts against the source (dependency-isolation, sans-I/O purity,
  no exposed `async fn`, active-prose boundaries) is **deferred** until the behaviour built
  bet-first gives it invariants worth reacting against; the naming guard is sufficient for the
  project-shape phase.
