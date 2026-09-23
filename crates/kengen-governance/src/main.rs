//! Executable architectural governance for the kengen workspace.
//!
//! It enforces normal-dependency boundaries, the core's sans-I/O purity as far as a source scan
//! sees it (no inline `std::io`/`fs`/`net`/`process` call, no inline `std::time` `now` call, no
//! public `async fn`), the facade's re-exports-only shape, and workspace coverage. I/O, a clock
//! read, or asynchrony the scan cannot see (macro-expanded I/O, a call through a method on a
//! value, a public `fn` returning `impl Future`) stays review-governed, as the accepted boundary
//! reasons state. The axiom that the core never decides what a rule means has no syntactic
//! marker: it stays review-governed too.

#![forbid(unsafe_code)]

use std::{
    env, fs,
    path::{Path, PathBuf},
    process::ExitCode,
};

use tianheng::prelude::*;

const CONTRACT_REASON: &str = "kengen-contract is the adjudication core and a leaf of the discipline: it declares no normal dependency, so the verdict lattice pulls in no framework, runtime, or policy source.";
const GOVERNANCE_REASON: &str = "the governance gate must stay independent of the workspace graph it judges: its normal dependencies are Tianheng's composed adopter surface alone, never an individual governance instrument or a workspace crate under judgment.";
const FACADE_REASON: &str = "kengen is the curated published entrypoint: its normal dependencies are kengen-contract alone, never a policy source, enforcement point, runtime, or external framework.";
const CORE_PURITY_REASON: &str = "kengen-contract's library makes no inline `std::time` `now` call and declares no public `async fn`: time and asynchronous driving belong to the consumer that composes it. Coverage is partial by nature (a clock read through a method on a value, such as `Instant::elapsed`, and a `now` path taken as a value rather than called are invisible to a source scan, and a public `fn` returning `impl Future` is not an async fn), so this tooth complements review rather than replacing it.";
const CORE_NO_IO_REASON: &str = "kengen-contract's library makes no inline call into std::io/fs/net/process, because loading policy, storing it, and enforcing a verdict are the consumer's. Coverage is partial by nature (macro-expanded I/O such as println!, a method called on an I/O value such as `write_all` on a `std::io::Write` parameter, and an I/O path taken as a value are invisible to a source scan), so this tooth complements review rather than replacing it.";
const FACADE_REEXPORT_REASON: &str =
    "the kengen facade must stay a pure re-export entrypoint and hold no logic of its own";
const FACADE_NON_REEXPORT: &str = "non-re-export item in facade library";

/// The facade source tree the re-exports-only scan guards, relative to the workspace root.
const FACADE_SOURCE_DIR: &str = "crates/kengen/src";

fn constitution() -> Constitution {
    let constitution = Constitution::new("kengen")
        .boundary(
            CrateBoundary::crate_("kengen-contract")
                .restrict_dependencies_to(Vec::<&str>::new())
                .because(CONTRACT_REASON),
        )
        .boundary(
            CrateBoundary::crate_("kengen-governance")
                .restrict_dependencies_to(["tianheng"])
                .because(GOVERNANCE_REASON),
        )
        .boundary(
            CrateBoundary::crate_("kengen")
                .restrict_dependencies_to(["kengen-contract"])
                .because(FACADE_REASON),
        )
        .sans_io_pure(
            SansIoPure::in_crate("kengen-contract")
                .module("crate")
                .reading_clock_via("std::time", ["now"])
                .because(CORE_PURITY_REASON),
        );

    ["std::io", "std::fs", "std::net", "std::process"]
        .into_iter()
        .fold(constitution, |constitution, path| {
            constitution.boundary(
                ModuleBoundary::in_crate("kengen-contract")
                    .module("crate")
                    .must_not_call_inline(path)
                    .because(CORE_NO_IO_REASON),
            )
        })
}

