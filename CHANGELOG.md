# Changelog

All notable changes to this project are documented in this file. The format
follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the
project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-11-16

### Added

- `supply-chain.yml`: cargo-deny's bans, licenses, and sources checks,
  cargo-lockpatrol's advisory check over every tracked `Cargo.lock`, and
  cargo-vet, which fails on a tag when `supply-chain/` is missing.
- `mutation.yml`: the per-diff mutation run on push and pull request, and
  the full sweep on manual and scheduled runs, with `shards`,
  `mutants-args`, and `setup` inputs.
- `semver-preview.yml`: `cargo semver-checks check-release` against the
  last crates.io release, on push and pull request.

[Unreleased]: https://github.com/padamson/rust-ci-workflows/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/padamson/rust-ci-workflows/releases/tag/v0.1.0
