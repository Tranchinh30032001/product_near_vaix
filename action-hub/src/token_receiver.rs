use crate::*;
use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
struct QuestMessage {
    quest_type: String,
    message_type: MessageType,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
enum MessageType {
    Event(EventMessage),
    Offchain(OffchainMessage),
    Onchain(OnchainMessage),
}

// TODO
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
struct EventMessage {
    event_id: EventId,
    price: Balance
    
}

// TODO
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
struct OffchainMessage {
    offchain_id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
struct OnchainMessage {
    onchain_id: String,
}

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        log!("in {} tokens from @{} ft_on_transfer", amount.0, sender_id.as_ref());
        let token_id = env::predecessor_account_id();
        env::log_str(format!("token_id:{}", token_id).as_str());

        let json_result: QuestMessage =
            near_sdk::serde_json::from_str(&msg).expect("Not valid payment args");

        let quest_type = json_result.quest_type;
        // case client deposit FT
        if quest_type == "Offchain" || quest_type == "Onchain" {
            // TODO
            // Process with offchain id and onchain id 
            self.internal_deposit(&sender_id, &token_id, amount.0);
            PromiseOrValue::Value(U128(0))
        }
        // case user deposit FT with msg on the format: event event_id
        else if quest_type == "Event" {
            // get event_id from msg
            match json_result.message_type {
                MessageType::Event(event_message) => {
                    let event_id = event_message.event_id;
                    let price = event_message.price;
                    self.internal_event(&sender_id, &event_id, &token_id, price);
                    PromiseOrValue::Value(U128(0))
                }
                MessageType::Offchain(_) => {
                    env::panic_str("Invalid Event ID");
                }
                MessageType::Onchain(_) => {
                    env::panic_str("Invalid Event ID");
                }
            }
        }
        // todo
        else {
            log!("Fungible token is unvalid");
            PromiseOrValue::Value(amount)
        }
    }
}
