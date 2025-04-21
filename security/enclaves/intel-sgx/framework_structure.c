#include "enclave_interface.h"
#include <sgx_urts.h>

#define ENCLAVE_FILE "sgx_impl/enclave.signed.so"

sgx_enclave_id_t init_enclave() {
    sgx_launch_token_t token = {0};
    sgx_enclave_id_t eid;
    int updated;
    
    sgx_status_t ret = sgx_create_enclave(
        ENCLAVE_FILE, 
        SGX_DEBUG_FLAG, 
        &token,
        &updated,
        &eid,
        NULL
    );
    
    if (ret != SGX_SUCCESS) {
        // Handle enclave initialization failure
        return SGX_INVALID_ENCLAVE_ID;
    }
    return eid;
}
