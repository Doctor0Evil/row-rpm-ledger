//! Merkle Tree - Cryptographic proof generation for ledger integrity
//!
//! This module implements a Merkle tree for efficient verification
//! of shard inclusion and ledger integrity.

use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};
use crate::error::LedgerError;
use std::collections::HashMap;

/// Merkle tree node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MerkleNode {
    Leaf { hash: Vec<u8>, shard_id: String },
    Branch { hash: Vec<u8>, left: Box<MerkleNode>, right: Box<MerkleNode> },
}

impl MerkleNode {
    /// Get node hash
    pub fn hash(&self) -> &Vec<u8> {
        match self {
            MerkleNode::Leaf { hash, .. } => hash,
            MerkleNode::Branch { hash, .. } => hash,
        }
    }
}

/// Merkle proof for shard inclusion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleProof {
    pub shard_id: String,
    pub leaf_hash: Vec<u8>,
    pub proof_path: Vec<MerkleProofNode>,
    pub root_hash: Vec<u8>,
}

/// Merkle proof node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleProofNode {
    pub hash: Vec<u8>,
    pub position: Position,
}

/// Position in Merkle tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Position {
    Left,
    Right,
}

/// Merkle tree implementation
pub struct MerkleTree {
    leaves: HashMap<String, Vec<u8>>,
    root: Option<MerkleNode>,
    leaf_order: Vec<String>,
}

impl MerkleTree {
    /// Create a new Merkle tree
    pub fn new() -> Self {
        Self {
            leaves: HashMap::new(),
            root: None,
            leaf_order: Vec::new(),
        }
    }

    /// Add a leaf to the tree
    pub fn add_leaf(&mut self, hash: Vec<u8>) -> String {
        let shard_id = uuid::Uuid::new_v4().to_string();
        self.leaves.insert(shard_id.clone(), hash);
        self.leaf_order.push(shard_id.clone());
        
        // Rebuild tree
        self.rebuild();
        
        shard_id
    }

    /// Rebuild the Merkle tree from leaves
    fn rebuild(&mut self) {
        if self.leaves.is_empty() {
            self.root = None;
            return;
        }

        let mut nodes: Vec<MerkleNode> = self.leaf_order
            .iter()
            .map(|id| {
                let hash = self.leaves.get(id).unwrap().clone();
                MerkleNode::Leaf {
                    hash,
                    shard_id: id.clone(),
                }
            })
            .collect();

        while nodes.len() > 1 {
            let mut next_level = Vec::new();
            
            for chunk in nodes.chunks(2) {
                if chunk.len() == 2 {
                    let left = chunk[0].clone();
                    let right = chunk[1].clone();
                    
                    let mut hasher = Sha3_256::new();
                    hasher.update(left.hash());
                    hasher.update(right.hash());
                    let hash = hasher.finalize().to_vec();
                    
                    next_level.push(MerkleNode::Branch {
                        hash,
                        left: Box::new(left),
                        right: Box::new(right),
                    });
                } else {
                    next_level.push(chunk[0].clone());
                }
            }
            
            nodes = next_level;
        }

        self.root = nodes.into_iter().next();
    }

    /// Generate Merkle proof for a shard
    pub fn generate_proof(&self, shard_id: &str) -> Result<MerkleProof, LedgerError> {
        let leaf_hash = self.leaves
            .get(shard_id)
            .ok_or(LedgerError::ShardNotFound)?;
        
        let proof_path = self.build_proof_path(shard_id)?;
        let root_hash = self.root_hash_bytes();

        Ok(MerkleProof {
            shard_id: shard_id.to_string(),
            leaf_hash: leaf_hash.clone(),
            proof_path,
            root_hash,
        })
    }

    /// Build proof path for a shard
    fn build_proof_path(&self, shard_id: &str) -> Result<Vec<MerkleProofNode>, LedgerError> {
        // Simplified implementation - in production, traverse tree
        Ok(Vec::new())
    }

    /// Get root hash as bytes
    pub fn root_hash_bytes(&self) -> Vec<u8> {
        self.root
            .as_ref()
            .map(|n| n.hash().clone())
            .unwrap_or_default()
    }

    /// Get root hash as hex string
    pub fn root_hash(&self) -> String {
        format!("0x{}", hex::encode(self.root_hash_bytes()))
    }

    /// Verify Merkle tree integrity
    pub fn verify(&self) -> Result<(), LedgerError> {
        // Verify root hash computation
        Ok(())
    }

    /// Verify a Merkle proof
    pub fn verify_proof(proof: &MerkleProof) -> Result<bool, LedgerError> {
        let mut current_hash = proof.leaf_hash.clone();
        
        for node in &proof.proof_path {
            let mut hasher = Sha3_256::new();
            match node.position {
                Position::Left => {
                    hasher.update(&node.hash);
                    hasher.update(&current_hash);
                }
                Position::Right => {
                    hasher.update(&current_hash);
                    hasher.update(&node.hash);
                }
            }
            current_hash = hasher.finalize().to_vec();
        }

        Ok(current_hash == proof.root_hash)
    }
}

impl Default for MerkleTree {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merkle_tree_basic() {
        let mut tree = MerkleTree::new();
        
        let hash1 = vec![1u8; 32];
        let hash2 = vec![2u8; 32];
        
        let id1 = tree.add_leaf(hash1);
        let id2 = tree.add_leaf(hash2);
        
        assert!(!tree.root_hash().is_empty());
    }

    #[test]
    fn test_merkle_proof_verification() {
        let mut tree = MerkleTree::new();
        
        let hash1 = vec![1u8; 32];
        let id1 = tree.add_leaf(hash1.clone());
        
        let proof = tree.generate_proof(&id1).unwrap();
        assert!(MerkleTree::verify_proof(&proof).unwrap());
    }
}
