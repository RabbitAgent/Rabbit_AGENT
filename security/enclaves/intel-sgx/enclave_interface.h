#pragma once
#include <sgx_key.h>
#include <sgx_report.h>

#define ENCRYPTED_DATA_MAX 4096
#define SEALED_DATA_SIZE (sizeof(sgx_sealed_data_t) + ENCRYPTED_DATA_MAX)

typedef struct {
    sgx_sealed_data_t sealed_payload;
    uint32_t payload_size;
} secure_buffer_t;

#ifdef __cplusplus
extern "C" {
#endif

sgx_status_t SGX_CDECL ecall_secure_inference(
    const secure_buffer_t* input,
    secure_buffer_t* output,
    sgx_report_t* attestation_report
);

sgx_status_t SGX_CDECL ecall_generate_quote(
    const sgx_report_data_t* report_data,
    sgx_quote_t* quote_buffer,
    uint32_t quote_size
);

#ifdef __cplusplus
}
#endif
