pub fn seal_payload(data: &[u8], key: &sgx_key_128bit_t) -> SgxResult<secure_payload_t> {
    let mut sealed = secure_payload_t::default();
    let mut mac = sgx_aes_gcm::AesGcm::new();
    
    let sealed_size = unsafe {
        sgx_seal_data_ex(
            SGX_KEYPOLICY_MRENCLAVE,
            SGX_SEAL_TAG_SIZE,
            key.as_ptr(),
            data.len() as u32,
            data.as_ptr() as *const u8,
            0,
            ptr::null(),
            &mut sealed.sealed as *mut sgx_sealed_data_t,
            mac.as_mut_ptr()
        )
    }?;
    
    sealed.size = sealed_size;
    Ok(sealed)
}

// Remote Attestation Evidence Generation
pub fn generate_attestation(report_data: &sgx_report_data_t) -> SgxResult<sgx_quote_t> {
    let mut report = sgx_report_t::default();
    let qe_target = sgx_get_qe_identity()?;
    
    let status = unsafe {
        sgx_create_report(&qe_target, report_data, &mut report)
    };
    
    if status != sgx_status_t::SGX_SUCCESS {
        return Err(status.into());
    }
    
    let mut quote = sgx_quote_t::default();
    let quote_size = mem::size_of::<sgx_quote_t>() as u32;
    
    unsafe {
        sgx_get_quote(
            &report,
            SGX_QUOTE_TYPE_LINKABLE,
            &sgx_spid_t::default(),
            ptr::null(),
            0,
            ptr::null(),
            &mut quote as *mut sgx_quote_t,
            quote_size
        )
    }?;
    
    Ok(quote)
}
