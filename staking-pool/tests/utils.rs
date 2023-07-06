#![allow(dead_code)]

use near_sdk::json_types::U128;
use near_sdk::serde::de::DeserializeOwned;
use near_sdk::serde_json::{self, json};
use near_sdk::{AccountId, Balance};
use near_sdk_sim::account::{AccessKey, Account};
use near_sdk_sim::errors::{RuntimeError, TxExecutionError};
use near_sdk_sim::hash::CryptoHash;
use near_sdk_sim::near_crypto::{InMemorySigner, KeyType, Signer};
use near_sdk_sim::runtime::{init_runtime, RuntimeStandalone};
use near_sdk_sim::transaction::{ExecutionOutcome, ExecutionStatus, Transaction};
use staking_pool::RewardFeeFraction;

pub const POOL_ACCOUNT_ID: &str = "pool";
pub const MAX_GAS: u64 = 300_000_000_000_000;

pub fn ntoy(near_amount: Balance) -> Balance {
    near_amount * 10u128.pow(24)
}

lazy_static::lazy_static! {
    static ref POOL_WASM_BYTES: &'static [u8] = include_bytes!("../res/staking_pool.wasm").as_ref();
}

type TxResult = Result<ExecutionOutcome, ExecutionOutcome>;

fn outcome_into_result(outcome: ExecutionOutcome) -> TxResult {
    match outcome.status {
        ExecutionStatus::SuccessValue(_) => Ok(outcome),
        ExecutionStatus::Failure(_) => Err(outcome),
        ExecutionStatus::SuccessReceiptId(_) => panic!("Unresolved ExecutionOutcome run runitme.resolve(tx) to resolve the filnal outcome of tx"),
        ExecutionStatus::Unknown => unreachable!()
    }
}
pub struct ExternalUser {
    account_id: AccountId,
    signer: InMemorySigner,
}

impl ExternalUser {
    pub fn new(account_id: AccountId, signer: InMemorySigner) -> Self {
        Self { account_id, signer }
    }

    #[allow(dead_code)]
    pub fn account_id(&self) -> &AccountId {
        &self.account_id
    }

    #[allow(dead_code)]
    pub fn signer(&self) -> &InMemorySigner {
        &self.signer
    }

    pub fn account(&self, runtime: &mut RuntimeStandalone) -> Account {
        runtime
            .view_account(&self.account_id.to_string())
            .expect("Account should be there")
    }

    pub fn create_external(
        &self,
        runtime: &mut RuntimeStandalone,
        new_account_id: AccountId,
        amount: Balance,
    ) -> Result<ExternalUser, ExecutionOutcome> {
        let new_signer = InMemorySigner::from_seed(
            &new_account_id.to_string(),
            KeyType::ED25519,
            &new_account_id.to_string(),
        );
        let tx = self
            .new_tx(runtime, new_account_id.clone())
            .create_account()
            .add_key(new_signer.public_key(), AccessKey::full_access())
            .transfer(amount)
            .sign(&self.signer);
        let res = runtime.resolve_tx(tx);

        // TODO: this temporary hack, must be rewritten
        if let Err(err) = res.clone() {
            if let RuntimeError::InvalidTxError(tx_err) = err {
                let mut out = ExecutionOutcome::default();
                out.status = ExecutionStatus::Failure(TxExecutionError::InvalidTxError(tx_err));
                return Err(out);
            } else {
                unreachable!();
            }
        } else {
            outcome_into_result(res.unwrap().1)?;
            runtime.process_all().unwrap();
            Ok(ExternalUser {
                account_id: new_account_id,
                signer: new_signer,
            })
        }
    }

    pub fn pool_init_new(
        &self,
        runtime: &mut RuntimeStandalone,
        amount: Balance,
        reward_fee_fraction: RewardFeeFraction,
    ) -> TxResult {
        let args = json!({
            "owner_id": self.account_id,
            "stake_public_key": "ed25519:3tysLvy7KGoE8pznUgXvSHa4vYyGvrDZFcT8jgb8PEQ6", // not relevant for now
            "reward_fee_fraction": reward_fee_fraction
        })
        .to_string()
        .as_bytes()
        .to_vec();

        let tx = self
            .new_tx(
                runtime,
                AccountId::new_unchecked(POOL_ACCOUNT_ID.to_string()),
            )
            .create_account()
            .transfer(amount)
            .deploy_contract(POOL_WASM_BYTES.to_vec())
            .function_call("new".into(), args, MAX_GAS, 0)
            .sign(&self.signer);
        let res = runtime.resolve_tx(tx).unwrap();
        runtime.process_all().unwrap();
        outcome_into_result(res.1)
    }

