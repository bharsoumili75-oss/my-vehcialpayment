#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol, symbol_short, Address, Map};

#[contract]
pub struct VehiclePaymentContract;

#[contractimpl]
impl VehiclePaymentContract {

    // Store a vehicle payment record
    pub fn make_payment(
        env: Env,
        buyer: Address,
        seller: Address,
        vehicle_id: Symbol,
        amount: i128,
    ) {
        // Require buyer authentication
        buyer.require_auth();

        // Get existing storage or create new map
        let mut payments: Map<Symbol, (Address, Address, i128)> =
            env.storage()
                .instance()
                .get(&symbol_short!("PAYMENTS"))
                .unwrap_or(Map::new(&env));

        // Store payment
        payments.set(vehicle_id.clone(), (buyer.clone(), seller.clone(), amount));

        // Save updated map
        env.storage()
            .instance()
            .set(&symbol_short!("PAYMENTS"), &payments);
    }

    // Retrieve payment details
    pub fn get_payment(
        env: Env,
        vehicle_id: Symbol,
    ) -> Option<(Address, Address, i128)> {
        let payments: Map<Symbol, (Address, Address, i128)> =
            env.storage()
                .instance()
                .get(&symbol_short!("PAYMENTS"))
                .unwrap_or(Map::new(&env));

        payments.get(vehicle_id)
    }
}