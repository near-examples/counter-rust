Counter example in Rust - Gitpod version
==================================================

This README is specific to Gitpod and this example. For local development, please see [README.md](README.md).

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


## To Test

```
cd contract
cargo test -- --nocapture
```

## To Explore

Explore the following files to see the code which runs the counter:

- `contract/src/lib.rs` for the contract code
- `src/index.html` for the front-end HTML
- `src/main.js` for the JavaScript front-end code and how to integrate contracts
- `src/test.js` for the JS tests for the contract


## Data collection

By using Gitpod in this project, you agree to opt-in to basic, anonymous analytics. No personal information is transmitted. Instead, these usage statistics aid in discovering potential bugs and user flow information.
