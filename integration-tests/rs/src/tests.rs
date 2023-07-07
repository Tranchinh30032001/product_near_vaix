use near_sdk::borsh::BorshSerialize;
use near_sdk::json_types::U128;
use near_sdk::BorshStorageKey;
use near_units::{parse_gas, parse_near};
use serde::{Deserialize, Serialize};
use serde_json::json;
use workspaces::{Account, AccountId, Contract};
// use tracing_subscriber::filter::LevelFilter;
// use tracing_subscriber::EnvFilter;
// use std::env;
const ACTION_HUB_WASM_FILEPATH: &str = "../../res/action_hub.wasm";
const FT_WASM_FILEPATH: &str = "../../res/fungible_token.wasm";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let filter = if env::var(EnvFilter::DEFAULT_ENV).is_ok() {
    //     EnvFilter::from_default_env()
    // } else {
    //     EnvFilter::default().add_directive(LevelFilter::INFO.into())
    // };
    // tracing_subscriber::fmt().with_env_filter(filter).init();

    // initiate environemnt
    let worker = workspaces::sandbox().await?;

    // deploy contracts
    let action_wasm = std::fs::read(ACTION_HUB_WASM_FILEPATH)?;
    let action_contract = worker.dev_deploy(&action_wasm).await?;
    let ft_wasm = std::fs::read(FT_WASM_FILEPATH)?;
    let ft_contract_1 = worker.dev_deploy(&ft_wasm).await?;

    let ft_contract_2 = worker.dev_deploy(&ft_wasm).await?;

    // create accounts
    let owner = worker.root_account().unwrap();

    let alice = owner
        .create_subaccount("alice")
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await?
        .into_result()?;

    let bob = owner
        .create_subaccount("bob")
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await?
        .into_result()?;
    let mike = owner
        .create_subaccount("mike")
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await?
        .into_result()?;
    let charlie = owner
        .create_subaccount("charlie")
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await?
        .into_result()?;
    let dave = owner
        .create_subaccount("dave")
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await?
        .into_result()?;

    let eve = owner
        .create_subaccount("eve")
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await?
        .into_result()?;

    let client_event: Account = owner
        .create_subaccount("client_event")
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await?
        .into_result()?;

    let user_event: Account = owner
        .create_subaccount("user_event")
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await?
        .into_result()?;

    let user1 = owner
        .create_subaccount("user1")
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await?
        .into_result()?;

    let user2 = owner
        .create_subaccount("user2")
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await?
        .into_result()?;

    let user3 = owner
        .create_subaccount("user3")
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await?
        .into_result()?;

    let user4 = owner
        .create_subaccount("user4")
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await?
        .into_result()?;

    let not_owner = owner
        .create_subaccount("not_owner")
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await?
        .into_result()?;

    ft_contract_1
        .call("new_default_meta")
        .args_json(json!({
            "owner_id": owner.id(),
            "total_supply": parse_near!("1,000,000,000 N").to_string(),
        }))
        .transact()
        .await?
        .into_result()?;

    ft_contract_2
        .call("new_default_meta")
        .args_json(json!({
            "owner_id": owner.id(),
            "total_supply": parse_near!("1,000,000,000 N").to_string(),
        }))
        .transact()
        .await?
        .into_result()?;

    action_contract
        .call("new")
        .args_json(json!({
            "owner_id": owner.id()
        }))
        .transact()
        .await?
        .into_result()?;

    /*
    test_user_deposit_native(&alice, &bob, &action_contract).await?;

    test_should_fail_deposit_native_wrong_amount(&alice, &action_contract).await?;

    test_should_fail_deposit_native_greater_than_current_balance(&alice, &action_contract).await?;

    test_should_fail_deposit_ft_not_in_whitelisted_token(
        &owner,
        &mike,
        &action_contract,
        &ft_contract_1,
    )
    .await?;

    test_should_fail_deposit_ft_greater_than_current_balance(
        &mike,
        &ft_contract_1,
        &action_contract,
    )
    .await?;

    test_should_revert_ft_when_deposit_ft_wrong_message(
        &owner,
        &mike,
        &ft_contract_1,
        &action_contract,
    )
    .await?;
    test_should_fail_user_claim_not_whitelisted_token(&user3, &ft_contract_1, &action_contract)
        .await?;

    test_user_deposit_fungible_token_1(&owner, &charlie, &dave, &action_contract, &ft_contract_1)
        .await?;

    test_user_deposit_fungible_token_2(
        &owner,
        &charlie,
        &dave,
        &eve,
        &action_contract,
        &ft_contract_2,
    )
    .await?;

    test_should_fail_not_owner_create_reward(&not_owner, &action_contract, &ft_contract_1).await?;
    test_reward_work(&owner, &action_contract, &ft_contract_1, &ft_contract_2).await?;

    test_work_user_claim(&user1, &user2, &ft_contract_1, &ft_contract_2, &action_contract).await?;

    test_should_fail_user_reclaim(&user1, &action_contract, &ft_contract_1).await?;
    test_should_fail_user_is_not_lucky(&user3, &ft_contract_1, &action_contract).await?;
    test_should_fail_user_claim_wrong_fungible_token(&user4, &ft_contract_2, &action_contract)
        .await?;
    */
    test_user_buy_ticket_native_token(&client_event, &user_event, &action_contract).await?;
    test_user_buy_ticket_fungible_token(&owner,&client_event,&user_event, &ft_contract_1, &action_contract).await?;
    Ok(())
}

