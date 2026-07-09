# CI notes

`NIM_KEY` is expected to exist as a GitHub Actions secret.

Rules:

- Normal CI must not require `NIM_KEY`.
- The normal Rust job runs metadata, workspace tests, product-source guardrails, and line-count guardrails.
- NIM checks are separate from normal CI until live NIM calls exist.
- CI may check whether `NIM_KEY` exists.
- CI must not print `NIM_KEY`.
- Runtime code reads `NIM_KEY` from the environment.
- `.env` files stay gitignored.

Current workflow shape:

- `rust`: always-on workspace and guardrail job.
- `nim-secret`: non-blocking secret preflight. It exits successfully if the secret is unavailable to the workflow event.

Local mirror:

```bash
bash scripts/verify.sh
```

That script runs the same checks as the `rust` CI job:

```bash
cargo metadata --format-version 1 --no-deps
cargo test --workspace
bash scripts/check_no_placeholders.sh
python3 scripts/check_line_count.py
```

If GitHub reports no workflow run for connector-created commits, the workflow definition may still be valid; use the manual `workflow_dispatch` trigger from the Actions tab or push a normal commit from a local clone to verify Actions event delivery.

The placeholder scanner checks Rust/product source, not policy docs, because the docs intentionally mention forbidden placeholder terms.
