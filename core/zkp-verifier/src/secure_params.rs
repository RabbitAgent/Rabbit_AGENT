impl ToxicWasteDestroyer {
    pub fn secure_erase(params: &mut Parameters) {
        unsafe {
            let ptr = params as *mut _ as *mut u8;
            let len = std::mem::size_of_val(params);
            sodium_memzero(ptr, len);
        }
    }
}
