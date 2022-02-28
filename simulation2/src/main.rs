use serde_json::json;
// use workspaces::prelude::*;

// const STATUS_MSG_WASM_FILEPATH: &str = "./examples/res/status_message.wasm";
// use std::{collections::HashMap, convert::TryInto};
// use near_sdk::{json_types::Base58PublicKey, serde_json::json}; //, U128};
use near_units::{parse_gas, parse_near};


use workspaces::prelude::*;
// use workspaces::{Account, AccountId, Contract, Network, Worker};


// const MEME_WASM_FILEPATH: &str = "../../target/wasm32-unknown-unknown/release/meme.wasm";
const MUSEUM_WASM_FILEPATH: &str = "../target/wasm32-unknown-unknown/release/museum.wasm";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let worker = workspaces::sandbox();
    let owner = worker.root_account().id().clone();

    let contributor = worker.dev_create_account().await?.id().clone();

    // let wasm = std::fs::read(STATUS_MSG_WASM_FILEPATH)?;
    let wasm = std::fs::read(MUSEUM_WASM_FILEPATH)?;

    // Setup museum contract
    let contract = worker.dev_deploy(wasm).await?;

    // initialize museum

    println!("Init Museum");

    let outcome = contract
        .call(&worker, "new")
        .deposit(parse_near!("4 N"))
        .args_json(json!({
            "name": "meme museum",
            "owners": Vec::from([owner]),
        }))?
        .transact()
        .await?;

    println!("{:#?}", outcome);

    println!("Add contributor to museum");

    let res = contract
        .call(&worker, "add_contributor")
        .args_json(json!({
            "account": contributor,
        }))?
        .transact()
        .await?;

    println!("{:#?}\n", res);
    // res.assert_success();// contributor has been added

    // let result: String = contract
    //     .view(
    //         &worker,
    //         "get_status",
    //         json!({
    //             "account_id": contract.id(),
    //         })
    //         .to_string()
    //         .into_bytes(),
    //     )
    //     .await?
    //     .json()?;

    println!("Create MEME");

    let name = String::from("usain");
    let title = String::from("usain refrain");
    let data = String::from("https://9gag.com/gag/ayMDG8Y");
    let category: u8 = 0;

    let res = contract
        .call(&worker, "add_meme")
        .args_json(json!({
            "meme": name,
            "title": title,
            "data": data,
            "category": category,
        }))?
        .transact()
        .await?;

    println!("{:#?}\n", res);
    // res.assert_success();

    // Still need to test verify meme


    Ok(())
}


// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }
