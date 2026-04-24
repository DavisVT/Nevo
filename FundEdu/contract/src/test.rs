#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env, String};

use crate::contract::{FundEduContract, FundEduContractClient};

fn setup() -> (Env, FundEduContractClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(FundEduContract, ());
    let client = FundEduContractClient::new(&env, &contract_id);
    (env, client)
}

#[test]
fn test_create_pool_returns_incremental_ids() {
    let (env, client) = setup();
    let sponsor = Address::generate(&env);
    let token = Address::generate(&env);

    let id0 = client.create_pool(
        &sponsor,
        &String::from_str(&env, "STEM 2026"),
        &50_000_000,
        &token,
    );
    let id1 = client.create_pool(
        &sponsor,
        &String::from_str(&env, "Arts 2026"),
        &20_000_000,
        &token,
    );

    assert_eq!(id0, 0);
    assert_eq!(id1, 1);
}

#[test]
fn test_get_pool_returns_correct_data() {
    let (env, client) = setup();
    let sponsor = Address::generate(&env);
    let token = Address::generate(&env);

    let pool_id = client.create_pool(
        &sponsor,
        &String::from_str(&env, "STEM 2026"),
        &50_000_000,
        &token,
    );

    let pool = client.get_pool(&pool_id).unwrap();
    assert_eq!(pool.name, String::from_str(&env, "STEM 2026"));
    assert_eq!(pool.target_amount, 50_000_000);
    assert_eq!(pool.sponsor, sponsor);
    assert!(pool.is_active);
}

#[test]
fn test_get_pool_returns_none_for_missing_id() {
    let (_env, client) = setup();
    assert!(client.get_pool(&99).is_none());
}

#[test]
fn test_claim_funds_success() {
    let (env, client) = setup();
    let sponsor = Address::generate(&env);
    let student = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token_id = env
        .register_stellar_asset_contract_v2(token_admin.clone())
        .address();
    let token = soroban_sdk::token::Client::new(&env, &token_id);

    // Create pool
    let pool_id = client.create_pool(
        &sponsor,
        &String::from_str(&env, "Scholarship 2026"),
        &100_000,
        &token_id,
    );

    // Manually set an approved application in storage
    let app = crate::types::Application {
        student: student.clone(),
        requested_amount: 50_000,
        total_granted: 50_000,
        amount_claimed: 0,
        status: crate::types::ApplicationStatus::Approved,
        milestone_index: 0,
    };

    env.as_contract(&client.address, || {
        crate::storage::set_application(&env, pool_id, student.clone(), &app);
    });

    // Fund the contract with tokens
    let token_admin_client = soroban_sdk::token::StellarAssetClient::new(&env, &token_id);
    token_admin_client.mint(&client.address, &50_000);

    // Claim part of the funds
    client.claim_funds(&pool_id, &student, &20_000);

    // Verify state
    let updated_app = env.as_contract(&client.address, || {
        crate::storage::get_application(&env, pool_id, student.clone()).unwrap()
    });
    assert_eq!(updated_app.amount_claimed, 20_000);
    assert_eq!(token.balance(&student), 20_000);

    // Claim the rest
    client.claim_funds(&pool_id, &student, &30_000);
    assert_eq!(token.balance(&student), 50_000);
}

#[test]
#[should_panic(expected = "Error(Contract, #3)")] // ExceedsGrant
fn test_claim_funds_exceeds_grant() {
    let (env, client) = setup();
    let sponsor = Address::generate(&env);
    let student = Address::generate(&env);
    let token_id = env
        .register_stellar_asset_contract_v2(Address::generate(&env))
        .address();

    let pool_id = client.create_pool(
        &sponsor,
        &String::from_str(&env, "Scholarship 2026"),
        &100_000,
        &token_id,
    );

    let app = crate::types::Application {
        student: student.clone(),
        requested_amount: 50_000,
        total_granted: 50_000,
        amount_claimed: 0,
        status: crate::types::ApplicationStatus::Approved,
        milestone_index: 0,
    };

    env.as_contract(&client.address, || {
        crate::storage::set_application(&env, pool_id, student.clone(), &app);
    });

    client.claim_funds(&pool_id, &student, &60_000);
}
