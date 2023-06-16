# Near whitelisted staking pool

This repo containce three contracts:

`staking-pool-factory` - deploys and automatically whitelists new staking pool contracts. It allows any user to create a new whitelisted staking pool;

`staking-pool` - deploys and manages a staking pool with possibilty of "whitelisting" deposit and staking operation. It allows whitelisted users to stake NEAR tokens, unstake and withdraw rewards.

`whitelist` - the purpose of this contract is to maintain the whitelist of the staking pool contracts account IDs that are approved
by NEAR Foundation..

## Staking pool implementation details

`staking-pool` based on the [standard](https://github.com/near/core-contracts/tree/master/staking-pool) NEAR staking pool contract with some modifications:

- added `whitelist_account_ids` LookupSet to store whitelisted accounts;
- added additional checks for `deposit`, `stake`, `deposit_and_stake`, `deposit_all`, `stake_all` methods to allow only whitelisted accounts to perform these operations;
- added `add_to_whitelist` and `remove_from_whitelist` methods to manage `whitelist_account_ids` LookupSet;
- added `is_whitelisted_account_id` method to check if account is whitelisted;
- fixed existing and added some new unit-tests to cover new functionality.

## Staking pool factory implementation details

`staking-pool-factory` has same code as [standard](https://github.com/near/core-contracts/tree/master/staking-pool-factory) NEAR staking pool factory contract without any modifications. Purpose to add this contract is next: contract use relative path to staking pool contract binary (`"../../staking-pool/res/staking_pool.wasm"`) and should be recompiled with new staking pool contract binary.

## Whitelist implementation details

`whitelist` contract has same code as [standard](https://github.com/near/core-contracts/tree/master/whitelist) NEAR whitelist contract without any modifications. Purpose to add this contract is next: integration tests of `staking-pool-factory` uses whitelist contract folder to achive whitelist contract binary.

# Building

It's **highly** recommended to use the provided Docker image to set up a consistent development environment due to the usage of native dependencies.

If you're using VSCode, a `devcontainer` configuration is provided already. [You can read more about it here](https://code.visualstudio.com/docs/devcontainers/containers)

Each project contains a `build.sh` and `test.sh` which build and test each project, respectively. You should use this scripts instead of `cargo build` and `cargo test` directly since some post-build commands are necessary after building each project.
