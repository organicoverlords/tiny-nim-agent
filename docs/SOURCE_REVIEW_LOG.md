# Source review log

Created: 2026-07-09

This file records what has actually been reviewed before implementation work. It exists to avoid fake claims like "fully analyzed" when only summaries or memory were used.

## Current review status

Status: INITIAL DOCS REVIEW ONLY

A complete line-by-line source audit of every old repo has not been completed yet. Runtime coding must begin with targeted source review for the slice being implemented.

## Sources reviewed for initial architecture

### Public reference projects

| Project | Reviewed material | Used for | Not used for |
|---|---|---|---|
| Agno | GitHub README/repo overview | control plane, sessions, traces, approval, scheduling as future concepts | full platform clone |
| Hermes Agent | GitHub README/repo overview and developer architecture docs | agent loop, prompt/provider/tool split, session storage, tool registry, gateway separation, context compression lessons | full Python agent clone |
| OpenCode | GitHub README/repo overview | coding-agent product boundary, non-interactive prompt concept, terminal-agent discipline | terminal-first UX clone |
| LibreChat | GitHub README/repo overview | chat-first UI, tools/MCP/agents/model switching ideas | full multi-user chat platform |

### User-owned project material

| Project | Reviewed material | Key finding |
|---|---|---|
| `forge-unified` | PR #1, PR #2, PR #3 metadata/body | useful experiments, but not a clean base; PR #3 is too large and final-head parity was not claimed |
| `localgpt-cockpit-rust` | `FEATURE-AUDIT.md`, `.localgpt/PROJECT_STATE.md` | strong historical contract; also shows danger of too many features before tiny loop is proven |
| `agent-benchmark-app` | repository presence/role from prior work and repo search | benchmark must stay external validator, not product runtime |
| `magicmusic` / `gigstack` | repository presence/role from prior work and repo search | runner/proof transport only, not agent intelligence |

## Required source-review workflow before copying code

For each implementation slice:

1. Identify exact source repo, branch/ref, and file paths to inspect.
2. Read the relevant source files, not just docs.
3. Write a short extraction note in this file:
   - source file,
   - behavior extracted,
   - behavior intentionally not copied,
   - risks,
   - tests required.
4. Implement only the minimal behavior needed for the slice.
5. Add tests and proof in the same slice.
6. Update `FEATURE-AUDIT.md`.

## Immediate next source-review targets

### Slice 1 — minimal Rust workspace

No external source copy needed.

### Slice 2 — NIM router

Review before coding:

- LocalGPT provider/router config and fallback tests,
- Forge-unified NIM router/failure classification code,
- FreeLLMAPI routing pattern if available in user source or docs.

Extract only:

- deterministic model order,
- cooldown classification,
- visible route ledger,
- no hidden fallback.

### Slice 3 — model contract

Review before coding:

- Forge-unified and LocalGPT tool-call parsing,
- OpenCode/Hermes tool-dispatch concepts,
- NIM OpenAI-compatible streaming quirks from prior proof logs if available.

Extract only:

- normalization fixtures,
- bounded repair behavior,
- strict schema validation.

### Slice 4 — tools

Review before coding:

- LocalGPT workspace sandbox/path guard,
- Hermes file/terminal tool boundaries,
- OpenCode file editing/session ideas.

Extract only:

- safe path policy,
- command timeout classification,
- tool result shape.

## No broad migration rule

Do not import an old module wholesale unless:

- it is smaller than a fresh rewrite,
- it has tests that can be ported,
- it does not drag unrelated features,
- the extraction note explains why copying is safer than rewriting.

## Current blocker to claiming full source audit

A complete source audit requires cloning or API-reading the key old repos and reference modules file-by-file. That has not been completed in this docs-first pass.

Until that happens, claims must say:

```text
initial architecture review complete; targeted source review required before implementation slice
```
