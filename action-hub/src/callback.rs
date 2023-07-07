use near_sdk::{env, near_bindgen, AccountId, Balance, PromiseResult};

use crate::*;
#[near_bindgen]
impl Contract {
    pub fn claim_token_callback(
        &mut self,
        receiver_id: AccountId,
        token_id: &AccountId,
        amount: Balance,
    ) {
        assert_eq!(env::promise_results_count(), 1, "ERR_TOO_MANY_RESULTS");
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(_) => {
                //update reward
                match self.rewards.get(&receiver_id) {
                    Some(mut res) => res.insert(token_id, &0),
                    None => env::panic_str("Loi khong the update"),
                };
                //update owner
                let balance_owner = self.internal_get_owner_balance(token_id);
                match balance_owner.checked_sub(amount) {
                    Some(new_balance) => {
                        self.owner_balance.insert(token_id, &new_balance);
                    }
                    None => env::panic_str("Balance Overflow"),
                }
            }
            PromiseResult::Failed => {
                env::panic_str("user claim token failed");
            }
        }
    }

    pub fn storage_deposit_callback_add_token(&mut self, token_id: AccountId) {
        assert_eq!(env::promise_results_count(), 1, "ERR_TOO_MANY_RESULTS");

        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(_) => {
                //add whitelisted token of smart contract
                self.whitelisted_tokens.insert(&token_id);
            }
            PromiseResult::Failed => {
                env::panic_str("storage_deposit for owner failed");
            }
        }
    }

    pub fn storage_deposit_callback_claim(
        &mut self,
        receiver_id: AccountId, //receiver_id
        token_id: AccountId,
        amount: Balance,
    ) {
        assert_eq!(env::promise_results_count(), 1, "ERR_TOO_MANY_RESULTS");
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(_) => {
                // if success and then update reward and owner
                self.claim_token(receiver_id, amount, token_id);
            }
            PromiseResult::Failed => {
                env::panic_str("storage_deposit for lucky user failed");
            }
        }
    }

    pub fn deposit_callback(&mut self, user_id: AccountId, token_id: &AccountId, amount: Balance) {
        assert_eq!(env::promise_results_count(), 1, "ERR_TOO_MANY_RESULTS");
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(_) => {
                self.internal_deposit(&user_id, token_id, amount);
            }
            PromiseResult::Failed => {
                env::panic_str("Deposit token failed. Please try again");
            }
        }
    }

    pub fn distribute_callback(&mut self, event_id: EventId) {
        assert_eq!(env::promise_results_count(), 1, "ERR_TOO_MANY_RESULTS");

        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(_) => {
                let mut event_info = self.get_event_info(&event_id);
                event_info.is_distributed = true;
                self.events.insert(&event_id, &event_info);
            }
            PromiseResult::Failed => {
                env::panic_str("Distribute token failed. Please try again");
            }
        }
    }
}