async fn test_user_deposit_native(
    alice: &Account,
    bob: &Account,
    action_contract: &Contract,
) -> anyhow::Result<()> {
    let deposit_amount = U128::from(parse_near!("3 N"));

    alice
        .call(action_contract.id(), "deposit_native")
        .args_json(json!({ "amount": deposit_amount,"quest_type":"Offchain" }))
        .deposit(parse_near!("3 N"))
        .transact()
        .await?
        .into_result()?;

    let result_action_balance: u128 = action_contract
        .as_account()
        .call(action_contract.id(), "get_owner_balance")
        .args_json(json!({ "token_id": action_contract.id() }))
        .transact()
        .await?
        .json()?;

    // check  action contract balance
    assert_eq!(result_action_balance, parse_near!("3 N"));

    // check user balance

    let result_alice_balance: u128 = alice
        .call(action_contract.id(), "get_client_balance")
        .args_json(json!({"account_id":alice.id(), "token_id":action_contract.id()}))
        .transact()
        .await?
        .json()?;

    assert_eq!(result_alice_balance, parse_near!("3 N"));

    let second_deposit_amount = U128::from(parse_near!("2 N"));

    // Second deposit native
    alice
        .call(action_contract.id(), "deposit_native")
        .args_json(json!({ "amount": second_deposit_amount, "quest_type":"Offchain" }))
        .deposit(parse_near!("2 N"))
        .transact()
        .await?
        .into_result()?;

    let result_second_alice_balance: u128 = alice
        .call(action_contract.id(), "get_client_balance")
        .args_json(json!({"account_id":alice.id(), "token_id":action_contract.id()}))
        .transact()
        .await?
        .json()?;

    assert_eq!(result_second_alice_balance, parse_near!("5 N"));

    let result_second_action_balance: u128 = action_contract
        .as_account()
        .call(action_contract.id(), "get_owner_balance")
        .args_json(json!({ "token_id":action_contract.id() }))
        .transact()
        .await?
        .json()?;

    assert_eq!(result_second_action_balance, parse_near!("5 N"));

    let third_deposit_amount = U128::from(parse_near!("2 N"));

    bob.call(action_contract.id(), "deposit_native")
        .args_json(json!({ "amount": third_deposit_amount, "quest_type": "Offchain" }))
        .deposit(parse_near!("2 N"))
        .transact()
        .await?
        .into_result()?;

    let result_bob_balance: u128 = bob
        .call(action_contract.id(), "get_client_balance")
        .args_json(json!({"account_id":bob.id(), "token_id":action_contract.id()}))
        .transact()
        .await?
        .json()?;

    assert_eq!(result_bob_balance, parse_near!("2 N"));

    let result_third_action_balance: u128 = action_contract
        .as_account()
        .call(action_contract.id(), "get_owner_balance")
        .args_json(json!({ "token_id":action_contract.id() }))
        .transact()
        .await?
        .json()?;

    assert_eq!(result_third_action_balance, parse_near!("7 N"));
    println!("      Passed ✅ test_user_deposit_native");
    Ok(())
}

async fn test_should_fail_deposit_native_wrong_amount(
    alice: &Account,
    action_contract: &Contract,
) -> anyhow::Result<()> {
    let res = alice
        .call(action_contract.id(), "deposit_native")
        .args_json(json!({ "amount": U128::from(parse_near!("2 N")), "quest_type":"Offchain" }))
        .deposit(parse_near!("0.008 N"))
        .transact()
        .await?;
    assert_eq!(true, res.is_failure());
    //Smart contract panicked: The attached_deposit must equal to the amount"
    println!("      Passed ✅ test_should_fail_deposit_native_wrong_amount");
    Ok(())
}

async fn test_should_fail_deposit_native_greater_than_current_balance(
    alice: &Account,
    action_contract: &Contract,
) -> anyhow::Result<()> {
    // Current alice balance is 25 NEAR
    let wrong_amount = U128::from(parse_near!("28 N"));
    match alice
        .call(action_contract.id(), "deposit_native")
        .args_json(json!({ "amount": wrong_amount, "quest_type":"Offchain" }))
        .deposit(parse_near!("28 N"))
        .transact()
        .await
    {
        Ok(_res) => panic!("This function can not be execute "),
        Err(_) => {
            println!("      Passed ✅ test_should_fail_deposit_native_greater_than_current_balance")
        }
    }
    //NotEnoughBalance

    Ok(())
}

async fn test_should_fail_deposit_ft_not_in_whitelisted_token(
    owner: &Account,
    mike: &Account,
    action_contract: &Contract,
    ft_contract_1: &Contract,
) -> anyhow::Result<()> {
    //Register mike storage deposit ft_contract_1
    owner
        .call(ft_contract_1.id(), "storage_deposit")
        .args_json(serde_json::json!({
            "account_id": mike.id()
        }))
        .deposit(parse_near!("0.008 N"))
        .transact()
        .await?
        .into_result()?;

    owner
        .call(ft_contract_1.id(), "storage_deposit")
        .args_json(serde_json::json!({
            "account_id": action_contract.id()
        }))
        .deposit(parse_near!("0.008 N"))
        .transact()
        .await?
        .into_result()?;

    // Transfer fungible token ft_contract_1
    let transfer_amount_str = parse_near!("1,000 N").to_string();

    // Transfer charlie ft_contract_1
    owner
        .call(ft_contract_1.id(), "ft_transfer")
        .args_json(serde_json::json!({
            "receiver_id": mike.id(),
            "amount": transfer_amount_str
        }))
        .deposit(1)
        .transact()
        .await?
        .into_result()?;
    // Deposit ft_contract_1
    let deposit_amount = U128::from(parse_near!("10 N"));

    let message = String::from(
        r#"
        {
            "message_type": {
                "Offchain": {
                    "offchain_id": "1"
                }
            },
            "quest_type": "Offchain"
        }
        "#,
    );

    let res = mike
        .call(ft_contract_1.id(), "ft_transfer_call")
        .args_json(
            json!({"amount":deposit_amount, "receiver_id":action_contract.id(), "msg":message}),
        )
        .deposit(1)
        .gas(parse_gas!("200 Tgas") as u64)
        .transact()
        .await?;

    assert_eq!(true, res.is_success());
    println!("logs :{:?}", res.logs());

    // Mike ft balance is still 1000 FT
    // due to Smart contract panicked: This fungible token id dev-20230412125609-40982671207909 is not among whitelisted token"
    let mike_fungible_token_after_deposit: U128 = owner
        .call(ft_contract_1.id(), "ft_balance_of")
        .args_json(json!({"account_id":mike.id()}))
        .transact()
        .await?
        .json()?;

    assert_eq!(mike_fungible_token_after_deposit, U128::from(parse_near!("1,000 N")));

    println!("      Passed ✅ test_should_fail_deposit_ft_not_in_whitelisted_token");

    Ok(())
}

