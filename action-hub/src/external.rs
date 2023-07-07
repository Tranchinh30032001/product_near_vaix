use crate::*;

use near_sdk::json_types::U128;
use near_sdk::{ext_contract, AccountId};

#[ext_contract(ext_self)]
pub trait CallbackSelf {
    fn storage_deposit_callback_add_token(&mut self, token_id: AccountId);
    fn storage_deposit_callback_claim(
        &mut self,
        receiver_id: AccountId,
        token_id: &AccountId,
        amount: Balance,
    );
    fn claim_token_callback(
        &mut self,
        receiver_id: AccountId,
        token_id: &AccountId,
        amount: Balance,
    );
    fn deposit_callback(&mut self, user_id: AccountId, token_id: &AccountId, amount: Balance);

    fn distribute_callback(&mut self, event_id: EventId); 
}
#[ext_contract(ext_ft_fungible_token)]
pub trait FungibleTokenCore {
    fn ft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    );
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);
}

#[ext_contract(ext_ft_storage)]
pub trait StorageManagement {
    fn storage_deposit(&mut self, account_id: Option<AccountId>, registration_only: Option<bool>);
}
