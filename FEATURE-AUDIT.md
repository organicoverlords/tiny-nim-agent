# Feature Audit — tiny-nim-agent

Created: 2026-07-09
Repo: `organicoverlords/tiny-nim-agent`
Default branch: `main`

This is the canonical product scope, status, proof, and completion contract for the repo.

## Status definitions

| Status | Meaning |
|---|---|
| MISSING | Not implemented yet. No product claim allowed. |
| PLANNED | Designed in docs but no runtime code yet. |
| PARTIAL | Some code exists but not enough tests/proof. Do not claim done. |
| DONE | Implemented, tested, and proof-backed. |
| REJECTED | User/manual proof says the claim failed or is still broken. |
| BLOCKED | Cannot proceed without secret/environment/user decision. |

## Priority definitions

| Priority | Meaning |
|---|---|
| P0 | Required for the first usable agentic coder. |
| P1 | Required before trusting long autonomous coding work. |
| P2 | Important but can come after the first reliable WebUI loop. |
| P3 | Later expansion. |

## Product rules

- Product direction: ChatGPT-like WebUI with OpenCode-like agentic coding capabilities.
- Copy proven product behavior before inventing systems.
- Keep docs and code minimal, neat, and grounded in working principles.
- No tracked file may exceed 600 lines. Target under 400 lines.
- This repo must stay smaller and stricter than the old app stack.
- Features start as MISSING, not DONE.
- A feature row can move to DONE only with test/proof references.
- Runtime-visible features need a ledger/proof path, not just unit tests.
- Benchmark success must go through normal WebUI/session/runtime paths.
- No benchmark prompt edits to make the app pass.
- No hidden fallback or undocumented model order mutation.

## Latest CI proof

PR #2 proved Actions event delivery and Rust guardrails:

- Actions Smoke run `28987144229`: `Actions event smoke` succeeded.
- CI run `28987144205`: `Rust workspace and guardrails` succeeded.
- CI run `28987144205`: `NIM secret preflight` succeeded.

PR #4 proved the first dry-run agent foundation:

- Actions Smoke run `28987457257`: `Actions event smoke` succeeded.
- CI run `28987457255`: `NIM secret preflight` succeeded.
- CI run `28987457255`: `Rust workspace and guardrails` succeeded.

PR #6 proved the WebUI app-layer dry-run API:

- Actions Smoke run `28987842199`: `Actions event smoke` succeeded.
- CI run `28987842212`: `NIM secret preflight` succeeded.
- CI run `28987842212`: `Rust workspace and guardrails` succeeded.

PR #8 proved the minimal HTTP smoke route and page:

- Actions Smoke run `28988458709`: `Actions event smoke` succeeded.
- CI run `28988458741`: `NIM secret preflight` succeeded.
- CI run `28988458741`: `Rust workspace and guardrails` succeeded.

PR #10 proved visible smoke proof cards and an event-stream route:

- Actions Smoke run `28989521234`: `Actions event smoke` succeeded.
- CI run `28989521201`: `NIM secret preflight` succeeded.
- CI run `28989521201`: `Rust workspace and guardrails` succeeded.

PR #12 proved proof-backed smoke ledger and final report routes:

- Actions Smoke run `28989994344`: `Actions event smoke` succeeded.
- CI run `28989994376`: `NIM secret preflight` succeeded.
- CI run `28989994376`: `Rust workspace and guardrails` succeeded.

PR #14 proved runtime smoke proof artifact writing:

- Actions Smoke run `29055057707`: `Actions event smoke` succeeded.
- CI run `29055057694`: `NIM secret preflight` succeeded.
- CI run `29055057694`: `Rust workspace and guardrails` succeeded.

The Rust job passed:

- `cargo metadata --format-version 1 --no-deps`
- `cargo test --workspace`
- `bash scripts/check_no_placeholders.sh`
- `python3 scripts/check_line_count.py`

## MVP feature matrix

### 1. Repository and documentation