async fn test_should_fail_deposit_ft_greater_than_current_balance(
    mike: &Account,
    ft_contract_1: &Contract,
    action_contract: &Contract,
) -> anyhow::Result<()> {
    // Current mike ft balance = 1000  FT
    let wrong_amount = U128::from(parse_near!("1001 N"));
    let message = String::from(
        r#"
        {
            "message_type": {
                "Offchain": {
                    "offchain_id": "2"
                }
            },
            "quest_type": "Offchain"
        }
        "#,
    );
    let res = mike
        .call(ft_contract_1.id(), "ft_transfer_call")
        .args_json(
            json!({"amount":wrong_amount, "receiver_id":action_contract.id(), "msg":message}),
        )
        .deposit(1)
        .gas(parse_gas!("200 Tgas") as u64)
        .transact()
        .await?;

    assert_eq!(true, res.is_failure());
    println!("      Passed ✅ test_should_fail_deposit_ft_greater_than_current_balance");
    Ok(())
}

async fn test_should_revert_ft_when_deposit_ft_wrong_message(
    owner: &Account,
    mike: &Account,
    ft_contract_1: &Contract,
    action_contract: &Contract,
) -> anyhow::Result<()> {
    // Current mike ft balance = 1000  FT
    let deposit_amount = U128::from(parse_near!("10 N"));
    let res =  mike
        .call(ft_contract_1.id(), "ft_transfer_call")
        .args_json(
            json!({"amount":deposit_amount, "receiver_id":action_contract.id(), "msg":"wrong-message"}),
        )
        .deposit(1)
        .gas(parse_gas!("200 Tgas") as u64)
        .transact()
        .await?;

    assert_eq!(true, res.is_success());
    let mike_fungible_token_after_deposit: U128 = owner
        .call(ft_contract_1.id(), "ft_balance_of")
        .args_json(json!({"account_id":mike.id()}))
        .transact()
        .await?
        .json()?;

    assert_eq!(mike_fungible_token_after_deposit, U128::from(parse_near!("1,000 N")));

    println!("      Passed ✅ test_should_revert_ft_when_deposit_ft_wrong_message");
    Ok(())
}

async fn test_should_fail_user_claim_not_whitelisted_token(
    user3: &Account,
    ft_contract_1: &Contract,
    action_contract: &Contract,
) -> anyhow::Result<()> {
    let res = user3
        .call(action_contract.id(), "claim")
        .args_json(json!({ "token_id": ft_contract_1.id() }))
        .deposit(parse_near!("0.008 N"))
        .gas(parse_gas!("200 Tgas") as u64)
        .transact()
        .await?;

    //mart contract panicked: The fungible token dev-20230412133748-49113630377400 is not in whitelisted token"
    assert_eq!(true, res.is_failure());

    println!("      Passed ✅ test_should_fail_user_claim_not_whitelisted_token");
    Ok(())
}

