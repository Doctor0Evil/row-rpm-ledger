//! Query Engine - Ledger query with NDM-aware filtering
//!
//! This module provides query capabilities for ledger entries
//! with support for NDM state filtering and time-range queries.

use serde::{Deserialize, Serialize};
use crate::types::LedgerEntry;
use crate::error::LedgerError;

/// Ledger filter for queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerFilter {
    /// Filter by shard type
    pub shard_type: Option<ShardTypeFilter>,
    /// Filter by time range
    pub time_range: Option<TimeRange>,
    /// Filter by NDM state
    pub ndm_state: Option<String>,
    /// Filter by DID
    pub did: Option<String>,
    /// Filter by session ID
    pub session_id: Option<String>,
    /// Limit results
    pub limit: Option<usize>,
    /// Offset for pagination
    pub offset: Option<usize>,
}

/// Shard type filter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShardTypeFilter {
    Row,
    Rpm,
    Both,
}

/// Time range filter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    pub start: i64,
    pub end: i64,
}

impl LedgerFilter {
    /// Create a new filter with defaults
    pub fn new() -> Self {
        Self {
            shard_type: None,
            time_range: None,
            ndm_state: None,
            did: None,
            session_id: None,
            limit: Some(100),
            offset: Some(0),
        }
    }

    /// Filter by shard type
    pub fn with_shard_type(mut self, shard_type: ShardTypeFilter) -> Self {
        self.shard_type = Some(shard_type);
        self
    }

    /// Filter by time range
    pub fn with_time_range(mut self, start: i64, end: i64) -> Self {
        self.time_range = Some(TimeRange { start, end });
        self
    }

    /// Filter by NDM state
    pub fn with_ndm_state(mut self, state: String) -> Self {
        self.ndm_state = Some(state);
        self
    }

    /// Filter by DID
    pub fn with_did(mut self, did: String) -> Self {
        self.did = Some(did);
        self
    }

    /// Apply filter to entries
    pub fn apply(&self, entries: &[LedgerEntry]) -> Result<Vec<LedgerEntry>, LedgerError> {
        let mut filtered: Vec<LedgerEntry> = entries.to_vec();

        // Filter by shard type
        if let Some(ref shard_type) = self.shard_type {
            filtered = filtered
                .into_iter()
                .filter(|e| match shard_type {
                    ShardTypeFilter::Row => matches!(e, LedgerEntry::Row(_)),
                    ShardTypeFilter::Rpm => matches!(e, LedgerEntry::Rpm(_)),
                    ShardTypeFilter::Both => true,
                })
                .collect();
        }

        // Filter by time range
        if let Some(ref time_range) = self.time_range {
            filtered = filtered
                .into_iter()
                .filter(|e| {
                    let timestamp = e.timestamp();
                    timestamp >= time_range.start && timestamp <= time_range.end
                })
                .collect();
        }

        // Filter by NDM state
        if let Some(ref ndm_state) = self.ndm_state {
            filtered = filtered
                .into_iter()
                .filter(|e| e.ndm_state() == Some(ndm_state.as_str()))
                .collect();
        }

        // Filter by DID
        if let Some(ref did) = self.did {
            filtered = filtered
                .into_iter()
                .filter(|e| e.did_requester() == Some(did.as_str()))
                .collect();
        }

        // Apply limit and offset
        let offset = self.offset.unwrap_or(0);
        let limit = self.limit.unwrap_or(filtered.len());
        filtered = filtered.into_iter().skip(offset).take(limit).collect();

        Ok(filtered)
    }
}

impl Default for LedgerFilter {
    fn default() -> Self {
        Self::new()
    }
}

/// Query result with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    pub entries: Vec<LedgerEntry>,
    pub total_count: usize,
    pub returned_count: usize,
    pub has_more: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_creation() {
        let filter = LedgerFilter::new();
        assert!(filter.limit.is_some());
        assert!(filter.offset.is_some());
    }
}
