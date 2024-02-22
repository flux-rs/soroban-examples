flux_rs::flux!(
use crate::storage_types::{DataKey, Balance, BALANCE_BUMP_AMOUNT, BALANCE_LIFETIME_THRESHOLD};
use soroban_sdk::{Address, Env};

pub fn read_balance(e: &Env, addr: Address) -> i128{v: v >= 0} {
    let key = DataKey::Balance(addr);
    if let Some(balance) = e.storage().persistent().get::<DataKey, Balance>(&key) {
        e.storage()
            .persistent()
            .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
        balance.amount
    } else {
        0
    }
}

fn write_balance(e: &Env, addr: Address, amount: i128{v: v >= 0}) {
    let key = DataKey::Balance(addr);
    e.storage().persistent().set(&key, &Balance { amount });
    e.storage()
        .persistent()
        .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}

pub fn receive_balance(e: &Env, addr: Address, amount: i128{v: v >= 0}) {
    let balance = read_balance(e, addr.clone());
    if balance >= i128::MAX - amount {
        panic!("bad amount");
    }
    write_balance(e, addr, balance + amount);
}

pub fn spend_balance(e: &Env, addr: Address, amount: i128{v: v >=0}) {
    let balance = read_balance(e, addr.clone());
    if balance < amount {
        panic!("insufficient balance");
    }
    write_balance(e, addr, balance - amount);
}

);