fn main() -> ExitCode {
    let args = env::args().collect::<Vec<_>>();

    if args.iter().skip(1).any(|arg| arg == "check") {
        let manifest = manifest_path_from_args(&args);
        let root = manifest
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| PathBuf::from("."));

        if let Err(violations) = check_facade_reexports_only(&root) {
            eprintln!("kengen facade governance failed: {FACADE_REEXPORT_REASON}");
            for violation in violations {
                eprintln!(
                    "{}:{}: `{}`",
                    violation.path, violation.line, violation.marker
                );
            }
            return ExitCode::from(1);
        }
    }

    tianheng::run(&constitution(), args)
}

fn manifest_path_from_args(args: &[String]) -> PathBuf {
    for index in 0..args.len() {
        if args[index] == "--manifest-path"
            && let Some(path) = args.get(index + 1)
        {
            return PathBuf::from(path);
        }

        if let Some(path) = args[index].strip_prefix("--manifest-path=") {
            return PathBuf::from(path);
        }
    }

    PathBuf::from("Cargo.toml")
}

#[derive(Debug, PartialEq, Eq)]
struct SourceViolation {
    path: String,
    line: usize,
    marker: &'static str,
}

fn check_facade_reexports_only(root: &Path) -> Result<(), Vec<SourceViolation>> {
    let mut violations = Vec::new();
    let files = collect_rs_files(&root.join(FACADE_SOURCE_DIR));

    // An empty or missing facade tree is a vacuous pass; fail it instead.
    if files.is_empty() {
        violations.push(SourceViolation {
            path: FACADE_SOURCE_DIR.to_owned(),
            line: 0,
            marker: "no facade source files found",
        });
    }

    for file in files {
        let relative = file
            .strip_prefix(root)
            .unwrap_or(&file)
            .to_string_lossy()
            .into_owned();
        let Ok(content) = fs::read_to_string(&file) else {
            violations.push(SourceViolation {
                path: relative,
                line: 0,
                marker: "unreadable facade source",
            });
            continue;
        };
        violations.extend(check_facade_content(&relative, &content));
    }

    if violations.is_empty() {
        Ok(())
    } else {
        Err(violations)
    }
}

/// A brace-depth-aware line scan: at depth zero the facade may hold only re-exports, `use`
/// imports, attributes, and comments. It is a line scan, not a parser, because this crate may
/// depend only on `tianheng`; `cargo fmt --all --check` splits co-located items onto their own
/// lines, where the scan then sees them.
fn check_facade_content(path: &str, content: &str) -> Vec<SourceViolation> {
    let mut violations = Vec::new();
    let mut depth: i32 = 0;

    for (index, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        let is_comment = trimmed.starts_with("//");

        if depth == 0
            && !trimmed.is_empty()
            && !is_comment
            && !trimmed.starts_with('#')
            && !trimmed.starts_with("pub use ")
            && !trimmed.starts_with("use ")
        {
            violations.push(SourceViolation {
                path: path.to_owned(),
                line: index + 1,
                marker: FACADE_NON_REEXPORT,
            });
        }

        if !is_comment {
            depth += line.matches('{').count() as i32;
            depth -= line.matches('}').count() as i32;
            depth = depth.max(0);
        }
    }

    violations
}

fn collect_rs_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();

    let Ok(entries) = fs::read_dir(dir) else {
        return files;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            files.extend(collect_rs_files(&path));
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            files.push(path);
        }
    }

    files
}

#[cfg(test)]
mod tests {
    use super::*;

    const LAW_PROJECTION_PREAMBLE: &str = "\
# Kengen Tianheng Law Projection

This file is generated from `constitution()` in `crates/kengen-governance/src/main.rs`.
The Rust declaration is authoritative; do not edit the projection by hand.
Regenerate it with `BLESS=1 cargo test -p kengen-governance law_projection_is_fresh`.

";

