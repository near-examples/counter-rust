#![cfg(test)]

use serde_json::json;
use workspaces::prelude::*;
use workspaces::{Worker, DevNetwork, Contract};

const COUNTER_WASM_FILEPATH: &str = "../../out/main.wasm";

async fn setup() -> anyhow::Result<(workspaces::Worker<impl DevNetwork>, workspaces::Contract)> {
    let worker = workspaces::sandbox();
    let wasm = std::fs::read(COUNTER_WASM_FILEPATH)?;
    let contract = worker.dev_deploy(wasm).await?;
    Ok((worker, contract))
}

#[tokio::test]
async fn increment() -> anyhow::Result<()> {
    println!("Starting increment test");
    let (worker, contract) = setup().await?;

    let status = contract
        .call(&worker, "increment")
        .transact()
        .await?;
    println!("status: {:?}", status);
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
    println!("Starting decrement test");
    let (worker, contract) = setup().await?;

    let status = contract
        .call(&worker, "decrement")
        .transact()
        .await?;
    println!("status: {:?}", status);
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
    println!("Starting increment_and_reset test");
    let (worker, contract) = setup().await?;

    let status = contract
        .call(&worker, "increment")
        .transact()
        .await?;
    println!("status: {:?}", status);
    let status = contract
        .call(&worker, "reset")
        .transact()
        .await?;
    println!("status: {:?}", status);
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