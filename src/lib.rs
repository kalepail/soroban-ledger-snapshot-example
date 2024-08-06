#![no_std]
use soroban_sdk::{contract, contractimpl, token, Address, Env};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn pay(env: Env, sac_address: Address, from: Address, to: Address, amount: i128) {
        token::Client::new(&env, &sac_address).transfer(&from, &to, &amount);
    }
}

mod test;
