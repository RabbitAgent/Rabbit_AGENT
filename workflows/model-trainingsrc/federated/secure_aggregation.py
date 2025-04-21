import torch
from cryptography.hazmat.primitives import hashes, hmac
from sgx_utils import EnclaveWrapper

class FederatedTrainer:
    def __init__(self, enclave_path: str):
        self.enclave = EnclaveWrapper(enclave_path)
        self.model = self._init_model()
        
    def _init_model(self) -> torch.nn.Module:
        """Load base model inside TEE-protected memory"""
        with self.enclave.secure_context():
            model = torch.jit.load("base_model.pt")
            model.train()
            return model

    def secure_aggregate(self, client_updates: List[bytes]) -> None:
        """TEE-verified parameter aggregation"""
        with self.enclave.secure_context():
            aggregated = self._validate_and_aggregate(client_updates)
            self._apply_gradients(aggregated)

    def _validate_and_aggregate(self, updates: List[bytes]) -> torch.Tensor:
        verified = []
        for update in updates:
            h = hmac.HMAC(self.enclave.get_root_key(), hashes.SHA256())
            h.update(update)
            h.verify(self.enclave.get_attestation_tag())  # Throws on invalid
            verified.append(torch.load(update))
            
        return self._sma(verified)  # Secure multi-party aggregation

    def export_encrypted_model(self) -> bytes:
        """Generate TEE-attested model package"""
        with self.enclave.secure_context():
            torchscript = torch.jit.script(self.model)
            return self.enclave.seal(torchscript.save_to_buffer())