async fn test_user_deposit_fungible_token_1(
    owner: &Account,
    charlie: &Account,
    dave: &Account,
    action_contract: &Contract,
    ft_contract_1: &Contract,
) -> anyhow::Result<()> {
    //Register charlie storage deposit ft_contract_1
    owner
        .call(ft_contract_1.id(), "storage_deposit")
        .args_json(serde_json::json!({
            "account_id": charlie.id()
        }))
        .deposit(parse_near!("0.008 N"))
        .transact()
        .await?
        .into_result()?;

    //Register dave storage deposit ft_contract_1
    owner
        .call(ft_contract_1.id(), "storage_deposit")
        .args_json(serde_json::json!({
            "account_id": dave.id()
        }))
        .deposit(parse_near!("0.008 N"))
        .transact()
        .await?
        .into_result()?;

    // Transfer fungible token ft_contract_1
    let transfer_amount_str = parse_near!("1,000 N").to_string();

    // Transfer charlie ft_contract_1
    owner
        .call(ft_contract_1.id(), "ft_transfer")
        .args_json(serde_json::json!({
            "receiver_id": charlie.id(),
            "amount": transfer_amount_str
        }))
        .deposit(1)
        .transact()
        .await?
        .into_result()?;

    // Transfer dave ft_contract_1
    owner
        .call(ft_contract_1.id(), "ft_transfer")
        .args_json(serde_json::json!({
            "receiver_id": dave.id(),
            "amount": transfer_amount_str
        }))
        .deposit(1)
        .transact()
        .await?
        .into_result()?;

    // current charlie balance in ft_contract_1
    let charlie_fungible_token_before_deposit: U128 = owner
        .call(ft_contract_1.id(), "ft_balance_of")
        .args_json(json!({"account_id":charlie.id()}))
        .transact()
        .await?
        .json()?;

    assert_eq!(charlie_fungible_token_before_deposit, U128::from(parse_near!("1,000 N")));

    // Add fungible token 1
    owner
        .call(action_contract.id(), "add_whitelisted_token")
        .args_json(json!({"token_id":ft_contract_1.id()}))
        .deposit(parse_near!("0.008 N"))
        .gas(parse_gas!("200 Tgas") as u64)
        .transact()
        .await?
        .into_result()?;

    // Deposit ft_contract_1
    let deposit_amount = U128::from(parse_near!("10 N"));

    let message = String::from(
        r#"
        {
            "message_type": {
                "Offchain": {
                    "offchain_id": "1"
                }
            },
            "quest_type": "Offchain"
        }
        "#,
    );

    charlie
        .call(ft_contract_1.id(), "ft_transfer_call")
        .args_json(
            json!({"amount":deposit_amount, "receiver_id":action_contract.id(), "msg":message}),
        )
        .deposit(1)
        .gas(parse_gas!("200 Tgas") as u64)
        .transact()
        .await?
        .into_result()?;

    // charlie balance in ft_contract_1 after deposit
    let charlie_fungible_token_after_deposit: U128 = owner
        .call(ft_contract_1.id(), "ft_balance_of")
        .args_json(json!({"account_id":charlie.id()}))
        .transact()
        .await?
        .json()?;

    assert_eq!(charlie_fungible_token_after_deposit, U128::from(parse_near!("990 N")));

    // action balance in ft_contract_1
    let action_fungible_token_balance: U128 = owner
        .call(ft_contract_1.id(), "ft_balance_of")
        .args_json(json!({"account_id":action_contract.id()}))
        .transact()
        .await?
        .json()?;

    assert_eq!(action_fungible_token_balance, U128::from(parse_near!("10 N")));

    // charlie balance in action contract after deposit
    let charlie_fungible_token_balance: u128 = charlie
        .call(action_contract.id(), "get_client_balance")
        .args_json(json!({"account_id":charlie.id(), "token_id": ft_contract_1.id()}))
        .transact()
        .await?
        .json()?;

    assert_eq!(charlie_fungible_token_balance, parse_near!("10 N"));

    // action balance in action contract after deposit
    let action_fungible_token_balance: u128 = charlie
        .call(action_contract.id(), "get_owner_balance")
        .args_json(json!({ "token_id": ft_contract_1.id() }))
        .transact()
        .await?
        .json()?;

    assert_eq!(action_fungible_token_balance, parse_near!("10 N"));

    // Second deposit ft_contract_1 in action contract
    let second_deposit_amount = U128::from(parse_near!("10 N"));

    charlie
        .call(ft_contract_1.id(), "ft_transfer_call")
        .args_json(json!({"amount":second_deposit_amount, "receiver_id":action_contract.id(), "msg":message}))
        .deposit(1)
        .gas(parse_gas!("200 Tgas") as u64)
        .transact()
        .await?
        .into_result()?;

    // charlie balance in ft_contract_1 after deposit
    let charlie_fungible_token_after_deposit_second: U128 = owner
        .call(ft_contract_1.id(), "ft_balance_of")
        .args_json(json!({"account_id":charlie.id()}))
        .transact()
        .await?
        .json()?;

    assert_eq!(charlie_fungible_token_after_deposit_second, U128::from(parse_near!("980 N")));

    // action balance in ft_contract_1
    let action_fungible_token_balance_second: U128 = owner
        .call(ft_contract_1.id(), "ft_balance_of")
        .args_json(json!({"account_id":action_contract.id()}))
        .transact()
        .await?
        .json()?;

    assert_eq!(action_fungible_token_balance_second, U128::from(parse_near!("20 N")));

    // charlie balance in action contract after deposit second
    let charlie_fungible_token_balance_second: u128 = charlie
        .call(action_contract.id(), "get_client_balance")
        .args_json(json!({"account_id":charlie.id(), "token_id":ft_contract_1.id()}))
        .transact()
        .await?
        .json()?;

    assert_eq!(charlie_fungible_token_balance_second, parse_near!("20 N"));

    // action balance in action contract after deposit
    let action_fungible_token_balance_second: u128 = charlie
        .call(action_contract.id(), "get_owner_balance")
        .args_json(json!({ "token_id": ft_contract_1.id() }))
        .transact()
        .await?
        .json()?;

    assert_eq!(action_fungible_token_balance_second, parse_near!("20 N"));

    // Dave deposit fungible token 1 for action contract
    let third_deposit_amount = U128::from(parse_near!("30 N"));

    dave.call(ft_contract_1.id(), "ft_transfer_call")
        .args_json(json!({"amount":third_deposit_amount, "receiver_id":action_contract.id(), "msg":message}))
        .deposit(1)
        .gas(parse_gas!("200 Tgas") as u64)
        .transact()
        .await?
        .into_result()?;

    // dave fungible token 1 balance in action contract after deposit
    let dave_fungible_token_balance: u128 = dave
        .call(action_contract.id(), "get_client_balance")
        .args_json(json!({"account_id":dave.id(), "token_id": ft_contract_1.id()}))
        .transact()
        .await?
        .json()?;

    assert_eq!(dave_fungible_token_balance, parse_near!("30 N"));

    // action balance in action contract after deposit
    let action_fungible_token_balance_third: u128 = dave
        .call(action_contract.id(), "get_owner_balance")
        .args_json(json!({ "token_id": ft_contract_1.id() }))
        .transact()
        .await?
        .json()?;

    assert_eq!(action_fungible_token_balance_third, parse_near!("50 N"));

    println!("      Passed ✅ test_user_deposit_fungible_token_1");

    Ok(())
}

