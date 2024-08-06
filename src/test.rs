#![cfg(test)]

extern crate std;
use std::println;

use soroban_sdk::{token, Address, Env, String};

use crate::{Contract, ContractClient};

#[test]
fn test() {
    let env = Env::from_ledger_snapshot_file("snapshot.json");
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    let native_sac_address = Address::from_string(&String::from_str(
        &env,
        "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC",
    ));
    let native_sac_client = token::Client::new(&env, &native_sac_address);

    let g_address_1 = Address::from_string(&String::from_str(
        &env,
        "GBYEUXTQY4S7XN6O3RIELIL25AMWMTBGLEDZLHL5UHDDNEY7LK5EJTL2",
    ));

    let g_address_2 = Address::from_string(&String::from_str(
        &env,
        "GBTFJPSA4GC7WLQIOFQ35345LJTCRZPD4ZLFCDEWVZ7WY4MVM7SCHIEQ",
    ));

    let balance_1 = native_sac_client.balance(&g_address_1);
    let balance_2 = native_sac_client.balance(&g_address_2);

    println!("G1 before balance: {}", balance_1);
    println!("G2 before balance: {}", balance_2);

    client.mock_all_auths_allowing_non_root_auth().pay(
        &native_sac_address,
        &g_address_1,
        &g_address_2,
        &10_000_000,
    );

    let balance_1 = native_sac_client.balance(&g_address_1);
    let balance_2 = native_sac_client.balance(&g_address_2);

    println!("G1 after balance: {}", balance_1);
    println!("G2 after balance: {}", balance_2);
}
