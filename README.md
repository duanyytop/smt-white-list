# smt-white-list

## Prerequisites

- [CoTA Syncer](https://github.com/nervina-labs/cota-nft-entries-syncer): The server to index CoTA data from CKB

> The smt-white-list and syncer share the same mysql database, and the smt-white-list use CoTA data from the database to filter lock hash and smt root

## Quick Start

### Manual

- Rename `.env.example` to `.env` 
  - Update the database connection string in `DATABASE_URL` key 
  - Update the ckb-indexer url string in `CKB_INDEXER`
- Build with release profile: `make build-release`
- Run with release profile: `make run-release`

### Release

```shell
RUST_LOG=info DATABASE_URL=mysql://root:password@localhost:3306/db_name CKB_INDEXER=http://localhost:8116 ./smt-white-list
```
