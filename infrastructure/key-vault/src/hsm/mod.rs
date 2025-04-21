use pkcs11::Context;
use std::path::Path;

pub struct HsmClient {
    ctx: Arc<Mutex<Context>>,
    session: CKSessionHandle,
}

impl HsmClient {
    pub fn connect(config: &HsmConfig) -> Result<Self, HsmError> {
        let ctx = Context::new(Path::new(&config.module_path))?;
        ctx.initialize()?;
        
        let session = ctx.open_session(
            config.slot_id,
            CKF_RW_SESSION | CKF_SERIAL_SESSION
        )?;
        
        ctx.login(session, CKU_USER, &config.pin)?;
        
        Ok(Self {
            ctx: Arc::new(Mutex::new(ctx)),
            session,
        })
    }

    pub fn generate_secure_key(&mut self, opts: KeyGenOptions) -> Result<[u8; 32], HsmError> {
        let template = vec![
            CKA_CLASS(CKO_SECRET_KEY),
            CKA_KEY_TYPE(opts.key_type.into()),
            CKA_TOKEN(opts.persistent),
            CKA_EXPORTABLE(opts.exportable),
            CKA_SENSITIVE(true),
            CKA_EXTRACTABLE(false),
        ];
        
        let key = self.ctx.lock()
            .generate_key(self.session, &template)?;
        
        let mut output = [0u8; 32];
        self.ctx.lock()
            .get_attribute_value(self.session, key, &[CKA_VALUE])?
            .read(&mut output)?;
            
        Ok(output)
    }
}

pub struct KeyGenOptions {
    pub key_type: KeyType,
    pub persistent: bool,
    pub exportable: bool,
}
