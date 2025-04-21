pragma solidity ^0.8.0;

contract AuditRegistry {
    struct AuditRoot {
        bytes32 rootHash;
        uint256 blockNumber;
        address submitter;
    }
    
    mapping(bytes32 => AuditRoot) public auditRoots;
    mapping(bytes32 => mapping(uint256 => bool)) public verifiedLeaves;
    
    event RootRegistered(bytes32 indexed contentId, address indexed submitter);
    event LeafVerified(bytes32 indexed rootHash, uint256 indexed leafIndex);
    
    function registerRoot(bytes32 contentId, bytes32 rootHash) external {
        require(auditRoots[contentId].blockNumber == 0, "Root already exists");
        
        auditRoots[contentId] = AuditRoot({
            rootHash: rootHash,
            blockNumber: block.number,
            submitter: msg.sender
        });
        
        emit RootRegistered(contentId, msg.sender);
    }
    
    function verifyInclusion(
        bytes32 contentId,
        bytes32 leafHash,
        bytes32[] calldata proof,
        uint256 leafIndex
    ) external {
        AuditRoot storage root = auditRoots[contentId];
        require(root.blockNumber != 0, "Root not found");
        
        bytes32 currentHash = leafHash;
        uint256 index = leafIndex;
        
        for (uint256 i = 0; i < proof.length; i++) {
            if (index % 2 == 0) {
                currentHash = keccak256(abi.encodePacked(currentHash, proof[i]));
            } else {
                currentHash = keccak256(abi.encodePacked(proof[i], currentHash));
            }
            index = index / 2;
        }
        
        require(currentHash == root.rootHash, "Invalid proof");
        verifiedLeaves[contentId][leafIndex] = true;
        emit LeafVerified(root.rootHash, leafIndex);
    }
}
