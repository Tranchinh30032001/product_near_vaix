#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use crate::CategoryToken;
    use near_contract_standards::fungible_token::core::FungibleTokenCore;
    use near_contract_standards::storage_management::StorageManagement;
    use near_sdk::json_types::U128;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::MockedBlockchain;
    use near_sdk::{env, testing_env, AccountId, Balance};

    use crate::Contract;

    const TOTAL_SUPPLY: Balance = 1_000_000_000_000_000;

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    fn test_new() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Contract::new_default_meta(accounts(1).into(), TOTAL_SUPPLY.into());
        testing_env!(context.is_view(true).build());
        assert_eq!(contract.ft_total_supply().0, TOTAL_SUPPLY);
        assert_eq!(contract.ft_balance_of(accounts(1)).0, TOTAL_SUPPLY);
    }

    #[test]
    #[should_panic(expected = "The contract is not initialized")]
    fn test_default() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let _contract = Contract::default();
    }

    #[test]
    fn test_transfer() {
        let mut context = get_context(accounts(2));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(2).into(), TOTAL_SUPPLY.into());
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(contract.storage_balance_bounds().min.into())
            .predecessor_account_id(accounts(1))
            .build());
        // Paying for account registration, aka storage deposit
        contract.storage_deposit(None, None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(2))
            .build());
        let transfer_amount = TOTAL_SUPPLY / 3;
        contract.ft_transfer(accounts(1), transfer_amount.into(), None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .account_balance(env::account_balance())
            .is_view(true)
            .attached_deposit(0)
            .build());
        assert_eq!(contract.ft_balance_of(accounts(2)).0, (TOTAL_SUPPLY - transfer_amount));
        assert_eq!(contract.ft_balance_of(accounts(1)).0, transfer_amount);
    }

    #[test]
    #[should_panic(expected = "Required attached deposit of at least 1 yoctoNEAR")]
    fn test_deposit_native_token_yocto() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(1).into(), TOTAL_SUPPLY.into());
        contract.deposit(U128(10), CategoryToken::NativeToken);
    }

    #[test]
    #[should_panic(expected = "The attached_deposit must be equa the amount")]
    fn test_deposit_native_token_equa() {
        let mut context = get_context(accounts(1));
        context.attached_deposit(2 * 10u128.pow(24));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(1).into(), TOTAL_SUPPLY.into());
        contract.deposit(U128(10), CategoryToken::NativeToken);
    }
    #[test]
    fn test_deposit_native_token_success() {
        let mut context = get_context(accounts(1));
        context.attached_deposit(10 * 10u128.pow(24));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(1).into(), TOTAL_SUPPLY.into());
        contract.deposit(U128(10), CategoryToken::NativeToken);
        assert_eq!(
            contract
                .contributor
                .get(&env::predecessor_account_id())
                .unwrap()
                .get(&CategoryToken::NativeToken)
                .unwrap(),
            10 * 10u128.pow(24)
        );
    }

    #[test]
    fn test_deposit_fungible_token() {
        let mut context = get_context(accounts(1));
        context.attached_deposit(10 * 10u128.pow(24));
        context.storage_usage(100);
        testing_env!(context.build());
        assert_eq!(env::account_balance(), 110 * 10u128.pow(24));
        let mut contract = Contract::new_default_meta(accounts(1).into(), TOTAL_SUPPLY.into());
        contract.deposit(U128(100), CategoryToken::FungibleToken);
        assert_ne!(env::account_balance(), 110 * 10u128.pow(24));
        assert_eq!(
            contract
                .contributor
                .get(&accounts(1))
                .unwrap()
                .get(&CategoryToken::FungibleToken)
                .unwrap(),
            100 * 10u128.pow(24)
        );
    }
}
