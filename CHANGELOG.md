# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Project shape: the sans-I/O `Verdict` lattice (`Allow`/`Ask`/`Deny`) with `combine` and
  `adjudicate` (commutative, associative, idempotent; `Deny` dominates, `Ask` escalates over
  `Allow`), the curated crate surface, the governance scaffolding (`PROJECT.md`, `AGENTS.md`,
  `BACKLOG.md`, `docs/naming.md`, the naming-guard), and the OpenSpec layout. Behaviour grows
  bet-first — see `BACKLOG.md`.
