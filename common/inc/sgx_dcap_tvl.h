/*
 * Copyright (C) 2011-2021 Intel Corporation. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 *   * Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 *   * Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in
 *     the documentation and/or other materials provided with the
 *     distribution.
 *   * Neither the name of Intel Corporation nor the names of its
 *     contributors may be used to endorse or promote products derived
 *     from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 */
/**
 * File: sgx_dcap_tqvl.h
 *
 * Description: Trusted library for app enclave to verify QvE Report and Identity
 *
 */

#ifndef _SGX_DCAP_TVL_H_
#define _SGX_DCAP_TVL_H_

#include "sgx_qve_header.h"
#include "sgx_ql_quote.h"


#if defined(__cplusplus)
extern "C" {
#endif


#ifndef _MSC_VER
    #define SGX_TVL_API __attribute__ ((visibility("default")))
#else
    #define SGX_TVL_API
#endif

/**
 * Verify QvE Report and Identity
 *
 * @param p_quote[IN] - Pointer to SGX Quote.
 * @param quote_size[IN] - Size of the buffer pointed to by p_quote (in bytes).
 * @param p_qve_report_info[IN] - The output of API "sgx_qv_verify_quote", it should contain QvE report and nonce
 * @param expiration_check_date[IN] - This is the date to verify QvE report data, you should use same value for this API and "sgx_qv_verify_quote"
 * @param collateral_expiration_status[IN] - The output of API "sgx_qv_verify_quote" about quote verification collateral's expiration status
 * @param quote_verification_result[IN] - The output of API "sgx_qv_verify_quote" about quote verification result
 * @param p_supplemental_data[IN] - The output of API "sgx_qv_verify_quote", the pointer to supplemental data
 * @param supplemental_data_size[IN] - Size of the buffer pointed to by p_quote (in bytes)
 * @param qve_isvsvn_threshold [IN] - The threshold of QvE ISVSVN, the ISVSVN of QvE used to verify quote must be greater or equal to this threshold.
 * @You can get latest QvE ISVSVN from QvE configuration file (Github) or QvE Identity (JSON) from Intel PCS.
 * @Warning: The function may return erroneous result if QvE ISV SVN has been modified maliciously.
 *
 * @return Status code of the operation, one of:
 *   - SGX_QL_SUCCESS
 *   - SGX_QL_ERROR_INVALID_PARAMETER
 *   - SGX_QL_ERROR_REPORT           // Error when verifying QvE report
 *   - SGX_QL_ERROR_UNEXPECTED       // Error when comparing QvE report data
 *   - SGX_QL_QVEIDENTITY_MISMATCH   // Error when comparing QvE identity
 *   - SGX_QL_QVE_OUT_OF_DATE        // QvE ISVSVN is smaller than input QvE ISV SVN threshold
 **/

SGX_TVL_API quote3_error_t sgx_tvl_verify_qve_report_and_identity(
        const uint8_t *p_quote,
        uint32_t quote_size,
        const sgx_ql_qe_report_info_t *p_qve_report_info,
        time_t expiration_check_date,
        uint32_t collateral_expiration_status,
        sgx_ql_qv_result_t quote_verification_result,
        const uint8_t *p_supplemental_data,
        uint32_t supplemental_data_size,
        sgx_isv_svn_t qve_isvsvn_threshold);

#if defined(__cplusplus)
}
#endif

#endif /* !_SGX_DCAP_TVL_H_*/
