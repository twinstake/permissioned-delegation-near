# Near whitelisted staking pool

This repo contains a single contract:

`staking-pool` - deploys and manages a staking pool with possibilty of "whitelisting" deposit and staking operation. It allows whitelisted users to stake NEAR tokens, unstake and withdraw rewards.

## Implementation details

`staking-pool` based on the [standard](https://github.com/near/core-contracts/tree/master/staking-pool) NEAR staking pool contract with some modifications:

- added `whitelist_account_ids` LookupSet to store whitelisted accounts;
- added additional checks for `deposit`, `stake`, `deposit_and_stake`, `deposit_all`, `stake_all` methods to allow only whitelisted accounts to perform these operations;
- added `add_to_whitelist` and `remove_from_whitelist` methods to manage `whitelist_account_ids` LookupSet;
- added `is_whitelisted_account_id` method to check if account is whitelisted;
- fixed existing and added some new unit-tests to cover new functionality.

# Building

It's **highly** recommended to use the provided Docker image to set up a consistent development environment due to the usage of native dependencies.

If you're using VSCode, a `devcontainer` configuration is provided already. [You can read more about it here](https://code.visualstudio.com/docs/devcontainers/containers)

Each project contains a `build.sh` and `test.sh` which build and test each project, respectively. You should use this scripts instead of `cargo build` and `cargo test` directly since some post-build commands are necessary after building each project.
