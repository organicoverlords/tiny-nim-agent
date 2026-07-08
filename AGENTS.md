# AGENTS.md

Rules for every human, ChatGPT worker, MagicMusic-run script, or coding agent working in this repository.

## Read order before code changes

1. `README.md`
2. `FEATURE-AUDIT.md`
3. `docs/ARCHITECTURE.md`
4. `docs/NIM_ROUTING_AND_TOOL_LOOP.md`
5. `docs/NO_STUBS_POLICY.md`
6. `docs/ROADMAP.md`
7. Current git status and branch state

Do not start feature work before reading those files.

## Repo policy

- This is a clean repository.
- Do not merge old app branches into this repo.
- Do not copy large modules from ForgeStack V4, LocalGPT, Superapp, ForgeMerge, GigStack, or `forge-unified` without a focused source review and a written extraction note.
- External projects such as Agno, Hermes, OpenCode, and LibreChat are references for behavior and architecture only.
- Keep the repo small. Prefer one complete path over many partial systems.

## No-stub rule

A feature is not allowed to exist as a visible claim unless it is real.

Forbidden in production paths unless explicitly isolated in tests/examples:

- `todo!()`
- `unimplemented!()`
- `panic!("not implemented")`
- fake buttons with no backend behavior
- fake routes returning success without doing the action
- benchmark-only shortcuts
- hardcoded benchmark answers
- hidden provider fallback
- swallowing tool errors as success

If a feature is not implemented, say so in `FEATURE-AUDIT.md` and do not expose it as done.

## Required preflight before repo edits

Record:

```bash
pwd
git rev-parse --show-toplevel
git branch --show-current
git rev-parse --short HEAD
git remote -v
git status --short
```

Then identify:

- build system,
- package manager,
- test commands,
- major folders,
- docs read,
- current blockers.

## Done claim rule

A task is done only when all are true:

1. Code exists.
2. Tests exist.
3. Tests pass.
4. Proof or run ledger exists when the feature is runtime-visible.
5. `FEATURE-AUDIT.md` status is updated.
6. No new stubs or placeholder claims were introduced.
7. Final answer cites exact commit(s), files changed, and proof command(s).

## NIM/tool-loop rule

Do not route away from normal tool failures.

Provider/model fallback is allowed for:

- HTTP 429,
- HTTP 5xx,
- timeout,
- empty response,
- unavailable model,
- repeated invalid model output after bounded repair.

Provider/model fallback is not allowed for:

- shell command failed,
- file missing,
- git dirty,
- test failed,
- permission denied,
- path blocked,
- verifier failed.

Those are runtime/tool facts and must be surfaced to the agent ledger.

## MagicMusic rule

MagicMusic is a dumb scripting helper. It may run deterministic scripts and collect proof. It is not the agent, not the planner, and not the source of truth.
