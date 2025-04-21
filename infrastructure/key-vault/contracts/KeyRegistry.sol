pragma solidity ^0.8.0;

contract KeyRegistry {
    struct KeyMetadata {
        bytes32 keyHash;
        uint256 activationBlock;
        address[] approvers;
    }
    
    mapping(bytes32 => KeyMetadata) public registeredKeys;
    mapping(bytes32 => mapping(address => bool)) public approvals;
    
    event KeyRegistered(bytes32 indexed keyId, address indexed initiator);
    event ApprovalReceived(bytes32 indexed keyId, address approver);
    
    function registerKey(
        bytes32 keyId,
        bytes32 keyHash,
        address[] calldata approvers
    ) external {
        require(registeredKeys[keyId].activationBlock == 0, "Key already exists");
        
        registeredKeys[keyId] = KeyMetadata({
            keyHash: keyHash,
            activationBlock: block.number,
            approvers: approvers
        });
        
        emit KeyRegistered(keyId, msg.sender);
    }
    
    function approveUsage(
        bytes32 keyId,
        bytes32 operationHash
    ) external {
        KeyMetadata storage meta = registeredKeys[keyId];
        require(isApprover(keyId, msg.sender), "Not authorized approver");
        
        approvals[operationHash][msg.sender] = true;
        emit ApprovalReceived(keyId, msg.sender);
    }
    
    function validateOperation(
        bytes32 keyId,
        bytes32 operationHash,
        uint8 requiredApprovals
    ) public view returns (bool) {
        uint count;
        for (uint i = 0; i < registeredKeys[keyId].approvers.length; i++) {
            if (approvals[operationHash][registeredKeys[keyId].approvers[i]]) {
                count++;
                if (count >= requiredApprovals) return true;
            }
        }
        return false;
    }
}
