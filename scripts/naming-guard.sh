#!/usr/bin/env bash
# Naming-worldview guard (see docs/naming.md).
#
# Fails if an enforcement / engine / I/O / auth-state word names a kengen type, enum, trait,
# module, or type alias — the semantic drift that turns a sans-I/O adjudicator into a policy
# engine or enforcement point. It matches DECLARATIONS only (struct/enum/trait/type/mod), so it is
# high-precision: it does not flag prose, method names, or a `Policy`/`Rule` used as an input type.
# The softer cases in docs/naming.md stay review-governed.
set -euo pipefail
cd "$(dirname "$0")/.."

banned='Enforcer|Gateway|Firewall|Interceptor|Middleware|Engine|Runtime|Daemon|Session|Token|Credential|Store|Cache|Repository|Loader|Fetcher'

if grep -rnE "\b(struct|enum|trait|type|mod)[[:space:]]+[A-Za-z0-9_]*(${banned})[A-Za-z0-9_]*" --include='*.rs' crates/; then
    echo "" >&2
    echo "naming-guard: an enforcement/engine/I/O/auth-state word names a kengen type/module — see docs/naming.md." >&2
    echo "kengen adjudicates (returns a Verdict); it does not enforce, run, hold auth state, or do I/O." >&2
    echo "those belong to the consumer that composes kengen." >&2
    exit 1
fi

echo "naming-guard: clean"
