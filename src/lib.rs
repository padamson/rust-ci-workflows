//! A fixture for this repo's CI. `mutation.yml` mutates this file, so its
//! tests must kill every mutant cargo-mutants makes of it.

// Under GitHub Actions the test build requires SELFTEST_SETUP, which only
// ci.yml sets: through `mutation.yml`'s `setup` input, and on its own
// fixture job. A setup step that stops running, or whose variable stops
// reaching cargo-mutants, then fails the mutation baseline.
#[cfg(test)]
const _: () = assert!(
    option_env!("GITHUB_ACTIONS").is_none() || option_env!("SELFTEST_SETUP").is_some(),
    "SELFTEST_SETUP is unset: mutation.yml's setup input did not reach the build"
);

/// The most mutants any one of `shards` shards gets when `mutants` mutants
/// are split among them.
///
/// # Panics
///
/// Panics if `shards` is zero.
pub fn per_shard(mutants: u32, shards: u32) -> u32 {
    if shards == 0 {
        panic!("shards must be at least 1");
    }
    mutants.div_ceil(shards)
}

/// The shard, counting from zero, that round-robin sharding gives the
/// mutant at `index`.
///
/// # Panics
///
/// Panics if `shards` is zero.
pub fn shard_of(index: u32, shards: u32) -> u32 {
    index % shards
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn per_shard_rounds_up() {
        assert_eq!(per_shard(10, 4), 3);
        assert_eq!(per_shard(8, 4), 2);
        assert_eq!(per_shard(0, 4), 0);
    }

    #[test]
    #[should_panic(expected = "shards must be at least 1")]
    fn per_shard_rejects_zero_shards() {
        per_shard(1, 0);
    }

    #[test]
    fn shard_of_wraps_around() {
        assert_eq!(shard_of(0, 3), 0);
        assert_eq!(shard_of(1, 3), 1);
        assert_eq!(shard_of(5, 3), 2);
    }
}
