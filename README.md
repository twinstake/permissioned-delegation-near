# Near whitelisted staking pool

This repo containce two contracts:

`staking-pool-factory` - deploys and automatically whitelists new staking pool contracts. It allows any user to create a new whitelisted staking pool;

`staking-pool` - deploys and manages a staking pool with possibilty of "whitelisting" deposit and staking operation. It allows whitelisted users to stake NEAR tokens, unstake and withdraw rewards.


## Staking pool implementation details

`staking-pool` based on the [standart](https://github.com/near/core-contracts/tree/master/staking-pool) NEAR staking pool contract with some modifications:

- added `whitelist_account_ids` LookupSet to store whitelisted accounts;
- added additional checks for `deposit`, `stake`, `deposit_and_stake`, `deposit_all`, `stake_all` methods to allow only whitelisted accounts to perform these operations;
- added `add_to_whitelist` and `remove_from_whitelist` methods to manage `whitelist_account_ids` LookupSet;
- added `is_whitelisted_account_id` method to check if account is whitelisted;
- fixed existing and added some new unit-tests to cover new functionality.


## Staking pool factory implementation details

`staking-pool-factory` has same code as [standart](https://github.com/near/core-contracts/tree/master/staking-pool-factory) NEAR staking pool factory contract without any modifications. Purpose to add this contract is next: contract use relative path to staking pool contract binary (`"../../staking-pool/res/staking_pool.wasm"`) and should be recompiled with new staking pool contract binary.