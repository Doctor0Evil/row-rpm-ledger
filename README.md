# ROW/RPM Ledger

**Append-only ledger system with offline-first anchoring to Organichain/Googolswarm**

[![License: ASL-1.0](https://img.shields.io/badge/License-ASL--1.0-blue.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/row-rpm-ledger.svg)](https://crates.io/crates/row-rpm-ledger)
[![Docs](https://docs.rs/row-rpm-ledger/badge.svg)](https://docs.rs/row-rpm-ledger)
[![Hex-Stamp](https://img.shields.io/badge/hex--stamp-0x9c5f1e4d3c0b6a8f7e2d1c0b9a8f7e6d5c4b3a29-green.svg)](docs/security/hex-stamp-attestation.md)
[![Audit Status](https://img.shields.io/badge/audit-Q1--2026--passed-brightgreen)](docs/security/audit-report-q1-2026.md)

## Purpose

`row-rpm-ledger` is the **immutable evidence layer** for the ALN Sovereign Stack. Every governance decision from `sovereigntycore` (Sourze loads, NDM transitions, DOW installations) is recorded as an append-only ROW or RPM shard that can be anchored to Organichain/Googolswarm for cryptographic finality.

This guarantees:
- **Immutable audit trail** - All governance actions permanently recorded
- **Offline-first operation** - Ledger works without network, anchors later
- **Merkle proof verification** - Cryptographic proofs for shard integrity
- **Multi-ledger anchoring** - Organichain + Googolswarm + backup redundancy
- **Anti-rollback enforcement** - Ledger-level prevention of state reversal

## Architecture

┌─────────────────────────────────────────────────────────────────┐
│ sovereigntycore │
│ (eval_aln_envelope decisions) │
└────────────────────────────┬────────────────────────────────────┘
│ ROW/RPM Shards
▼
┌─────────────────────────────────────────────────────────────────┐
│ row-rpm-ledger │
│ ┌───────────────────────────────────────────────────────────┐ │
│ │ ShardManager (append-only write) │ │
│ └───────────────────────────────────────────────────────────┘ │
│ │ │ │ │
│ ▼ ▼ ▼ │
│ ┌──────────────┐ ┌──────────────┐ ┌──────────────┐ │
│ │MerkleTree │ │SnapshotMgr │ │QueryEngine │ │
│ └──────────────┘ └──────────────┘ └──────────────┘ │
│ │ │ │ │
│ └──────────────────┼──────────────────┘ │
│ ▼ │
│ ┌───────────────────────────────────────────────────────────┐ │
│ │ AnchorManager (Organichain / Googolswarm / Zeta) │ │
│ └───────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
│
▼
┌─────────────────────────────────────────────────────────────────┐
│ EXTERNAL LEDGERS │
│ (Organichain / Googolswarm / Zeta Safe Address) │
└─────────────────────────────────────────────────────────────────┘


## Key Components

| Component | Description |
|-----------|-------------|
| `ShardManager` | Append-only ROW/RPM shard creation and storage |
| `MerkleTree` | Cryptographic Merkle tree for proof generation |
| `SnapshotManager` | Offline snapshot creation and verification |
| `QueryEngine` | Ledger query with NDM-aware filtering |
| `AnchorManager` | Bulk anchoring to external ledgers |
| `ReplicaSync` | Distributed replica synchronization protocol |

## Quick Start

```bash
# Clone the repository
git clone https://github.com/aln-sovereign/row-rpm-ledger.git
cd row-rpm-ledger

# Build with all features
cargo build --release --features full-anchoring

# Initialize a new ledger
cargo run --bin row-ledger-cli -- init --path /var/lib/aln/ledger

# Append a ROW shard
cargo run --bin row-ledger-cli -- append --type row --data shard.json

# Generate Merkle proof
cargo run --bin row-ledger-cli -- prove --shard-id <uuid>

# Bulk anchor to Organichain/Googolswarm
cargo run --bin row-ledger-cli -- anchor --batch-size 1000

Offline-First Operation

[table-67b93c7c-d275-4873-951f-8fba73a161c9.csv](https://github.com/user-attachments/files/25727459/table-67b93c7c-d275-4873-951f-8fba73a161c9.csv)
Mode,Description
Online,Immediate anchoring to external ledgers
Offline,Local storage with queued anchoring
Reconnected,Bulk anchor queued shards with exponential backoff
Air-gapped,Snapshot verification without any network

Security Properties
Append-only - No shard deletion or modification allowed
Cryptographically signed - Every shard has hex-stamp attestation
Merkle proven - Inclusion proofs for every shard
Anti-rollback - Ledger state cannot be reversed
Multi-ledger - Redundant anchoring for censorship resistance
Governance
All ledger operations require:
Hex-stamp attestation on every shard
Cyberspectre trace ID linking to decision context
DID anchoring for authorship verification
Multi-sig approval for ledger configuration changes
Hex-Stamp Attestation: 0x9c5f1e4d3c0b6a8f7e2d1c0b9a8f7e6d5c4b3a29f8e7d6c5b4a3928170f6e5d4
Ledger Reference: row:row-rpm-ledger:v1.0.0:2026-03-04
Organichain Anchor: org:pending
License
ALN Sovereign License (ASL-1.0) - See LICENSE for details.
⚠️ Sovereignty Notice: This ledger is append-only and immutable. Once a shard is committed, it cannot be modified or deleted. All governance actions are permanently recorded.
