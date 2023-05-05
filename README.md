# Near whitelisted staking pool

This repo containce three contracts:

`staking-pool-factory` - deploys and automatically whitelists new staking pool contracts. It allows any user to create a new whitelisted staking pool;

`staking-pool` - deploys and manages a staking pool with possibilty of "whitelisting" deposit and staking operation. It allows whitelisted users to stake NEAR tokens, unstake and withdraw rewards.

`whitelist` - the purpose of this contract is to maintain the whitelist of the staking pool contracts account IDs that are approved
by NEAR Foundation..

## Staking pool implementation details

`staking-pool` based on the [standart](https://github.com/near/core-contracts/tree/master/staking-pool) NEAR staking pool contract with some modifications:

- added `whitelist_account_ids` LookupSet to store whitelisted accounts;
- added additional checks for `deposit`, `stake`, `deposit_and_stake`, `deposit_all`, `stake_all` methods to allow only whitelisted accounts to perform these operations;
- added `add_to_whitelist` and `remove_from_whitelist` methods to manage `whitelist_account_ids` LookupSet;
- added `is_whitelisted_account_id` method to check if account is whitelisted;
- fixed existing and added some new unit-tests to cover new functionality.


## Staking pool factory implementation details

`staking-pool-factory` has same code as [standart](https://github.com/near/core-contracts/tree/master/staking-pool-factory) NEAR staking pool factory contract without any modifications. Purpose to add this contract is next: contract use relative path to staking pool contract binary (`"../../staking-pool/res/staking_pool.wasm"`) and should be recompiled with new staking pool contract binary.

## Whitelist implementation details

`whitelist` contract has same code as [standart](https://github.com/near/core-contracts/tree/master/whitelist) NEAR whitelist contract without any modifications. Purpose to add this contract is next: integration tests of `staking-pool-factory` uses whitelist contract folder to achive whitelist contract binary.

# Possible issues

### Build error

if you get error like this:

```
error[E0407]: method `to_i128` is not a member of trait `ToPrimitive`
```

just remove folder `/home/<your_account>/.cargo/registry` and try

### Test error

Staking-pool contract integration test doensn't compile at all (even for standart contract)

However, staking-pool-factory integration test compiles and runs successfully but you can get error like this:

```
error[E0554]: `#![feature]` may not be used on the stable release channel
  --> /home/<your_account>/.cargo/registry/src/github.com-1ecc6299db9ec823/wasmer-singlepass-backend-0.17.1/src/lib.rs:10:1
   |
10 | #![feature(proc_macro_hygiene)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```
just remove this line in the corresponding file and try again