# CLAUDE.md

This repo holds reusable GitHub Actions workflows (`on: workflow_call`)
that the Rust repos call from their `security.yml` and `release.yml`:
`supply-chain.yml`, `mutation.yml`, and `semver-preview.yml`. `README.md`
says what each does and how to call it.

The crate at the root (`rust-ci-workflows-selftest`) is a fixture, not a
library. `ci.yml` calls the three workflows against it through `./`, so a
change runs end to end on the push before it is tagged. Keep its tests
killing every mutant: the per-diff mutation run is part of this repo's CI.

What the self-test does not prove:

- **The per-diff run's `cargo mutants` call** runs only when the push
  changes a `.rs` file. A push that changes only workflows ends at "no
  Rust source changed since <base>; nothing to mutate". To prove a change
  to `mutation.yml`'s diff job, make a small real edit to `src/lib.rs`
  (with its test) in the same push.
  The weekly and manual runs cover the full sweep, sharded in two.
- **The semver check itself.** The fixture is `publish = false`, so
  `semver-preview.yml` exercises only its crate listing; its `setup` step
  and the cargo-semver-checks install and run are first exercised by a
  caller with a published crate.
- **A failing excluded manifest.** `fixtures/excluded` proves that
  `supply-chain.yml`'s `deny-manifests` step runs and passes, not that it
  fails on a bad license or source.

## Development commands

    actionlint                              # lint every workflow
    zizmor --offline .                      # security audit (CI runs it online)
    cargo fmt --check
    cargo clippy --all-targets --locked -- -D warnings
    cargo nextest run --locked
    cargo mutants                           # the fixture: every mutant caught
    cargo deny --workspace --locked check
    cargo vet

`prek install` once per clone puts the first five behind the commit hook.
The tools come from: actionlint, a release binary
(https://github.com/rhysd/actionlint/releases), checked against its
checksums file; zizmor, `cargo install --locked zizmor`; the cargo
subcommands, `cargo install` or `taiki-e/install-action` in CI.

## Gates

A change is ready when actionlint and zizmor are clean and the fixture's
fmt, clippy, nextest, and mutation runs pass. CI then proves the
workflows themselves: watch the push's `CI` run until every called job
is green before tagging.

## Changing a workflow

- Every action is pinned by commit SHA with the version in a comment, and
  every `taiki-e/install-action` step sets `fallback: none`.
- A job's `name:` is part of each caller's required check
  (`<caller job> / <name>`). Renaming one, removing or renaming an input,
  or making a gate stricter by default is a breaking change, which on 0.x
  is a minor bump.
- A repo-specific need becomes an input only when a local job beside the
  call cannot cover it.

## Releasing

1. Move the `## [Unreleased]` entries in `CHANGELOG.md` under a new
   version heading, and add the compare link.
2. Commit, push, and wait for `CI` on `main` to go green.
3. Tag that commit `vX.Y.Z` and push the tag. The `release tags` ruleset
   lets only an admin create, move, or delete a `v*` tag.

rust-project-template takes each release first; the other repos' Dependabot
moves their pins after its cooldown.

## Manual checks

What a session cannot do here:

- **`.claude/settings.json`.** The sandbox refuses writes to it. It carries
  the standard per-repo block (loopback network, `allowMachLookup` for
  `configd` and `trustd.agent`, `excludedCommands` for `git *` and `gh *`).
  Without it, `gh` cannot read its config and `cargo vet` panics in
  `system-configuration`.
- **`cargo vet`**, until that file exists: run it in a terminal.
- **GitHub settings**, which are admin changes made from a terminal:

      gh api -X POST repos/padamson/rust-ci-workflows/rulesets --input .github/rulesets/release-tags.json
      gh api -X PUT repos/padamson/rust-ci-workflows/vulnerability-alerts
      gh api -X PUT repos/padamson/rust-ci-workflows/automated-security-fixes

- **Pushing a tag.** The ruleset reserves it for an admin.