async fn test_user_deposit_fungible_token_2(
    owner: &Account,
    charlie: &Account,
    dave: &Account,
    eve: &Account,
    action_contract: &Contract,
    ft_contract_2: &Contract,
) -> anyhow::Result<()> {
    owner
        .call(ft_contract_2.id(), "storage_deposit")
        .args_json(serde_json::json!({
            "account_id": dave.id()
        }))
        .deposit(parse_near!("0.008 N"))
        .transact()
        .await?
        .into_result()?;

    owner
        .call(ft_contract_2.id(), "storage_deposit")
        .args_json(serde_json::json!({
            "account_id": charlie.id()
        }))
        .deposit(parse_near!("0.008 N"))
        .transact()
        .await?
        .into_result()?;

    owner
        .call(ft_contract_2.id(), "storage_deposit")
        .args_json(serde_json::json!({
            "account_id": eve.id()
        }))
        .deposit(parse_near!("0.008 N"))
        .transact()
        .await?
        .into_result()?;

    // Transfer fungible token ft_contract_2
    let transfer_amount_str = parse_near!("1,000 N").to_string();

    // Transfer dave ft_contract_2
    owner
        .call(ft_contract_2.id(), "ft_transfer")
        .args_json(serde_json::json!({
            "receiver_id": dave.id(),
            "amount": transfer_amount_str
        }))
        .deposit(1)
        .transact()
        .await?
        .into_result()?;

    owner
        .call(ft_contract_2.id(), "ft_transfer")
        .args_json(serde_json::json!({
            "receiver_id": charlie.id(),
            "amount": transfer_amount_str
        }))
        .deposit(1)
        .transact()
        .await?
        .into_result()?;

    owner
        .call(ft_contract_2.id(), "ft_transfer")
        .args_json(serde_json::json!({
            "receiver_id": eve.id(),
            "amount": transfer_amount_str
        }))
        .deposit(1)
        .transact()
        .await?
        .into_result()?;

    // Add fungible token 2
    owner
        .call(action_contract.id(), "add_whitelisted_token")
        .args_json(json!({"token_id":ft_contract_2.id()}))
        .deposit(parse_near!("0.008 N"))
        .gas(parse_gas!("200 Tgas") as u64)
        .transact()
        .await?
        .into_result()?;

    let message = String::from(
        r#"
        {
            "message_type": {
                "Offchain": {
                    "offchain_id": "2"
                }
            },
            "quest_type": "Offchain"
        }
        "#,
    );

    eve.call(ft_contract_2.id(), "ft_transfer_call")
        .args_json(json!({"amount":U128::from(parse_near!("100 N")), "receiver_id":action_contract.id(), "msg":message}))
        .deposit(1)
        .gas(parse_gas!("200 Tgas") as u64)
        .transact()
        .await?
        .into_result()?;

    let eve_fungible_balance_after_first_deposit: u128 = action_contract
        .as_account()
        .call(action_contract.id(), "get_client_balance")
        .args_json(json!({"account_id":eve.id(), "token_id":ft_contract_2.id()}))
        .transact()
        .await?
        .json()?;

    assert_eq!(eve_fungible_balance_after_first_deposit, parse_near!("100 N"));

    let first_deposit_amount = U128::from(parse_near!("40 N"));

    charlie
        .call(ft_contract_2.id(), "ft_transfer_call")
        .args_json(json!({"amount":first_deposit_amount, "receiver_id":action_contract.id(), "msg":message}))
        .deposit(1)
        .gas(parse_gas!("200 Tgas") as u64)
        .transact()
        .await?
        .into_result()?;

    let charlie_fungible_balance_after_first_deposit: u128 = action_contract
        .as_account()
        .call(action_contract.id(), "get_client_balance")
        .args_json(json!({"account_id":charlie.id(), "token_id":ft_contract_2.id()}))
        .transact()
        .await?
        .json()?;

    assert_eq!(charlie_fungible_balance_after_first_deposit, parse_near!("40 N"));

    let eve_fungible_balance: u128 = action_contract
        .as_account()
        .call(action_contract.id(), "get_client_balance")
        .args_json(json!({"account_id":eve.id(), "token_id":ft_contract_2.id()}))
        .transact()
        .await?
        .json()?;

    assert_eq!(eve_fungible_balance, parse_near!("100 N"));

    // action balance in action contract after deposit
    let action_fungible_token_balance: u128 = action_contract
        .as_account()
        .call(action_contract.id(), "get_owner_balance")
        .args_json(json!({ "token_id": ft_contract_2.id() }))
        .transact()
        .await?
        .json()?;

    assert_eq!(action_fungible_token_balance, parse_near!("140 N"));

    let second_deposit_amount = U128::from(parse_near!("30 N"));

    dave.call(ft_contract_2.id(), "ft_transfer_call")
        .args_json(json!({"amount":second_deposit_amount, "receiver_id":action_contract.id(), "msg":message}))
        .deposit(1)
        .gas(parse_gas!("200 Tgas") as u64)
        .transact()
        .await?
        .into_result()?;

    let charlie_fungible_balance: u128 = charlie
        .call(action_contract.id(), "get_client_balance")
        .args_json(json!({"account_id":charlie.id(), "token_id": ft_contract_2.id()}))
        .transact()
        .await?
        .json()?;

    assert_eq!(charlie_fungible_balance, parse_near!("40 N"));

    // dave fungible token 2 balance in action contract after deposit
    let dave_fungible_token_balance_after_deposit: u128 = dave
        .call(action_contract.id(), "get_client_balance")
        .args_json(json!({"account_id":dave.id(), "token_id":ft_contract_2.id()}))
        .transact()
        .await?
        .json()?;

    assert_eq!(dave_fungible_token_balance_after_deposit, parse_near!("30 N"));

    // action balance in action contract after deposit
    let action_fungible_token_balance_second: u128 = action_contract
        .as_account()
        .call(action_contract.id(), "get_owner_balance")
        .args_json(json!({ "token_id": ft_contract_2.id() }))
        .transact()
        .await?
        .json()?;

    assert_eq!(action_fungible_token_balance_second, parse_near!("170 N"));

    let third_deposit_amount = U128::from(parse_near!("50 N"));

    dave.call(ft_contract_2.id(), "ft_transfer_call")
        .args_json(json!({"amount":third_deposit_amount, "receiver_id":action_contract.id(), "msg":message}))
        .deposit(1)
        .gas(parse_gas!("200 Tgas") as u64)
        .transact()
        .await?
        .into_result()?;

    let dave_fungible_token_balance: u128 = dave
        .call(action_contract.id(), "get_client_balance")
        .args_json(json!({"account_id":dave.id(), "token_id":ft_contract_2.id()}))
        .transact()
        .await?
        .json()?;

    assert_eq!(dave_fungible_token_balance, parse_near!("80 N"));

    let fourth_deposit_amount = U128::from(parse_near!("50 N"));

    charlie
        .call(ft_contract_2.id(), "ft_transfer_call")
        .args_json(json!({"amount":fourth_deposit_amount, "receiver_id":action_contract.id(), "msg":message}))
        .deposit(1)
        .gas(parse_gas!("200 Tgas") as u64)
        .transact()
        .await?
        .into_result()?;

    // charlie fungible token 2 balance in action contract after deposit
    let charlie_fungible_token_balance: u128 = charlie
        .call(action_contract.id(), "get_client_balance")
        .args_json(json!({"account_id":charlie.id(), "token_id":ft_contract_2.id()}))
        .transact()
        .await?
        .json()?;

    assert_eq!(charlie_fungible_token_balance, parse_near!("90 N"));

    // action balance in action contract after deposit
    let action_fungible_token_balance_four: u128 = action_contract
        .as_account()
        .call(action_contract.id(), "get_owner_balance")
        .args_json(json!({ "token_id": ft_contract_2.id() }))
        .transact()
        .await?
        .json()?;

    assert_eq!(action_fungible_token_balance_four, parse_near!("270 N"));

    println!("      Passed ✅ test_user_deposit_fungible_token_2");

    Ok(())
}

async fn test_should_fail_not_owner_create_reward(
    not_owner: &Account,
    action_contract: &Contract,
    ft_contract_1: &Contract,
) -> anyhow::Result<()> {
    let reward_balance = parse_near!("5 N");

    let user1: AccountId = "user1.test.near".parse().unwrap();

    #[derive(Serialize, Deserialize)]
    #[serde(crate = "near_sdk::serde")]
    pub struct InforReward {
        pub user: AccountId,
        pub balance: u128,
        pub token_id: AccountId,
    }

    let user1_info = InforReward {
        user: user1.clone(),
        balance: reward_balance,
        token_id: ft_contract_1.id().clone(),
    };

    let data = vec![user1_info];

    let res = not_owner
        .call(action_contract.id(), "create_reward")
        .args_json(json!({ "data": data }))
        .transact()
        .await?;
    assert_eq!(true, res.is_failure());

    println!("      Passed ✅ test_should_fail_not_owner_create_reward");
    Ok(())
}

