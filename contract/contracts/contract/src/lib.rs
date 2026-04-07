#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env};

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    // Initialize escrow with buyer, seller, and amount
    pub fn init(env: Env, buyer: Address, seller: Address, amount: i128) {
        buyer.require_auth();

        env.storage().instance().set(&symbol_short!("buyer"), &buyer);
        env.storage().instance().set(&symbol_short!("seller"), &seller);
        env.storage().instance().set(&symbol_short!("amount"), &amount);
        env.storage().instance().set(&symbol_short!("released"), &false);
    }

    // Buyer deposits funds (simulated logic)
    pub fn deposit(env: Env, buyer: Address) {
        buyer.require_auth();

        let stored_buyer: Address = env.storage().instance().get(&symbol_short!("buyer")).unwrap();
        if buyer != stored_buyer {
            panic!("Not authorized buyer");
        }

        // In real contract, token transfer logic would go here
    }

    // Release funds to seller
    pub fn release(env: Env, buyer: Address) {
        buyer.require_auth();

        let stored_buyer: Address = env.storage().instance().get(&symbol_short!("buyer")).unwrap();
        let seller: Address = env.storage().instance().get(&symbol_short!("seller")).unwrap();
        let released: bool = env.storage().instance().get(&symbol_short!("released")).unwrap();

        if buyer != stored_buyer {
            panic!("Only buyer can release funds");
        }

        if released {
            panic!("Already released");
        }

        // Simulated transfer
        // In real implementation, transfer tokens to seller

        env.storage().instance().set(&symbol_short!("released"), &true);
    }

    // Refund buyer if conditions fail
    pub fn refund(env: Env, seller: Address) {
        seller.require_auth();

        let stored_seller: Address = env.storage().instance().get(&symbol_short!("seller")).unwrap();
        let released: bool = env.storage().instance().get(&symbol_short!("released")).unwrap();

        if seller != stored_seller {
            panic!("Only seller can refund");
        }

        if released {
            panic!("Already released");
        }

        // Simulated refund logic
    }

    // View escrow details
    pub fn get_details(env: Env) -> (Address, Address, i128, bool) {
        let buyer: Address = env.storage().instance().get(&symbol_short!("buyer")).unwrap();
        let seller: Address = env.storage().instance().get(&symbol_short!("seller")).unwrap();
        let amount: i128 = env.storage().instance().get(&symbol_short!("amount")).unwrap();
        let released: bool = env.storage().instance().get(&symbol_short!("released")).unwrap();

        (buyer, seller, amount, released)
    }
}