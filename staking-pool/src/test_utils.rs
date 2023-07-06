use std::collections::HashMap;
use std::convert::TryFrom;

use near_sdk::{AccountId, Gas, MockedBlockchain, PromiseResult, PublicKey, VMContext, testing_env, VMConfig, RuntimeFeesConfig};
use near_sdk::{Balance, BlockHeight, EpochHeight};

pub fn staking() -> AccountId {
    AccountId::new_unchecked("staking".to_string())
}

pub fn alice() -> AccountId {
    AccountId::new_unchecked("alice".to_string())
}
pub fn bob() -> AccountId {
    AccountId::new_unchecked("bob".to_string())
}
pub fn owner() -> AccountId {
    AccountId::new_unchecked("owner".to_string())
}

pub fn ntoy(near_amount: Balance) -> Balance {
    near_amount * 10u128.pow(24)
}

/// Rounds to nearest
pub fn yton(yocto_amount: Balance) -> Balance {
    (yocto_amount + (5 * 10u128.pow(23))) / 10u128.pow(24)
}

#[macro_export]
macro_rules! assert_eq_in_near {
    ($a:expr, $b:expr) => {
        assert_eq!(yton($a), yton($b))
    };
    ($a:expr, $b:expr, $c:expr) => {
        assert_eq!(yton($a), yton($b), $c)
    };
}

pub struct VMContextBuilder {
    context: VMContext,
}

impl VMContextBuilder {
    pub fn new() -> Self {
        Self {
            context: near_sdk::test_utils::VMContextBuilder::new().build()
            // context: VMContext {
            //     current_account_id: AccountId::new_unchecked("".to_string()),
            //     signer_account_id: AccountId::new_unchecked("".to_string()),
            //     signer_account_pk: PublicKey::try_from(vec![0, 1, 2]).unwrap(),
            //     predecessor_account_id: AccountId::new_unchecked("".to_string()),
            //     input: vec![],
            //     epoch_height: 0,
            //     block_index: 0,
            //     block_timestamp: 0,
            //     account_balance: 0,
            //     account_locked_balance: 0,
            //     storage_usage: 10u64.pow(6),
            //     attached_deposit: 0,
            //     prepaid_gas: Gas(10u64.pow(18)),
            //     random_seed: {
            //         let mut arr = [0; 32];
            //         arr[1] = 1;
            //         arr[2] = 2;
            //         arr
            //     },
            //     view_config: None,
            //     output_data_receivers: vec![],
            // },
        }
    }

    pub fn current_account_id(mut self, account_id: AccountId) -> Self {
        self.context.current_account_id = account_id;
        self
    }

    #[allow(dead_code)]
    pub fn signer_account_id(mut self, account_id: AccountId) -> Self {
        self.context.signer_account_id = account_id;
        self
    }

    pub fn predecessor_account_id(mut self, account_id: AccountId) -> Self {
        self.context.predecessor_account_id = account_id;
        self
    }

    #[allow(dead_code)]
    pub fn block_index(mut self, block_index: BlockHeight) -> Self {
        self.context.block_index = block_index;
        self
    }

    pub fn epoch_height(mut self, epoch_height: EpochHeight) -> Self {
        self.context.epoch_height = epoch_height;
        self
    }

    pub fn attached_deposit(mut self, amount: Balance) -> Self {
        self.context.attached_deposit = amount;
        self
    }

    pub fn account_balance(mut self, amount: Balance) -> Self {
        self.context.account_balance = amount;
        self
    }

    pub fn account_locked_balance(mut self, amount: Balance) -> Self {
        self.context.account_locked_balance = amount;
        self
    }

    pub fn finish(self) -> VMContext {
        self.context
    }
}

pub fn testing_env_with_promise_results(context: VMContext, promise_result: PromiseResult) {
    testing_env!(context, VMConfig::test(), RuntimeFeesConfig::test(), HashMap::default(), vec![promise_result])
    // let storage = near_sdk::mock::with_mocked_blockchain(|mb| mb.take_storage());

    // near_sdk::env::set_blockchain_interface(MockedBlockchain::new(
    //     context,
    //     near_sdk::VMConfig::test(),
    //     near_sdk::RuntimeFeesConfig::test(),
    //     vec![promise_result],
    //     storage,
    //     Default::default(),
    //     None,
    // ));
}
