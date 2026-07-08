# CI notes

`NIM_KEY` is expected to exist as a GitHub Actions secret.

Rules:

- CI may check whether `NIM_KEY` exists.
- CI must not print `NIM_KEY`.
- Runtime code reads `NIM_KEY` from the environment.
- `.env` files stay gitignored.

The placeholder scanner checks Rust/product source, not policy docs, because the docs intentionally mention forbidden placeholder terms.