async fn test_reward_work(
    owner: &Account,
    action_contract: &Contract,
    ft_contract_1: &Contract,
    ft_contract_2: &Contract,
) -> anyhow::Result<()> {
    let reward_balance = parse_near!("5 N");

    let user1: AccountId = "user1.test.near".parse().unwrap();
    let user2: AccountId = "user2.test.near".parse().unwrap();
    let user4: AccountId = "user4.test.near".parse().unwrap();
    #[derive(Serialize, Deserialize)]
    #[serde(crate = "near_sdk::serde")]
    pub struct InforReward {
        pub user: AccountId,
        pub balance: u128,
        pub token_id: AccountId,
    }

    let user1_info = InforReward {
        user: user1.clone(),
        balance: reward_balance,
        token_id: ft_contract_1.id().clone(),
    };

    let user2_info = InforReward {
        user: user2.clone(),
        balance: reward_balance,
        token_id: ft_contract_1.id().clone(),
    };

    let user4_info = InforReward {
        user: user4.clone(),
        balance: reward_balance,
        token_id: ft_contract_1.id().clone(),
    };

    let user1_info_ft2 = InforReward {
        user: user1.clone(),
        balance: parse_near!("7 N"),
        token_id: ft_contract_2.id().clone(),
    };

    let user2_info_ft2 = InforReward {
        user: user2.clone(),
        balance: parse_near!("10 N"),
        token_id: ft_contract_2.id().clone(),
    };

    let data = vec![user1_info, user2_info, user4_info, user2_info_ft2, user1_info_ft2];

    owner
        .call(action_contract.id(), "create_reward")
        .args_json(json!({ "data": data }))
        .transact()
        .await?
        .into_result()?;

    let user1_ft2_reward: u128 = action_contract
        .as_account()
        .call(action_contract.id(), "get_user_reward")
        .args_json(json!({"account_id":user1, "token_id":ft_contract_2.id()}))
        .transact()
        .await?
        .json()?;

    assert_eq!(user1_ft2_reward, parse_near!("7 N"));

    let user2_ft2_reward: u128 = action_contract
        .as_account()
        .call(action_contract.id(), "get_user_reward")
        .args_json(json!({"account_id":user2, "token_id":ft_contract_2.id()}))
        .transact()
        .await?
        .json()?;

    assert_eq!(user2_ft2_reward, parse_near!("10 N"));

    let user1_ft1_reward: u128 = action_contract
        .as_account()
        .call(action_contract.id(), "get_user_reward")
        .args_json(json!({"account_id":user1, "token_id":ft_contract_1.id()}))
        .transact()
        .await?
        .json()?;

    assert_eq!(user1_ft1_reward, parse_near!("5 N"));

    let user2_ft1_reward: u128 = action_contract
        .as_account()
        .call(action_contract.id(), "get_user_reward")
        .args_json(json!({"account_id":user2, "token_id":ft_contract_1.id()}))
        .transact()
        .await?
        .json()?;

    assert_eq!(user2_ft1_reward, parse_near!("5 N"));

    let user2_ft2_reward: u128 = action_contract
        .as_account()
        .call(action_contract.id(), "get_user_reward")
        .args_json(json!({"account_id":user2, "token_id":ft_contract_2.id()}))
        .transact()
        .await?
        .json()?;

    assert_eq!(user2_ft2_reward, parse_near!("10 N"));

    let user1_ft2_reward: u128 = action_contract
        .as_account()
        .call(action_contract.id(), "get_user_reward")
        .args_json(json!({"account_id":user1, "token_id":ft_contract_2.id()}))
        .transact()
        .await?
        .json()?;

    assert_eq!(user1_ft2_reward, parse_near!("7 N"));

    println!("      Passed ✅ test_reward_work");

    Ok(())
}

