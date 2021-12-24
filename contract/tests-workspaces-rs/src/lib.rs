#![cfg(test)]

use serde_json::json;
use workspaces::prelude::*;

const COUNTER_WASM_FILEPATH: &str = "../../out/main.wasm";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let worker = workspaces::sandbox();
    let wasm = std::fs::read(COUNTER_WASM_FILEPATH)?;
    let contract = worker.dev_deploy(wasm).await.unwrap();
    Ok(())
}
