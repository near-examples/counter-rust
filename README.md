Counter example in Rust
=================================

[![Open in Gitpod!](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/near-examples/rust-counter)

<!-- MAGIC COMMENT: DO NOT DELETE! Everything above this line is hidden on NEAR Examples page -->

## Description

This contract implements simple counter backed by storage on blockchain.
Contract in `contract/src/lib.rs` provides methods to increment / decrement counter and get its current value or reset.

### Starting the Counter

In Gitpod, the counter will start automatically. Please look in the terminal for a link to follow.

If you see a popup that says "A service is available on port 1234", click "Open Browser" button.  *The "Open Preview" button will open the preview pane but it will not be able to connect to the wallet.*


### Operation of the Counter

+ `+` and `-` buttons increase and decrease value correspondingly.
+ `L` button turns on the light, just for fun.
+ `RS` button resets the counter
+ Use the `LE` and `RE` buttons then observe the robot's face ;)


### Viewing Transaction Logs

To view transaction logs for blockchain activity, see the Activity pane of the wallet you connected to the counter. It will look something like:

```
Called method: reset in contract: @dev-1595673156644   8 minutes ago  Succeeded
```

You can also view the actual transactions on a block explorer, for example https://explorer.testnet.near.org/ under Recent Transactions.

## To Run
Open in the Gitpod link above or clone the repository.

```
git clone https://github.com/near-examples/rust-counter
```


## Setup [Or skip to Login if in Gitpod](#login)
Install dependencies:

```
yarn
```

If you don't have `Rust` installed, complete the following 3 steps:

1) Install Rustup by running:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

([Taken from official installation guide](https://www.rust-lang.org/tools/install))

2) Configure your current shell by running:

```
source $HOME/.cargo/env
```

3) Add wasm target to your toolchain by running:

```
rustup target add wasm32-unknown-unknown
```

Next, make sure you have `near-cli` by running:

```
near --version
```

If you need to install `near-cli`:

```
npm install near-cli -g
```

## Login
If you do not have a NEAR account, please create one with [NEAR Wallet](https://wallet.testnet.near.org).

In the project root, login with `near-cli` by following the instructions after this command:

```
near login
```

Modify the top of `src/config.js`, changing the `CONTRACT_NAME` to be the NEAR account that was just used to log in.

```javascript
…
const CONTRACT_NAME = 'YOUR_ACCOUNT_NAME_HERE'; /* TODO: fill this in! */
…
```

Start the example!

```
yarn start
```

## To Test

```
yarn test
```

## To Explore

- `contract/src/lib.rs` for the contract code
- `src/index.html` for the front-end HTML
- `src/main.js` for the JavaScript front-end code and how to integrate contracts
- `src/test.js` for the JS tests for the contract

## To Build the Documentation

```
cd contract
cargo doc --no-deps --open
```
