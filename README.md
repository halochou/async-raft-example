# async-raft-example

A minimal example application with rust [async-raft](https://github.com/async-raft/async-raft)

- RaftStorage: async-raft's memstore
- RaftNetwork: axum/reqwest based HTTP transport
- Peer address hardcoded as 127.0.0.1:5000, 127.0.0.1:5001 and 127.0.0.1:5002

To try it, open 3 terminal windows and run:

```
# window 1
NODE_ID=0 ./async-raft-example
# window 2
NODE_ID=1 ./async-raft-example
# window 3
NODE_ID=2 ./async-raft-example

```
