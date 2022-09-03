use near_sdk::json_types::U128;
use near_sdk::{ext_contract, AccountId, Gas};

pub const DEFAULT_GAS: Gas = Gas(10_000_000_000_000);

#[ext_contract(ext_self)]
pub trait Donation {
    fn donation_callback(&mut self, donator: AccountId, amount: U128);
}

#[ext_contract(ext_mail)]
pub trait Mail {
    pub fn send_mail(
        &mut self,
        receiver: AccountId,
        title: String,
        content: String,
        fee: Option<U128>,
    );
}
