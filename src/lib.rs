//! ROW/RPM Ledger - Append-only ledger with offline-first anchoring
//!
//! This crate provides immutable storage for all governance decisions
//! from the ALN Sovereign Stack, with Merkle proof generation and
//! multi-ledger anchoring to Organichain/Googolswarm.
//!
//! # Architecture
//!
//! ```text
//! sovereigntycore → ShardManager → MerkleTree → AnchorManager → External Ledgers
//! ```
//!
//! # Example
//!
//! ```rust
//! use row_rpm_ledger::{LedgerManager, RowShard, LedgerConfig};
//!
//! let config = LedgerConfig::default();
//! let mut ledger = LedgerManager::new(config);
//!
//! let shard = RowShard::new(/* ... */);
//! let shard_id = ledger.append_row(shard)?;
//!
//! // Generate Merkle proof for verification
//! let proof = ledger.generate_merkle_proof(&shard_id)?;
//!
//! // Bulk anchor to external ledgers
//! ledger.bulk_anchor(1000).await?;
//! ```

#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
#![allow(clippy::module_name_repetitions)]

pub mod shard;
pub mod merkle;
pub mod snapshot;
pub mod query;
pub mod anchor;
pub mod replica;
pub mod error;
pub mod types;
pub mod hex_stamp;

/// Crate version
pub const VERSION: &str = "1.0.0";

/// Hex-stamp attestation for this release
pub const HEX_STAMP: &str = "0x9c5f1e4d3c0b6a8f7e2d1c0b9a8f7e6d5c4b3a29f8e7d6c5b4a3928170f6e5d4";

/// Ledger reference for this release
pub const LEDGER_REF: &str = "row:row-rpm-ledger:v1.0.0:2026-03-04";

/// Re-export commonly used types
pub use shard::{RowShard, RpmShard, ShardType};
pub use types::{LedgerConfig, LedgerEntry, LedgerState};
pub use error::LedgerError;
pub use merkle::MerkleProof;

/// Ledger manager for all ROW/RPM operations
pub struct LedgerManager {
    config: LedgerConfig,
    state: LedgerState,
    merkle_tree: merkle::MerkleTree,
    anchor_queue: anchor::AnchorQueue,
}

impl LedgerManager {
    /// Create a new ledger manager
    pub fn new(config: LedgerConfig) -> Result<Self, LedgerError> {
        Ok(Self {
            config,
            state: LedgerState::new(),
            merkle_tree: merkle::MerkleTree::new(),
            anchor_queue: anchor::AnchorQueue::new(),
        })
    }

    /// Append a ROW shard to the ledger
    pub fn append_row(&mut self, shard: RowShard) -> Result<String, LedgerError> {
        let entry = LedgerEntry::Row(shard);
        let shard_id = entry.id().to_string();
        
        // Add to Merkle tree
        self.merkle_tree.add_leaf(entry.hash()?);
        
        // Add to anchor queue
        self.anchor_queue.queue(entry.clone());
        
        // Update state
        self.state.append(entry)?;
        
        Ok(shard_id)
    }

    /// Append an RPM shard to the ledger
    pub fn append_rpm(&mut self, shard: RpmShard) -> Result<String, LedgerError> {
        let entry = LedgerEntry::Rpm(shard);
        let shard_id = entry.id().to_string();
        
        // Add to Merkle tree
        self.merkle_tree.add_leaf(entry.hash()?);
        
        // Add to anchor queue
        self.anchor_queue.queue(entry.clone());
        
        // Update state
        self.state.append(entry)?;
        
        Ok(shard_id)
    }

    /// Generate Merkle proof for a shard
    pub fn generate_merkle_proof(&self, shard_id: &str) -> Result<MerkleProof, LedgerError> {
        self.merkle_tree.generate_proof(shard_id)
    }

    /// Bulk anchor queued shards to external ledgers
    #[cfg(feature = "full-anchoring")]
    pub async fn bulk_anchor(&mut self, batch_size: usize) -> Result<(), LedgerError> {
        self.anchor_queue.process_batch(batch_size).await
    }

    /// Query ledger entries
    pub fn query(&self, filter: query::LedgerFilter) -> Result<Vec<LedgerEntry>, LedgerError> {
        self.state.query(filter)
    }

    /// Create offline snapshot
    pub fn create_snapshot(&self) -> Result<snapshot::LedgerSnapshot, LedgerError> {
        snapshot::LedgerSnapshot::create(&self.state, &self.merkle_tree)
    }

    /// Verify ledger integrity
    pub fn verify_integrity(&self) -> Result<(), LedgerError> {
        self.merkle_tree.verify()?;
        self.state.verify()?;
        Ok(())
    }

    /// Get ledger statistics
    pub fn stats(&self) -> LedgerStats {
        LedgerStats {
            total_shards: self.state.count(),
            pending_anchors: self.anchor_queue.len(),
            merkle_root: self.merkle_tree.root_hash(),
        }
    }
}

/// Ledger statistics
#[derive(Debug, Clone)]
pub struct LedgerStats {
    pub total_shards: usize,
    pub pending_anchors: usize,
    pub merkle_root: String,
}

/// Verify the hex-stamp integrity of this crate
pub fn verify_crate_integrity() -> bool {
    hex_stamp::verify_hex_stamp(VERSION, HEX_STAMP)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crate_version() {
        assert_eq!(VERSION, "1.0.0");
    }

    #[test]
    fn test_hex_stamp_format() {
        assert!(HEX_STAMP.starts_with("0x"));
        assert_eq!(HEX_STAMP.len(), 66);
    }

    #[test]
    fn test_ledger_creation() {
        let config = LedgerConfig::default();
        let ledger = LedgerManager::new(config);
        assert!(ledger.is_ok());
    }
}
