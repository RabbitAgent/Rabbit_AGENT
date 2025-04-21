use memmap::MmapOptions;
use std::{
    fs::File,
    sync::Arc
};

pub struct ParameterLoader {
    params_map: HashMap<ProofSystem, Arc<Parameters>>,
}

impl ParameterLoader {
    pub fn new() -> Self {
        Self {
            params_map: HashMap::new(),
        }
    }

    pub fn load_from_file(
        &mut self,
        system: ProofSystem,
        path: &Path
    ) -> Result<(), ParameterError> {
        let file = File::open(path)?;
        let mmap = unsafe { MmapOptions::new().map(&file)? };
        
        let params = match system {
            ProofSystem::Groth16 => {
                groth16::Parameters::read(&mmap[..], true)?
            }
            ProofSystem::Plonk => {
                // Plonk parameter parsing
            }
        };
        
        self.params_map.insert(system, Arc::new(params));
        Ok(())
    }

    pub fn get_params(&self, system: ProofSystem) -> Option<Arc<Parameters>> {
        self.params_map.get(&system).cloned()
    }
}
