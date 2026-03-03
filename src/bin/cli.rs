//! ROW/RPM Ledger CLI - Command-line interface for ledger operations

use row_rpm_ledger::{LedgerManager, LedgerConfig, LedgerFilter, ShardTypeFilter};
use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "row-ledger-cli")]
#[command(about = "ROW/RPM Ledger CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Ledger path
    #[arg(short, long, default_value = "/var/lib/aln/ledger")]
    path: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new ledger
    Init,
    /// Append a ROW shard
    Append {
        /// Shard type (row or rpm)
        #[arg(short, long)]
        r#type: String,
        /// Shard data file
        #[arg(short, long)]
        data: String,
    },
    /// Query ledger entries
    Query {
        /// Filter by shard type
        #[arg(short, long)]
        r#type: Option<String>,
        /// Limit results
        #[arg(short, long, default_value = "100")]
        limit: usize,
    },
    /// Generate Merkle proof
    Prove {
        /// Shard ID
        #[arg(short, long)]
        shard_id: String,
    },
    /// Bulk anchor to external ledgers
    Anchor {
        /// Batch size
        #[arg(short, long, default_value = "1000")]
        batch_size: usize,
    },
    /// Create snapshot
    Snapshot {
        /// Output path
        #[arg(short, long)]
        output: String,
    },
    /// Verify ledger integrity
    Verify,
    /// Show ledger statistics
    Stats,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let cli = Cli::parse();

    let config = LedgerConfig {
        path: cli.path.clone(),
        ..Default::default()
    };

    let mut ledger = LedgerManager::new(config)?;

    match cli.command {
        Commands::Init => {
            println!("Initializing ledger at {}", cli.path);
            println!("Ledger initialized successfully");
        }
        Commands::Append { r#type, data } => {
            println!("Appending {} shard from {}", r#type, data);
            // Implementation would read and append shard
        }
        Commands::Query { r#type, limit } => {
            let mut filter = LedgerFilter::new().with_limit(limit);
            if let Some(t) = r#type {
                filter = filter.with_shard_type(match t.as_str() {
                    "row" => ShardTypeFilter::Row,
                    "rpm" => ShardTypeFilter::Rpm,
                    _ => ShardTypeFilter::Both,
                });
            }
            let results = ledger.query(filter)?;
            println!("Found {} entries", results.len());
        }
        Commands::Prove { shard_id } => {
            let proof = ledger.generate_merkle_proof(&shard_id)?;
            println!("Merkle proof generated for {}", shard_id);
            println!("Root: {}", hex::encode(&proof.root_hash));
        }
        Commands::Anchor { batch_size } => {
            println!("Anchoring batch of {} entries", batch_size);
            ledger.bulk_anchor(batch_size).await?;
            println!("Anchor complete");
        }
        Commands::Snapshot { output } => {
            let snapshot = ledger.create_snapshot()?;
            snapshot.save_to_file(&output)?;
            println!("Snapshot saved to {}", output);
        }
        Commands::Verify => {
            ledger.verify_integrity()?;
            println!("Ledger integrity verified");
        }
        Commands::Stats => {
            let stats = ledger.stats();
            println!("Total shards: {}", stats.total_shards);
            println!("Pending anchors: {}", stats.pending_anchors);
            println!("Merkle root: {}", stats.merkle_root);
        }
    }

    Ok(())
}
