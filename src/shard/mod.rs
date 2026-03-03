//! ROW/RPM Shard Definitions - Immutable ledger entry structures
//!
//! This module defines the core shard types for Resource Ownership
//! and Resource Performance records, with hex-stamp attestation.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::hex_stamp;
use crate::error::LedgerError;

/// Shard type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ShardType {
    Row,
    Rpm,
}

/// Resource Ownership Record (ROW) Shard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RowShard {
    /// Unique shard identifier
    pub row_id: String,
    /// Timestamp of creation
    pub timestamp: i64,
    /// Session identifier
    pub session_id: String,
    /// Workload type description
    pub workload_type: String,
    /// Requested resources
    pub requested_resources: ResourceRequest,
    /// Granted resources
    pub granted_resources: ResourceGrant,
    /// EcoVector metrics
    pub eco_vector: EcoVector,
    /// NDM state at time of request
    pub ndm_state: String,
    /// Requester DID
    pub did_requester: String,
    /// Granter DID
    pub did_granter: String,
    /// Cyberspectre trace ID
    pub cyberspectre_trace_id: String,
    /// Hex-stamp attestation
    pub hex_stamp: String,
    /// Ledger anchor reference
    pub ledger_anchor: LedgerAnchor,
    /// Previous ROW ID (chain linkage)
    pub previous_row_id: Option<String>,
}

impl RowShard {
    /// Create a new ROW shard
    pub fn new(
        session_id: String,
        workload_type: String,
        requested: ResourceRequest,
        granted: ResourceGrant,
        eco_vector: EcoVector,
        ndm_state: String,
        did_requester: String,
        did_granter: String,
        cyberspectre_trace_id: String,
    ) -> Self {
        let row_id = Uuid::new_v4().to_string();
        let timestamp = Utc::now().timestamp();
        
        Self {
            row_id,
            timestamp,
            session_id,
            workload_type,
            requested_resources: requested,
            granted_resources: granted,
            eco_vector,
            ndm_state,
            did_requester,
            did_granter,
            cyberspectre_trace_id,
            hex_stamp: String::new(),
            ledger_anchor: LedgerAnchor::default(),
            previous_row_id: None,
        }
    }

    /// Generate hex-stamp for this shard
    pub fn generate_hex_stamp(&mut self) -> Result<(), LedgerError> {
        self.hex_stamp = hex_stamp::generate_hex_stamp(self);
        Ok(())
    }

    /// Verify hex-stamp integrity
    pub fn verify_hex_stamp(&self) -> Result<(), LedgerError> {
        if hex_stamp::verify_hex_stamp(self, &self.hex_stamp) {
            Ok(())
        } else {
            Err(LedgerError::HexStampVerificationFailed)
        }
    }

    /// Get shard hash for Merkle tree
    pub fn hash(&self) -> Result<Vec<u8>, LedgerError> {
        let serialized = serde_json::to_vec(self)?;
        Ok(sha3::Sha3_256::digest(&serialized).to_vec())
    }
}

/// Resource Performance Metric (RPM) Shard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpmShard {
    /// Unique shard identifier
    pub rpm_id: String,
    /// Timestamp of creation
    pub timestamp: i64,
    /// Session identifier
    pub session_id: String,
    /// Performance metrics
    pub performance_metrics: PerformanceMetrics,
    /// Eco impact metrics
    pub eco_impact: EcoImpact,
    /// NDM delta
    pub ndm_delta: f64,
    /// RoH delta
    pub roh_delta: f64,
    /// Cyberspectre trace ID
    pub cyberspectre_trace_id: String,
    /// Hex-stamp attestation
    pub hex_stamp: String,
    /// Ledger anchor reference
    pub ledger_anchor: LedgerAnchor,
    /// Related ROW ID
    pub related_row_id: Option<String>,
}

