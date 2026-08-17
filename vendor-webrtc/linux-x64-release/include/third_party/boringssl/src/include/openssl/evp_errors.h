// Copyright 1995-2016 The OpenSSL Project Authors. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef OPENSSL_HEADER_EVP_ERRORS_H
#define OPENSSL_HEADER_EVP_ERRORS_H

#define EVP_R_BUFFER_TOO_SMALL 100
#define EVP_R_COMMAND_NOT_SUPPORTED 101
#define EVP_R_DECODE_ERROR 102
#define EVP_R_DIFFERENT_KEY_TYPES 103
#define EVP_R_DIFFERENT_PARAMETERS 104
#define EVP_R_ENCODE_ERROR 105
#define EVP_R_EXPECTING_AN_EC_KEY_KEY 106
#define EVP_R_EXPECTING_AN_RSA_KEY 107
#define EVP_R_EXPECTING_A_DSA_KEY 108
#define EVP_R_ILLEGAL_OR_UNSUPPORTED_PADDING_MODE 109
#define EVP_R_INVALID_DIGEST_LENGTH 110
#define EVP_R_INVALID_DIGEST_TYPE 111
#define EVP_R_INVALID_KEYBITS 112
#define EVP_R_INVALID_MGF1_MD 113
#define EVP_R_INVALID_OPERATION 114
#define EVP_R_INVALID_PADDING_MODE 115
#define EVP_R_INVALID_PSS_SALTLEN 116
#define EVP_R_KEYS_NOT_SET 117
#define EVP_R_MISSING_PARAMETERS 118
#define EVP_R_NO_DEFAULT_DIGEST 119
#define EVP_R_NO_KEY_SET 120
#define EVP_R_NO_MDC2_SUPPORT 121
#define EVP_R_NO_NID_FOR_CURVE 122
#define EVP_R_NO_OPERATION_SET 123
#define EVP_R_NO_PARAMETERS_SET 124
#define EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE 125
#define EVP_R_OPERATON_NOT_INITIALIZED 126
#define EVP_R_UNKNOWN_PUBLIC_KEY_TYPE 127
#define EVP_R_UNSUPPORTED_ALGORITHM 128
#define EVP_R_UNSUPPORTED_PUBLIC_KEY_TYPE 129
#define EVP_R_NOT_A_PRIVATE_KEY 130
#define EVP_R_INVALID_SIGNATURE 131
#define EVP_R_MEMORY_LIMIT_EXCEEDED 132
#define EVP_R_INVALID_PARAMETERS 133
#define EVP_R_INVALID_PEER_KEY 134
#define EVP_R_NOT_XOF_OR_INVALID_LENGTH 135
#define EVP_R_EMPTY_PSK 136
#define EVP_R_INVALID_BUFFER_SIZE 137
#define EVP_R_EXPECTING_A_DH_KEY 138

#endif  // OPENSSL_HEADER_EVP_ERRORS_H
