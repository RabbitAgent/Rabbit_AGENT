pragma solidity ^0.8.0;

contract StorageRegistry {
    struct StorageNode {
        bytes32 merkleRoot;
        uint256 timestamp;
        address[] holders;
    }
    
    mapping(bytes32 => StorageNode) public contentNodes;
    
    event ContentRegistered(bytes32 indexed contentId, address indexed registrant);
    
    function registerContent(
        bytes32 contentId,
        bytes32 merkleRoot,
        address[] calldata holders
    ) external {
        require(contentNodes[contentId].timestamp == 0, "Content already exists");
        
        contentNodes[contentId] = StorageNode({
            merkleRoot: merkleRoot,
            timestamp: block.timestamp,
            holders: holders
        });
        
        emit ContentRegistered(contentId, msg.sender);
    }
    
    function verifyInclusion(
        bytes32 contentId,
        bytes32 leaf,
        bytes32[] calldata proof,
        uint256 index
    ) external view returns (bool) {
        StorageNode storage node = contentNodes[contentId];
        bytes32 currentHash = leaf;
        
        for (uint256 i = 0; i < proof.length; i++) {
            if (index % 2 == 0) {
                currentHash = keccak256(abi.encode(currentHash, proof[i]));
            } else {
                currentHash = keccak256(abi.encode(proof[i], currentHash));
            }
            index = index / 2;
        }
        
        return currentHash == node.merkleRoot;
    }
}
