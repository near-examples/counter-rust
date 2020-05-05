Deploying multiple contracts
============================

This file will describe the commands to run (using near-shell) to deploy your contracts.

The `out/` directory should have two files.
1. `rust_counter_tutorial.wasm`
2. `rust_donation_tutorial.wasm`

On your terminal:
1. Ensure you navigate to project directory, containing folders such as `contracts`, `out`, `src`, etc.
2. Check that the compiled contract files exist by running `ls out/` and expect to see the filenames mentioned above. If not, please run `yarn build` and check again.
3. Ensure you have `near-shell` installed by running `near` and expect to see a list of available commands. If it's not found, please run `npm install -g near-shell`
4. Ensure you've run `near login` and selected the account `YOU.testnet` where `YOU` is your NEAR account name created from https://wallet.testnet.nearprotocol.com

We'll be running a command like this:

    near deploy --accountId counter.YOU.testnet --wasmFile out/rust_counter_tutorial.wasm
    
and

    near deploy  --accountId donation.YOU.testnet --wasmFile out/rust_donation_tutorial.wasm

These commands essentially say, "deploy the smart contract file specified to the account after 'deploy', you can find the keys for this because I've logged in already."