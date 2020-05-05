use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId, env, ext_contract, near_bindgen};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// For this example contract here's a hardcoded, prepaid gas value good for making a single, simple call
const SINGLE_CALL_GAS: u64 = 200000000000000;

// If the name is not provided, the namespace for generated methods in derived by applying snake
// case to the trait name, e.g. ext_my_counter
#[ext_contract(ext)]
pub trait ExtMyCounter {
    fn increment(&mut self);
}

// Add the following attributes to prepare your code for serialization and invocation on the blockchain.
// More built-in Rust attributes here: https://doc.rust-lang.org/reference/attributes.html#built-in-attributes-index
// Here an empty struct is okay, as we're only using this for a cross-contract call.
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Donation {}

#[near_bindgen]
impl Donation {
    // No #[init] attribute or new() function is needed here.

    /// The account_id is the NEAR account where the counter smart contract has been deployed
    pub fn increment_my_number(&mut self, account_id: AccountId) {
        // The 0 is the amount of NEAR (Ⓝ) to send.
        // The final parameter is the amount of (extra) gas to add.
        ext::increment(&account_id, 0, SINGLE_CALL_GAS);
    }
}