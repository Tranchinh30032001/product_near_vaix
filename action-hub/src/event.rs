use crate::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::{AccountId, Balance};
use std::collections::{HashMap, HashSet};

pub type EventId = String;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]

pub struct EventInfo {
    pub event_id: EventId,
    pub client_id: AccountId,
    pub token_id: AccountId,
    pub price: Balance,
    pub total_paid_tickets: Balance,
    pub is_distributed: bool,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Participant {
    pub events: HashSet<EventId>,
    //amount that participant buy ticket for multi tokens
    pub paid: HashMap<EventId, Balance>,
}

impl Contract {
    pub(crate) fn check_exist_event(&self, event_id: &EventId) -> bool {
        match self.events.get(event_id) {
            Some(_) => true,
            None => false,
        }
    }

    pub(crate) fn internal_event(
        &mut self,
        client_id: &AccountId,
        event_id: &EventId,
        token_id: &AccountId,
        amount: Balance,
    ) {
        if self.check_ft_exists(&token_id) {
            if !self.check_exist_event(&event_id) {
                let event_info = EventInfo {
                    event_id: event_id.clone(),
                    client_id: client_id.clone(),
                    token_id: token_id.clone(),
                    price: amount,
                    total_paid_tickets: 0,
                    is_distributed: false,
                };

                self.events.insert(&event_id, &event_info);
            } else {
                env::panic_str("Participant already buy ticket");
            }
        } else {
            env::panic_str(
                format!("This fungible token id {} is not among whitelisted token", token_id)
                    .as_str(),
            );
        }
    }

    pub(crate) fn internal_buy_ticket(
        &mut self,
        buyer: &AccountId,
        event_id: EventId,
        balance: Balance,
    ) {
        if self.check_exist_event(&event_id) {
            // We can use this unwrap due to re-check above **check_exist_event**
            let mut event_info = self.internal_get_event_info(&event_id);
            event_info.total_paid_tickets += balance;
            // Update total paid tickets
            self.events.insert(&event_id, &event_info);

            match self.participants.get(&buyer) {
                Some(participant) => {
                    // Participant join a new event
                    let mut current_event = participant.events;
                    let mut current_paid = participant.paid;
                    match current_event.insert(event_id.clone()) {
                        true => {
                            current_paid.insert(event_id, balance);
                        }
                        // todo: if user want to buy many tickets
                        // Current version: only 1 ticket
                        false => env::panic_str("You can not buy ticket any more"),
                    }

                    let current_participant =
                        Participant { events: current_event, paid: current_paid };
                    self.participants.insert(&buyer, &current_participant);
                }
                None => {
                    // First time participant buy ticket
                    let mut events: HashSet<String> = HashSet::new();
                    events.insert(event_id.clone());

                    let mut paid: HashMap<EventId, Balance> = HashMap::new();
                    paid.insert(event_id.clone(), balance);

                    let participant = Participant { events, paid };
                    self.participants.insert(&buyer, &participant);
                }
            }
        } else {
            env::panic_str("EventId not exist");
        }
    }

    // How to distribute with system and client
    pub(crate) fn internal_distribute(&mut self, event_info: EventInfo, num: u128, denom: u128) {
        let total = event_info.total_paid_tickets;

        let to_admin = u128_ratio(total, num, denom);
        let to_client = total - to_admin;
        let client_id = event_info.client_id;
        let token_id = event_info.token_id;
        let event_id = event_info.event_id;
        // Is native NEAR token
        if self.is_owner(token_id.clone()) {
            Promise::new(client_id).transfer(to_client);
        } else {
            // fungible token

            ext_ft_fungible_token::ext(token_id)
                .with_attached_deposit(1)
                .with_static_gas(FT_TRANSFER_GAS)
                .ft_transfer(client_id.clone(), to_client.into(), None)
                .then(
                    ext_self::ext(env::current_account_id())
                        .with_static_gas(FT_TRANSFER_GAS)
                        // if success update distribution status
                        .distribute_callback(event_id),
                );
        }
    }

    pub(crate) fn get_total_paid_tickets(&self, event_id: &EventId) -> Balance {
        let balance = match self.events.get(event_id) {
            Some(event_info) => event_info.total_paid_tickets,
            None => env::panic_str("EventId not exist"),
        };
        balance
    }

    pub(crate) fn internal_get_event_info(&self, event_id: &EventId) -> EventInfo {
        match self.events.get(event_id) {
            Some(event) => event,
            None => env::panic_str("EventId not exist"),
        }
    }
}
