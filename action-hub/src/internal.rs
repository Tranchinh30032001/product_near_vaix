use crate::*;
use near_sdk::collections::LookupMap;
use near_sdk::{log, CryptoHash};

impl Contract {
    pub(crate) fn internal_unwrap_balance(
        &self,
        account_id: &AccountId,
        token_id: &AccountId,
    ) -> Option<Balance> {
        match self.clients.get(account_id) {
            Some(token_accounts) => match token_accounts.get(token_id) {
                Some(balance) => {
                    return Some(balance);
                }
                None => Some(0),
            },
            None => None,
        }
    }
    
    pub(crate) fn internal_deposit(
        &mut self,
        account_id: &AccountId,
        token_id: &AccountId,
        amount: Balance,
    ) {
        if self.check_ft_exists(token_id) {
            if let Some(balance) = self.internal_unwrap_balance(account_id, token_id) {
                if let Some(new_balance) = balance.checked_add(amount) {
                    let mut token_account = self
                        .clients
                        .get(account_id)
                        .unwrap_or_else(|| env::panic_str("Error deposit token"));
                    token_account.insert(token_id, &new_balance);
                } else {
                    env::panic_str("Balance Overflow");
                }
            } else {
                // the first time
                let mut token_account: LookupMap<AccountId, Balance> =
                    LookupMap::new(Prefix::ClientsNest {
                        account_id_hash: hash_data(&account_id.to_string()),
                        time_stamp: env::block_timestamp(),
                    });
                token_account.insert(token_id, &amount);
                self.clients.insert(account_id, &token_account);
            }
            let balance_owner = self.internal_get_owner_balance(token_id);
            if let Some(new_owner_balance) = balance_owner.checked_add(amount) {
                self.owner_balance.insert(token_id, &new_owner_balance);
            }
        } else {
            env::panic_str(
                format!("This fungible token id {} is not among whitelisted token", token_id)
                    .as_str(),
            );
        }
    }

    pub(crate) fn check_ft_exists(&self, token_id: &AccountId) -> bool {
        self.whitelisted_tokens.contains(token_id)
    }

    pub(crate) fn internal_add_whitelisted_token(
        &self,
        token_id: AccountId,
        attached_deposit: Balance,
    ) {
        ext_ft_storage::ext(token_id.clone())
            .with_attached_deposit(attached_deposit)
            .with_static_gas(FT_TRANSFER_GAS)
            .storage_deposit(Some(env::current_account_id()), None)
            .then(
                ext_self::ext(env::current_account_id())
                    .with_static_gas(FT_TRANSFER_GAS)
                    .storage_deposit_callback_add_token(token_id),
            );
    }

    pub(crate) fn internal_claim(
        &self,
        receiver_id: AccountId,
        token_id: AccountId,
        balance: Balance,
        attached_deposit: Balance,
    ) {
        ext_ft_storage::ext(token_id.clone())
            .with_attached_deposit(attached_deposit)
            .with_static_gas(FT_TRANSFER_GAS)
            .storage_deposit(Some(receiver_id.clone()), None)
            .then(
                ext_self::ext(env::current_account_id())
                    .with_static_gas(FT_TRANSFER_GAS)
                    // check storage_deposit success
                    .storage_deposit_callback_claim(
                        receiver_id, // receiver_id
                        &token_id,
                        balance,
                    ),
            );
    }

    pub(crate) fn claim_token(&self, receiver_id: AccountId, amount: Balance, token_id: AccountId) {
        // check transfer thanh cong roi moi update lai reward cung nhu balance owner.
        ext_ft_fungible_token::ext(token_id.clone())
            .with_attached_deposit(1)
            .with_static_gas(FT_TRANSFER_GAS)
            .ft_transfer(receiver_id.clone(), amount.into(), None)
            .then(
                ext_self::ext(env::current_account_id())
                    .with_static_gas(FT_TRANSFER_GAS)
                    // if success update reward and owner
                    .claim_token_callback(receiver_id, &token_id, amount),
            );
    }

    pub(crate) fn internal_get_client_balance(
        &self,
        account_id: AccountId,
        token_id: &AccountId,
    ) -> Balance {
        match self.check_ft_exists(token_id) {
            true => match self.internal_unwrap_balance(&account_id, token_id) {
                Some(balance) => {
                    if balance == 0 {
                        log!("The account {} havent's deposit {} token yet", account_id, token_id);
                        return 0;
                    }
                    balance
                }
                None => env::panic_str(
                    format!("client doesnt deposit {} fungible token ", token_id).as_str(),
                ),
            },
            false => env::panic_str(
                format!("The fungible token {} has  not exsits in this smartcontract", token_id)
                    .as_str(),
            ),
        }
    }


    pub(crate) fn internal_get_owner_balance(&self, token_id: &AccountId) -> Balance {
        match self.check_ft_exists(token_id) {
            true => match self.owner_balance.get(token_id) {
                Some(balance) => balance,
                None => 0,
            },
            false => env::panic_str(
                format!(
                    "The fungible token {} has not existed in this smartcontract",
                    token_id.clone()
                )
                .as_str(),
            ),
        }
    }
    pub(crate) fn internal_get_user_reward(
        &self,
        account_id: &AccountId,
        token_id: &AccountId,
    ) -> Balance {
        match self.rewards.get(account_id) {
            Some(reward) => {
                match reward.get(token_id) {
                    Some(balance) => return balance,
                    None => return 0,
                };
            }
            None => env::panic_str("you have not received a reward from the client"),
        };
    }

}

pub(crate) fn hash_data(data: &String) -> CryptoHash {
    //get the default hash
    let mut hash = CryptoHash::default();
    //we hash the account ID and return it
    hash.copy_from_slice(&env::sha256(data.as_bytes()));
    hash
}
