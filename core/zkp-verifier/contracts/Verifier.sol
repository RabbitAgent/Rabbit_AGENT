pragma solidity ^0.8.0;

library Pairing {
    struct G1Point {
        uint X;
        uint Y;
    }
    
    struct G2Point {
        uint[2] X;
        uint[2] Y;
    }
    
    function pairing(G1Point[] memory p1, G2Point[] memory p2) internal view returns (bool) {
        require(p1.length == p2.length);
        uint inputSize = p1.length * 6;
        uint[] memory input = new uint[](inputSize);
        
        for (uint i = 0; i < p1.length; i++) {
            input[i*6 + 0] = p1[i].X;
            input[i*6 + 1] = p1[i].Y;
            input[i*6 + 2] = p2[i].X[0];
            input[i*6 + 3] = p2[i].X[1];
            input[i*6 + 4] = p2[i].Y[0];
            input[i*6 + 5] = p2[i].Y[1];
        }
        
        uint[1] memory out;
        bool success;
        
        assembly {
            success := staticcall(sub(gas(), 2000), 8, add(input, 0x20), mul(inputSize, 0x20), out, 0x20)
        }
        require(success);
        return out[0] != 0;
    }
}

contract Groth16Verifier {
    using Pairing for *;
    
    struct VerifyingKey {
        Pairing.G1Point alpha;
        Pairing.G2Point beta;
        Pairing.G2Point gamma;
        Pairing.G2Point delta;
        Pairing.G1Point[] gamma_abc;
    }
    
    function verify(
        uint[] memory input,
        Pairing.G1Point memory a,
        Pairing.G2Point memory b,
        Pairing.G1Point memory c,
        Pairing.G2Point memory d,
        VerifyingKey memory vk
    ) internal view returns (bool) {
        Pairing.G1Point memory alpha = vk.alpha;
        Pairing.G2Point memory beta = vk.beta;
        Pairing.G2Point memory gamma = vk.gamma;
        Pairing.G2Point memory delta = vk.delta;
        Pairing.G1Point[] memory gammaABC = vk.gamma_abc;

        Pairing.G1Point memory accum;
        accum.X = gammaABC[0].X;
        accum.Y = gammaABC[0].Y;
        
        for (uint i = 0; i < input.length; i++) {
            accum = Pairing.addition(
                accum,
                Pairing.scalar_mul(gammaABC[i+1], input[i])
            );
        }

        Pairing.G1Point[] memory p1 = new Pairing.G1Point[](4);
        Pairing.G2Point[] memory p2 = new Pairing.G2Point[](4);
        
        p1[0] = Pairing.negate(a);
        p2[0] = alpha;
        
        p1[1] = accum;
        p2[1] = beta;
        
        p1[2] = c;
        p2[2] = gamma;
        
        p1[3] = Pairing.negate(accum);
        p2[3] = delta;

        return Pairing.pairing(p1, p2);
    }
}
