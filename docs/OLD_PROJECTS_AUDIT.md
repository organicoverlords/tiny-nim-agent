# Old projects audit boundary

Created: 2026-07-09

## Decision

The new product is `organicoverlords/tiny-nim-agent`.

The old projects are references only. Do not merge them into this repo.

## Repositories observed

Installed/user repositories visible from GitHub search include:

- `organicoverlords/localgpt-cockpit-rust`
- `organicoverlords/forgestack-command`
- `organicoverlords/gigstack`
- `organicoverlords/forge-unified`
- `organicoverlords/agent-benchmark-app`
- `organicoverlords/magicmusic`
- others in the same account

The important diagnosis is not that these projects are useless. It is that they should not all be merged.

## Forge-unified PR #3 diagnosis

`forge-unified` PR #3 was a useful experiment but not a clean product base.

Observed state from PR metadata:

- open,
- unmerged,
- 1,661 commits,
- 387 changed files,
- 32,689 additions,
- 2,352 deletions,
- final-head parity explicitly not claimed in the PR body.

That is too large to use as the foundation for a tiny reliable agentic coder.

Useful lessons to extract:

- browser proof must be tied to the same run ID as the tool ledger,
- full benchmark checker success is not enough if user-visible proof remains confusing,
- final-head workflow state must be inspected before any parity claim,
- natural WebUI proof paths can fail separately from benchmark checker logic,
- source-reference notes must not leak external project identity into runtime UI.

Do not copy wholesale.

## Forge-unified PR #1 diagnosis

PR #1 already identified overstated status claims:

- WebSocket chat was an echo scaffold,
- main chat frontend was missing,
- persistence was partial/in-memory,
- cancel/pause/resume were API-shaped but not proven inside execution loops,
- benchmark adapter was shallow and not artifact-backed,
- CI branch filter missed `master`.

This is exactly why this new repo must enforce no-stub/no-placeholder rules from the first commit.

## Forge-unified PR #2 diagnosis

PR #2 added chat stream, visible tool cards, autosaved conversations, repo snapshot data, and stream smoke coverage, but its own body said CI was not proven from that chat.

Useful lesson:

- UI stream work is valuable only when backed by proof that runs on the current head.

## LocalGPT diagnosis

LocalGPT's feature audit and project state are useful as a historical source of requirements:

- canonical feature audit discipline,
- repo-root inventory before planning,
- visible provider order,
- no hidden fallback,
- workspace sandboxing,
- read-only vs build mode,
- batch result compaction to avoid context flooding,
- model/tool loop issues with large prompt and tool-result context,
- tool-card UI and conversation features.

However, LocalGPT also shows the danger of implementing too many features before the smallest complete coding loop is proven.

Use as reference for isolated ideas only.

## Agent benchmark app diagnosis

The benchmark app is useful to validate behavior, but it must not become the product.

Benchmark integration must obey:

- normal WebUI path,
- same run ID,
- real tool ledger,
- screenshot/browser proof attached to run,
- no prompt edits,
- no benchmark-specific result injection.

## GigStack / MagicMusic diagnosis

MagicMusic and GigStack-style tooling may help execute deterministic scripts and collect evidence.

They must not become:

- the planner,
- the agent runtime,
- the source of truth,
- the reason a WebUI action is counted as completed.

## What to extract into tiny-nim-agent

| Source | Extract concept | Do not extract |
|---|---|---|
| LocalGPT | feature audit, root preflight, sandbox ideas, provider order discipline | giant feature matrix as implementation target |
| Forge-unified | WebUI proof/run-ID lessons, NIM-specific failure experience | 1,661-commit PR branch |
| Superapp | screenshot/browser proof ideas | broad superapp scope |
| Agent benchmark app | checker concepts | product-as-benchmark harness |
| MagicMusic/GigStack | deterministic script running | agent intelligence |

## Rule for future extraction

Before copying any code from an old repo:

1. identify exact source file and commit,
2. explain why tiny-nim-agent needs it,
3. copy the smallest behavior, not the whole module,
4. rewrite names and boundaries for this repo,
5. add tests first or in the same commit,
6. update `FEATURE-AUDIT.md`.

No blind migrations.
