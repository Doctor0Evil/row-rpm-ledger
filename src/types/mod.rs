//! Ledger Types and State Management
//!
//! This module defines core ledger types and state management
//! for append-only ledger operations.

use serde::{Deserialize, Serialize};
use crate::shard::{RowShard, RpmShard};
use crate::error::LedgerError;
use uuid::Uuid;

/// Ledger entry (ROW or RPM)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LedgerEntry {
    Row(RowShard),
    Rpm(RpmShard),
}

impl LedgerEntry {
    /// Get entry ID
    pub fn id(&self) -> &str {
        match self {
            LedgerEntry::Row(s) => &s.row_id,
            LedgerEntry::Rpm(s) => &s.rpm_id,
        }
    }

    /// Get entry timestamp
    pub fn timestamp(&self) -> i64 {
        match self {
            LedgerEntry::Row(s) => s.timestamp,
            LedgerEntry::Rpm(s) => s.timestamp,
        }
    }

    /// Get NDM state
    pub fn ndm_state(&self) -> Option<&str> {
        match self {
            LedgerEntry::Row(s) => Some(&s.ndm_state),
            LedgerEntry::Rpm(_) => None,
        }
    }

    /// Get requester DID
    pub fn did_requester(&self) -> Option<&str> {
        match self {
            LedgerEntry::Row(s) => Some(&s.did_requester),
            LedgerEntry::Rpm(_) => None,
        }
    }

    /// Compute entry hash
    pub fn hash(&self) -> Result<Vec<u8>, LedgerError> {
        match self {
            LedgerEntry::Row(s) => s.hash(),
            LedgerEntry::Rpm(s) => s.hash(),
        }
    }
}

/// Ledger configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerConfig {
    pub path: String,
    pub max_entries: usize,
    pub enable_merkle: bool,
    pub enable_anchoring: bool,
}

impl Default for LedgerConfig {
    fn default() -> Self {
        Self {
            path: "/var/lib/aln/ledger".to_string(),
            max_entries: 10_000_000,
            enable_merkle: true,
            enable_anchoring: true,
        }
    }
}

/// Ledger state (in-memory representation)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerState {
    entries: Vec<LedgerEntry>,
    entry_index: std::collections::HashMap<String, usize>,
    count: usize,
}

impl LedgerState {
    /// Create new ledger state
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            entry_index: std::collections::HashMap::new(),
            count: 0,
        }
    }

    /// Append an entry
    pub fn append(&mut self, entry: LedgerEntry) -> Result<(), LedgerError> {
        let id = entry.id().to_string();
        
        if self.entry_index.contains_key(&id) {
            return Err(LedgerError::DuplicateEntry);
        }

        self.entry_index.insert(id, self.entries.len());
        self.entries.push(entry);
        self.count += 1;

        Ok(())
    }

    /// Query entries
    pub fn query(&self, filter: crate::query::LedgerFilter) -> Result<Vec<LedgerEntry>, LedgerError> {
        filter.apply(&self.entries)
    }

    /// Get entry count
    pub fn count(&self) -> usize {
        self.count
    }

    /// Verify state integrity
    pub fn verify(&self) -> Result<(), LedgerError> {
        // Verify index consistency
        for (i, entry) in self.entries.iter().enumerate() {
            let indexed = self.entry_index.get(entry.id());
            if indexed != Some(&i) {
                return Err(LedgerError::IndexInconsistency);
            }
        }
        Ok(())
    }
}

impl Default for LedgerState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ledger_state() {
        let mut state = LedgerState::new();
        assert_eq!(state.count(), 0);
    }
}
