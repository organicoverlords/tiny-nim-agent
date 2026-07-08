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

- This repo must stay smaller and stricter than the old app stack.
- Features start as MISSING, not DONE.
- A feature row can move to DONE only with test/proof references.
- Runtime-visible features need a ledger/proof path, not just unit tests.
- Benchmark success must go through normal WebUI/session/runtime paths.
- No benchmark prompt edits to make the app pass.
- No hidden fallback or undocumented model order mutation.

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

### 2. Rust workspace

| Feature | Pri | Status | Missing work | Required proof |
|---|---:|---|---|---|
| Workspace `Cargo.toml` | P0 | MISSING | Create minimal workspace | `cargo metadata` passes |
| `crates/nim_router` | P0 | MISSING | Implement deterministic NIM route attempts | Unit tests for failure classification and cooldowns |
| `crates/model_contract` | P0 | MISSING | Normalize NIM/OpenAI-compatible streaming and tool-call payloads | Fixture tests for valid, malformed, empty responses |
| `crates/agent_core` | P0 | MISSING | State machine, objective ledger, max-turn guard, loop detector | Unit tests and one dry-run session proof |
| `crates/tools` | P0 | MISSING | Filesystem/shell/git tools with safety gates | Integration tests using temp repos |
| `crates/proof` | P0 | MISSING | Run ledger, tool ledger, screenshot/artifact references, claim verifier | JSON schema tests and proof fixture |
| `apps/webui` | P0 | MISSING | Axum server + static chat UI + SSE events | Browser proof screenshot tied to run ID |

### 3. NIM routing

| Feature | Pri | Status | Missing work | Required proof |
|---|---:|---|---|---|
| NIM provider config | P0 | MISSING | Env/config loader for NIM base URL and API key | Redacted config test |
| Deterministic model order | P0 | MISSING | Manual ordered list, no hidden mutation | Test proves order stable across runs |
| Failure classification | P0 | MISSING | Distinguish provider/model/tool/runtime failures | Unit table tests |
| Cooldowns | P0 | MISSING | Per-model cooldown for 429/5xx/timeouts | Time-controlled unit tests |
| Route ledger | P0 | MISSING | Persist attempts, selected model, error reasons | Ledger fixture test |
| Malformed tool-call repair | P1 | MISSING | One bounded repair turn before fallback | Fixture test with malformed JSON |
| Required-tool correction | P1 | MISSING | One correction turn when tool call is required but missing | Fixture test |

### 4. Agent loop

| Feature | Pri | Status | Missing work | Required proof |
|---|---:|---|---|---|
| Session state machine | P0 | MISSING | States: queued, planning, model_turn, tool_turn, verifying, final, failed | State transition tests |
| Objective ledger | P0 | MISSING | Track required objectives and phase completion | Fixture test |
| Tool-call loop | P0 | MISSING | Execute validated tool calls and feed observations back | Dry-run proof |
| Loop detector | P0 | MISSING | Detect repeated action/input/tool pattern | Unit tests |
| Max turn / budget guard | P0 | MISSING | Stop safely with partial proof | Unit tests |
| Final-claim verifier | P0 | MISSING | Compare final answer to ledger before claiming done | Fixture tests |
| Context compaction | P1 | MISSING | Summarize old turns without losing objective ledger | Long-run fixture |
| Pause/stop/resume | P2 | MISSING | Real runtime cancellation, not fake controls | Browser/runtime proof |

### 5. Tools

| Feature | Pri | Status | Missing work | Required proof |
|---|---:|---|---|---|
| `read_file` | P0 | MISSING | Safe path-scoped file read | Temp repo test |
| `write_file` | P0 | MISSING | Safe path-scoped file write with approval mode | Temp repo test |
| `delete_file` | P0 | MISSING | Safe path-scoped delete with approval mode | Temp repo test |
| `list_dir` | P0 | MISSING | Ignore `.git`, target, node_modules by default | Temp repo test |
| `shell` | P0 | MISSING | Bounded command runner with timeout and allow/block policy | Integration test |
| `git_status` | P0 | MISSING | Read-only git status | Temp repo test |
| `git_diff` | P0 | MISSING | Read-only diff display | Temp repo test |
| `git_commit` | P1 | MISSING | Explicit approval required | Integration test |
| Browser screenshot proof | P1 | MISSING | Browser-driven screenshot with run ID | Browser proof artifact |
| Web/search adapter | P2 | MISSING | Optional; not required for first local coding loop | Network-gated test |

### 6. WebUI and proof

| Feature | Pri | Status | Missing work | Required proof |
|---|---:|---|---|---|
| Chat-first UI | P0 | MISSING | Basic conversation page | Screenshot |
| SSE streaming | P0 | MISSING | Token/tool/progress events | Browser proof |
| Visible model route cards | P0 | MISSING | Show selected model and fallback attempts | Browser proof |
| Visible tool cards | P0 | MISSING | Running/succeeded/failed tool states | Browser proof |
| Run proof panel | P0 | MISSING | Link ledger, screenshots, final verifier | Browser proof |
| Export run ledger | P0 | MISSING | JSON export route | API test |
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
