# tiny-nim-agent

Tiny Rust agentic coding app with NIM-only FreeLLMAPI-style routing, visible tool loops, and WebUI proof ledger.

This repository is a clean product reset. It is **not** a merge target for ForgeStack V4, LocalGPT, Superapp, ForgeMerge, GigStack, or `forge-unified`.

## Product target

Build one small reliable agentic coding app that can:

1. accept a natural-language coding task in the WebUI,
2. plan the task,
3. call safe tools,
4. write/read/delete files when explicitly allowed,
5. run verification commands,
6. keep a durable ledger of every model route, tool call, output, screenshot, and final claim,
7. pass the six-phase benchmark through the normal WebUI path without benchmark-specific shortcuts.

## Non-negotiables

- Rust runtime owns the agent loop. NIM is only the model provider.
- NIM routing is deterministic and visible.
- Provider fallback is only for provider/model failures, not normal tool failures.
- No hidden fallback to unapproved providers.
- No benchmark-specific code paths.
- No feature is marked done without tests and proof.
- No stubs, fake UI controls, placeholder routes, or `todo!()` implementations can ship as product features.
- MagicMusic/GigStack-style runners may execute scripts, but they are not the agent.

## Documentation source of truth

Read these before coding:

- [`FEATURE-AUDIT.md`](FEATURE-AUDIT.md) — canonical feature/proof contract.
- [`AGENTS.md`](AGENTS.md) — rules for agents and workers.
- [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) — target architecture and reference-app lessons.
- [`docs/NIM_ROUTING_AND_TOOL_LOOP.md`](docs/NIM_ROUTING_AND_TOOL_LOOP.md) — NIM failure classification and loop contract.
- [`docs/NO_STUBS_POLICY.md`](docs/NO_STUBS_POLICY.md) — anti-placeholder rules.
- [`docs/OLD_PROJECTS_AUDIT.md`](docs/OLD_PROJECTS_AUDIT.md) — why old projects are references only.
- [`docs/ROADMAP.md`](docs/ROADMAP.md) — implementation slices in required order.

## Current state

Docs-first reset. Runtime code has not started yet.

The first code milestone is a minimal Rust workspace with:

```text
crates/nim_router
crates/model_contract
crates/agent_core
crates/tools
crates/proof
apps/webui
```

No crate should expose a claimed feature until its tests and proof path exist.
