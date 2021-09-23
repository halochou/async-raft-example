use anyhow::Result;
use std::{env, sync::Arc};

mod api;
mod raft;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let node_id = env::var("NODE_ID").unwrap();
    let node_id: u64 = node_id.parse().unwrap();
    let raft = Arc::new(raft::new(node_id));
    // tokio::spawn(raft::monitor(raft.clone()));

    api::serve(node_id as u16, raft.clone()).await;

    Ok(())
}
