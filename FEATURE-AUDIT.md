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
| 600-line file ceiling | P0 | DONE | README, AGENTS, audit, CI guard script |
| Reference-first implementation rule | P0 | DONE | README, AGENTS, and this audit define the rule |
| CI guardrails | P0 | PARTIAL | Workflow exists; latest Actions result not observed yet |

### 2. Rust workspace

| Feature | Pri | Status | Missing work | Required proof |
|---|---:|---|---|---|
| Workspace `Cargo.toml` | P0 | PARTIAL | CI result not observed | `cargo metadata` passes |
| `crates/nim_router` | P0 | PARTIAL | CI result not observed; no live NIM call yet | Unit tests for failure classification and cooldowns |
| `crates/model_contract` | P0 | PARTIAL | CI result not observed | Fixture tests for valid, malformed, empty responses |
| `crates/agent_core` | P0 | PARTIAL | CI result not observed; no dry-run model loop yet | Unit tests and one dry-run session proof |
| `crates/tools` | P0 | PARTIAL | File tools and path guard exist; CI result not observed | Integration tests using temp repos |
| `crates/proof` | P0 | PARTIAL | CI result not observed; no JSON export yet | JSON schema tests and proof fixture |
| `apps/webui` | P0 | PARTIAL | Target descriptor only; no UI/server yet | Browser proof screenshot tied to run ID |

### 3. NIM routing

| Feature | Pri | Status | Missing work | Required proof |
|---|---:|---|---|---|
| NIM provider config | P0 | PARTIAL | Reads `NIM_KEY`; CI/liveness not observed | Redacted config test |
| Deterministic model order | P0 | PARTIAL | Config parser exists; CI not observed | Test proves order stable across runs |
| Failure classification | P0 | PARTIAL | Provider/tool classification exists; CI not observed | Unit table tests |
| Cooldowns | P0 | PARTIAL | Cooldown policy exists; CI not observed | Time-controlled unit tests |
| Route ledger | P0 | MISSING | Route attempt type exists, but no persisted run ledger integration yet | Ledger fixture test |
| Malformed tool-call repair | P1 | MISSING | Not implemented | Fixture test with malformed JSON |
| Required-tool correction | P1 | MISSING | Contract can classify missing tool; correction loop not implemented | Fixture test |

### 4. Agent loop

| Feature | Pri | Status | Missing work | Required proof |
|---|---:|---|---|---|
| Session state machine | P0 | PARTIAL | Minimal state machine exists; CI not observed | State transition tests |
| Objective ledger | P0 | PARTIAL | Minimal objective/evidence verification exists; CI not observed | Fixture test |
| Tool-call loop | P0 | MISSING | Execute validated tool calls and feed observations back | Dry-run proof |
| Loop detector | P0 | MISSING | Detect repeated action/input/tool pattern | Unit tests |
| Max turn / budget guard | P0 | PARTIAL | Minimal max-turn guard exists; CI not observed | Unit tests |
| Final-claim verifier | P0 | PARTIAL | Required evidence verifier exists; no final-answer integration | Fixture tests |
| Context compaction | P1 | MISSING | Summarize old turns without losing objective ledger | Long-run fixture |
| Pause/stop/resume | P2 | MISSING | Real runtime cancellation, not fake controls | Browser/runtime proof |

### 5. Tools

| Feature | Pri | Status | Missing work | Required proof |
|---|---:|---|---|---|
| `read_file` | P0 | PARTIAL | Implemented in `crates/tools`; CI result not observed | Temp repo test |
| `write_file` | P0 | PARTIAL | Implemented in `crates/tools`; approval mode not wired yet | Temp repo test |
| `delete_file` | P0 | PARTIAL | Implemented in `crates/tools`; approval mode not wired yet | Temp repo test |
| `list_dir` | P0 | PARTIAL | Implemented in `crates/tools`; CI result not observed | Temp repo test |
| `shell` | P0 | MISSING | Bounded command runner with timeout and allow/block policy | Integration test |
| `git_status` | P0 | MISSING | Read-only git status | Temp repo test |
| `git_diff` | P0 | MISSING | Read-only diff display | Temp repo test |
| `git_commit` | P1 | MISSING | Explicit approval required | Integration test |
| Browser screenshot proof | P1 | MISSING | Browser-driven screenshot with run ID | Browser proof artifact |
| Web/search adapter | P2 | MISSING | Optional; not required for first local coding loop | Network-gated test |

### 6. WebUI and proof

| Feature | Pri | Status | Missing work | Required proof |
|---|---:|---|---|---|
| ChatGPT-like chat UI | P0 | MISSING | Basic conversation page with familiar composer/messages/sidebar feel | Screenshot |
| SSE streaming | P0 | MISSING | Token/tool/progress events | Browser proof |
| OpenCode-like agentic coding loop | P0 | MISSING | Plan/tool/observe/verify loop for coding tasks | Ledger proof |
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
