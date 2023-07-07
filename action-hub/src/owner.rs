use crate::*;


impl Contract {
    pub(crate) fn assert_owner(&self) {
        assert_eq!(env::predecessor_account_id(), self.owner_id, "NOT OWNER");
    }

    pub(crate) fn is_owner(&self, account_id: AccountId) -> bool {
        account_id  == self.owner_id
    
    }
}
