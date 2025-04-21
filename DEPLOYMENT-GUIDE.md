# RabbitAGENT Deployment Guide

## Prerequisites
- **Hardware**: 
  - x86_64 with Intel SGX (Flexible Launch Control) / ARMv8.3+ TrustZone  
  - 32GB RAM (ECC recommended)
  - NVIDIA GPU (Turing+ for CUDA 12.1)
- **OS**: Ubuntu 22.04 LTS (5.19+ kernel)  
  ```bash
  sudo apt install -y az-dcap-client ocl-icd-opencl-dev
Component	Port	Dependencies
TEE Orchestrator	9666	SGX DCAP, AESNI+AVX512
ZKP Verifier	9667	Groth16, Bellman 0.12.1
Inference Engine	9668	CUDA 12.1, ONNX 1.14.1
Blockchain Adapter	9669	Geth 1.13.8, Hyperlane 3.2
git clone https://github.com/RabbitAGENT/security/enclaves
cd intel-sgx && SGX_MODE=HW make production
sudo ./configure_enclave --dcap --memory-encryption=aes-xts
export BELLMAN_CPU_UTILIZATION=0.75  # Optimal for Groth16
cargo build --release -p zk-circuits --features "cuda,multicore"
# config/node.toml
[tee]
attestation_url = "https://sgx-verifier.rabbitagent.com"  
[zkp]
trusted_setup = "./phase1.rad"  # Pre-download from IPFS
curl -X POST http://localhost:9667/verify \
  -H "Content-Type: application/cbor" \
  --data-binary @proof.bin
# Expect 200 OK with {"valid":true}
openssl rand -hex 32 > .session_key && systemctl restart rabbitagent
sudo apt install intel-sgx-dcap-ql flex-launch-control
./audit_tool --hipaa --gdpr --report=pdf

**Key Technical Points**:
1. Hardware-Specific TEE Configuration (AES-XTS memory encryption)
2. Performance-Tuned ZKP Circuit Builds
3. End-to-End Verification Workflow
4. Regulatory Compliance Automation