async fn test_work_user_claim(
    user1: &Account,
    user2: &Account,
    ft_contract_1: &Contract,
    ft_contract_2: &Contract,
    action_contract: &Contract,
) -> anyhow::Result<()> {
    let user1_ft1_before_claim: U128 = user1
        .call(ft_contract_1.id(), "ft_balance_of")
        .args_json(json!({"account_id":user1.id()}))
        .transact()
        .await?
        .json()?;

    assert_eq!(user1_ft1_before_claim, U128::from(0));

    let user1_ft2_before_claim: U128 = user1
        .call(ft_contract_2.id(), "ft_balance_of")
        .args_json(json!({"account_id":user1.id()}))
        .transact()
        .await?
        .json()?;

    assert_eq!(user1_ft2_before_claim, U128::from(0));

    let user2_ft1_before_claim: U128 = user2
        .call(ft_contract_1.id(), "ft_balance_of")
        .args_json(json!({"account_id":user1.id()}))
        .transact()
        .await?
        .json()?;

    assert_eq!(user2_ft1_before_claim, U128::from(0));

    let user2_ft2_before_claim: U128 = user2
        .call(ft_contract_2.id(), "ft_balance_of")
        .args_json(json!({"account_id":user1.id()}))
        .transact()
        .await?
        .json()?;

    assert_eq!(user2_ft2_before_claim, U128::from(0));

    let action_ft1_before_claim: U128 = action_contract
        .as_account()
        .call(ft_contract_1.id(), "ft_balance_of")
        .args_json(json!({"account_id":action_contract.id()}))
        .transact()
        .await?
        .json()?;

    assert_eq!(action_ft1_before_claim, U128::from(parse_near!("50 N")));

    let action_ft2_before_claim: U128 = action_contract
        .as_account()
        .call(ft_contract_2.id(), "ft_balance_of")
        .args_json(json!({"account_id":action_contract.id()}))
        .transact()
        .await?
        .json()?;

    assert_eq!(action_ft2_before_claim, U128::from(parse_near!("270 N")));

    let user1_reward_ft1_before_claim: u128 = user1
        .call(action_contract.id(), "get_user_reward")
        .args_json(json!({ "account_id":user1.id(),"token_id": ft_contract_1.id() }))
        .transact()
        .await?
        .json()?;
    assert_eq!(user1_reward_ft1_before_claim, parse_near!("5 N"));

    let user1_reward_ft2_before_claim: u128 = user1
        .call(action_contract.id(), "get_user_reward")
        .args_json(json!({ "account_id":user1.id(),"token_id": ft_contract_2.id() }))
        .transact()
        .await?
        .json()?;

    assert_eq!(user1_reward_ft2_before_claim, parse_near!("7 N"));

    user1
        .call(action_contract.id(), "claim")
        .args_json(json!({ "token_id": ft_contract_1.id() }))
        .deposit(parse_near!("0.008 N"))
        .gas(parse_gas!("200 Tgas") as u64)
        .transact()
        .await?
        .into_result()?;

    let user1_ft1_after_claim: U128 = user1
        .call(ft_contract_1.id(), "ft_balance_of")
        .args_json(json!({"account_id":user1.id()}))
        .transact()
        .await?
        .json()?;

    assert_eq!(user1_ft1_after_claim, U128::from(parse_near!("5 N")));

    let action_ft1_after_claim: U128 = action_contract
        .as_account()
        .call(ft_contract_1.id(), "ft_balance_of")
        .args_json(json!({"account_id":action_contract.id()}))
        .transact()
        .await?
        .json()?;

    assert_eq!(action_ft1_after_claim, U128::from(parse_near!("45 N")));

    user2
        .call(action_contract.id(), "claim")
        .args_json(json!({ "token_id": ft_contract_1.id() }))
        .deposit(parse_near!("0.008 N"))
        .gas(parse_gas!("200 Tgas") as u64)
        .transact()
        .await?
        .into_result()?;

    let user2_ft1_after_claim: U128 = user2
        .call(ft_contract_1.id(), "ft_balance_of")
        .args_json(json!({"account_id":user2.id()}))
        .transact()
        .await?
        .json()?;

    assert_eq!(user2_ft1_after_claim, U128::from(parse_near!("5 N")));

    let action_ft1_after_claim: U128 = action_contract
        .as_account()
        .call(ft_contract_1.id(), "ft_balance_of")
        .args_json(json!({"account_id":action_contract.id()}))
        .transact()
        .await?
        .json()?;

    assert_eq!(action_ft1_after_claim, U128::from(parse_near!("40 N")));

    // check reward state
    let action_ft1_after_claim: u128 = action_contract
        .as_account()
        .call(action_contract.id(), "get_owner_balance")
        .args_json(json!({"token_id": ft_contract_1.id()}))
        .transact()
        .await?
        .json()?;

    assert_eq!(action_ft1_after_claim, parse_near!("40 N"));

    // User1 claim FT2

    let user1_reward_ft2_before_claim: u128 = user1
        .call(action_contract.id(), "get_user_reward")
        .args_json(json!({ "account_id":user1.id(),"token_id": ft_contract_2.id() }))
        .transact()
        .await?
        .json()?;

    assert_eq!(user1_reward_ft2_before_claim, parse_near!("7 N"));

    user1
        .call(action_contract.id(), "claim")
        .args_json(json!({ "token_id": ft_contract_2.id() }))
        .deposit(parse_near!("0.008 N"))
        .gas(parse_gas!("200 Tgas") as u64)
        .transact()
        .await?
        .into_result()?;

    let user1_ft2_after_claim: U128 = user1
        .call(ft_contract_2.id(), "ft_balance_of")
        .args_json(json!({"account_id":user1.id()}))
        .transact()
        .await?
        .json()?;

    assert_eq!(user1_ft2_after_claim, U128::from(parse_near!("7 N")));

    let action_ft2_after_claim: U128 = action_contract
        .as_account()
        .call(ft_contract_2.id(), "ft_balance_of")
        .args_json(json!({"account_id":action_contract.id()}))
        .transact()
        .await?
        .json()?;

    assert_eq!(action_ft2_after_claim, U128::from(parse_near!("263 N")));

    let user1_reward_ft2_after_claim: u128 = user1
        .call(action_contract.id(), "get_user_reward")
        .args_json(json!({ "account_id":user1.id(),"token_id":ft_contract_2.id() }))
        .transact()
        .await?
        .json()?;

    assert_eq!(user1_reward_ft2_after_claim, parse_near!("0 N"));

    // User2 claim FT2

    user2
        .call(action_contract.id(), "claim")
        .args_json(json!({ "token_id":ft_contract_2.id() }))
        .deposit(parse_near!("0.008 N"))
        .gas(parse_gas!("200 Tgas") as u64)
        .transact()
        .await?
        .into_result()?;

    let user2_reward_ft2_after_claim: u128 = user1
        .call(action_contract.id(), "get_user_reward")
        .args_json(json!({ "account_id":user1.id(),"token_id":ft_contract_2.id() }))
        .deposit(parse_near!("0.008 N"))
        .gas(parse_gas!("200 Tgas") as u64)
        .transact()
        .await?
        .json()?;

    assert_eq!(user2_reward_ft2_after_claim, parse_near!("0 N"));

    let user2_ft2_after_claim: U128 = user1
        .call(ft_contract_2.id(), "ft_balance_of")
        .args_json(json!({"account_id":user2.id()}))
        .transact()
        .await?
        .json()?;

    assert_eq!(user2_ft2_after_claim, U128::from(parse_near!("10 N")));

    let action_ft2_after_claim: U128 = action_contract
        .as_account()
        .call(ft_contract_2.id(), "ft_balance_of")
        .args_json(json!({"account_id":action_contract.id()}))
        .transact()
        .await?
        .json()?;

    assert_eq!(action_ft2_after_claim, U128::from(parse_near!("253 N")));

    //check reward state action contract
    let action_ft2_after_claim: u128 = action_contract
        .as_account()
        .call(action_contract.id(), "get_owner_balance")
        .args_json(json!({"token_id":ft_contract_2.id() }))
        .transact()
        .await?
        .json()?;

    assert_eq!(action_ft2_after_claim, parse_near!("253 N"));

    println!("      Passed ✅ test_work_user_claim");
    Ok(())
}