impl RpmShard {
    /// Create a new RPM shard
    pub fn new(
        session_id: String,
        performance: PerformanceMetrics,
        eco_impact: EcoImpact,
        ndm_delta: f64,
        roh_delta: f64,
        cyberspectre_trace_id: String,
    ) -> Self {
        let rpm_id = Uuid::new_v4().to_string();
        let timestamp = Utc::now().timestamp();
        
        Self {
            rpm_id,
            timestamp,
            session_id,
            performance_metrics: performance,
            eco_impact,
            ndm_delta,
            roh_delta,
            cyberspectre_trace_id,
            hex_stamp: String::new(),
            ledger_anchor: LedgerAnchor::default(),
            related_row_id: None,
        }
    }

    /// Generate hex-stamp for this shard
    pub fn generate_hex_stamp(&mut self) -> Result<(), LedgerError> {
        self.hex_stamp = hex_stamp::generate_hex_stamp(self);
        Ok(())
    }

    /// Verify hex-stamp integrity
    pub fn verify_hex_stamp(&self) -> Result<(), LedgerError> {
        if hex_stamp::verify_hex_stamp(self, &self.hex_stamp) {
            Ok(())
        } else {
            Err(LedgerError::HexStampVerificationFailed)
        }
    }

    /// Get shard hash for Merkle tree
    pub fn hash(&self) -> Result<Vec<u8>, LedgerError> {
        let serialized = serde_json::to_vec(self)?;
        Ok(sha3::Sha3_256::digest(&serialized).to_vec())
    }
}

/// Resource Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequest {
    pub cpu_cores: u32,
    pub memory_mb: u64,
    pub network_bandwidth_mbps: f64,
    pub storage_gb: u64,
    pub swarm_nodes: u32,
    pub duration_seconds: i64,
}

/// Resource Grant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceGrant {
    pub cpu_cores: u32,
    pub memory_mb: u64,
    pub network_bandwidth_mbps: f64,
    pub storage_gb: u64,
    pub swarm_nodes: u32,
    pub duration_seconds: i64,
    pub quota_remaining_pct: f64,
}

/// EcoVector metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcoVector {
    pub gco2_per_joule: f64,
    pub eco_impact_score: f64,
    pub energy_autonomy_pct: f64,
    pub eco_floor_minimum: f64,
}

/// Performance Metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub cpu_utilization_pct: f64,
    pub memory_utilization_pct: f64,
    pub network_throughput_mbps: f64,
    pub task_completion_rate: f64,
    pub latency_avg_ms: f64,
}

/// Eco Impact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcoImpact {
    pub total_gco2: f64,
    pub total_joules: f64,
    pub eco_efficiency_score: f64,
}

/// Ledger Anchor Reference
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LedgerAnchor {
    pub ledger_type: String,
    pub transaction_id: String,
    pub block_height: u64,
    pub merkle_proof: String,
    pub anchor_timestamp: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_shard_creation() {
        let mut shard = RowShard::new(
            "session-123".to_string(),
            "nanoswarm_mission".to_string(),
            ResourceRequest {
                cpu_cores: 4,
                memory_mb: 8192,
                network_bandwidth_mbps: 100.0,
                storage_gb: 100,
                swarm_nodes: 10,
                duration_seconds: 3600,
            },
            ResourceGrant {
                cpu_cores: 4,
                memory_mb: 8192,
                network_bandwidth_mbps: 100.0,
                storage_gb: 100,
                swarm_nodes: 10,
                duration_seconds: 3600,
                quota_remaining_pct: 0.8,
            },
            EcoVector {
                gco2_per_joule: 0.001,
                eco_impact_score: 0.5,
                energy_autonomy_pct: 0.8,
                eco_floor_minimum: 0.3,
            },
            "Normal".to_string(),
            "bostrom1requester".to_string(),
            "bostrom1granter".to_string(),
            "cyb:trace:456".to_string(),
        );

        assert!(!shard.row_id.is_empty());
        assert!(shard.generate_hex_stamp().is_ok());
        assert!(shard.verify_hex_stamp().is_ok());
    }
}
