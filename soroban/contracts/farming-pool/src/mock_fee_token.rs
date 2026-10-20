//! A test-only token that skims a percentage fee from every `transfer` —
//! the classic fee-on-transfer / deflationary token (#458). `stake` credits
//! the caller's position with `amount`, but this token delivers only
//! `amount - fee` to the pool, so without a received-amount check the pool
//! would record liabilities it can never pay out.

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

const FEE_DENOMINATOR: i128 = 10_000;

#[contracttype]
enum DataKey {
    Balance(Address),
    FeeBps,
}

#[contract]
pub struct MockFeeToken;

#[contractimpl]
impl MockFeeToken {
    pub fn mint(env: Env, to: Address, amount: i128) {
        let balance: i128 = env
            .storage()
            .instance()
            .get(&DataKey::Balance(to.clone()))
            .unwrap_or(0);
        env.storage()
            .instance()
            .set(&DataKey::Balance(to), &(balance + amount));
    }

    /// Sets the transfer fee in basis points (e.g. `100` = 1%).
    pub fn set_fee_bps(env: Env, fee_bps: u32) {
        env.storage().instance().set(&DataKey::FeeBps, &fee_bps);
    }

    pub fn balance(env: Env, id: Address) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::Balance(id))
            .unwrap_or(0)
    }

    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        let fee_bps: u32 = env.storage().instance().get(&DataKey::FeeBps).unwrap_or(0);
        let fee = amount.saturating_mul(fee_bps.into()) / FEE_DENOMINATOR;

        let from_balance: i128 = env
            .storage()
            .instance()
            .get(&DataKey::Balance(from.clone()))
            .unwrap_or(0);
        env.storage()
            .instance()
            .set(&DataKey::Balance(from), &(from_balance - amount));

        let to_balance: i128 = env
            .storage()
            .instance()
            .get(&DataKey::Balance(to.clone()))
            .unwrap_or(0);
        env.storage()
            .instance()
            .set(&DataKey::Balance(to), &(to_balance + amount - fee));
    }
}
