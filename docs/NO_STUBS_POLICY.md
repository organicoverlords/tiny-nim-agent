# No-stubs policy

Created: 2026-07-09

## Purpose

The old stack accumulated too many UI controls, endpoints, proof paths, and feature rows that looked finished before they were actually usable.

This repository must not repeat that.

## Definition of a stub

A stub is any code or UI that implies a feature works while the real behavior is missing, fake, bypassed, oversized, or unverified.

Examples:

- route returns `200 OK` but does not perform the action,
- button exists but has no backend behavior,
- handler returns canned benchmark output,
- function contains `todo!()` or `unimplemented!()` in product path,
- feature row says DONE without tests/proof,
- proof file is generated without being tied to the actual run ID,
- fallback silently changes providers/model order,
- final answer claims success without verifier evidence,
- a file grows past 600 lines instead of being split,
- a new custom subsystem is invented when a proven reference behavior should be copied.

## Allowed incomplete work

Incomplete work is allowed only when it is explicitly marked:

- `MISSING` in `FEATURE-AUDIT.md`, or
- `PLANNED` in `FEATURE-AUDIT.md`, or
- hidden behind a clearly disabled feature flag, or
- inside tests/examples where it cannot be confused for product behavior.

## Forbidden in product paths

Do not commit product-path code containing:

```rust
todo!()
unimplemented!()
panic!("not implemented")
```

Do not commit product-path routes that return fake success.

Do not commit a visible UI button unless either:

1. the backend behavior exists and is tested, or
2. the button is disabled and says the feature is not implemented.

Do not commit any tracked file over 600 lines.

## Reference-first rule

Before creating a feature, name the existing product behavior being copied:

- ChatGPT for chat UI feel,
- OpenCode for agentic coding behavior,
- LibreChat for chat/tool organization,
- Hermes for agent-loop boundaries,
- LocalGPT/Forge experiments only for isolated lessons.

If no reference exists, write down why a new local design is needed before coding.

## Completion checklist for every feature

A feature may move to DONE only if:

- there is implementation code,
- there are unit or integration tests,
- there is runtime proof when user-visible,
- failure paths are tested,
- the feature is visible in the ledger if it affects agent runs,
- `FEATURE-AUDIT.md` is updated,
- no benchmark-specific shortcut was added,
- all changed files are at or below 600 lines.

## Anti-cheat rules

The following are rejected even if tests pass:

- hardcoded handling for the six-phase prompt,
- prompt rewriting to make a benchmark easier,
- disabling real browser proof and accepting DOM summaries as equivalent screenshots,
- pretending a tool executed when the action came from the harness,
- accepting final answer text as proof,
- counting score artifacts not tied to the same run ID.

## Review command suggestions

Before a DONE claim, workers should search for placeholders and oversized files:

```bash
rg -n "todo!\(|unimplemented!\(|not implemented|stub|placeholder|fake|TODO|FIXME|HACK" . \
  -g '!target' \
  -g '!node_modules' \
  -g '!.git'

find . -type f \
  -not -path './.git/*' \
  -not -path './target/*' \
  -not -path './node_modules/*' \
  -print0 | xargs -0 wc -l | sort -nr | head -20
```

This search is not enough by itself, but it catches obvious mistakes.

## Manual proof wins

If user/manual browser proof says a feature is still broken, mark the claim as:

```text
REJECTED / NOT PROVEN
```

Then inspect the real failing path. Do not keep broad-running tests that do not exercise the failing path.
