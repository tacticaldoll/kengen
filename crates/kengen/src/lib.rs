//! Kengen: sans-I/O access-policy adjudication.
//!
//! Kengen answers one question, purely: given the verdicts a set of policy rules reached about a
//! request, what is the single decision — **Allow**, **Ask** (a human/authority must confirm), or
//! **Deny**? It owns the *combination mechanism* — the verdict lattice and its precedence — and
//! nothing else.
//!
//! # The mechanism / content boundary (the sans-I/O bill)
//!
//! A sans-I/O core cannot make a *semantic* judgment, so the semantic responsibility is the
//! caller's: **which** rules exist, **what** each rule decides about a request, and the **default**
//! stance when no rule speaks. Kengen never fetches a policy, evaluates a rule, reads a clock, or
//! performs I/O. It receives per-rule [`Verdict`]s already decided by the domain and folds them
//! into one, by a fixed precedence. Policy *content* is forever the caller's; Kengen owns only the
//! *lattice*.
//!
//! This keeps Kengen an **adjudicator**, never an enforcer: it returns a verdict, it does not act
//! on one (no gateway, no session, no I/O — see `PROJECT.md` non-goals).

#![forbid(unsafe_code)]

/// A single adjudication decision. The variants form a lattice ordered by **restrictiveness**:
/// `Allow < Ask < Deny`. Combining verdicts keeps the most restrictive, so a single `Deny`
/// dominates and any `Ask` escalates over `Allow`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Verdict {
    /// The request may proceed.
    Allow,
    /// The request may proceed only after an authority (e.g. a human) confirms it.
    Ask,
    /// The request must not proceed.
    Deny,
}

impl Verdict {
    /// Combine two verdicts, keeping the **more restrictive** (`Deny` > `Ask` > `Allow`).
    ///
    /// This is the core mechanism: a commutative, associative, idempotent join on the lattice.
    /// It carries no policy content — it only says how already-decided verdicts compose.
    #[must_use]
    pub fn combine(self, other: Verdict) -> Verdict {
        if self >= other { self } else { other }
    }
}

/// Adjudicate a request by folding the domain's per-rule `verdicts` over `default`.
///
/// `default` is the caller's stance when a fold is otherwise empty — it is *policy content*
/// (deny-by-default = pass [`Verdict::Deny`]; allow-by-default = pass [`Verdict::Allow`]), never a
/// stance Kengen bakes in. The result is the most restrictive of `default` and every verdict.
#[must_use]
pub fn adjudicate<I>(verdicts: I, default: Verdict) -> Verdict
where
    I: IntoIterator<Item = Verdict>,
{
    verdicts.into_iter().fold(default, Verdict::combine)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deny_dominates() {
        assert_eq!(
            adjudicate(
                [Verdict::Allow, Verdict::Ask, Verdict::Deny],
                Verdict::Allow
            ),
            Verdict::Deny
        );
    }

    #[test]
    fn ask_escalates_over_allow() {
        assert_eq!(
            adjudicate(
                [Verdict::Allow, Verdict::Ask, Verdict::Allow],
                Verdict::Allow
            ),
            Verdict::Ask
        );
    }

    #[test]
    fn all_allow_allows() {
        assert_eq!(
            adjudicate([Verdict::Allow, Verdict::Allow], Verdict::Allow),
            Verdict::Allow
        );
    }

    #[test]
    fn the_empty_fold_is_the_callers_default_stance() {
        // No rule spoke: the decision is whatever stance the caller supplied — Kengen bakes none.
        assert_eq!(adjudicate([], Verdict::Deny), Verdict::Deny);
        assert_eq!(adjudicate([], Verdict::Allow), Verdict::Allow);
    }

    #[test]
    fn combine_is_commutative_and_idempotent() {
        for a in [Verdict::Allow, Verdict::Ask, Verdict::Deny] {
            assert_eq!(a.combine(a), a, "idempotent");
            for b in [Verdict::Allow, Verdict::Ask, Verdict::Deny] {
                assert_eq!(a.combine(b), b.combine(a), "commutative");
            }
        }
    }

    #[test]
    fn combine_is_associative() {
        let vs = [Verdict::Allow, Verdict::Ask, Verdict::Deny];
        for a in vs {
            for b in vs {
                for c in vs {
                    assert_eq!(
                        a.combine(b).combine(c),
                        a.combine(b.combine(c)),
                        "associative"
                    );
                }
            }
        }
    }
}
