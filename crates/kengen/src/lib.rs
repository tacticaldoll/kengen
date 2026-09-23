//! Kengen: sans-I/O access-policy adjudication.
//!
//! Kengen answers one question, purely: given the verdicts a set of policy rules reached about a
//! request, what is the single decision — **Allow**, **Ask** (a human/authority must confirm), or
//! **Deny**? It owns the *combination mechanism* — the verdict lattice and its precedence — and
//! nothing else.
//!
//! This crate is the curated public entrypoint. It carries no logic of its own: every item here
//! is a re-export of [`kengen_contract`], where the mechanism / content boundary is documented.
//!
//! ```
//! use kengen::{Verdict, adjudicate};
//!
//! // The consumer evaluated its own rules to these verdicts, with an allow-by-default stance:
//! let decision = adjudicate([Verdict::Allow, Verdict::Ask, Verdict::Allow], Verdict::Allow);
//! assert_eq!(decision, Verdict::Ask); // Ask escalates over Allow; nothing downgrades it
//!
//! // A deny-by-default stance is itself the most restrictive verdict, so the fold keeps it:
//! assert_eq!(adjudicate([Verdict::Allow, Verdict::Ask], Verdict::Deny), Verdict::Deny);
//! ```

#![forbid(unsafe_code)]

pub use kengen_contract::{Verdict, adjudicate};
