use infor_reward::InforReward;
use internal::hash_data;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::U128;
use near_sdk::{
    env, log, near_bindgen, require, AccountId, Balance, BorshStorageKey, CryptoHash, Gas,
    PanicOnDefault, Promise, PromiseOrValue,
};
mod callback;
mod event;
mod external;
mod infor_reward;
mod internal;
mod owner;
// mod test;
mod token_receiver;
mod utils;
mod common;

use event::*;
use external::*;
use utils::*;

pub const FT_TRANSFER_GAS: Gas = Gas(10_000_000_000_000);
#[derive(BorshSerialize, BorshStorageKey)]

pub enum Prefix {
    Owner,
    Clients,
    ClientsNest { account_id_hash: CryptoHash, time_stamp: u64 },
    ListToken,
    CategoryOwner,
    Reward,
    RewardNest { account_id_hash: CryptoHash, time_stamp: u64 },
    WhiteList,
    Event,
    Participants,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    //owner of this contract
    owner_id: AccountId,
    pub whitelisted_tokens: UnorderedSet<AccountId>,
    // Category token -> Balance of this smartcontract
    pub owner_balance: LookupMap<AccountId, Balance>,
    // AccountId of Depositer -> Category Token -> Balance
    pub clients: LookupMap<AccountId, LookupMap<AccountId, Balance>>,
    // list Lucky_user
    pub rewards: LookupMap<AccountId, LookupMap<AccountId, Balance>>,

    // List events
    pub events: UnorderedMap<EventId, EventInfo>,
    // Participants join events
    pub participants: LookupMap<AccountId, Participant>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        assert!(!env::state_exists(), "Just initial once time");
        //todo check only initialize one time
        let mut this = Self {
            owner_id: owner_id.clone(),
            whitelisted_tokens: UnorderedSet::new(Prefix::WhiteList.try_to_vec().unwrap()),
            owner_balance: LookupMap::new(Prefix::Owner.try_to_vec().unwrap()),
            clients: LookupMap::new(Prefix::Clients.try_to_vec().unwrap()),
            rewards: LookupMap::new(Prefix::Reward.try_to_vec().unwrap()),

            events: UnorderedMap::new(Prefix::Event.try_to_vec().unwrap()),
            participants: LookupMap::new(Prefix::Participants.try_to_vec().unwrap()),
        };
        // Insert native token in smart contract state
        let current_contract = env::current_account_id();
        this.whitelisted_tokens.insert(&current_contract);

        this
    }

    #[payable]
    pub fn add_whitelisted_token(&mut self, token_id: AccountId) {
        self.assert_owner();
        let attached_deposit = env::attached_deposit();
        match self.check_ft_exists(&token_id) {
            true => env::panic_str(
                format!("The fungible token {} has exsits in this smartcontract", token_id)
                    .as_str(),
            ),
            false => {
                self.internal_add_whitelisted_token(token_id, attached_deposit);
            }
        }
    }
    // call by client
    // deposit native token
    #[payable]
    pub fn deposit_native(&mut self, amount: U128, event_id: Option<EventId>, quest_type: String) {
        // kiem tra client co du so ft de deposit khong
        assert_at_least_one_yocto();
        let amount = amount.into();
        let sender_id = env::predecessor_account_id();


        // with native context: token is current smart contract
        let current_contract = env::current_account_id();

        if quest_type == "Event" {
            // Support for Event
            // User want to join Event -> deposit
            let event_id = event_id.unwrap_or_else(|| env::panic_str("Should be event ID"));
            self.internal_event(&sender_id, &event_id, &current_contract, amount);
        
        }
        else {
            // Support for Offchain Quest and Onchain Quest
            // Client should deposit token 
            let attached_deposit = env::attached_deposit();
            require!(attached_deposit == amount, "The attached_deposit must equal to the amount");
            self.internal_deposit(&sender_id, &current_contract, amount);
        }

    }

    // User buy ticket by NEAR native token
    #[payable]
    pub fn buy_ticket(&mut self, event_id: String, amount: U128) {
        assert_at_least_one_yocto();
        let participant_id = env::signer_account_id();
        let attached_deposit = env::attached_deposit();
        let amount = amount.into();
        require!(attached_deposit == amount, "The attached_deposit must equal to the amount");

        self.internal_buy_ticket(&participant_id, event_id, amount);
    }

    /// Create reward transaction
    /// Only owner
    /// Store reward state for users
    pub fn create_reward(&mut self, data: Vec<InforReward>) {
        // check just owner must be called this function
        self.assert_owner();
        require!(data.len() > 0, "The vector data must greater than zero");
        for item in data.iter() {
            match self.rewards.get(&item.user) {
                Some(mut res) => match res.get(&item.token_id) {
                    Some(balance) => {
                        if let Some(new_balance) = balance.checked_add(item.balance) {
                            res.insert(&item.token_id, &new_balance);
                        } else {
                            env::panic_str("Balance overflow");
                        }
                    }
                    None => {
                        res.insert(&item.token_id, &item.balance);
                    }
                },
                None => {
                    let mut token_account: LookupMap<AccountId, Balance> =
                        LookupMap::new(Prefix::RewardNest {
                            account_id_hash: hash_data(&item.user.to_string()),
                            time_stamp: env::block_timestamp(),
                        });
                    token_account.insert(&item.token_id, &item.balance);
                    self.rewards.insert(&item.user, &token_account);
                }
            }
        }
    }

    // call by user
    #[payable]
    pub fn claim(&mut self, token_id: AccountId) {
        assert_fee_storage_deposit();
        let init_storage = env::storage_usage();
        if !self.check_ft_exists(&token_id) {
            env::panic_str(
                format!("The fungible token {} is not in whitelisted token", token_id).as_str(),
            );
        }
        let attached_deposit = env::attached_deposit();
        let receiver_id = env::signer_account_id();
        match self.rewards.get(&receiver_id) {
            Some(res) => match res.get(&token_id) {
                Some(balance) => {
                    require!(balance > 0, "The token reward must greater than zero");

                    // storage_deposit for receiver
                    self.internal_claim(receiver_id, token_id, balance, attached_deposit)
                }
                None => {
                    env::panic_str(format!("You can not claim the {} token", token_id).as_str());
                }
            },
            None => env::panic_str("You were not a lucky user"),
        }
        refund_deposit(init_storage);
    }


    // Call by owner
    // Execute after event ended
    // TODO : multiple events
    pub fn paid_tickets_distribute(&mut self, event_id: EventId, num: u128, denom: u128) {
        self.assert_owner();
        
        if !self.check_exist_event(&event_id) {
            env::panic_str("EventId not exist");
        }
        let event_info = self.get_event_info(&event_id);


        self.internal_distribute(event_info, num, denom);


    }
    pub fn get_owner_balance(&self, token_id: AccountId) -> Balance {
        let balance = self.internal_get_owner_balance(&token_id);
        balance
    }
    pub fn get_client_balance(&self, account_id: AccountId, token_id: AccountId) -> Balance {
        let balance = self.internal_get_client_balance(account_id, &token_id);
        balance
    }

    pub fn get_user_reward(&self, account_id: AccountId, token_id: AccountId) -> Balance {
        let balance = self.internal_get_user_reward(&account_id, &token_id);
        balance
    }

    pub fn get_total_paid_tickets_event(&self, event_id: &EventId) -> Balance {
        let balance = self.get_total_paid_tickets(event_id);
        balance
    }

    pub fn get_event_info(&self, event_id: &EventId) -> EventInfo {
        let event_info = self.internal_get_event_info(event_id);

        event_info

    }
}
