//! Snapshot Manager - Offline snapshot creation and verification
//!
//! This module enables offline-first operation by creating verifiable
//! snapshots of ledger state that can be used without network access.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::types::LedgerState;
use crate::merkle::MerkleTree;
use crate::error::LedgerError;
use crate::hex_stamp;

/// Ledger snapshot for offline verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerSnapshot {
    /// Snapshot identifier
    pub snapshot_id: String,
    /// Creation timestamp
    pub timestamp: i64,
    /// Merkle root at snapshot time
    pub merkle_root: String,
    /// Total shard count
    pub shard_count: usize,
    /// Snapshot hash
    pub snapshot_hash: String,
    /// Hex-stamp attestation
    pub hex_stamp: String,
    /// Snapshot data (compressed)
    pub data: Vec<u8>,
}

impl LedgerSnapshot {
    /// Create a new ledger snapshot
    pub fn create(state: &LedgerState, merkle_tree: &MerkleTree) -> Result<Self, LedgerError> {
        let snapshot_id = uuid::Uuid::new_v4().to_string();
        let timestamp = Utc::now().timestamp();
        let merkle_root = merkle_tree.root_hash();
        let shard_count = state.count();
        
        // Serialize snapshot data
        let data = bincode::serialize(state)?;
        
        let mut snapshot = Self {
            snapshot_id,
            timestamp,
            merkle_root,
            shard_count,
            snapshot_hash: String::new(),
            hex_stamp: String::new(),
            data,
        };

        // Generate snapshot hash
        snapshot.snapshot_hash = snapshot.compute_hash()?;
        
        // Generate hex-stamp
        snapshot.hex_stamp = hex_stamp::generate_hex_stamp(&snapshot);
        
        Ok(snapshot)
    }

    /// Compute snapshot hash
    pub fn compute_hash(&self) -> Result<String, LedgerError> {
        let mut hasher = Sha3_256::new();
        hasher.update(&self.data);
        Ok(format!("0x{}", hex::encode(hasher.finalize())))
    }

    /// Verify snapshot integrity
    pub fn verify(&self) -> Result<(), LedgerError> {
        // Verify hex-stamp
        if !hex_stamp::verify_hex_stamp(self, &self.hex_stamp) {
            return Err(LedgerError::HexStampVerificationFailed);
        }

        // Verify snapshot hash
        let computed_hash = self.compute_hash()?;
        if computed_hash != self.snapshot_hash {
            return Err(LedgerError::SnapshotHashMismatch);
        }

        Ok(())
    }

    /// Deserialize snapshot data
    pub fn deserialize_state(&self) -> Result<LedgerState, LedgerError> {
        Ok(bincode::deserialize(&self.data)?)
    }

    /// Save snapshot to file
    pub fn save_to_file(&self, path: &str) -> Result<(), LedgerError> {
        use std::fs::File;
        use std::io::Write;
        
        let mut file = File::create(path)?;
        let serialized = serde_json::to_vec(self)?;
        file.write_all(&serialized)?;
        
        Ok(())
    }

    /// Load snapshot from file
    pub fn load_from_file(path: &str) -> Result<Self, LedgerError> {
        use std::fs::File;
        use std::io::Read;
        
        let mut file = File::open(path)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;
        
        Ok(serde_json::from_slice(&contents)?)
    }
}

/// Snapshot taxonomy for different use cases
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SnapshotType {
    /// Full ledger snapshot (all shards)
    Full,
    /// Recent shards only (for quick sync)
    Recent { limit: usize },
    /// NDM-relevant shards only
    NdmRelevant,
    /// Eco metrics only
    EcoMetrics,
}

/// Snapshot retention policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    /// Maximum number of snapshots to retain
    pub max_snapshots: usize,
    /// Maximum age in days
    pub max_age_days: u32,
    /// Minimum snapshots to keep regardless of age
    pub min_keep: usize,
}

impl Default for RetentionPolicy {
    fn default() -> Self {
        Self {
            max_snapshots: 100,
            max_age_days: 365,
            min_keep: 10,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snapshot_creation() {
        let state = LedgerState::new();
        let tree = MerkleTree::new();
        
        let snapshot = LedgerSnapshot::create(&state, &tree);
        assert!(snapshot.is_ok());
    }
}
