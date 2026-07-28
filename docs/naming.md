# Naming Worldview

Kengen's vocabulary is governance: the words it allows itself to use fence what it is allowed to
become. Kengen is a **sans-I/O adjudicator** — it *decides* (`Allow`/`Ask`/`Deny`), it never
*acts*, *stores*, *loads*, or *runs*. The register keeps it there.

## Native register

Kengen speaks in **adjudication** terms: `Verdict`, `combine`, `adjudicate`, `rule verdict`,
`default stance`, `lattice`, `restrictiveness`. A request is *adjudicated*; a decision is a
*verdict*. Policy *content* and *rule evaluation* are named only to place them **outside** Kengen.

## Seam rule

At a seam a consumer may name its own domain (its requests, its rules, its enforcement). Kengen's
core stays in the adjudication register; consumer vocabulary does not leak inward.

## Banned vocabulary (drift toward enforcer / engine / I/O)

Kengen must not grow into a policy engine, an enforcement point, or a stateful I/O component. The
guard (`scripts/naming-guard.sh`) fails if any of these names a Kengen **type, enum, trait, module,
or type alias** — a mechanical, high-precision signal of that drift:

`Enforcer`, `Gateway`, `Firewall`, `Interceptor`, `Middleware`, `Engine`, `Runtime`, `Daemon`,
`Session`, `Token`, `Credential`, `Store`, `Cache`, `Repository`, `Loader`, `Fetcher`.

These name either **acting** (enforce/intercept), **running** (engine/runtime/daemon), **holding
auth state** (session/token/credential), or **I/O** (store/cache/repository/loader/fetcher) — all
of which are the consumer's, never Kengen's.

Softer cases (e.g. `Policy`, `Rule`) stay review-governed: they are legitimate as *inputs* Kengen
combines, but must never become content Kengen *owns or evaluates* — that is caught by the
adversarial review gate, not the mechanical guard.
