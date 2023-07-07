use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{AccountId, Balance};

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct InforReward {
    pub user: AccountId,
    pub balance: Balance,
    pub token_id: AccountId,
}
