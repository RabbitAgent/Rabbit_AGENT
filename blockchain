pragma solidity ^0.8.19;
import "@gnosis/zkp-verifier/contracts/Groth16.sol";

contract RabbitVerifier {
    using Groth16 for Groth16.Proof;
    
    struct Epoch {
        bytes32 modelHash;
        bytes32 hardwareAttestation;
        uint256 timestamp;
    }
    
    mapping(uint => Epoch) public epochs;
    address public enclaveOracle;
    
    constructor(address _oracle) {
        enclaveOracle = _oracle;
    }

    // Groth16 proof verification with TEE attestation
    function verifyInference(
        uint256[2] memory a,
        uint256[2][2] memory b,
        uint256[2] memory c,
        uint256[4] memory pubSignals,
        bytes calldata attestationReport
    ) external returns (bool) {
        // 1. Verify SGX attestation via trusted oracle
        require(ISGXOracle(enclaveOracle).verifyReport(attestationReport), "Invalid TEE");
        
        // 2. Verify ZKP proof on-chain
        Groth16.Proof memory proof = Groth16.Proof(a, b, c);
        require(proof.verify(pubSignals), "Invalid ZKP");
        
        // 3. Record model+hardware fingerprint
        epochs[block.number] = Epoch(
            bytes32(pubSignals[0]), 
            bytes32(pubSignals[1]),
            block.timestamp
        );
        
        return true;
    }
}

interface ISGXOracle {
    function verifyReport(bytes calldata report) external returns (bool);
}