    pub fn add_to_whitelist(
        &self,
        runtime: &mut RuntimeStandalone,
        account_id: &AccountId,
    ) -> TxResult {
        let args = json!({ "account_id": account_id })
            .to_string()
            .as_bytes()
            .to_vec();

        let tx = self
            .new_tx(
                runtime,
                AccountId::new_unchecked(POOL_ACCOUNT_ID.to_string()),
            )
            .function_call("add_to_whitelist".into(), args, MAX_GAS, 0)
            .sign(&self.signer);
        let res = runtime.resolve_tx(tx).unwrap();
        runtime.process_all().unwrap();
        outcome_into_result(res.1)
    }

    pub fn pool_deposit(&self, runtime: &mut RuntimeStandalone, amount: Balance) -> TxResult {
        let tx = self
            .new_tx(
                runtime,
                AccountId::new_unchecked(POOL_ACCOUNT_ID.to_string()),
            )
            .function_call("deposit".into(), vec![], MAX_GAS, amount)
            .sign(&self.signer);
        let res = runtime.resolve_tx(tx).unwrap();
        runtime.process_all().unwrap();
        outcome_into_result(res.1)
    }

    pub fn pool_ping(&self, runtime: &mut RuntimeStandalone) -> TxResult {
        let tx = self
            .new_tx(
                runtime,
                AccountId::new_unchecked(POOL_ACCOUNT_ID.to_string()),
            )
            .function_call("ping".into(), vec![], MAX_GAS, 0)
            .sign(&self.signer);
        let res = runtime.resolve_tx(tx).unwrap();
        runtime.process_all().unwrap();
        outcome_into_result(res.1)
    }

    pub fn pool_stake(&self, runtime: &mut RuntimeStandalone, amount: u128) -> TxResult {
        let args = json!({ "amount": format!("{}", amount) })
            .to_string()
            .as_bytes()
            .to_vec();
        let tx = self
            .new_tx(
                runtime,
                AccountId::new_unchecked(POOL_ACCOUNT_ID.to_string()),
            )
            .function_call("stake".into(), args, MAX_GAS, 0)
            .sign(&self.signer);
        let res = runtime.resolve_tx(tx).unwrap();
        runtime.process_all().unwrap();
        outcome_into_result(res.1)
    }

    pub fn pool_unstake(&self, runtime: &mut RuntimeStandalone, amount: u128) -> TxResult {
        let args = json!({ "amount": format!("{}", amount) })
            .to_string()
            .as_bytes()
            .to_vec();
        let tx = self
            .new_tx(
                runtime,
                AccountId::new_unchecked(POOL_ACCOUNT_ID.to_string()),
            )
            .function_call("unstake".into(), args, MAX_GAS, 0)
            .sign(&self.signer);
        let res = runtime.resolve_tx(tx).unwrap();
        runtime.process_all().unwrap();
        let outcome_res = outcome_into_result(res.1);
        if outcome_res.is_ok() {
            wait_epoch(runtime);
            let total_stake: U128 = call_pool(runtime, "get_total_staked_balance", "");
            let mut pool_account = runtime.view_account(POOL_ACCOUNT_ID).unwrap();
            pool_account.amount += pool_account.locked - total_stake.0;
            pool_account.locked = total_stake.0;
            runtime.force_account_update(
                AccountId::new_unchecked(POOL_ACCOUNT_ID.to_string()),
                &pool_account,
            );
        }
        outcome_res
    }

    pub fn pool_withdraw(&self, runtime: &mut RuntimeStandalone, amount: u128) -> TxResult {
        let args = json!({ "amount": format!("{}", amount) })
            .to_string()
            .as_bytes()
            .to_vec();
        let tx = self
            .new_tx(
                runtime,
                AccountId::new_unchecked(POOL_ACCOUNT_ID.to_string()),
            )
            .function_call("withdraw".into(), args, MAX_GAS, 0)
            .sign(&self.signer);
        let res = runtime.resolve_tx(tx).unwrap();
        runtime.process_all().unwrap();
        outcome_into_result(res.1)
    }

    pub fn pool_pause(&self, runtime: &mut RuntimeStandalone) -> TxResult {
        let tx = self
            .new_tx(
                runtime,
                AccountId::new_unchecked(POOL_ACCOUNT_ID.to_string()),
            )
            .function_call("pause_staking".into(), vec![], MAX_GAS, 0)
            .sign(&self.signer);
        let res = runtime.resolve_tx(tx).unwrap();
        runtime.process_all().unwrap();
        outcome_into_result(res.1)
    }

