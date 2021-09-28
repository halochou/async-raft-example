use std::sync::Arc;

use anyhow::Result;
use async_raft::{raft, Config, NodeId, RaftNetwork};
use async_trait::async_trait;
use memstore::{ClientRequest, ClientResponse, MemStore};

const NODES: [&str; 3] = ["127.0.0.1:5000", "127.0.0.1:5001", "127.0.0.1:5002"];

pub type MemRaft = async_raft::Raft<ClientRequest, ClientResponse, RaftRouter, MemStore>;

pub struct RaftRouter {
    client: reqwest::Client
}

impl RaftRouter {
    pub fn new() -> Self {
        RaftRouter {
            client: reqwest::Client::new()
        }
    }
}

#[async_trait]
impl RaftNetwork<ClientRequest> for RaftRouter {
    async fn append_entries(
        &self,
        target: NodeId,
        rpc: raft::AppendEntriesRequest<ClientRequest>,
    ) -> Result<raft::AppendEntriesResponse> {
        let addr = NODES.get(target as usize).unwrap();
        let url = format!("http://{}/append-entries", addr);
        let resp = self.client
            .post(url)
            .json(&rpc)
            .send()
            .await?
            .json::<raft::AppendEntriesResponse>()
            .await?;
        Ok(resp)
    }

    async fn install_snapshot(
        &self,
        target: NodeId,
        rpc: raft::InstallSnapshotRequest,
    ) -> Result<raft::InstallSnapshotResponse> {
        let addr = NODES.get(target as usize).unwrap();
        let url = format!("http://{}/install-snapshot", addr);
        let resp = self.client
            .post(url)
            .json(&rpc)
            .send()
            .await?
            .json::<raft::InstallSnapshotResponse>()
            .await?;
        Ok(resp)
    }

    async fn vote(&self, target: NodeId, rpc: raft::VoteRequest) -> Result<raft::VoteResponse> {
        let addr = NODES.get(target as usize).unwrap();
        let url = format!("http://{}/vote", addr);
        let resp = self.client
            .post(url)
            .json(&rpc)
            .send()
            .await?
            .json::<raft::VoteResponse>()
            .await?;
        Ok(resp)
    }
}

pub fn new(node_id: u64) -> MemRaft {
    let config = Arc::new(
        Config::build("primary-raft-group".into())
            .validate()
            .expect("failed to build Raft config"),
    );
    let network = Arc::new(RaftRouter::new());
    let storage = Arc::new(MemStore::new(node_id));
    raft::Raft::new(node_id, config, network, storage)
}