async fn test_should_fail_user_reclaim(
    user1: &Account,
    action_contract: &Contract,
    ft_contract_1: &Contract,
) -> anyhow::Result<()> {
    let res = user1
        .call(action_contract.id(), "claim")
        .args_json(json!({ "token_id": ft_contract_1.id() }))
        .deposit(parse_near!("0.008 N"))
        .gas(parse_gas!("200 Tgas") as u64)
        .transact()
        .await?;

    //Smart contract panicked: The token reward must greater than zero"
    assert_eq!(true, res.is_failure());

    println!("      Passed ✅ test_should_fail_user_reclaim");

    Ok(())
}
async fn test_should_fail_user_is_not_lucky(
    user3: &Account,
    ft_contract_1: &Contract,
    action_contract: &Contract,
) -> anyhow::Result<()> {
    let res = user3
        .call(action_contract.id(), "claim")
        .args_json(json!({ "token_id": ft_contract_1.id() }))
        .deposit(parse_near!("0.008 N"))
        .gas(parse_gas!("200 Tgas") as u64)
        .transact()
        .await?;

    //"Smart contract panicked: You were not a lucky user")
    assert_eq!(true, res.is_failure());

    println!("      Passed ✅ test_should_fail_user_is_not_lucky");
    Ok(())
}

async fn test_should_fail_user_claim_wrong_fungible_token(
    user4: &Account,
    ft_contract_2: &Contract,
    action_contract: &Contract,
) -> anyhow::Result<()> {
    // user4 can claim ft_contract 1 but can not claim ft_contract_2

    let res = user4
        .call(action_contract.id(), "claim")
        .args_json(json!({ "token_id": ft_contract_2.id() }))
        .deposit(parse_near!("0.008 N"))
        .gas(parse_gas!("200 Tgas") as u64)
        .transact()
        .await?;
    //You can not claim the dev-20230412135715-88244124528310 token
    // due to only ft_contract_1 not ft_contract_2
    assert_eq!(true, res.is_failure());

    Ok(())
}

async fn test_user_buy_ticket_native_token(
    client_event: &Account,
    user_event: &Account,
    action_contract: &Contract,
) -> anyhow::Result<()> {

    #[derive(Serialize, Deserialize, Debug)]
    pub struct EventInfo {
        pub event_id: String,
        pub client_id: AccountId,
        pub token_id: AccountId,
        pub price: u128,
        pub total_paid_tickets: u128,
        pub is_distributed: bool,
    }

    let price_event = U128::from(parse_near!("5"));


    client_event
        .call(action_contract.id(), "deposit_native")
        .args_json(json!({ "amount": price_event, "event_id":"1","quest_type":"Event" }))
        .deposit(parse_near!("0.0008 N"))
        .gas(parse_gas!("200 Tgas") as u64)
        .transact()
        .await?
        .into_result()?;

    let res: EventInfo = client_event
        .call(action_contract.id(), "get_event_info")
        .args_json(json!({ "event_id":"1" }))
        .transact()
        .await?
        .json()?;

    println!("event info before:{:?}", res);

    let res_buy_ticket = user_event
        .call(action_contract.id(), "buy_ticket")
        .args_json(json!({ "amount": price_event, "event_id":"1"}))
        .deposit(parse_near!("5 N"))
        .gas(parse_gas!("200 Tgas") as u64)
        .transact()
        .await?;

    println!("res buy ticket :{:?}",res_buy_ticket);
    println!("ok:{}",res_buy_ticket.is_success());

    let res: EventInfo = client_event
    .call(action_contract.id(), "get_event_info")
    .args_json(json!({ "event_id":"1" }))
    .transact()
    .await?
    .json()?;

    println!("event info after:{:?}", res);



    let second_buy_ticket = user_event
        .call(action_contract.id(), "buy_ticket")
        .args_json(json!({ "amount": price_event, "event_id":"1"}))
        .deposit(parse_near!("5 N"))
        .gas(parse_gas!("200 Tgas") as u64)
        .transact()
        .await?;
    println!("ok second:{}",second_buy_ticket.is_success());
    // do to: each user can buy only 1 ticket
    let second: EventInfo = client_event
    .call(action_contract.id(), "get_event_info")
    .args_json(json!({ "event_id":"1" }))
    .transact()
    .await?
    .json()?;

    println!("event info after:{:?}", second);
    println!("      Passed ✅ test_user_buy_ticket_native_token");

    Ok(())
}

async fn test_user_buy_ticket_fungible_token(owner: &Account,client_event: &Account ,user_event:&Account, ft_contract_1: &Contract, action_contract: &Contract ) -> anyhow::Result<()> {

        owner
        .call(ft_contract_1.id(), "storage_deposit")
        .args_json(serde_json::json!({
            "account_id": user_event.id()
        }))
        .deposit(parse_near!("0.008 N"))
        .transact()
        .await?
        .into_result()?;

    // Transfer fungible token ft_contract_2
    let transfer_amount_str = parse_near!("1,000 N").to_string();

    // Transfer dave ft_contract_2
    owner
        .call(ft_contract_1.id(), "ft_transfer")
        .args_json(serde_json::json!({
            "receiver_id": user_event.id(),
            "amount": transfer_amount_str
        }))
        .deposit(1)
        .transact()
        .await?
        .into_result()?;


    let user_event_ft_balance_before: U128 = owner
        .call(ft_contract_1.id(), "ft_balance_of")
        .args_json(json!({"account_id":user_event.id()}))
        .transact()
        .await?
        .json()?;

    assert_eq!(user_event_ft_balance_before, U128::from(parse_near!("1000 N")));

    let message = String::from(
        r#"
        {
            "message_type": {
                "Event": {
                    "event_id": "2"
                }
            },
            "quest_type": "Event"
        }
        "#,
    );

    let amount = U128::from(parse_near!("5 N"));
    let res = client_event
        .call(ft_contract_1.id(), "ft_transfer_call")
        .args_json(json!({"amount":amount, "receiver_id":action_contract.id(), "msg":message}))
        .deposit(1)
        .gas(parse_gas!("200 Tgas") as u64)
        .transact()
        .await?;

    println!("client ft is ok :{}",res.is_success());


    println!("      Passed ✅ test_user_buy_ticket_fungible_token");
    Ok(())
}
