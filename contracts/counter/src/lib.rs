use borsh::{BorshDeserialize, BorshSerialize};
// note the custom type AccountId, which is essentially a String
// to see the other types visit the link below and select the version if needed:
//   https://docs.rs/near-vm-logic/0.8.0/near_vm_logic/types/index.html
use near_sdk::{AccountId, env, near_bindgen};
use near_sdk::collections::Map;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// add the following attributes to prepare your code for serialization and invocation on the blockchain
// more built-in Rust attributes here: https://doc.rust-lang.org/reference/attributes.html#built-in-attributes-index
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Counter {
    // see more data types at https://doc.rust-lang.org/book/ch03-02-data-types.html
    // see performant data structures (like Map) here:
    //   https://github.com/near/near-sdk-rs/tree/master/near-sdk/src/collections
    user_counters: Map<AccountId, i8>,
}

#[near_bindgen]
impl Counter {
    // init attribute used for instantiation
    #[init]
    pub fn new() -> Self {
        // useful snippet to copy/paste, making sure state isn't already initialized
        assert!(env::state_read::<Self>().is_none(), "Already initialized");
        Self {
            user_counters: Map::new(b"my id which is super unique".to_vec()),
        }
    }

    /// returns 8-bit signed integer representing the number for the account argument
    // note the parameter is &self (without being mutable) meaning it doesn't modify state
    // in the frontend (/src/main.js) this is added to the "viewMethods" array
    // using near-shell we can call this by:
    // near view counter.YOU.testnet get_num '{"account": "donation.YOU.testnet"}'
    pub fn get_num(&self, account: AccountId) -> i8 {
        // call our first private function
        // try removing the .clone() below and note the error. this may happen from time to time
        let caller_num = self.get_num_from_signer(account.clone());

        // here's a way to format multiple variables in order to log them
        let log_message = format!("{}'s number: {}", account, caller_num);
        env::log(log_message.as_bytes());
        return caller_num;
    }

    // our first private functions
    fn get_num_from_signer(&self, account: AccountId) -> i8 {
        // notice we've chosen to use an implicit "return" here
        match self.user_counters.get(&account) {
            Some(num) => {
                // found account user in map, return the number
                num
            },
            // did not find the account in the map
            // note: curly brackets after arrow are optional in simple cases, like other languages
            None => 0
        }
    }

    /// increment the counter *per account* that calls it
    // note the parameter is "&mut self" as this function modifies state
    // in the frontend (/src/main.js) this is added to the "changeMethods" array
    // using near-shell we can call this by:
    // near call counter.YOU.testnet increment --accountId donation.YOU.testnet
    pub fn increment(&mut self) {
        // note: adding one like this is an easy way to accidentally overflow
        // real smart contracts will want to have safety checks
        let caller = env::signer_account_id();
        let current_val = match self.user_counters.get(&caller) {
            Some(val) => val,
            None => 0i8
        };
        let new_value = current_val + 1;
        self.user_counters.insert(&caller.clone(), &new_value);

        let log_message = format!("Incremented to {}", new_value);
        env::log(log_message.as_bytes());
        after_counter_change();
    }

    /// decrement (subtract from) the counter *per account* that calls it
    // in (/src/main.js) this is also added to the "changeMethods" array
    // using near-shell we can call this by:
    // near call counter.YOU.testnet decrement --accountId donation.YOU.testnet
    pub fn decrement(&mut self) {
        // note: subtracting one like this is an easy way to accidentally overflow
        // real smart contracts will want to have safety checks
        let caller = env::signer_account_id();
        let current_val = match self.user_counters.get(&caller) {
            Some(val) => val,
            None => 0i8
        };
        let new_value = current_val - 1;
        self.user_counters.insert(&caller.clone(), &new_value);

        let log_message = format!("Decreased number to {}", new_value);
        env::log(log_message.as_bytes());
        after_counter_change();
    }

    /// reset to zero
    pub fn reset(&mut self) {
        let caller = env::signer_account_id();
        // 0 casted as i8 data type is "0i8"
        self.user_counters.insert(&caller, &0i8);
        // Another way to log on NEAR is to cast a string into bytes, hence "b" below:
        env::log(b"Reset counter to zero");
    }
}

// unlike the struct's functions above, this function cannot use attributes #[derive(…)] or #[near_bindgen]
// any attempts will throw helpful warnings upon 'cargo build'
// while this function cannot be invoked directly on the blockchain, it can be called from an invoked function
pub fn after_counter_change() {
    // show helpful warning that i8 (8-bit signed integer) will overflow above 127 or below -128
    env::log(b"Make sure you don't overflow, my friend.");
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test -- --nocapture
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // part of writing unit tests is setting up a mock context
    // this is also a useful list to peek at when wondering what's available in env::*
    fn get_context(input: Vec<u8>, is_view: bool, signer: AccountId) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: signer,
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19
        }
    }

    // mark individual unit tests with #[test] for them to be registered and fired
    // unlike other frameworks, the function names don't need to be special or have "test" in it
    #[test]
    fn increment() {
        // set up the mock context into the testing environment
        let context = get_context(vec![], false, "robert.testnet".to_string());
        testing_env!(context);
        // instantiate a contract variable with the counter at zero
        let mut contract = Counter::new();
        contract.increment();
        // we can do println! in tests, but reminder to use env::log outside of tests
        println!("Value after increment: {}", contract.get_num("robert.testnet".to_string()));
        // confirm that we received 1 when calling get_num
        assert_eq!(1, contract.get_num("robert.testnet".to_string()));
    }

    #[test]
    fn decrement() {
        let context = get_context(vec![], false, "robert.testnet".to_string());
        testing_env!(context);
        let mut contract = Counter::new();
        contract.decrement();
        println!("Value after decrement: {}", contract.get_num("robert.testnet".to_string()));
        // confirm that we received -1 when calling get_num
        assert_eq!(-1, contract.get_num("robert.testnet".to_string()));
    }

    #[test]
    fn increment_and_reset() {
        let context = get_context(vec![], false, "robert.testnet".to_string());
        testing_env!(context);
        let mut contract = Counter::new();
        contract.increment();
        contract.reset();
        println!("Value after reset: {}", contract.get_num("robert.testnet".to_string()));
        // confirm that we received 0 after reset
        assert_eq!(0, contract.get_num("robert.testnet".to_string()));
    }

    #[test]
    fn use_two_accounts_and_verify() {
        let context_robert = get_context(vec![], false, "robert.testnet".to_string());
        let context_alice = get_context(vec![], false, "alice.testnet".to_string());
        // increment twice on robert's account
        testing_env!(context_robert);
        let mut contract = Counter::new();
        contract.increment();
        contract.increment();
        // decrement once on alice's account
        testing_env!(context_alice);
        contract.decrement();
        // confirm values per account
        println!("Value from robert.testnet's account: {}", contract.get_num("robert.testnet".to_string()));
        println!("Value from alice.testnet's account: {}", contract.get_num("alice.testnet".to_string()));
        assert_eq!(2, contract.get_num("robert.testnet".to_string()));
        assert_eq!(-1, contract.get_num("alice.testnet".to_string()));
    }
}