# Roadmap

Created: 2026-07-09

This roadmap is intentionally small. Complete each slice before starting the next one.

## Rule

No feature is added as a stub. If a slice cannot be completed, leave it MISSING in `FEATURE-AUDIT.md` and stop with a blocker.

## Slice 0 — docs and contracts

Status: DONE

Deliverables:

- README
- AGENTS rules
- feature audit
- architecture
- NIM/tool-loop contract
- no-stubs policy
- old-project audit boundary
- roadmap

Proof:

- files committed on `main`

## Slice 1 — minimal Rust workspace

Goal: create a compilable empty-but-real Rust workspace.

Deliverables:

```text
Cargo.toml
crates/nim_router/Cargo.toml
crates/model_contract/Cargo.toml
crates/agent_core/Cargo.toml
crates/tools/Cargo.toml
crates/proof/Cargo.toml
apps/webui/Cargo.toml
```

Required behavior:

- each crate has at least one real type and one test,
- no placeholder public feature claims,
- `cargo test --workspace` passes.

Acceptance:

```bash
cargo metadata --format-version 1
cargo test --workspace
rg -n "todo!\(|unimplemented!\(|not implemented|placeholder|fake" . -g '!target'
```

## Slice 2 — `nim_router`

Goal: deterministic NIM-only routing with visible failure classification.

Deliverables:

- model config type,
- manual ordered model list,
- provider/model failure enum,
- cooldown table,
- route attempt event structs,
- redacted config display.

Tests:

- deterministic order remains stable,
- 429 creates cooldown,
- timeout falls to next model,
- tool failure does not route,
- hidden non-NIM fallback impossible in NIM-only mode.

No network call required yet. Use fixtures.

## Slice 3 — `model_contract`

Goal: normalize provider responses and validate tool calls.

Deliverables:

- OpenAI-compatible chat request/response structs,
- streaming event enum,
- normalized model response,
- tool call schema representation,
- malformed JSON classification,
- required-tool missing classification.

Tests:

- valid tool call accepted,
- unknown tool rejected,
- invalid args rejected,
- empty response classified,
- malformed tool call produces repair request.

## Slice 4 — `tools`

Goal: safe local repo tools.

Deliverables:

- `read_file`,
- `write_file`,
- `delete_file`,
- `list_dir`,
- `git_status`,
- bounded `shell` command.

Safety:

- all paths are scoped to workspace root,
- `.git`, `.env`, secret-like files, target, node_modules guarded as appropriate,
- shell has timeout,
- destructive actions require explicit permission mode.

Tests:

- temp repo read/write/delete,
- outside-root path blocked,
- timeout classified,
- nonzero exit is tool failure not provider failure.

## Slice 5 — `proof`

Goal: append-only run ledger and final claim verifier.

Deliverables:

- run ID,
- event schema,
- route attempt events,
- tool call/result events,
- objective events,
- final claim verification verdict,
- JSON export.

Tests:

- ledger round-trip,
- final claim rejected if required evidence missing,
- final claim passes when evidence exists,
- proof entries reference same run ID.

## Slice 6 — `agent_core`

Goal: one complete dry-run session loop.

Deliverables:

- session state machine,
- objective ledger,
- max-turn guard,
- loop detector,
- tool execution integration,
- final answer verifier integration.

Acceptance prompt fixture:

```text
Create a file named agent-smoke.txt containing one sentence, read it back, delete it, and report exactly what you did.
```

This can run against a fake model fixture first.

Tests:

- complete write/read/delete loop,
- repeated same failed tool triggers loop detector,
- final claim rejected without read/delete evidence,
- max-turn failure writes partial proof.

## Slice 7 — WebUI MVP

Goal: normal browser path for the small smoke prompt.

Deliverables:

- Axum server,
- static chat UI,
- SSE stream,
- visible route cards,
- visible tool cards,
- proof panel,
- run ledger download.

Acceptance:

- user submits smoke prompt in browser,
- NIM route shown,
- tool cards shown,
- final proof references same run ID,
- screenshot is produced.

## Slice 8 — real NIM call

Goal: use real NIM through the same router/contract.

Deliverables:

- NIM API client,
- env config,
- redaction,
- route ledger,
- streamed response support if available,
- strict timeouts.

Acceptance:

- small smoke prompt works with NIM,
- tool-required correction works when model answers text instead of tool call,
- 429/timeout failure path visible.

## Slice 9 — six-phase benchmark readiness

Goal: run the six-phase benchmark as a normal WebUI prompt.

Required before attempting:

- objective ledger can represent phases,
- proof verifier can check required evidence,
- browser proof is tied to run ID,
- context compaction keeps objective ledger,
- loop detector proven.

Acceptance:

- full benchmark run through WebUI,
- route ledger attached,
- tool ledger attached,
- screenshots attached,
- final answer verified against ledger,
- no benchmark-specific code paths.

## Things explicitly deferred

- multi-user auth,
- local model providers,
- Groq/OpenRouter fallback,
- MCP server marketplace,
- subagents,
- scheduled background autonomy,
- browser-control tools beyond proof screenshot,
- voice/mobile app.

These are not rejected forever. They are deferred until the first reliable NIM WebUI coding loop exists.