| Feature | Pri | Status | Required proof |
|---|---:|---|---|
| Canonical README | P0 | DONE | README committed |
| Agent worker rules | P0 | DONE | `AGENTS.md` committed |
| Canonical feature audit | P0 | DONE | This file committed |
| Architecture document | P0 | DONE | `docs/ARCHITECTURE.md` committed |
| NIM/tool-loop contract | P0 | DONE | `docs/NIM_ROUTING_AND_TOOL_LOOP.md` committed |
| No-stubs policy | P0 | DONE | `docs/NO_STUBS_POLICY.md` committed |
| Old-project audit boundary | P0 | DONE | `docs/OLD_PROJECTS_AUDIT.md` committed |
| Roadmap | P0 | DONE | `docs/ROADMAP.md` committed |
| 600-line file ceiling | P0 | DONE | README, AGENTS, audit, CI guard script, PR #2/#4/#6/#8/#10/#12/#14 CI proof |
| Runtime proof artifacts ignored | P0 | DONE | `.gitignore` ignores `.tiny-nim-agent/` in PR #14 |
| Reference-first implementation rule | P0 | DONE | README, AGENTS, and this audit define the rule |
| CI guardrails | P0 | DONE | PR #2, PR #4, PR #6, PR #8, PR #10, PR #12, and PR #14 CI proof |

### 2. Rust workspace

| Feature | Pri | Status | Missing work | Required proof |
|---|---:|---|---|---|
| Workspace `Cargo.toml` | P0 | DONE | None for workspace scaffold | Repeated PR CI metadata success |
| `crates/nim_router` | P0 | PARTIAL | No live NIM call yet | Unit tests passed in PR #2/#4/#6/#8/#10/#12/#14 |
| `crates/model_contract` | P0 | PARTIAL | Streaming normalization still minimal | Contract tests passed in PR #2/#4/#6/#8/#10/#12/#14 |
| `crates/agent_core` | P0 | PARTIAL | Dry-run smoke session exists; no model loop yet | Dry-run tests passed in PR #4/#6/#8/#10/#12/#14 |
| `crates/tools` | P0 | PARTIAL | File, shell, and git-read tools exist; approval policy still minimal | Integration-style tests passed in PR #2/#4/#6/#8/#10/#12/#14 |
| `crates/proof` | P0 | PARTIAL | Ledger run-id accessor exists; generic persistent proof artifact still lives in WebUI smoke layer | Unit tests passed in PR #2/#4/#6/#8/#10/#12/#14 |
| `apps/webui` | P0 | PARTIAL | HTTP route/page/cards/events/report/artifact exist; no browser screenshot/live NIM yet | PR #14 proof artifact tests passed |

### 3. NIM routing

| Feature | Pri | Status | Missing work | Required proof |
|---|---:|---|---|---|
| NIM provider config | P0 | PARTIAL | Reads `NIM_KEY`; no live NIM request yet | Redacted config test passed in PR #2/#4/#6/#8/#10/#12/#14 |
| Deterministic model order | P0 | PARTIAL | Config parser exists; live route not wired | Test proves order stable across runs |
| Failure classification | P0 | PARTIAL | Provider/tool classification exists; no live route ledger yet | Unit table tests |
| Cooldowns | P0 | PARTIAL | Cooldown policy exists; no persisted route ledger yet | Time-controlled unit tests |
| Route ledger | P0 | MISSING | Route attempt type exists, but no persisted run ledger integration yet | Ledger fixture test |
| Malformed tool-call repair | P1 | MISSING | Not implemented | Fixture test with malformed JSON |
| Required-tool correction | P1 | MISSING | Contract can classify missing tool; correction loop not implemented | Fixture test |

### 4. Agent loop

| Feature | Pri | Status | Missing work | Required proof |
|---|---:|---|---|---|
| Session state machine | P0 | PARTIAL | Minimal state machine exists; no model loop yet | State transition tests passed in PR #2/#4/#6/#8/#10/#12/#14 |
| Objective ledger | P0 | PARTIAL | Minimal objective/evidence verification exists | Fixture test passed in PR #2/#4/#6/#8/#10/#12/#14 |
| Tool-call loop | P0 | PARTIAL | Dry-run file/git tool calls execute and write proof evidence; no model loop yet | Dry-run smoke tests passed in PR #4/#6/#8/#10/#12/#14 |
| Loop detector | P0 | MISSING | Detect repeated action/input/tool pattern | Unit tests |
| Max turn / budget guard | P0 | PARTIAL | Minimal max-turn guard exists | Unit tests passed in PR #2/#4/#6/#8/#10/#12/#14 |
| Final-claim verifier | P0 | PARTIAL | Smoke final answer is gated by required evidence; no model-generated final-answer verification yet | PR #12 ledger/report tests passed |
| First local smoke session | P0 | PARTIAL | Exposed through HTTP/card/event/ledger/report/artifact routes; no live NIM/browser screenshot yet | PR #14 proof artifact tests passed |
| Context compaction | P1 | MISSING | Summarize old turns without losing objective ledger | Long-run fixture |
| Pause/stop/resume | P2 | MISSING | Real runtime cancellation, not fake controls | Browser/runtime proof |

