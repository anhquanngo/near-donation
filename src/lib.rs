use near_sdk::borsh::{self, BorshDeserialize, BorshDeserialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::U128;
use near_sdk::{
    env, near_bindgen, AccountId, AccountId, Balance, Balance, BorshStorageKey, PanicOnDefault,
    Promise, PromiseResult,
};
use utils::*;
mod utils;

#[derive(BorshDeserialize, BorshStorageKey)]
pub enum StorageKeys {
    Donation,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshDeserialize, PanicOnDefault)]

pub struct Contract {
    message_contract_account: AccountId,
    donation: LookupMap<AccountId, Balance>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(message_contract_account: AccountId) -> Self {
        Self {
            message_contract_account,
            donation: LookupMap::new(StorageKeys::Donation),
        }
    }

    #[payable]
    pub fn donate(&mut self, from: AccountId, to: AccountId, title: String, content: String) {
        let fee = env::attached_deposit();
        ext_mail::ext(self.message_contract_account.clone())
            .with_attached_deposit(1)
            .with_static_gas(DEFAULT_GAS)
            .send_mail(to, title, content, Some(U128::from(fee)))
            .then(ext_self::ext(env::current_account_id()).donation_callback(from, U128((fee))))
    }

    #[private]
    pub fn donation_callback(&mut self, donator: AccountId, amount: U128) {
        assert_eq!(env::promise_results_count(), 1, "ERR_TOO_MANY_RESULTS");
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(val) => {
                let donation_amount = self.donation.get(&donator).unwrap_or(0);

                self.donation
                    .insert(&donator, &(donation_amount + amount.0))
            }
            PromiseResult::Failed => Promise::new(donator).transfer(0),
        }
    }
}
