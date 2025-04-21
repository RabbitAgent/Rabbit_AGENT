use ark_merkle_tree::{MerkleTree, Config};
use ark_serialize::{CanonicalSerialize, CanonicalDeserialize};
use sha3::Keccak256;

#[derive(Clone, CanonicalSerialize, CanonicalDeserialize)]
pub struct AuditEntry {
    pub timestamp: u64,
    pub action_type: [u8; 4],
    pub data_hash: [u8; 32],
    pub parties: Vec<[u8; 32]>,
    pub zk_proof: Option<Vec<u8>>,
}

pub struct AuditMerkleTree {
    tree: MerkleTree<Keccak256, AuditEntry>,
    leaf_index: u64,
}

impl AuditMerkleTree {
    pub fn new() -> Self {
        Self {
            tree: MerkleTree::new(),
            leaf_index: 0,
        }
    }

    pub fn append_entry(&mut self, entry: AuditEntry) -> [u8; 32] {
        let leaf_data = self.serialize_entry(&entry);
        let leaf_hash = Keccak256::hash(&leaf_data);
        
        self.tree.push(leaf_hash);
        self.leaf_index += 1;
        
        leaf_hash
    }

    pub fn generate_inclusion_proof(&self, index: u64) -> Option<MerkleProof> {
        if index >= self.leaf_index {
            return None;
        }
        
        Some(self.tree.generate_proof(index as usize))
    }

    fn serialize_entry(&self, entry: &AuditEntry) -> Vec<u8> {
        let mut bytes = Vec::new();
        entry.serialize(&mut bytes).unwrap();
        bytes
    }
}

#[derive(CanonicalSerialize)]
pub struct MerkleProof {
    pub root: [u8; 32],
    pub path: Vec<([u8; 32], bool)>,
    pub index: u64,
}
