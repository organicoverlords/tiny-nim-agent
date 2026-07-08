# Architecture — tiny-nim-agent

Created: 2026-07-09

## Goal

Build a tiny reliable Rust agentic coding app. The runtime, not the model, owns progress.

The core design principle:

```text
NIM is the provider. Rust is the agent.
```

## Reference projects and what to copy conceptually

### Agno / PhiData lineage

Useful lesson: production agent platforms need a control plane, session storage, traces, permissions, scheduling, human approval, and observability.

Do copy conceptually:

- run/session records,
- traces/audit logs,
- tool permission boundaries,
- human approval as a first-class state,
- scheduled/background runs later.

Do not copy now:

- full multi-tenant platform shape,
- huge integration catalog,
- RBAC/auth complexity,
- production deployment framework.

### Hermes Agent

Useful lesson: the agent should preserve useful experience, skills, and session continuity, but those must not make the first runtime unstable.

Do copy conceptually:

- persistent session memory later,
- explicit skill files later,
- checkpoint summaries,
- tool gateway separation,
- warning that runtime dependencies must not live inside the agent-controlled checkout.

Do not copy now:

- self-improvement loop,
- complex gateway ecosystem,
- scheduled autonomy before basic tool-loop reliability.

### OpenCode

Useful lesson: a coding agent needs durable sessions, tool execution, file-change awareness, provider boundaries, and a small runtime loop that keeps acting until task completion.

Do copy conceptually:

- session/run state,
- tool-call lifecycle,
- file-change tracking,
- permissions,
- compact terminal-like proof discipline,
- non-interactive prompt support later.

Do not copy now:

- terminal-first UX,
- full provider matrix,
- any source identity or UI metadata from OpenCode,
- benchmark-specific behavior.

### LibreChat

Useful lesson: the UI should be chat-first, with agents/tools/model switching organized cleanly, and tool integrations kept understandable.

Do copy conceptually:

- chat-first UI,
- visible model selection,
- visible tool cards,
- searchable conversations later,
- MCP-style grouped tools later.

Do not copy now:

- multi-user auth,
- massive provider surface,
- artifacts/plugins marketplace,
- complex preset system.

## Old user project lessons

### LocalGPT

Keep as reference for:

- Rust chat UI ideas,
- local model ergonomics,
- feature audit discipline,
- synthetic parallel batch lessons,
- context compaction warnings.

Do not merge.

### ForgeStack V4 / forge-unified

Keep as postmortem/reference for:

- why proof must correlate to one run ID,
- why branch sprawl destroys confidence,
- why PR-sized product mutations become impossible to review,
- why browser proof should be product runtime data, not after-the-fact cosmetics.

Do not merge.

### Superapp

Keep as reference for:

- browser proof,
- screenshot/vision review concepts,
- UI proof artifacts.

Do not merge.

### GigStack / MagicMusic

Keep as runner/proof transport only.

Do not model it as agent intelligence.

## System shape

```text
apps/webui
  -> HTTP/SSE API
  -> agent_core SessionRunner
  -> nim_router RouteEngine
  -> model_contract ResponseNormalizer
  -> tools ToolExecutor
  -> proof RunLedger + ClaimVerifier
  -> storage SQLite/files
```

## Crate boundaries

### `crates/nim_router`

Owns:

- configured NIM models,
- deterministic route order,
- route attempt ledger events,
- provider/model failure classification,
- cooldowns.

Does not own:

- tool failure policy,
- benchmark scoring,
- app UI.

### `crates/model_contract`

Owns:

- OpenAI-compatible request/response structs,
- streaming event normalization,
- tool-call JSON extraction,
- malformed tool-call repair request generation,
- strict schemas for internal events.

Does not own:

- provider fallback decisions,
- shell execution,
- benchmark logic.

### `crates/agent_core`

Owns:

- session state machine,
- objective ledger,
- turn budgets,
- loop detector,
- tool-required policy,
- final answer gate.

This is the heart of the app.

### `crates/tools`

Owns:

- safe tool definitions,
- input validation,
- path scope,
- command timeout,
- permission checks,
- normalized tool results.

### `crates/proof`

Owns:

- run IDs,
- route ledger,
- tool ledger,
- screenshots/artifact records,
- final-claim verifier,
- exportable JSON proof.

### `apps/webui`

Owns:

- chat UI,
- SSE stream rendering,
- visible model route cards,
- visible tool cards,
- run proof panel.

It must not fake backend state.

## Session state machine

Minimum states:

```text
queued
planning
model_turn
tool_validation
tool_execution
observation_recorded
verifying
final_answer
failed
cancelled
```

The model cannot skip the state machine.

## Event model

Every run writes append-only events:

```json
{"type":"run_started","run_id":"..."}
{"type":"route_attempt","provider":"nvidia_nim","model":"...","status":"started"}
{"type":"model_delta","text":"..."}
{"type":"tool_call_requested","name":"read_file","args":{}}
{"type":"tool_call_validated","id":"..."}
{"type":"tool_result","status":"ok","output_preview":"..."}
{"type":"objective_completed","objective_id":"phase_1_repo_inventory"}
{"type":"final_claim_checked","status":"pass"}
```

## Why this should stop loops

Loops happen when progress is implicit in model context.

This app makes progress explicit in the runtime ledger:

- objectives cannot complete without evidence,
- repeated action patterns are detected,
- tool errors are facts, not hidden retries,
- context compaction preserves the objective ledger,
- the final answer is checked against proof before display.

## First runtime objective

Do not start with the six-phase benchmark.

Start with one small complete loop:

```text
write file -> read file -> delete file -> verify final claim
```

Only after that works through WebUI should the project add git/shell/repo-inspection depth and then the full benchmark.
