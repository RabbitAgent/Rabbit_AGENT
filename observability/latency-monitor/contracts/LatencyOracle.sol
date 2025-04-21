pragma solidity ^0.8.0;

contract LatencyOracle {
    struct MetricPackage {
        uint256 p99;
        uint256 maxLatency;
        bytes32 modelHash;
        bytes signature;
    }
    
    mapping(address => MetricPackage) public nodeMetrics;
    mapping(bytes32 => uint256) public modelUpdateTimestamps;
    
    event MetricsUpdated(address indexed node, uint256 blockNumber);
    event ModelCertified(bytes32 indexed modelHash, address auditor);
    
    function submitMetrics(MetricPackage calldata pkg) external {
        require(verifySignature(pkg), "Invalid signature");
        
        nodeMetrics[msg.sender] = pkg;
        modelUpdateTimestamps[pkg.modelHash] = block.timestamp;
        
        emit MetricsUpdated(msg.sender, block.number);
    }
    
    function verifySignature(MetricPackage calldata pkg) internal view returns (bool) {
        bytes32 hash = keccak256(abi.encode(
            msg.sender,
            pkg.p99,
            pkg.maxLatency,
            pkg.modelHash
        ));
        
        return ecrecover(hash, v, r, s) == trustedSigner;
    }
}
