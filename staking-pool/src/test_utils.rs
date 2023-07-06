use std::collections::HashMap;

use near_sdk::{
    testing_env, AccountId, Balance, PromiseResult, RuntimeFeesConfig, VMConfig, VMContext,
};

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

pub fn testing_env_with_promise_results(context: VMContext, promise_result: PromiseResult) {
    testing_env!(
        context,
        VMConfig::test(),
        RuntimeFeesConfig::test(),
        HashMap::default(),
        vec![promise_result]
    )
}
