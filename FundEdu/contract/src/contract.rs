use soroban_sdk::{contract, contractimpl, token, Address, Env, String};

use crate::{
    errors::ContractError,
    storage::{get_application, get_pool, next_pool_id, set_application, set_pool},
    types::{ApplicationStatus, ScholarshipPool},
};

#[contract]
pub struct FundEduContract;

#[contractimpl]
impl FundEduContract {
    /// Create a new scholarship pool and return its assigned pool_id.
    pub fn create_pool(
        env: Env,
        sponsor: Address,
        name: String,
        target_amount: i128,
        token_address: Address,
    ) -> u64 {
        sponsor.require_auth();

        let pool_id = next_pool_id(&env);
        let pool = ScholarshipPool {
            name,
            sponsor,
            target_amount,
            token_address,
            is_active: true,
        };
        set_pool(&env, pool_id, &pool);
        pool_id
    }

    /// Retrieve a scholarship pool by its id. Returns None if not found.
    pub fn get_pool(env: Env, pool_id: u64) -> Option<ScholarshipPool> {
        get_pool(&env, pool_id)
    }

    /// Claim awarded scholarship funds.
    /// Follows Check-Effects-Interactions (CEI) pattern.
    pub fn claim_funds(
        env: Env,
        pool_id: u64,
        student: Address,
        amount: i128,
    ) -> Result<(), ContractError> {
        student.require_auth();

        if amount <= 0 {
            return Err(ContractError::InvalidAmount);
        }

        // 1. Checks
        let pool = get_pool(&env, pool_id).ok_or(ContractError::PoolNotFound)?;
        if !pool.is_active {
            return Err(ContractError::PoolNotActive);
        }

        let mut app = get_application(&env, pool_id, student.clone())
            .ok_or(ContractError::ApplicationNotFound)?;

        if app.status != ApplicationStatus::Approved {
            return Err(ContractError::NotApproved);
        }

        if app.amount_claimed + amount > app.total_granted {
            return Err(ContractError::ExceedsGrant);
        }

        // 2. Effects (Update state BEFORE interaction)
        app.amount_claimed += amount;
        set_application(&env, pool_id, student.clone(), &app);

        // 3. Interactions (External call after state update)
        let token_client = token::Client::new(&env, &pool.token_address);
        token_client.transfer(&env.current_contract_address(), &student, &amount);

        Ok(())
    }
}
