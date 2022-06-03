use near_units::parse_near;
use workspaces::prelude::*;
use workspaces::{sandbox, Account, Contract, DevNetwork, Worker};

lazy_static_include::lazy_static_include_bytes! {
  pub COUNTER => "../target/res/counter_contract.wasm"
}

async fn init() -> anyhow::Result<(Account, Contract, Worker<impl DevNetwork>)> {
    let worker = sandbox().await?;
    let contract = worker.dev_deploy(&COUNTER).await?;

    // create accounts
    let owner = worker.root_account();
    let user = owner
        .create_subaccount(&worker, "user")
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await?
        .into_result()?;
    Ok((user, contract, worker))
}

#[tokio::test]
async fn test_increment() -> anyhow::Result<()> {
    let (user, contract, sandbox) = init().await?;
    let start_counter: u64 = user
        .call(&sandbox, contract.id(), "get_num")
        .transact()
        .await?
        .json()?;

    user.call(&sandbox, contract.id(), "increment")
        .transact()
        .await?;

    let end_counter: u64 = user
        .call(&sandbox, contract.id(), "get_num")
        .transact()
        .await?
        .json()?;

    assert_eq!(end_counter, start_counter + 1);
    Ok(())
}

#[tokio::test]
async fn test_decrement() -> anyhow::Result<()> {
    let (user, contract, sandbox) = init().await?;
    user.call(&sandbox, contract.id(), "increment")
        .transact()
        .await?;

    let start_counter: u64 = user
        .call(&sandbox, contract.id(), "get_num")
        .transact()
        .await?
        .json()?;

    user.call(&sandbox, contract.id(), "decrement")
        .transact()
        .await?;

    let end_counter: u64 = user
        .call(&sandbox, contract.id(), "get_num")
        .transact()
        .await?
        .json()?;

    assert_eq!(end_counter, start_counter - 1);
    Ok(())
}

#[tokio::test]
async fn test_reset() -> anyhow::Result<()> {
    let (user, contract, sandbox) = init().await?;
    user.call(&sandbox, contract.id(), "increment")
        .transact()
        .await?;

    user.call(&sandbox, contract.id(), "increment")
        .transact()
        .await?;

    user.call(&sandbox, contract.id(), "reset")
        .transact()
        .await?;

    let end_counter: u64 = user
        .call(&sandbox, contract.id(), "get_num")
        .transact()
        .await?
        .json()?;

    assert_eq!(end_counter, 0);
    Ok(())
}
