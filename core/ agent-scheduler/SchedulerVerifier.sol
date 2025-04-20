pragma solidity ^0.8.0;

contract SchedulerVerifier {
    mapping(bytes32 => bool) public validatedTasks;
    
    function verifyTaskConstraints(
        bytes32 taskHash,
        bytes calldata zkProof
    ) external returns (bool) {
        require(!validatedTasks[taskHash], "Already verified");
        
        bool isValid = ZkVerifierLib.verifyGroth16(
            taskHash, 
            zkProof,
            verifierKey
        );
        
        if (isValid) {
            validatedTasks[taskHash] = true;
        }
        return isValid;
    }
}
