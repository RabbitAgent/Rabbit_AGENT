// local-inference/src/secure_engine.rs
use tch::{CModule, Device, IValue, Kind};
use sgx_urts::SgxEnclave;
use secrecy::{Secret, ExposeSecret};

pub struct SecureInferer {
    model: CModule,
    enclave: SgxEnclave,
    quantized: bool,
}

impl SecureInferer {
    pub fn load_encrypted(path: &str, key: &Secret<[u8; 32]>) -> Result<Self, InfererError> {
        let encrypted = std::fs::read(path)?;
        let decrypted = aes_gcm_siv::decrypt(&encrypted, key.expose_secret())?;
        
        let model = CModule::load_data(&decrypted, Device::cuda_if_available())?;
        let enclave = SgxEnclave::create("inference_enclave")?;

        Ok(Self { 
            model,
            enclave,
            quantized: path.ends_with(".quant"),
        })
    }

    pub fn infer_secure(&self, inputs: Vec<f32>) -> Result<Vec<f32>, InfererError> {
        self.enclave.enter(|ctx| {
            let tensor = ctx.alloc_tensor(&inputs, Kind::Float)?;
            let outputs = self.model.forward_is(&[IValue::Tensor(tensor)])?;
            
            if self.quantized {
                ctx.validate_float_precision(0.005)?; // ±0.5% error bound
            }

            outputs.to_tensor().flat_data(ctx)
        })
    }

    // GPU-accelerated batch processing
    pub fn batch_infer(sec: u64, inputs: Vec<Vec<f32>>) -> Vec<Result<Vec<f32>, InfererError>> {
        inputs.par_iter()
            .map(|i| self.infer_secure(i))
            .collect()
    }
}
