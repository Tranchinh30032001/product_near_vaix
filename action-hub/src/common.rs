use crate::*;

#[derive(BorshSerialize, BorshDeserialize)]
pub enum QuestType {
    OffChainQuest,
    OnchainQuest,
    Event
}
impl From<&QuestType> for String {
    fn from(quest_type: &QuestType) -> Self {
        match *quest_type {
            QuestType::OffChainQuest => {String::from("Offchain")},
            QuestType::OnchainQuest => {String::from("Onchain")},
            QuestType::Event => {String::from("Event")},
        }
    }
}

