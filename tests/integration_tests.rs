//! ROW/RPM Ledger Integration Tests

use row_rpm_ledger::{LedgerManager, LedgerConfig, RowShard, ResourceRequest, ResourceGrant, EcoVector};

#[test]
fn test_full_ledger_lifecycle() {
    let config = LedgerConfig::default();
    let mut ledger = LedgerManager::new(config).unwrap();

    // Create ROW shard
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

    shard.generate_hex_stamp().unwrap();

    // Append to ledger
    let shard_id = ledger.append_row(shard).unwrap();
    assert!(!shard_id.is_empty());

    // Verify integrity
    assert!(ledger.verify_integrity().is_ok());

    // Get stats
    let stats = ledger.stats();
    assert_eq!(stats.total_shards, 1);
}

#[test]
fn test_merkle_proof_generation() {
    let config = LedgerConfig::default();
    let mut ledger = LedgerManager::new(config).unwrap();

    // Append multiple shards
    for i in 0..5 {
        let mut shard = RowShard::new(
            format!("session-{}", i),
            "test".to_string(),
            ResourceRequest {
                cpu_cores: 1,
                memory_mb: 1024,
                network_bandwidth_mbps: 10.0,
                storage_gb: 10,
                swarm_nodes: 1,
                duration_seconds: 60,
            },
            ResourceGrant {
                cpu_cores: 1,
                memory_mb: 1024,
                network_bandwidth_mbps: 10.0,
                storage_gb: 10,
                swarm_nodes: 1,
                duration_seconds: 60,
                quota_remaining_pct: 1.0,
            },
            EcoVector {
                gco2_per_joule: 0.001,
                eco_impact_score: 0.5,
                energy_autonomy_pct: 0.8,
                eco_floor_minimum: 0.3,
            },
            "Normal".to_string(),
            "bostrom1test".to_string(),
            "bostrom1test".to_string(),
            format!("cyb:trace:{}", i),
        );

        shard.generate_hex_stamp().unwrap();
        ledger.append_row(shard).unwrap();
    }

    // Generate proof for first shard
    let stats = ledger.stats();
    let proof = ledger.generate_merkle_proof(&stats.merkle_root).unwrap();
    assert!(!proof.root_hash.is_empty());
}

#[test]
fn test_snapshot_creation_and_verification() {
    use row_rpm_ledger::snapshot::LedgerSnapshot;
    
    let config = LedgerConfig::default();
    let ledger = LedgerManager::new(config).unwrap();

    let snapshot = ledger.create_snapshot().unwrap();
    assert!(snapshot.verify().is_ok());
}
