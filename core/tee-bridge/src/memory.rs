impl SecureBuffer {
    pub fn allocate_secure<T: Sized>(data: T) -> Result<Self, TeeError> {
        let size = std::mem::size_of::<T>();
        let mut buffer = Self {
            ptr: Box::into_raw(Box::new(data)) as *mut u8,
            size,
            _marker: PhantomData,
        };
        
        unsafe {
            sgx::protect_memory(buffer.ptr, buffer.size, sgx::PagePerm::ReadWrite)?;
        }
        
        Ok(buffer)
    }
    
    pub fn zeroize(&mut self) {
        unsafe {
            std::ptr::write_bytes(self.ptr, 0, self.size);
            sgx::unprotect_memory(self.ptr, self.size)?;
        }
    }
}