    pub fn pool_resume(&self, runtime: &mut RuntimeStandalone) -> TxResult {
        let tx = self
            .new_tx(
                runtime,
                AccountId::new_unchecked(POOL_ACCOUNT_ID.to_string()),
            )
            .function_call("resume_staking".into(), vec![], MAX_GAS, 0)
            .sign(&self.signer);
        let res = runtime.resolve_tx(tx).unwrap();
        runtime.process_all().unwrap();
        outcome_into_result(res.1)
    }

    #[allow(dead_code)]
    pub fn pool_vote(&self, runtime: &mut RuntimeStandalone, amount: u128) -> TxResult {
        let args = json!({ "amount": format!("{}", amount) })
            .to_string()
            .as_bytes()
            .to_vec();
        let tx = self
            .new_tx(
                runtime,
                AccountId::new_unchecked(POOL_ACCOUNT_ID.to_string()),
            )
            .function_call("withdraw".into(), args, MAX_GAS, 0)
            .sign(&self.signer);
        let res = runtime.resolve_tx(tx).unwrap();
        runtime.process_all().unwrap();
        outcome_into_result(res.1)
    }

    #[allow(dead_code)]
    pub fn get_account_staked_balance(&self, runtime: &RuntimeStandalone) -> Balance {
        let balance = runtime
            .view_method_call(
                POOL_ACCOUNT_ID,
                "get_account_staked_balance",
                json!({"account_id": self.account_id})
                    .to_string()
                    .as_bytes(),
            )
            .unwrap();
        u128::from(serde_json::from_slice::<U128>(balance.as_slice()).unwrap())
    }

    pub fn get_account_unstaked_balance(&self, runtime: &RuntimeStandalone) -> Balance {
        let balance = runtime
            .view_method_call(
                POOL_ACCOUNT_ID,
                "get_account_unstaked_balance",
                json!({"account_id": self.account_id})
                    .to_string()
                    .as_bytes(),
            )
            .unwrap();
        u128::from(serde_json::from_slice::<U128>(balance.as_slice()).unwrap())
    }

    fn new_tx(&self, runtime: &RuntimeStandalone, receiver_id: AccountId) -> Transaction {
        let nonce = runtime
            .view_access_key(&self.account_id.to_string(), &self.signer.public_key())
            .unwrap()
            .nonce
            + 1;
        Transaction::new(
            self.account_id.to_string(),
            self.signer.public_key(),
            receiver_id.to_string(),
            nonce,
            CryptoHash::default(),
        )
    }
}

pub fn init_pool(initial_transfer: Balance) -> (RuntimeStandalone, ExternalUser) {
    let (mut runtime, signer, root_id) = init_runtime(None);
    let root = ExternalUser::new(root_id, signer);

    root.pool_init_new(
        &mut runtime,
        initial_transfer,
        RewardFeeFraction {
            numerator: 10,
            denominator: 100,
        },
    )
    .unwrap();
    return (runtime, root);
}

pub fn is_pool_paused(runtime: &mut RuntimeStandalone) -> bool {
    call_view(
        runtime,
        &AccountId::new_unchecked(POOL_ACCOUNT_ID.to_string()),
        "is_staking_paused",
        "{}",
    )
}

pub fn reward_pool(runtime: &mut RuntimeStandalone, amount: Balance) {
    let mut pool_account = runtime.view_account(POOL_ACCOUNT_ID).unwrap();
    pool_account.locked += amount;
    runtime.force_account_update(
        AccountId::new_unchecked(POOL_ACCOUNT_ID.to_string()),
        &pool_account,
    );
}

pub fn wait_epoch(runtime: &mut RuntimeStandalone) {
    let epoch_height = runtime.current_block().epoch_height;
    while epoch_height == runtime.current_block().epoch_height {
        runtime.produce_block().unwrap();
    }
}

pub fn call_pool<I: ToString, O: DeserializeOwned>(
    runtime: &mut RuntimeStandalone,
    method: &str,
    args: I,
) -> O {
    call_view(
        runtime,
        &AccountId::new_unchecked(POOL_ACCOUNT_ID.to_string()),
        method,
        args,
    )
}

pub fn pool_account(runtime: &mut RuntimeStandalone) -> Account {
    runtime.view_account(POOL_ACCOUNT_ID).unwrap()
}

fn call_view<I: ToString, O: DeserializeOwned>(
    runtime: &mut RuntimeStandalone,
    account_id: &AccountId,
    method: &str,
    args: I,
) -> O {
    let args = args.to_string();
    let result = runtime
        .view_method_call(&account_id.to_string(), method, args.as_bytes())
        .unwrap();
    let output: O = serde_json::from_reader(result.as_slice()).unwrap();
    output
}
