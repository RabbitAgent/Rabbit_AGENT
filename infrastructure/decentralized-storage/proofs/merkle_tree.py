from merkly.mtree import MerkleTree
from typing import List
import hashlib

class StorageProver:
    def __init__(self, chunks: List[bytes]):
        self.tree = MerkleTree(
            leaves=[self.hash_chunk(c) for c in chunks],
            hash_function=lambda x: hashlib.blake2b(x).digest()
        )
    
    def get_proof(self, index: int) -> dict:
        return {
            "root": self.tree.root,
            "path": self.tree.proof(index),
            "index": index,
            "leaf": self.tree.leaves[index]
        }
    
    @staticmethod
    def hash_chunk(data: bytes) -> bytes:
        return hashlib.blake3(data).digest()
