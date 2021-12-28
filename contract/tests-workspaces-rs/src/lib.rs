#![cfg(test)]

use workspaces::prelude::*;
use workspaces::DevNetwork;


const COUNTER_WASM_FILEPATH: &str = "./compiled-files/main.wasm";

async fn setup() -> anyhow::Result<(workspaces::Worker<impl DevNetwork>, workspaces::Contract)> {
    let worker = workspaces::sandbox();
    let wasm = std::fs::read(COUNTER_WASM_FILEPATH)?;
    let contract = worker.dev_deploy(wasm).await?;
    Ok((worker, contract))
}

#[tokio::test]
async fn increment() -> anyhow::Result<()> {
    let (worker, contract) = setup().await?;

    let status = contract
        .call(&worker, "increment")
        .transact()
        .await?;
    println!("increment result status: {:?}", status);
    let result: i8 = contract
        .view(
            &worker,
            "get_num",
            "".to_string()
            .into_bytes(),
        )
    .await?
    .json()?;

    assert_eq!(result, 1);
    Ok(())
}

#[tokio::test]
async fn decrement() -> anyhow::Result<()> {
    let (worker, contract) = setup().await?;

    let status = contract
        .call(&worker, "decrement")
        .transact()
        .await?;
    println!("decrement result status: {:?}", status);
    let result: i8 = contract
        .view(
            &worker,
            "get_num",
            "".to_string()
            .into_bytes(),
        )
    .await?
    .json()?;

    assert_eq!(result, -1);
    Ok(())
}

#[tokio::test]
async fn increment_and_reset() -> anyhow::Result<()> {
    let (worker, contract) = setup().await?;

    let status1 = contract
        .call(&worker, "increment")
        .transact()
        .await?;
    println!("increment_and_reset result status1: {:?}", status1);
    let status2 = contract
        .call(&worker, "reset")
        .transact()
        .await?;
    println!("increment_and_reset result status2: {:?}", status2);
    let result: i8 = contract
        .view(
            &worker,
            "get_num",
            "".to_string()
            .into_bytes(),
        )
    .await?
    .json()?;
    
    assert_eq!(result, 0);
    Ok(())
}