### 5. Tools

| Feature | Pri | Status | Missing work | Required proof |
|---|---:|---|---|---|
| `read_file` | P0 | PARTIAL | Implemented and dry-run wired; not wired to model loop | PR #14 proof artifact tests passed |
| `write_file` | P0 | PARTIAL | Implemented and dry-run wired; approval mode not wired yet | PR #14 proof artifact tests passed |
| `delete_file` | P0 | PARTIAL | Implemented and dry-run wired; approval mode not wired yet | PR #14 proof artifact tests passed |
| `list_dir` | P0 | PARTIAL | Implemented and dry-run wired; not wired to model loop | PR #14 proof artifact tests passed |
| `shell` | P0 | PARTIAL | Bounded runner exists; policy still minimal | Integration-style tests passed in PR #2/#4/#6/#8/#10/#12/#14 |
| `git_status` | P0 | PARTIAL | Implemented and dry-run wired; not wired to model loop | PR #14 proof artifact tests passed |
| `git_diff` | P0 | PARTIAL | Implemented and dry-run wired; not wired to model loop | PR #14 proof artifact tests passed |
| `git_commit` | P1 | MISSING | Explicit approval required | Integration test |
| Browser screenshot proof | P1 | MISSING | Browser-driven screenshot with run ID | Browser proof artifact |
| Web/search adapter | P2 | MISSING | Optional; not required for first local coding loop | Network-gated test |

### 6. WebUI and proof

| Feature | Pri | Status | Missing work | Required proof |
|---|---:|---|---|---|
| ChatGPT-like chat UI | P0 | MISSING | Basic conversation page with familiar composer/messages/sidebar feel | Screenshot |
| WebUI app-layer smoke API | P0 | PARTIAL | Smoke path has HTTP, cards, events, ledger, report, and proof-file route; no browser proof/live NIM yet | PR #14 route tests passed |
| Minimal smoke HTTP route | P0 | PARTIAL | `/`, `/api/smoke/dry-run`, `/api/smoke/ledger`, `/api/smoke/proof-file`, `/smoke/cards`, `/smoke/report`, and `/api/smoke/events` exist | PR #14 route tests passed |
| SSE streaming | P0 | PARTIAL | Smoke proof event stream exists; no live model token streaming yet | PR #10 event-stream route test passed |
| OpenCode-like agentic coding loop | P0 | MISSING | Plan/tool/observe/verify loop for coding tasks | Ledger proof |
| Visible model route cards | P0 | MISSING | Show selected model and fallback attempts | Browser proof |
| Visible tool cards | P0 | PARTIAL | Smoke tool cards exist; full live run cards need browser proof | PR #10 smoke cards route test passed |
| Run proof panel | P0 | PARTIAL | Smoke cards/events/ledger/report/proof-file expose proof; no dedicated full run panel yet | PR #14 route tests passed |
| Export run ledger | P0 | PARTIAL | Smoke ledger export and runtime proof file exist; persistent live run ledger export still needed | PR #14 proof artifact route test passed |
| Proof-backed final answer | P0 | PARTIAL | Smoke final answer is evidence-gated; no live model answer verifier yet | PR #12 report tests passed |
| Proof artifact linked to run id | P0 | PARTIAL | Smoke proof writes `.tiny-nim-agent/proofs/<run_id>.json`; browser screenshot link still missing | PR #14 proof artifact tests passed |
| Six-phase benchmark runner | P1 | MISSING | Normal WebUI prompt only, no harness bypass | Full proof artifacts |

## First acceptance target

The first real product milestone is not the full six-phase benchmark.

It is this small prompt through the WebUI:

```text
Inspect this repo, create a file named agent-smoke.txt containing one sentence, read it back, delete it, and report exactly what you did with proof.
```

Pass criteria:

- normal WebUI path,
- NIM route ledger,
- file write/read/delete tool events,
- final answer verified against ledger,
- screenshot/proof linked to same run ID.
