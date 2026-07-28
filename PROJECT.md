# Project Contract

## Purpose

Kengen (権限) is a **sans-I/O access-policy adjudicator**. It answers one question, purely: given
the verdicts a set of policy rules have reached about a request, what is the single decision —
**Allow**, **Ask** (an authority must confirm before proceeding), or **Deny**?

Kengen owns the *combination mechanism* — the verdict lattice and its precedence — and nothing
else. It does not fetch a policy, evaluate a rule, hold a session, mint a token, or act on a
decision. It is the small, reusable heart of "is this permitted?", left free of the I/O and the
domain content that every consumer supplies differently.

## Positioning: a mechanism, not a policy engine

Access control dies as a monolith when one component tries to own *both* the decision mechanism
*and* the policy content *and* the enforcement. Kengen refuses that shape. It is a **bounded
context**: the adjudication lattice. The rules, their meaning, where policy is stored, and what
happens after a verdict are the consumer's — never Kengen's.

It is a **leaf of the discipline**: it depends on nothing and does no I/O; consumers compose it.

## Core Contract

The invariants to protect first:

- **Sans-I/O.** The core performs no I/O — no filesystem, network, clock, or global state. A
  verdict is a pure function of its inputs.
- **Mechanism, not content.** Kengen combines *already-decided* per-rule verdicts; it never
  decides what a rule means. **Which** rules exist, **what** each decides, and the **default**
  stance are the caller's — forever.
- **Adjudicate, do not enforce.** Kengen returns a `Verdict`; it never acts on one. Gating,
  sessions, tokens, and enforcement live outside.
- **The lattice is total and stable.** `Allow < Ask < Deny`; combination keeps the most
  restrictive and is commutative, associative, and idempotent — so the order rules are evaluated
  in never changes the decision.
- **Deny is never silently downgraded.** No combination turns a `Deny` (or an `Ask`) into
  something less restrictive.

## Terminology

`Verdict` (`Allow` / `Ask` / `Deny`), `combine` (the lattice join of two verdicts), `adjudicate`
(fold per-rule verdicts over a caller-supplied default), **default stance** (the caller's decision
for the empty fold — deny-by-default or allow-by-default), **rule verdict** (a verdict the domain
supplies for one rule). "Policy content" and "rule evaluation" are named only to place them
**outside** Kengen.

## Non-Goals

Kengen is not: a policy-evaluation engine (it does not read rules or match requests), a policy
store or loader, an enforcement point / gateway / firewall, a session or token service, an
identity provider, an audit log, or anything that performs I/O. Those compose Kengen; they are not
Kengen. Consumers bring the rules, the content, and the enforcement; Kengen brings the decision
lattice.

## References

- Operating protocol and Definition of Done: `AGENTS.md`
- Naming worldview (native register, banned enforcement/runtime vocabulary): `docs/naming.md`
- The bet, phased plan, and dependency stance: `BACKLOG.md`
- Shipped requirements: `openspec/specs/`
