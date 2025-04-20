pragma solidity ^0.8.0;

library InteropVerifier {
    struct Proof {
        bytes32[] inputs;
        uint256[2] a;
        uint256[2][2] b;
        uint256[2] c;
    }

    function verifyCrossChain(
        bytes32 _messageHash,
        Proof memory _proof,
        address _verifier
    ) internal view returns (bool) {
        bytes memory payload = abi.encodePacked(
            _messageHash,
            _proof.inputs,
            _proof.a,
            _proof.b,
            _proof.c
        );
        
        (bool success,) = _verifier.staticcall(payload);
        return success;
    }
}
