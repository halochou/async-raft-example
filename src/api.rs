use crate::raft::MemRaft;
use async_raft::raft;
use axum::{
    extract,
    handler::{get, post},
    response::IntoResponse,
    Json, Router,
};

use std::{collections::HashSet, net::SocketAddr, sync::Arc};

async fn root() -> &'static str {
    "Hello, World!"
}

async fn append_entries(
    Json(request): Json<raft::AppendEntriesRequest<memstore::ClientRequest>>,
    raft: extract::Extension<Arc<MemRaft>>,
) -> impl IntoResponse {
    log::debug!("append called");
    let resp = raft.append_entries(request).await.unwrap();
    Json(resp)
}

async fn install_snapshot(
    Json(request): Json<raft::InstallSnapshotRequest>,
    raft: extract::Extension<Arc<MemRaft>>,
) -> impl IntoResponse {
    log::debug!("install called");
    let resp = raft.install_snapshot(request).await.unwrap();
    Json(resp)
}

async fn vote(
    Json(request): Json<raft::VoteRequest>,
    raft: extract::Extension<Arc<MemRaft>>,
) -> impl IntoResponse {
    log::debug!("vote called");
    let resp = raft.vote(request).await.unwrap();
    Json(resp)
}

async fn bootstrap(raft: extract::Extension<Arc<MemRaft>>) -> impl IntoResponse {
    log::debug!("bootstrap called");
    let mut members = HashSet::new();
    members.insert(0);
    members.insert(1);
    members.insert(2);
    raft.initialize(members).await.unwrap();

    "true"
}

async fn metrics(raft: extract::Extension<Arc<MemRaft>>) -> impl IntoResponse {
    let ch = raft.metrics();
    let metrics = ch.borrow();
    format!("{:?}", metrics)
}

pub async fn serve(node_id: u16, raft: Arc<MemRaft>) {
    let app = Router::new()
        .route("/append-entries", post(append_entries))
        .route("/install-snapshot", post(install_snapshot))
        .route("/vote", post(vote))
        .route("/bootstrap", post(bootstrap))
        .route("/metrics", get(metrics))
        .route("/", get(root))
        .layer(axum::AddExtensionLayer::new(raft));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 5000 + node_id));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
