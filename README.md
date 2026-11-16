# rust-ci-workflows

Reusable GitHub Actions workflows for the CI gates my Rust repos share:
cargo-deny and cargo-vet, mutation testing, and a semver preview. Each repo
calls them from its own `security.yml` instead of carrying copies of the
jobs, so a fix to a gate is one release here and a Dependabot bump in each
repo.

This covers the CI half only. The local half stays in each repo, since a
pre-commit hook cannot call a workflow: the prek hooks, `deny.toml`,
`supply-chain/`, `.cargo/mutants.toml`, and `.config/nextest.toml`. The
workflows read those files from the caller's checkout.

## The workflows

| Workflow | Jobs (check names) | Runs on the caller's |
|---|---|---|
| `supply-chain.yml` | `License & Dependency Check`, `Supply Chain Review` | every event |
| `mutation.yml` | `Mutation Testing (diff)`; `Mutation Testing (full)` | diff: `push`, `pull_request`; full: `workflow_dispatch`, `schedule` |
| `semver-preview.yml` | `Semver preview` | `push`, `pull_request` |

A called job reports its check as `<caller job> / <job name>`, so a caller
job named `supply-chain` produces `supply-chain / Supply Chain Review`.
Branch rulesets and any script that lists required checks need the `/`
form. With `shards` above 1 the full sweep's jobs are named `Mutation
Testing (full, shard 0/8)` through `(full, shard 7/8)`, counting from zero
as cargo-mutants does; none of them suits a required check, since they run
only on manual and scheduled runs.

`supply-chain.yml` runs `cargo deny check bans licenses sources` with
`--locked`, then the advisory check over every tracked `Cargo.lock` through
[cargo-lockpatrol](https://github.com/padamson/cargo-lockpatrol), then `cargo
vet` (a release binary from install-action, so Dependabot moves its version
with the action's pin). A repo with no `supply-chain/audits.toml` skips vet, except on a tag,
where it fails: with no store, nothing vets what the release publishes.
Release workflows call it on the tag for that reason.

`mutation.yml` runs `cargo mutants --in-diff` over the lines a push or pull
request changed, and the full sweep on a manual or scheduled run. A
branch's first push, and a force-push whose previous tip is gone, are
diffed against the default branch, so every commit the branch adds is
covered. Call it once per workflow run: the report artifacts have fixed
names (`mutation-report-diff`, `mutation-report`), and a second call in the
same run fails at its upload. It fails on exit 3 (a mutant timed out). A mutant known to hang goes in the caller's
`exclude_re` with a comment saying the timeout detects it, so a new timeout
still shows up red. Its inputs:

| Input | Type | Default | What it does |
|---|---|---|---|
| `shards` | number | `1` | Splits the full sweep into that many jobs, round-robin. |
| `mutants-args` | string | `""` | Extra arguments for every `cargo mutants` call, parsed as shell words, so quote one that contains spaces. |
| `setup` | string | `""` | Bash run before the build in both jobs, *e.g.* stubbing wasm-pack output. Append to `$GITHUB_ENV` to set a variable for later steps. |

`semver-preview.yml` runs `cargo semver-checks check-release` on each
publishable crate that has a library target and is already on crates.io.
It is the early signal for a release's semver gate, not a required check:
between releases `Cargo.toml` still carries the last released version, so
a red run means the next release needs a bigger bump than planned.

## Calling them

Pin each call by commit SHA with the version in a comment, so Dependabot's
`github-actions` entry can move the pin:

```yaml
permissions:
  contents: read

jobs:
  supply-chain:
    uses: padamson/rust-ci-workflows/.github/workflows/supply-chain.yml@<sha> # v0.1.0

  semver-preview:
    uses: padamson/rust-ci-workflows/.github/workflows/semver-preview.yml@<sha> # v0.1.0

  mutation:
    uses: padamson/rust-ci-workflows/.github/workflows/mutation.yml@<sha> # v0.1.0
    with:
      shards: 8
      mutants-args: --test-workspace=true
      setup: |
        mkdir -p crates/viz/pkg
        touch crates/viz/pkg/viz.js crates/viz/pkg/viz_bg.wasm
        echo "PLAYWRIGHT_SKIP_DRIVER_DOWNLOAD=1" >> "$GITHUB_ENV"
```

A called workflow gets no more permission than its caller gives it, and
all three need only `contents: read`. A repo that publishes no crates
leaves out `semver-preview`. A repo that runs a weekly schedule for its
other jobs but wants the full mutation sweep on manual runs only puts `if:
github.event_name != 'schedule'` on its `mutation` job. A job only one repo
needs (a fuzz smoke test, say) stays a local job beside the calls.

## Versions

Releases are tagged `vX.Y.Z`, and the history is in
[`CHANGELOG.md`](CHANGELOG.md). Renaming a job (and so a required check),
removing or renaming an input, or making a gate stricter by default is a
breaking change. While the version is 0.x that means a minor bump.

A release reaches every calling repo within one Dependabot cycle, so
rust-project-template takes each release first, and the callers' Dependabot
cooldown is the window to catch a bad one.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.