    fn workspace_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
    }

    #[test]
    fn current_workspace_satisfies_constitution() {
        GovernanceTest::for_constitution(constitution())
            .with_manifest_dir(workspace_root())
            .assert_clean();
    }

    #[test]
    fn every_workspace_crate_is_covered() {
        GovernanceTest::for_constitution(constitution())
            .with_manifest_dir(workspace_root())
            .assert_all_workspace_members_covered();
    }

    #[test]
    fn law_projection_is_fresh() {
        GovernanceTest::for_constitution(constitution())
            .with_manifest_dir(workspace_root())
            .assert_projection_fresh_with_preamble("AGENTS.kengen-law.md", LAW_PROJECTION_PREAMBLE);
    }

    #[test]
    fn current_facade_is_reexports_only() {
        assert_eq!(check_facade_reexports_only(&workspace_root()), Ok(()));
    }

    #[test]
    fn unapproved_core_dependency_is_rejected() {
        let workspace = TempWorkspace::new("kengen-governance-core-dependency");
        workspace.write_package("tokio", "", "");
        workspace.write_package(
            "kengen-contract",
            "[dependencies]\ntokio = { path = \"../tokio\" }\n",
            "",
        );

        let report = workspace.violations(&["kengen-contract", "tokio"]);
        assert!(
            report.violations.iter().any(|violation| {
                violation.target() == "kengen-contract"
                    && violation.rule == "restrict dependencies to"
                    && violation.finding == "tokio"
            }),
            "expected the core dependency boundary to fire: {report:?}"
        );
    }

    #[test]
    fn core_io_calls_are_rejected() {
        for (path, call) in [
            ("std::fs", "let _ = std::fs::read(\"x\");"),
            ("std::io", "let _ = std::io::stdin();"),
            (
                "std::net",
                "let _ = std::net::TcpStream::connect(\"127.0.0.1:1\");",
            ),
            ("std::process", "let _ = std::process::Command::new(\"x\");"),
        ] {
            let workspace = TempWorkspace::new(&format!(
                "kengen-governance-core-{}",
                path.replace("::", "-")
            ));
            workspace.write_package(
                "kengen-contract",
                "",
                &format!("pub fn leak() {{\n    {call}\n}}\n"),
            );

            let report = workspace.violations(&["kengen-contract"]);
            assert!(
                report
                    .violations
                    .iter()
                    .any(|violation| violation.target() == path),
                "expected the core no-I/O boundary for {path} to fire: {report:?}"
            );
        }
    }

    #[test]
    fn unapproved_facade_dependency_is_rejected() {
        let workspace = TempWorkspace::new("kengen-governance-facade-dependency");
        workspace.write_package("kengen-contract", "", "");
        workspace.write_package("tokio", "", "");
        workspace.write_package(
            "kengen",
            "[dependencies]\nkengen-contract = { path = \"../kengen-contract\" }\ntokio = { path = \"../tokio\" }\n",
            "",
        );

        let report = workspace.violations(&["kengen-contract", "tokio"]);
        assert!(
            report.violations.iter().any(|violation| {
                violation.target() == "kengen" && violation.finding == "tokio"
            }),
            "expected the facade dependency boundary to fire: {report:?}"
        );
    }

    #[test]
    fn governance_dependency_beyond_tianheng_is_rejected() {
        let workspace = TempWorkspace::new("kengen-governance-gate-dependency");
        workspace.write_package("kengen-contract", "", "");
        workspace.write_package("tokio", "", "");
        workspace.write_package(
            "kengen-governance",
            "[dependencies]\ntianheng = { path = \"../tianheng\" }\ntokio = { path = \"../tokio\" }\n",
            "",
        );

        let report = workspace.violations(&["kengen-contract", "tokio"]);
        assert!(
            report.violations.iter().any(|violation| {
                violation.target() == "kengen-governance" && violation.finding == "tokio"
            }),
            "expected the governance dependency boundary to fire: {report:?}"
        );
    }

    #[test]
    fn core_ambient_clock_is_rejected() {
        let workspace = TempWorkspace::new("kengen-governance-core-clock");
        workspace.write_package(
            "kengen-contract",
            "",
            "pub fn leak() -> std::time::SystemTime {\n    std::time::SystemTime::now()\n}\n",
        );

        let report = workspace.violations(&["kengen-contract"]);
        assert!(
            report
                .violations
                .iter()
                .any(|violation| violation.target() == "std::time"),
            "expected the core ambient-clock boundary to fire: {report:?}"
        );
    }

    #[test]
    fn core_async_exposure_is_rejected() {
        let workspace = TempWorkspace::new("kengen-governance-core-async");
        workspace.write_package(
            "kengen-contract",
            "",
            "pub mod inner {\n    pub async fn leak() {}\n}\n",
        );

        let report = workspace.violations(&["kengen-contract"]);
        assert!(
            report
                .violations
                .iter()
                .any(|violation| violation.rule == "must not expose async fn"),
            "expected the core async-exposure boundary to fire: {report:?}"
        );
    }

    #[test]
    fn pure_core_stays_clean() {
        let workspace = TempWorkspace::new("kengen-governance-core-clean");
        workspace.write_package("kengen-contract", "", "pub fn decide() {}\n");

        let outcome = workspace.outcome(&["kengen-contract"]);
        assert!(
            matches!(outcome, Outcome::Clean(_)),
            "a pure core must raise no violation: {outcome:?}"
        );
    }

    #[test]
    fn facade_reexports_and_comments_are_allowed() {
        let content = "//! Facade docs.\n#![no_std]\n\npub use kengen_contract::{\n    Decision,\n    Verdict,\n};\n";
        assert!(check_facade_content("lib.rs", content).is_empty());
    }

    #[test]
    fn facade_logic_item_is_rejected() {
        assert_eq!(
            check_facade_content("lib.rs", "pub fn helper() {}\n"),
            vec![SourceViolation {
                path: "lib.rs".to_owned(),
                line: 1,
                marker: FACADE_NON_REEXPORT,
            }]
        );
    }

    #[test]
    fn empty_facade_source_tree_fails_loudly() {
        let workspace = TempWorkspace::new("kengen-governance-empty-facade");

        let Err(violations) = check_facade_reexports_only(&workspace.path) else {
            panic!("a root with no facade source must fail the gate");
        };
        assert!(
            violations
                .iter()
                .any(|violation| violation.marker == "no facade source files found"),
            "expected a no-facade-source violation: {violations:?}"
        );
    }

    /// A scratch workspace that always carries the governance and facade crates, so every
    /// crate-level boundary has a real target and a fixture differs from a clean one only in what
    /// the test writes.
    struct TempWorkspace {
        path: PathBuf,
    }

    impl TempWorkspace {
        fn new(name: &str) -> Self {
            let path = env::temp_dir().join(format!("{name}-{}", std::process::id()));
            if path.exists() {
                fs::remove_dir_all(&path).expect("stale temporary workspace should be removable");
            }
            fs::create_dir_all(&path).expect("temporary workspace should be creatable");
            let workspace = Self { path };
            workspace.write_package("tianheng", "", "");
            workspace.write_package(
                "kengen-governance",
                "[dependencies]\ntianheng = { path = \"../tianheng\" }\n",
                "",
            );
            workspace.write_package(
                "kengen",
                "[dependencies]\nkengen-contract = { path = \"../kengen-contract\" }\n",
                "",
            );
            workspace
        }

        fn outcome(&self, members: &[&str]) -> Outcome {
            let entries = ["tianheng", "kengen-governance", "kengen"]
                .iter()
                .chain(members)
                .map(|member| format!("    \"{member}\","))
                .collect::<Vec<_>>()
                .join("\n");
            fs::write(
                self.path.join("Cargo.toml"),
                format!("[workspace]\nresolver = \"2\"\nmembers = [\n{entries}\n]\n"),
            )
            .expect("workspace manifest should be writable");

            tianheng::check_constitution(&constitution(), &self.path.join("Cargo.toml"))
        }

        fn violations(&self, members: &[&str]) -> Report {
            match self.outcome(members) {
                Outcome::Violations(report) => report,
                other => panic!("expected violations, got {other:?}"),
            }
        }

        fn write_package(&self, name: &str, dependencies: &str, source: &str) {
            let package = self.path.join(name);
            fs::create_dir_all(package.join("src")).expect("package source dir should be writable");
            fs::write(
                package.join("Cargo.toml"),
                format!(
                    "[package]\nname = \"{name}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n{dependencies}"
                ),
            )
            .expect("package manifest should be writable");
            fs::write(package.join("src/lib.rs"), source)
                .expect("package source should be writable");
        }
    }

    impl Drop for TempWorkspace {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }
}
