// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include <gtest/gtest.h>

#include <openssl/x509.h>

#include "../internal.h"
#include "internal.h"
#include "../test/test_util.h"
#include "../test/x509_util.h"

/*

The default root certificate key, "ROOT_KEY_1", used for self-signed
roots, unless otherwise specified. This is EC prime256v1 key.
-----BEGIN PRIVATE KEY-----
MIGHAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBG0wawIBAQQgfVMH4tqIaJ6OzyxY
mqWXNwmK7gpXYDFhX80mXKgzrGGhRANCAATCqXrfbdTjFimzdBHxj71Ejcc/stea
5xAU/xxK+s77yXzB5lfy/zEbcYxuOrnwHrWsX9sugWgCy74ZRNWJPTDW
-----END PRIVATE KEY-----

The default intermediate certificate key, "INT_KEY_1", used for intermediate certs,
unless otherwise specified. This is EC prime256v1 key.

-----BEGIN PRIVATE KEY-----
MIGHAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBG0wawIBAQQgi8u0pAYRFQMfuPFA
lZbxzOm4ZKQ0WovdzFj+CzdvnzOhRANCAASeZNGM4efwO6Ui3wQtVProzLEi54vG
YNlyza0aFtw0n5noIoCV3hlbCk7W1kdat0QyFz9rMoDlDjXLaGLE3THR
-----END PRIVATE KEY-----

The default end-entity certificate key, "EE_KEY_1", used for ee certs,
unless otherwise specified. This is EC prime256v1 key.
-----BEGIN PRIVATE KEY-----
MIGHAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBG0wawIBAQQgfVMH4tqIaJ6OzyxY
mqWXNwmK7gpXYDFhX80mXKgzrGGhRANCAATCqXrfbdTjFimzdBHxj71Ejcc/stea
5xAU/xxK+s77yXzB5lfy/zEbcYxuOrnwHrWsX9sugWgCy74ZRNWJPTDW
-----END PRIVATE KEY-----
*/

/*
This self-signed root certificate's basicConstraints extension
cA:false. The root certificate though has the keyCertSign bit set for the
keyUsage extension. This is in violation of RFC 5280 4.2.1.9:
"If the cA boolean is not asserted, then the keyCertSign bit in the key
usage extension MUST NOT be asserted"

Certificate:
    Data:
        Version: 3 (0x2)
        Serial Number:
            0b:1e:77:95:de:6d:eb:b6:ab:2b:c4:51:3c:a6:70:02:99:f7:e5:f3
        Signature Algorithm: ecdsa-with-SHA256
        Issuer: C = US, ST = Washington, O = AWS Libcrypto, OU = Bad CA, CN = RFC 5280 4.2.1.9 cA:false
        Validity
            Not Before: Jan  1 00:00:00 2015 GMT
            Not After : Jan  1 00:00:00 2100 GMT
        Subject: C = US, ST = Washington, O = AWS Libcrypto, OU = Bad CA, CN = RFC 5280 4.2.1.9 cA:false
        Subject Public Key Info:
            Public Key Algorithm: id-ecPublicKey
                Public-Key: (256 bit)
                pub:
                    04:c2:a9:7a:df:6d:d4:e3:16:29:b3:74:11:f1:8f:
                    bd:44:8d:c7:3f:b2:d7:9a:e7:10:14:ff:1c:4a:fa:
                    ce:fb:c9:7c:c1:e6:57:f2:ff:31:1b:71:8c:6e:3a:
                    b9:f0:1e:b5:ac:5f:db:2e:81:68:02:cb:be:19:44:
                    d5:89:3d:30:d6
                ASN1 OID: prime256v1
                NIST CURVE: P-256
        X509v3 extensions:
            X509v3 Key Usage: critical
                Certificate Sign
            X509v3 Basic Constraints: critical
                CA:FALSE
            X509v3 Subject Key Identifier:
                19:19:E1:8C:09:E2:5D:5C:16:04:E1:9C:74:66:19:FD:B8:52:5B:DF
    Signature Algorithm: ecdsa-with-SHA256
    Signature Value:
        30:45:02:20:55:72:9f:65:36:59:eb:0f:c4:50:d0:d7:fb:58:
        3e:54:5e:dc:bf:7e:37:a8:a4:9c:41:a8:91:91:ae:ce:39:ff:
        02:21:00:8b:d0:01:0d:89:6f:61:4b:7a:ec:85:d4:ef:80:13:
        bd:52:44:ae:cd:56:32:08:5d:37:44:32:ac:50:f2:cb:c1
*/
static const char kRootBadBasicConstraints[] = R"(
-----BEGIN CERTIFICATE-----
MIICITCCAcegAwIBAgIUCx53ld5t67arK8RRPKZwApn35fMwCgYIKoZIzj0EAwIw
bzELMAkGA1UEBhMCVVMxEzARBgNVBAgMCldhc2hpbmd0b24xFjAUBgNVBAoMDUFX
UyBMaWJjcnlwdG8xDzANBgNVBAsMBkJhZCBDQTEiMCAGA1UEAwwZUkZDIDUyODAg
NC4yLjEuOSBjQTpmYWxzZTAgFw0xNTAxMDEwMDAwMDBaGA8yMTAwMDEwMTAwMDAw
MFowbzELMAkGA1UEBhMCVVMxEzARBgNVBAgMCldhc2hpbmd0b24xFjAUBgNVBAoM
DUFXUyBMaWJjcnlwdG8xDzANBgNVBAsMBkJhZCBDQTEiMCAGA1UEAwwZUkZDIDUy
ODAgNC4yLjEuOSBjQTpmYWxzZTBZMBMGByqGSM49AgEGCCqGSM49AwEHA0IABMKp
et9t1OMWKbN0EfGPvUSNxz+y15rnEBT/HEr6zvvJfMHmV/L/MRtxjG46ufAetaxf
2y6BaALLvhlE1Yk9MNajPzA9MA4GA1UdDwEB/wQEAwICBDAMBgNVHRMBAf8EAjAA
MB0GA1UdDgQWBBQZGeGMCeJdXBYE4Zx0Zhn9uFJb3zAKBggqhkjOPQQDAgNIADBF
AiBVcp9lNlnrD8RQ0Nf7WD5UXty/fjeopJxBqJGRrs45/wIhAIvQAQ2Jb2FLeuyF
1O+AE71SRK7NVjIIXTdEMqxQ8svB
-----END CERTIFICATE-----
)";

/*
This is an EE certificate signed by |kRootBadBasicConstraints|.
This should not be considered valid as the kRootBadBasicConstraints is a v3
certificate which has a basicConstraints extension indicating that it is not
a CA, and it violates the RFC 5280 condition.

Certificate:
    Data:
        Version: 3 (0x2)
        Serial Number:
            7b:8e:6c:6e:5a:d0:53:59:e7:fc:e2:93:b3:f4:19:16:1e:c3:81:82
        Signature Algorithm: ecdsa-with-SHA256
        Issuer: C = US, ST = Washington, O = AWS Libcrypto, OU = Bad CA, CN = RFC 5280 4.2.1.9 cA:false
        Validity
            Not Before: Jan  1 00:00:00 2015 GMT
            Not After : Jan  1 00:00:00 2100 GMT
        Subject: C = US, ST = Washington, O = AWS Libcrypto, OU = Bad Endpoint, CN = RFC 5280 4.2.1.9 cA:false
        Subject Public Key Info:
            Public Key Algorithm: id-ecPublicKey
                Public-Key: (256 bit)
                pub:
                    04:b2:b7:bd:35:f2:eb:da:86:d5:dc:40:44:c7:23:
                    14:f9:d0:a5:40:17:30:85:b6:c6:11:38:c2:db:2c:
                    c5:bc:0c:19:11:d8:68:61:d6:a3:92:6b:8a:18:52:
                    2c:dc:86:a7:ad:29:ad:91:ac:7e:df:87:24:3b:f3:
                    b4:71:2b:4e:58
                ASN1 OID: prime256v1
                NIST CURVE: P-256
        X509v3 extensions:
            X509v3 Key Usage: critical
                Digital Signature, Key Encipherment
            X509v3 Basic Constraints: critical
                CA:FALSE
            X509v3 Extended Key Usage:
                TLS Web Server Authentication, TLS Web Client Authentication
            X509v3 Subject Key Identifier:
                C8:78:64:E9:F7:9C:0F:56:E2:1D:CE:EE:ED:24:E0:9F:1D:4B:A3:BF
            X509v3 Authority Key Identifier:
                19:19:E1:8C:09:E2:5D:5C:16:04:E1:9C:74:66:19:FD:B8:52:5B:DF
    Signature Algorithm: ecdsa-with-SHA256
    Signature Value:
        30:45:02:20:38:ca:c4:54:ed:fc:bb:76:60:e9:4e:b5:85:91:
        f8:dc:a5:6a:54:9b:d2:22:a4:2c:6e:a6:df:fd:00:85:c0:06:
        02:21:00:ee:15:23:50:40:1b:67:b0:eb:13:75:6e:29:66:b0:
        e6:58:cf:1f:c2:5e:a1:85:01:45:9c:4d:ab:e7:61:ac:70
*/
static const char kEndEntitySignedByBadRoot[] = R"(
-----BEGIN CERTIFICATE-----
MIICZzCCAg2gAwIBAgIUe45sblrQU1nn/OKTs/QZFh7DgYIwCgYIKoZIzj0EAwIw
bzELMAkGA1UEBhMCVVMxEzARBgNVBAgMCldhc2hpbmd0b24xFjAUBgNVBAoMDUFX
UyBMaWJjcnlwdG8xDzANBgNVBAsMBkJhZCBDQTEiMCAGA1UEAwwZUkZDIDUyODAg
NC4yLjEuOSBjQTpmYWxzZTAgFw0xNTAxMDEwMDAwMDBaGA8yMTAwMDEwMTAwMDAw
MFowdTELMAkGA1UEBhMCVVMxEzARBgNVBAgMCldhc2hpbmd0b24xFjAUBgNVBAoM
DUFXUyBMaWJjcnlwdG8xFTATBgNVBAsMDEJhZCBFbmRwb2ludDEiMCAGA1UEAwwZ
UkZDIDUyODAgNC4yLjEuOSBjQTpmYWxzZTBZMBMGByqGSM49AgEGCCqGSM49AwEH
A0IABLK3vTXy69qG1dxARMcjFPnQpUAXMIW2xhE4wtssxbwMGRHYaGHWo5JrihhS
LNyGp60prZGsft+HJDvztHErTlijfzB9MA4GA1UdDwEB/wQEAwIFoDAMBgNVHRMB
Af8EAjAAMB0GA1UdJQQWMBQGCCsGAQUFBwMBBggrBgEFBQcDAjAdBgNVHQ4EFgQU
yHhk6fecD1biHc7u7STgnx1Lo78wHwYDVR0jBBgwFoAUGRnhjAniXVwWBOGcdGYZ
/bhSW98wCgYIKoZIzj0EAwIDSAAwRQIgOMrEVO38u3Zg6U61hZH43KVqVJvSIqQs
bqbf/QCFwAYCIQDuFSNQQBtnsOsTdW4pZrDmWM8fwl6hhQFFnE2r52GscA==
-----END CERTIFICATE-----
)";

/*
The is a valid root certificate that is expected to pass validation.
It may be used as a trust-anchor for both good or bad intermediate or
client certificates in testing.

Certificate:
    Data:
        Version: 3 (0x2)
        Serial Number:
            2e:0f:2e:da:e3:11:b2:fa:42:b3:29:09:b0:cc:93:87:ac:15:25:3d
        Signature Algorithm: ecdsa-with-SHA256
        Issuer: C = US, ST = Washington, O = AWS Libcrypto, OU = Good CA, CN = Root CA 1
        Validity
            Not Before: Jan  1 00:00:00 2015 GMT
            Not After : Jan  1 00:00:00 2100 GMT
        Subject: C = US, ST = Washington, O = AWS Libcrypto, OU = Good CA, CN = Root CA 1
        Subject Public Key Info:
            Public Key Algorithm: id-ecPublicKey
                Public-Key: (256 bit)
                    04:c2:a9:7a:df:6d:d4:e3:16:29:b3:74:11:f1:8f:
                    bd:44:8d:c7:3f:b2:d7:9a:e7:10:14:ff:1c:4a:fa:
                    ce:fb:c9:7c:c1:e6:57:f2:ff:31:1b:71:8c:6e:3a:
                    b9:f0:1e:b5:ac:5f:db:2e:81:68:02:cb:be:19:44:
                    d5:89:3d:30:d6
                ASN1 OID: prime256v1
                NIST CURVE: P-256
        X509v3 extensions:
            X509v3 Key Usage: critical
                Digital Signature, Certificate Sign, CRL Sign
            X509v3 Basic Constraints: critical
                CA:TRUE
            X509v3 Subject Key Identifier:
                19:19:E1:8C:09:E2:5D:5C:16:04:E1:9C:74:66:19:FD:B8:52:5B:DF
    Signature Algorithm: ecdsa-with-SHA256
    Signature Value:
        30:45:02:21:00:a4:39:29:1b:07:a2:3e:cc:21:eb:f6:5c:fd:
        b8:88:ee:79:46:37:25:e5:9a:79:2e:3d:2f:21:15:56:43:c8:
        b8:02:20:36:bd:03:bc:df:f6:7d:d2:a3:d2:a5:48:a8:64:75:
        00:4e:01:cd:67:b9:19:87:49:2b:bd:15:94:3e:f5:75:ca
*/
static const char kValidRootCA1[] = R"(
-----BEGIN CERTIFICATE-----
MIICBjCCAaygAwIBAgIULg8u2uMRsvpCsykJsMyTh6wVJT0wCgYIKoZIzj0EAwIw
YDELMAkGA1UEBhMCVVMxEzARBgNVBAgMCldhc2hpbmd0b24xFjAUBgNVBAoMDUFX
UyBMaWJjcnlwdG8xEDAOBgNVBAsMB0dvb2QgQ0ExEjAQBgNVBAMMCVJvb3QgQ0Eg
MTAgFw0xNTAxMDEwMDAwMDBaGA8yMTAwMDEwMTAwMDAwMFowYDELMAkGA1UEBhMC
VVMxEzARBgNVBAgMCldhc2hpbmd0b24xFjAUBgNVBAoMDUFXUyBMaWJjcnlwdG8x
EDAOBgNVBAsMB0dvb2QgQ0ExEjAQBgNVBAMMCVJvb3QgQ0EgMTBZMBMGByqGSM49
AgEGCCqGSM49AwEHA0IABMKpet9t1OMWKbN0EfGPvUSNxz+y15rnEBT/HEr6zvvJ
fMHmV/L/MRtxjG46ufAetaxf2y6BaALLvhlE1Yk9MNajQjBAMA4GA1UdDwEB/wQE
AwIBhjAPBgNVHRMBAf8EBTADAQH/MB0GA1UdDgQWBBQZGeGMCeJdXBYE4Zx0Zhn9
uFJb3zAKBggqhkjOPQQDAgNIADBFAiEApDkpGweiPswh6/Zc/biI7nlGNyXlmnku
PS8hFVZDyLgCIDa9A7zf9n3So9KlSKhkdQBOAc1nuRmHSSu9FZQ+9XXK
-----END CERTIFICATE-----
)";

/*
This is an EE certificate signed by |kValidRootCA1|, and is invalid as it
has an Authority Key Identifier (AKID) extension marked critical which
is not valid per RFC 5280 4.2.1.1:
"Conforming CAs MUST mark this extension as non-critical."

Certificate:
    Data:
        Version: 3 (0x2)
        Serial Number:
            27:8c:f7:17:16:47:56:c0:58:32:6c:dd:65:09:10:6b:44:bb:0e:a7
        Signature Algorithm: ecdsa-with-SHA256
        Issuer: C = US, ST = Washington, O = AWS Libcrypto, OU = Good CA, CN = Root CA 1
        Validity
            Not Before: Jan  1 00:00:00 2015 GMT
            Not After : Jan  1 00:00:00 2100 GMT
        Subject: C = US, ST = Washington, O = AWS Libcrypto, OU = Bad Endpoint, CN = RFC 5280 4.2.1.1 AKID MUST be non-critical
        Subject Public Key Info:
            Public Key Algorithm: id-ecPublicKey
                Public-Key: (256 bit)
                pub:
                    04:b2:b7:bd:35:f2:eb:da:86:d5:dc:40:44:c7:23:
                    14:f9:d0:a5:40:17:30:85:b6:c6:11:38:c2:db:2c:
                    c5:bc:0c:19:11:d8:68:61:d6:a3:92:6b:8a:18:52:
                    2c:dc:86:a7:ad:29:ad:91:ac:7e:df:87:24:3b:f3:
                    b4:71:2b:4e:58
                ASN1 OID: prime256v1
                NIST CURVE: P-256
        X509v3 extensions:
            X509v3 Key Usage: critical
                Digital Signature, Key Encipherment
            X509v3 Basic Constraints: critical
                CA:FALSE
            X509v3 Extended Key Usage:
                TLS Web Server Authentication, TLS Web Client Authentication
            X509v3 Authority Key Identifier: critical
                19:19:E1:8C:09:E2:5D:5C:16:04:E1:9C:74:66:19:FD:B8:52:5B:DF
            X509v3 Subject Key Identifier:
                C8:78:64:E9:F7:9C:0F:56:E2:1D:CE:EE:ED:24:E0:9F:1D:4B:A3:BF
    Signature Algorithm: ecdsa-with-SHA256
    Signature Value:
        30:46:02:21:00:9f:57:ba:de:8e:1a:82:1f:02:11:87:8d:00:
        fa:a2:eb:85:43:0d:c2:57:c6:12:c6:65:3b:e6:e9:aa:66:13:
        9e:02:21:00:fc:7f:a9:58:79:ba:bf:51:50:21:8f:f8:6e:89:
        c9:2c:14:fd:9f:9b:dd:f7:15:a9:6e:8d:9e:fe:0d:df:4e:35
*/
static const char kInvalidEECertificateWithCriticalAKID[] = R"(
-----BEGIN CERTIFICATE-----
MIICcDCCAhWgAwIBAgIUJ4z3FxZHVsBYMmzdZQkQa0S7DqcwCgYIKoZIzj0EAwIw
YDELMAkGA1UEBhMCVVMxEzARBgNVBAgMCldhc2hpbmd0b24xFjAUBgNVBAoMDUFX
UyBMaWJjcnlwdG8xEDAOBgNVBAsMB0dvb2QgQ0ExEjAQBgNVBAMMCVJvb3QgQ0Eg
MTAgFw0xNTAxMDEwMDAwMDBaGA8yMTAwMDEwMTAwMDAwMFowgYYxCzAJBgNVBAYT
AlVTMRMwEQYDVQQIDApXYXNoaW5ndG9uMRYwFAYDVQQKDA1BV1MgTGliY3J5cHRv
MRUwEwYDVQQLDAxCYWQgRW5kcG9pbnQxMzAxBgNVBAMMKlJGQyA1MjgwIDQuMi4x
LjEgQUtJRCBNVVNUIGJlIG5vbi1jcml0aWNhbDBZMBMGByqGSM49AgEGCCqGSM49
AwEHA0IABLK3vTXy69qG1dxARMcjFPnQpUAXMIW2xhE4wtssxbwMGRHYaGHWo5Jr
ihhSLNyGp60prZGsft+HJDvztHErTlijgYMwgYAwDgYDVR0PAQH/BAQDAgWgMAwG
A1UdEwEB/wQCMAAwHQYDVR0lBBYwFAYIKwYBBQUHAwEGCCsGAQUFBwMCMCIGA1Ud
IwEB/wQYMBaAFBkZ4YwJ4l1cFgThnHRmGf24UlvfMB0GA1UdDgQWBBTIeGTp95wP
VuIdzu7tJOCfHUujvzAKBggqhkjOPQQDAgNJADBGAiEAn1e63o4agh8CEYeNAPqi
64VDDcJXxhLGZTvm6apmE54CIQD8f6lYebq/UVAhj/huicksFP2fm933FalujZ7+
Dd9ONQ==
-----END CERTIFICATE-----
)";

/*
This certificate has a CRL Distribution Points extension, that per RFC 5280:
"The [CRL distribution points] extension SHOULD be non-critical, but this
profile RECOMMENDS support for this extension by CAs and applications."

OpenSSL 1.1.1 supports this extension being marked as critical, and will not
fail certificate verification because so.

Certificate:
    Data:
        Version: 3 (0x2)
        Serial Number:
            78:ea:d8:8b:b6:51:24:24:05:ed:24:af:8f:d5:1f:e2:43:bb:f6:1c
        Signature Algorithm: ecdsa-with-SHA256
        Issuer: C = US, ST = Washington, O = AWS Libcrypto, OU = Good CA, CN = Root CA 1
        Validity
            Not Before: Jan  1 00:00:00 2015 GMT
            Not After : Jan  1 00:00:00 2100 GMT
        Subject: C = US, ST = Washington, O = AWS Libcrypto, OU = Good Endpoint, CN = RFC 5280 4.2.1.13, SN = CRL distribution points ... extension SHOULD be non-critical
        Subject Public Key Info:
            Public Key Algorithm: id-ecPublicKey
                Public-Key: (256 bit)
                pub:
                    04:b2:b7:bd:35:f2:eb:da:86:d5:dc:40:44:c7:23:
                    14:f9:d0:a5:40:17:30:85:b6:c6:11:38:c2:db:2c:
                    c5:bc:0c:19:11:d8:68:61:d6:a3:92:6b:8a:18:52:
                    2c:dc:86:a7:ad:29:ad:91:ac:7e:df:87:24:3b:f3:
                    b4:71:2b:4e:58
                ASN1 OID: prime256v1
                NIST CURVE: P-256
        X509v3 extensions:
            X509v3 Key Usage: critical
                Digital Signature, Key Encipherment
            X509v3 Basic Constraints: critical
                CA:FALSE
            X509v3 Extended Key Usage:
                TLS Web Server Authentication, TLS Web Client Authentication
            X509v3 CRL Distribution Points: critical
                Full Name:
                  URI:http://example.com/crl
            X509v3 Subject Key Identifier:
                C8:78:64:E9:F7:9C:0F:56:E2:1D:CE:EE:ED:24:E0:9F:1D:4B:A3:BF
            X509v3 Authority Key Identifier:
                19:19:E1:8C:09:E2:5D:5C:16:04:E1:9C:74:66:19:FD:B8:52:5B:DF
    Signature Algorithm: ecdsa-with-SHA256
    Signature Value:
        30:46:02:21:00:cc:41:52:6e:40:01:46:d1:5e:4c:5b:23:27:
        55:ea:02:55:60:62:10:0c:9b:45:65:9a:a4:5b:9b:74:72:fa:
        c4:02:21:00:ba:2f:dc:ba:96:6d:ae:f3:19:3e:66:aa:18:9b:
        c5:ec:61:53:5a:d6:25:e5:66:bf:f3:9b:d6:d9:d2:e3:88:63
*/
static char kValidEECertWithCriticalCRLDistributionExt[] = R"(
-----BEGIN CERTIFICATE-----
MIICyDCCAm2gAwIBAgIUeOrYi7ZRJCQF7SSvj9Uf4kO79hwwCgYIKoZIzj0EAwIw
YDELMAkGA1UEBhMCVVMxEzARBgNVBAgMCldhc2hpbmd0b24xFjAUBgNVBAoMDUFX
UyBMaWJjcnlwdG8xEDAOBgNVBAsMB0dvb2QgQ0ExEjAQBgNVBAMMCVJvb3QgQ0Eg
MTAgFw0xNTAxMDEwMDAwMDBaGA8yMTAwMDEwMTAwMDAwMFowgbUxCzAJBgNVBAYT
AlVTMRMwEQYDVQQIDApXYXNoaW5ndG9uMRYwFAYDVQQKDA1BV1MgTGliY3J5cHRv
MRYwFAYDVQQLDA1Hb29kIEVuZHBvaW50MRowGAYDVQQDDBFSRkMgNTI4MCA0LjIu
MS4xMzFFMEMGA1UEBAw8Q1JMIGRpc3RyaWJ1dGlvbiBwb2ludHMgLi4uIGV4dGVu
c2lvbiBTSE9VTEQgYmUgbm9uLWNyaXRpY2FsMFkwEwYHKoZIzj0CAQYIKoZIzj0D
AQcDQgAEsre9NfLr2obV3EBExyMU+dClQBcwhbbGETjC2yzFvAwZEdhoYdajkmuK
GFIs3IanrSmtkax+34ckO/O0cStOWKOBrDCBqTAOBgNVHQ8BAf8EBAMCBaAwDAYD
VR0TAQH/BAIwADAdBgNVHSUEFjAUBggrBgEFBQcDAQYIKwYBBQUHAwIwKgYDVR0f
AQH/BCAwHjAcoBqgGIYWaHR0cDovL2V4YW1wbGUuY29tL2NybDAdBgNVHQ4EFgQU
yHhk6fecD1biHc7u7STgnx1Lo78wHwYDVR0jBBgwFoAUGRnhjAniXVwWBOGcdGYZ
/bhSW98wCgYIKoZIzj0EAwIDSQAwRgIhAMxBUm5AAUbRXkxbIydV6gJVYGIQDJtF
ZZqkW5t0cvrEAiEAui/cupZtrvMZPmaqGJvF7GFTWtYl5Wa/85vW2dLjiGM=
-----END CERTIFICATE-----
)";

/*
This is a v3 certificate that has the keyCertSign keyUsage bit set but is
missing the Basic Constraints extension. Per RFC 5280 4.2.1.9: "If the basic
constraints extension is not present in a version 3 certificate, or the
extension is present but the cA boolean is not asserted, then the certified
public key MUST NOT be used to verify certificate signatures."

Certificate:
    Data:
        Version: 3 (0x2)
        Serial Number:
            3e:d2:f9:bf:f8:43:a5:8a:69:cb:8f:6e:e6:29:43:a3:b8:be:2c:e1
        Signature Algorithm: ecdsa-with-SHA256
        Issuer: C = US, ST = Washington, O = AWS Libcrypto, OU = Bad CA, CN = RFC 528 4.2.1.9 not present, SN = MUST NOT be used to verify certificate signatures
        Validity
            Not Before: Jan  1 00:00:00 2015 GMT
            Not After : Jan  1 00:00:00 2100 GMT
        Subject: C = US, ST = Washington, O = AWS Libcrypto, OU = Bad CA, CN = RFC 528 4.2.1.9 not present, SN = MUST NOT be used to verify certificate signatures
        Subject Public Key Info:
            Public Key Algorithm: id-ecPublicKey
                Public-Key: (256 bit)
                pub:
                    04:c2:a9:7a:df:6d:d4:e3:16:29:b3:74:11:f1:8f:
                    bd:44:8d:c7:3f:b2:d7:9a:e7:10:14:ff:1c:4a:fa:
                    ce:fb:c9:7c:c1:e6:57:f2:ff:31:1b:71:8c:6e:3a:
                    b9:f0:1e:b5:ac:5f:db:2e:81:68:02:cb:be:19:44:
                    d5:89:3d:30:d6
                ASN1 OID: prime256v1
                NIST CURVE: P-256
        X509v3 extensions:
            X509v3 Key Usage: critical
                Digital Signature, Certificate Sign, CRL Sign
            X509v3 Subject Key Identifier:
                19:19:E1:8C:09:E2:5D:5C:16:04:E1:9C:74:66:19:FD:B8:52:5B:DF
    Signature Algorithm: ecdsa-with-SHA256
    Signature Value:
        30:44:02:20:35:fb:3a:0f:95:a5:bf:2d:bc:74:91:f9:f5:0f:
        bb:79:34:dc:e7:b5:cb:c4:21:5a:be:4d:10:e1:3e:97:e0:b8:
        02:20:54:2e:9c:98:89:3a:11:ec:7a:34:40:64:84:3f:b1:72:
        b1:bb:33:a2:d2:29:aa:ab:c1:1d:38:44:fa:62:fb:20
*/
static char kInvalidRootCertificateWithMissingBasicConstraintsExt[] = R"(
-----BEGIN CERTIFICATE-----
MIICkDCCAjegAwIBAgIUPtL5v/hDpYppy49u5ilDo7i+LOEwCgYIKoZIzj0EAwIw
ga0xCzAJBgNVBAYTAlVTMRMwEQYDVQQIDApXYXNoaW5ndG9uMRYwFAYDVQQKDA1B
V1MgTGliY3J5cHRvMQ8wDQYDVQQLDAZCYWQgQ0ExJDAiBgNVBAMMG1JGQyA1Mjgg
NC4yLjEuOSBub3QgcHJlc2VudDE6MDgGA1UEBAwxTVVTVCBOT1QgYmUgdXNlZCB0
byB2ZXJpZnkgY2VydGlmaWNhdGUgc2lnbmF0dXJlczAgFw0xNTAxMDEwMDAwMDBa
GA8yMTAwMDEwMTAwMDAwMFowga0xCzAJBgNVBAYTAlVTMRMwEQYDVQQIDApXYXNo
aW5ndG9uMRYwFAYDVQQKDA1BV1MgTGliY3J5cHRvMQ8wDQYDVQQLDAZCYWQgQ0Ex
JDAiBgNVBAMMG1JGQyA1MjggNC4yLjEuOSBub3QgcHJlc2VudDE6MDgGA1UEBAwx
TVVTVCBOT1QgYmUgdXNlZCB0byB2ZXJpZnkgY2VydGlmaWNhdGUgc2lnbmF0dXJl
czBZMBMGByqGSM49AgEGCCqGSM49AwEHA0IABMKpet9t1OMWKbN0EfGPvUSNxz+y
15rnEBT/HEr6zvvJfMHmV/L/MRtxjG46ufAetaxf2y6BaALLvhlE1Yk9MNajMTAv
MA4GA1UdDwEB/wQEAwIBhjAdBgNVHQ4EFgQUGRnhjAniXVwWBOGcdGYZ/bhSW98w
CgYIKoZIzj0EAwIDRwAwRAIgNfs6D5Wlvy28dJH59Q+7eTTc57XLxCFavk0Q4T6X
4LgCIFQunJiJOhHsejRAZIQ/sXKxuzOi0imqq8EdOET6Yvsg
-----END CERTIFICATE-----
)";

/*
This is a bad endpoint certificate that has been signed by
|kInvalidRootCertificateWithMissingBasicConstraintsExt| which is an invalid CA
due to it missing the Basic Constraints extension per RFC 528 4.2.1.9.

Certificate:
    Data:
        Version: 3 (0x2)
        Serial Number:
            64:73:40:dd:b0:f3:e9:45:6f:12:bf:f8:76:46:ef:77:f0:8d:02:2a
        Signature Algorithm: ecdsa-with-SHA256
        Issuer: C = US, ST = Washington, O = AWS Libcrypto, OU = Bad CA, CN = RFC 528 4.2.1.9 not present, SN = MUST NOT be used to verify certificate signatures
        Validity
            Not Before: Jan  1 00:00:00 2015 GMT
            Not After : Jan  1 00:00:00 2100 GMT
        Subject: C = US, ST = Washington, O = AWS Libcrypto, OU = Bad Endpoint, CN = RFC 528 4.2.1.9 not present, SN = MUST NOT be used to verify certificate signatures
        Subject Public Key Info:
            Public Key Algorithm: id-ecPublicKey
                Public-Key: (256 bit)
                pub:
                    04:b2:b7:bd:35:f2:eb:da:86:d5:dc:40:44:c7:23:
                    14:f9:d0:a5:40:17:30:85:b6:c6:11:38:c2:db:2c:
                    c5:bc:0c:19:11:d8:68:61:d6:a3:92:6b:8a:18:52:
                    2c:dc:86:a7:ad:29:ad:91:ac:7e:df:87:24:3b:f3:
                    b4:71:2b:4e:58
                ASN1 OID: prime256v1
                NIST CURVE: P-256
        X509v3 extensions:
            X509v3 Key Usage: critical
                Digital Signature, Key Encipherment
            X509v3 Basic Constraints: critical
                CA:FALSE
            X509v3 Extended Key Usage:
                TLS Web Server Authentication, TLS Web Client Authentication
            X509v3 Subject Key Identifier:
                C8:78:64:E9:F7:9C:0F:56:E2:1D:CE:EE:ED:24:E0:9F:1D:4B:A3:BF
            X509v3 Authority Key Identifier:
                19:19:E1:8C:09:E2:5D:5C:16:04:E1:9C:74:66:19:FD:B8:52:5B:DF
    Signature Algorithm: ecdsa-with-SHA256
    Signature Value:
        30:44:02:20:47:a2:ae:cc:22:a1:00:17:00:db:d6:f9:1d:73:
        09:c3:d4:cf:4f:f2:e0:2c:e9:3d:14:2f:46:c9:c7:73:c1:dd:
        02:20:3f:6a:d3:15:10:f6:38:fe:84:90:06:08:17:f7:cf:37:
        b7:9a:a2:6e:b1:ba:38:ba:ca:0f:c0:52:06:10:a5:4c
*/
static char kInvalidEECertificateSignedByRootMissingBasicConstraintsExt[] = R"(
-----BEGIN CERTIFICATE-----
MIIC5DCCAougAwIBAgIUZHNA3bDz6UVvEr/4dkbvd/CNAiowCgYIKoZIzj0EAwIw
ga0xCzAJBgNVBAYTAlVTMRMwEQYDVQQIDApXYXNoaW5ndG9uMRYwFAYDVQQKDA1B
V1MgTGliY3J5cHRvMQ8wDQYDVQQLDAZCYWQgQ0ExJDAiBgNVBAMMG1JGQyA1Mjgg
NC4yLjEuOSBub3QgcHJlc2VudDE6MDgGA1UEBAwxTVVTVCBOT1QgYmUgdXNlZCB0
byB2ZXJpZnkgY2VydGlmaWNhdGUgc2lnbmF0dXJlczAgFw0xNTAxMDEwMDAwMDBa
GA8yMTAwMDEwMTAwMDAwMFowgbMxCzAJBgNVBAYTAlVTMRMwEQYDVQQIDApXYXNo
aW5ndG9uMRYwFAYDVQQKDA1BV1MgTGliY3J5cHRvMRUwEwYDVQQLDAxCYWQgRW5k
cG9pbnQxJDAiBgNVBAMMG1JGQyA1MjggNC4yLjEuOSBub3QgcHJlc2VudDE6MDgG
A1UEBAwxTVVTVCBOT1QgYmUgdXNlZCB0byB2ZXJpZnkgY2VydGlmaWNhdGUgc2ln
bmF0dXJlczBZMBMGByqGSM49AgEGCCqGSM49AwEHA0IABLK3vTXy69qG1dxARMcj
FPnQpUAXMIW2xhE4wtssxbwMGRHYaGHWo5JrihhSLNyGp60prZGsft+HJDvztHEr
TlijfzB9MA4GA1UdDwEB/wQEAwIFoDAMBgNVHRMBAf8EAjAAMB0GA1UdJQQWMBQG
CCsGAQUFBwMBBggrBgEFBQcDAjAdBgNVHQ4EFgQUyHhk6fecD1biHc7u7STgnx1L
o78wHwYDVR0jBBgwFoAUGRnhjAniXVwWBOGcdGYZ/bhSW98wCgYIKoZIzj0EAwID
RwAwRAIgR6KuzCKhABcA29b5HXMJw9TPT/LgLOk9FC9Gycdzwd0CID9q0xUQ9jj+
hJAGCBf3zze3mqJusbo4usoPwFIGEKVM
-----END CERTIFICATE-----
)";

/*
This is technically an invalid certificate due to it having a negative serial
number which is not valid per RFC 5280 4.1.2.2: "The serial number MUST be a
positive integer assigned by the CA to each certificate".

Historically OpenSSL 1.1.1 supports negative serial numbers as had other
implementations.

Certificate is signed by |kValidRootCA1|.

Certificate:
    Data:
        Version: 3 (0x2)
        Serial Number: -1337 (-0x539)
        Signature Algorithm: ecdsa-with-SHA256
        Issuer: C = US, ST = Washington, O = AWS Libcrypto, OU = Good CA, CN = Root CA 1
        Validity
            Not Before: Jan  1 00:00:00 2015 GMT
            Not After : Jan  1 00:00:00 2100 GMT
        Subject: C = US, ST = Washington, O = AWS Libcrypto, OU = Bad Endpoint, CN = RFC 5280 serial number MUST be a positive integer
        Subject Public Key Info:
            Public Key Algorithm: id-ecPublicKey
                Public-Key: (256 bit)
                pub:
                    04:b2:b7:bd:35:f2:eb:da:86:d5:dc:40:44:c7:23:
                    14:f9:d0:a5:40:17:30:85:b6:c6:11:38:c2:db:2c:
                    c5:bc:0c:19:11:d8:68:61:d6:a3:92:6b:8a:18:52:
                    2c:dc:86:a7:ad:29:ad:91:ac:7e:df:87:24:3b:f3:
                    b4:71:2b:4e:58
                ASN1 OID: prime256v1
                NIST CURVE: P-256
        X509v3 extensions:
            X509v3 Key Usage: critical
                Digital Signature, Key Encipherment
            X509v3 Basic Constraints: critical
                CA:FALSE
            X509v3 Extended Key Usage:
                TLS Web Server Authentication, TLS Web Client Authentication
            X509v3 Subject Key Identifier:
                C8:78:64:E9:F7:9C:0F:56:E2:1D:CE:EE:ED:24:E0:9F:1D:4B:A3:BF
            X509v3 Authority Key Identifier:
                19:19:E1:8C:09:E2:5D:5C:16:04:E1:9C:74:66:19:FD:B8:52:5B:DF
    Signature Algorithm: ecdsa-with-SHA256
    Signature Value:
        30:45:02:21:00:a1:0d:15:19:11:bc:84:2f:9c:64:ae:c1:89:
        c6:37:90:df:c9:36:f0:bf:e5:4f:5b:53:54:48:55:dd:e0:f3:
        8c:02:20:21:22:ff:f5:9b:79:55:03:04:86:92:e1:c5:b2:11:
        6d:7f:f8:77:23:e4:c0:09:53:c0:01:07:3d:f5:00:77:ec
*/
static char kValidEECertificateWithNegativeSerialNumber[] = R"(
-----BEGIN CERTIFICATE-----
MIICXzCCAgWgAwIBAgIC+scwCgYIKoZIzj0EAwIwYDELMAkGA1UEBhMCVVMxEzAR
BgNVBAgMCldhc2hpbmd0b24xFjAUBgNVBAoMDUFXUyBMaWJjcnlwdG8xEDAOBgNV
BAsMB0dvb2QgQ0ExEjAQBgNVBAMMCVJvb3QgQ0EgMTAgFw0xNTAxMDEwMDAwMDBa
GA8yMTAwMDEwMTAwMDAwMFowgY0xCzAJBgNVBAYTAlVTMRMwEQYDVQQIDApXYXNo
aW5ndG9uMRYwFAYDVQQKDA1BV1MgTGliY3J5cHRvMRUwEwYDVQQLDAxCYWQgRW5k
cG9pbnQxOjA4BgNVBAMMMVJGQyA1MjgwIHNlcmlhbCBudW1iZXIgTVVTVCBiZSBh
IHBvc2l0aXZlIGludGVnZXIwWTATBgcqhkjOPQIBBggqhkjOPQMBBwNCAASyt701
8uvahtXcQETHIxT50KVAFzCFtsYROMLbLMW8DBkR2Ghh1qOSa4oYUizchqetKa2R
rH7fhyQ787RxK05Yo38wfTAOBgNVHQ8BAf8EBAMCBaAwDAYDVR0TAQH/BAIwADAd
BgNVHSUEFjAUBggrBgEFBQcDAQYIKwYBBQUHAwIwHQYDVR0OBBYEFMh4ZOn3nA9W
4h3O7u0k4J8dS6O/MB8GA1UdIwQYMBaAFBkZ4YwJ4l1cFgThnHRmGf24UlvfMAoG
CCqGSM49BAMCA0gAMEUCIQChDRUZEbyEL5xkrsGJxjeQ38k28L/lT1tTVEhV3eDz
jAIgISL/9Zt5VQMEhpLhxbIRbX/4dyPkwAlTwAEHPfUAd+w=
-----END CERTIFICATE-----
)";

/*
Intermediate CA signed by |kValidRootCA1|.

Certificate:
    Data:
        Version: 3 (0x2)
        Serial Number:
            31:b6:83:37:da:68:56:9c:ac:0a:fd:0c:f1:7b:d1:8c:c9:23:e7:8f
        Signature Algorithm: ecdsa-with-SHA256
        Issuer: C=US, ST=Washington, O=AWS Libcrypto, OU=Good CA, CN=Root CA 1
        Validity
            Not Before: Jan  1 00:00:00 2015 GMT
            Not After : Jan  1 00:00:00 2100 GMT
        Subject: C=US, ST=Washington, O=AWS Libcrypto, OU=Good CA, CN=Int NC CA 1
        Subject Public Key Info:
            Public Key Algorithm: id-ecPublicKey
                Public-Key: (256 bit)
                pub:
                    04:9e:64:d1:8c:e1:e7:f0:3b:a5:22:df:04:2d:54:
                    fa:e8:cc:b1:22:e7:8b:c6:60:d9:72:cd:ad:1a:16:
                    dc:34:9f:99:e8:22:80:95:de:19:5b:0a:4e:d6:d6:
                    47:5a:b7:44:32:17:3f:6b:32:80:e5:0e:35:cb:68:
                    62:c4:dd:31:d1
                ASN1 OID: prime256v1
                NIST CURVE: P-256
        X509v3 extensions:
            X509v3 Key Usage: critical
                Digital Signature, Certificate Sign, CRL Sign
            X509v3 Basic Constraints: critical
                CA:TRUE
            X509v3 Name Constraints: critical
                Permitted:
                  IP:10.1.0.0/255.255.0.0
                Excluded:
                  IP:10.1.1.0/255.255.255.0
            X509v3 Subject Key Identifier:
                B9:D1:CF:08:A5:F2:99:2B:5A:80:C9:78:23:78:D1:BC:79:70:9F:FB
            X509v3 Authority Key Identifier:
                19:19:E1:8C:09:E2:5D:5C:16:04:E1:9C:74:66:19:FD:B8:52:5B:DF
    Signature Algorithm: ecdsa-with-SHA256
    Signature Value:
        30:45:02:20:0f:7a:b1:66:83:b6:b7:70:af:14:ae:0f:98:3d:
        36:5c:9e:65:8f:e7:3c:f5:05:00:b4:14:53:10:15:2b:c5:7d:
        02:21:00:ac:53:f8:9c:65:e2:38:7f:66:da:96:b2:03:ad:68:
        57:30:30:8a:4c:0f:2b:4b:9f:36:ea:1f:e3:8e:25:14:9b
*/
static char kValidIntCAWithIPv4NameConstraints[] = R"(
-----BEGIN CERTIFICATE-----
MIICVTCCAfugAwIBAgIUMbaDN9poVpysCv0M8XvRjMkj548wCgYIKoZIzj0EAwIw
YDELMAkGA1UEBhMCVVMxEzARBgNVBAgMCldhc2hpbmd0b24xFjAUBgNVBAoMDUFX
UyBMaWJjcnlwdG8xEDAOBgNVBAsMB0dvb2QgQ0ExEjAQBgNVBAMMCVJvb3QgQ0Eg
MTAgFw0xNTAxMDEwMDAwMDBaGA8yMTAwMDEwMTAwMDAwMFowYjELMAkGA1UEBhMC
VVMxEzARBgNVBAgMCldhc2hpbmd0b24xFjAUBgNVBAoMDUFXUyBMaWJjcnlwdG8x
EDAOBgNVBAsMB0dvb2QgQ0ExFDASBgNVBAMMC0ludCBOQyBDQSAxMFkwEwYHKoZI
zj0CAQYIKoZIzj0DAQcDQgAEnmTRjOHn8DulIt8ELVT66MyxIueLxmDZcs2tGhbc
NJ+Z6CKAld4ZWwpO1tZHWrdEMhc/azKA5Q41y2hixN0x0aOBjjCBizAOBgNVHQ8B
Af8EBAMCAYYwDwYDVR0TAQH/BAUwAwEB/zAoBgNVHR4BAf8EHjAcoAwwCocICgEA
AP//AAChDDAKhwgKAQEA////ADAdBgNVHQ4EFgQUudHPCKXymStagMl4I3jRvHlw
n/swHwYDVR0jBBgwFoAUGRnhjAniXVwWBOGcdGYZ/bhSW98wCgYIKoZIzj0EAwID
SAAwRQIgD3qxZoO2t3CvFK4PmD02XJ5lj+c89QUAtBRTEBUrxX0CIQCsU/icZeI4
f2balrIDrWhXMDCKTA8rS5826h/jjiUUmw==
-----END CERTIFICATE-----
)";

/*
End-entity certificate signed by |kValidIntCAWithIPv4NameConstraints|.

Certificate:
    Data:
        Version: 3 (0x2)
        Serial Number:
            3c:e0:97:cf:b4:c1:15:28:aa:a8:21:76:23:3b:ad:08:31:18:a5:4f
        Signature Algorithm: ecdsa-with-SHA256
        Issuer: C=US, ST=Washington, O=AWS Libcrypto, OU=Good CA, CN=Int NC CA 1
        Validity
            Not Before: Jan  1 00:00:00 2015 GMT
            Not After : Jan  1 00:00:00 2100 GMT
        Subject: C=US, ST=Washington, O=AWS Libcrypto, OU=Good Endpoint, CN=Permitted IP
        Subject Public Key Info:
            Public Key Algorithm: id-ecPublicKey
                Public-Key: (256 bit)
                pub:
                    04:b2:b7:bd:35:f2:eb:da:86:d5:dc:40:44:c7:23:
                    14:f9:d0:a5:40:17:30:85:b6:c6:11:38:c2:db:2c:
                    c5:bc:0c:19:11:d8:68:61:d6:a3:92:6b:8a:18:52:
                    2c:dc:86:a7:ad:29:ad:91:ac:7e:df:87:24:3b:f3:
                    b4:71:2b:4e:58
                ASN1 OID: prime256v1
                NIST CURVE: P-256
        X509v3 extensions:
            X509v3 Key Usage: critical
                Digital Signature, Key Encipherment
            X509v3 Basic Constraints: critical
                CA:FALSE
            X509v3 Extended Key Usage:
                TLS Web Server Authentication, TLS Web Client Authentication
            X509v3 Subject Alternative Name:
                DNS:example.com, IP Address:10.1.2.1
            X509v3 Subject Key Identifier:
                C8:78:64:E9:F7:9C:0F:56:E2:1D:CE:EE:ED:24:E0:9F:1D:4B:A3:BF
            X509v3 Authority Key Identifier:
                B9:D1:CF:08:A5:F2:99:2B:5A:80:C9:78:23:78:D1:BC:79:70:9F:FB
    Signature Algorithm: ecdsa-with-SHA256
    Signature Value:
        30:45:02:21:00:da:b5:af:58:58:38:69:be:d1:c0:38:b3:7c:
        2e:18:fe:5a:a8:7e:2b:85:44:b2:d8:66:78:bf:fa:b7:44:9d:
        a7:02:20:7a:80:35:24:95:08:5e:98:af:3e:4a:4a:a5:8d:11:
        06:90:cf:b8:34:25:04:94:86:a4:b4:9b:84:b9:61:00:65
*/
static char kValidEECertWithValidIPv4SubjectAltName[] = R"(
-----BEGIN CERTIFICATE-----
MIICbjCCAhSgAwIBAgIUPOCXz7TBFSiqqCF2IzutCDEYpU8wCgYIKoZIzj0EAwIw
YjELMAkGA1UEBhMCVVMxEzARBgNVBAgMCldhc2hpbmd0b24xFjAUBgNVBAoMDUFX
UyBMaWJjcnlwdG8xEDAOBgNVBAsMB0dvb2QgQ0ExFDASBgNVBAMMC0ludCBOQyBD
QSAxMCAXDTE1MDEwMTAwMDAwMFoYDzIxMDAwMTAxMDAwMDAwWjBpMQswCQYDVQQG
EwJVUzETMBEGA1UECAwKV2FzaGluZ3RvbjEWMBQGA1UECgwNQVdTIExpYmNyeXB0
bzEWMBQGA1UECwwNR29vZCBFbmRwb2ludDEVMBMGA1UEAwwMUGVybWl0dGVkIElQ
MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEsre9NfLr2obV3EBExyMU+dClQBcw
hbbGETjC2yzFvAwZEdhoYdajkmuKGFIs3IanrSmtkax+34ckO/O0cStOWKOBnjCB
mzAOBgNVHQ8BAf8EBAMCBaAwDAYDVR0TAQH/BAIwADAdBgNVHSUEFjAUBggrBgEF
BQcDAQYIKwYBBQUHAwIwHAYDVR0RBBUwE4ILZXhhbXBsZS5jb22HBAoBAgEwHQYD
VR0OBBYEFMh4ZOn3nA9W4h3O7u0k4J8dS6O/MB8GA1UdIwQYMBaAFLnRzwil8pkr
WoDJeCN40bx5cJ/7MAoGCCqGSM49BAMCA0gAMEUCIQData9YWDhpvtHAOLN8Lhj+
Wqh+K4VEsthmeL/6t0SdpwIgeoA1JJUIXpivPkpKpY0RBpDPuDQlBJSGpLSbhLlh
AGU=
-----END CERTIFICATE-----
)";

/*
An invalid end-entity certificate signed by |kValidIntCAWithIPv4NameConstraints|
with an excluded IP address in the subject alternative name.

Certificate:
    Data:
        Version: 3 (0x2)
        Serial Number:
            33:19:62:14:a9:8a:02:74:a0:ba:55:ad:73:c9:a4:7e:83:ce:18:7e
        Signature Algorithm: ecdsa-with-SHA256
        Issuer: C=US, ST=Washington, O=AWS Libcrypto, OU=Good CA, CN=Int NC CA 1
        Validity
            Not Before: Jan  1 00:00:00 2015 GMT
            Not After : Jan  1 00:00:00 2100 GMT
        Subject: C=US, ST=Washington, O=AWS Libcrypto, OU=Bad Endpoint, CN=Excluded IP
        Subject Public Key Info:
            Public Key Algorithm: id-ecPublicKey
                Public-Key: (256 bit)
                pub:
                    04:b2:b7:bd:35:f2:eb:da:86:d5:dc:40:44:c7:23:
                    14:f9:d0:a5:40:17:30:85:b6:c6:11:38:c2:db:2c:
                    c5:bc:0c:19:11:d8:68:61:d6:a3:92:6b:8a:18:52:
                    2c:dc:86:a7:ad:29:ad:91:ac:7e:df:87:24:3b:f3:
                    b4:71:2b:4e:58
                ASN1 OID: prime256v1
                NIST CURVE: P-256
        X509v3 extensions:
            X509v3 Key Usage: critical
                Digital Signature, Key Encipherment
            X509v3 Basic Constraints: critical
                CA:FALSE
            X509v3 Extended Key Usage:
                TLS Web Server Authentication, TLS Web Client Authentication
            X509v3 Subject Alternative Name:
                DNS:example.com, IP Address:10.1.1.2
            X509v3 Subject Key Identifier:
                C8:78:64:E9:F7:9C:0F:56:E2:1D:CE:EE:ED:24:E0:9F:1D:4B:A3:BF
            X509v3 Authority Key Identifier:
                B9:D1:CF:08:A5:F2:99:2B:5A:80:C9:78:23:78:D1:BC:79:70:9F:FB
    Signature Algorithm: ecdsa-with-SHA256
    Signature Value:
        30:44:02:20:72:ec:70:f1:58:de:9c:66:8f:98:22:9d:ad:5a:
        b2:e2:4a:24:2f:82:43:0f:6d:17:e7:a6:3f:81:8a:e6:76:40:
        02:20:10:e0:6e:15:9b:9e:db:15:c8:39:5a:76:5f:17:f9:62:
        f3:8d:c7:87:0b:eb:bc:42:42:22:73:65:b5:82:01:91
*/
static char kInvalidEECertWithInvalidExcludedIPv4SubjAlt[] = R"(
-----BEGIN CERTIFICATE-----
MIICazCCAhKgAwIBAgIUMxliFKmKAnSgulWtc8mkfoPOGH4wCgYIKoZIzj0EAwIw
YjELMAkGA1UEBhMCVVMxEzARBgNVBAgMCldhc2hpbmd0b24xFjAUBgNVBAoMDUFX
UyBMaWJjcnlwdG8xEDAOBgNVBAsMB0dvb2QgQ0ExFDASBgNVBAMMC0ludCBOQyBD
QSAxMCAXDTE1MDEwMTAwMDAwMFoYDzIxMDAwMTAxMDAwMDAwWjBnMQswCQYDVQQG
EwJVUzETMBEGA1UECAwKV2FzaGluZ3RvbjEWMBQGA1UECgwNQVdTIExpYmNyeXB0
bzEVMBMGA1UECwwMQmFkIEVuZHBvaW50MRQwEgYDVQQDDAtFeGNsdWRlZCBJUDBZ
MBMGByqGSM49AgEGCCqGSM49AwEHA0IABLK3vTXy69qG1dxARMcjFPnQpUAXMIW2
xhE4wtssxbwMGRHYaGHWo5JrihhSLNyGp60prZGsft+HJDvztHErTlijgZ4wgZsw
DgYDVR0PAQH/BAQDAgWgMAwGA1UdEwEB/wQCMAAwHQYDVR0lBBYwFAYIKwYBBQUH
AwEGCCsGAQUFBwMCMBwGA1UdEQQVMBOCC2V4YW1wbGUuY29thwQKAQECMB0GA1Ud
DgQWBBTIeGTp95wPVuIdzu7tJOCfHUujvzAfBgNVHSMEGDAWgBS50c8IpfKZK1qA
yXgjeNG8eXCf+zAKBggqhkjOPQQDAgNHADBEAiBy7HDxWN6cZo+YIp2tWrLiSiQv
gkMPbRfnpj+BiuZ2QAIgEOBuFZue2xXIOVp2Xxf5YvONx4cL67xCQiJzZbWCAZE=
-----END CERTIFICATE-----
)";

/*
Signed by |kValidIntCAWithIPv4NameConstraints|, but with an invalid permitted ip address.

Certificate:
    Data:
        Version: 3 (0x2)
        Serial Number:
            7a:15:18:c0:d9:39:b6:d9:8e:9a:29:87:d8:6a:ad:ce:5b:90:cc:49
        Signature Algorithm: ecdsa-with-SHA256
        Issuer: C=US, ST=Washington, O=AWS Libcrypto, OU=Good CA, CN=Int NC CA 1
        Validity
            Not Before: Jan  1 00:00:00 2015 GMT
            Not After : Jan  1 00:00:00 2100 GMT
        Subject: C=US, ST=Washington, O=AWS Libcrypto, OU=Bad Endpoint, CN=Not Permitted IP
        Subject Public Key Info:
            Public Key Algorithm: id-ecPublicKey
                Public-Key: (256 bit)
                pub:
                    04:b2:b7:bd:35:f2:eb:da:86:d5:dc:40:44:c7:23:
                    14:f9:d0:a5:40:17:30:85:b6:c6:11:38:c2:db:2c:
                    c5:bc:0c:19:11:d8:68:61:d6:a3:92:6b:8a:18:52:
                    2c:dc:86:a7:ad:29:ad:91:ac:7e:df:87:24:3b:f3:
                    b4:71:2b:4e:58
                ASN1 OID: prime256v1
                NIST CURVE: P-256
        X509v3 extensions:
            X509v3 Key Usage: critical
                Digital Signature, Key Encipherment
            X509v3 Basic Constraints: critical
                CA:FALSE
            X509v3 Extended Key Usage:
                TLS Web Server Authentication, TLS Web Client Authentication
            X509v3 Subject Alternative Name:
                DNS:example.com, IP Address:10.2.0.1
            X509v3 Subject Key Identifier:
                C8:78:64:E9:F7:9C:0F:56:E2:1D:CE:EE:ED:24:E0:9F:1D:4B:A3:BF
            X509v3 Authority Key Identifier:
                B9:D1:CF:08:A5:F2:99:2B:5A:80:C9:78:23:78:D1:BC:79:70:9F:FB
    Signature Algorithm: ecdsa-with-SHA256
    Signature Value:
        30:45:02:21:00:de:8a:39:0b:90:fe:1b:37:f3:9f:ab:60:77:
        58:8b:79:1b:3a:06:ed:dc:91:85:98:b5:21:44:d5:a6:da:04:
        e9:02:20:22:60:af:9d:79:56:4c:bf:50:ae:eb:93:ac:fd:6e:
        d1:f9:60:d5:24:0e:09:00:33:8c:f1:46:6a:69:f8:24:03
*/
static char kInvalidEECertWithInvalidPermittedIPv4SubjAlt[] = R"(
-----BEGIN CERTIFICATE-----
MIICcTCCAhegAwIBAgIUehUYwNk5ttmOmimH2GqtzluQzEkwCgYIKoZIzj0EAwIw
YjELMAkGA1UEBhMCVVMxEzARBgNVBAgMCldhc2hpbmd0b24xFjAUBgNVBAoMDUFX
UyBMaWJjcnlwdG8xEDAOBgNVBAsMB0dvb2QgQ0ExFDASBgNVBAMMC0ludCBOQyBD
QSAxMCAXDTE1MDEwMTAwMDAwMFoYDzIxMDAwMTAxMDAwMDAwWjBsMQswCQYDVQQG
EwJVUzETMBEGA1UECAwKV2FzaGluZ3RvbjEWMBQGA1UECgwNQVdTIExpYmNyeXB0
bzEVMBMGA1UECwwMQmFkIEVuZHBvaW50MRkwFwYDVQQDDBBOb3QgUGVybWl0dGVk
IElQMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEsre9NfLr2obV3EBExyMU+dCl
QBcwhbbGETjC2yzFvAwZEdhoYdajkmuKGFIs3IanrSmtkax+34ckO/O0cStOWKOB
njCBmzAOBgNVHQ8BAf8EBAMCBaAwDAYDVR0TAQH/BAIwADAdBgNVHSUEFjAUBggr
BgEFBQcDAQYIKwYBBQUHAwIwHAYDVR0RBBUwE4ILZXhhbXBsZS5jb22HBAoCAAEw
HQYDVR0OBBYEFMh4ZOn3nA9W4h3O7u0k4J8dS6O/MB8GA1UdIwQYMBaAFLnRzwil
8pkrWoDJeCN40bx5cJ/7MAoGCCqGSM49BAMCA0gAMEUCIQDeijkLkP4bN/Ofq2B3
WIt5GzoG7dyRhZi1IUTVptoE6QIgImCvnXlWTL9QruuTrP1u0flg1SQOCQAzjPFG
amn4JAM=
-----END CERTIFICATE-----
)";

/*
Signed by |kValidRootCA1|.

Certificate:
    Data:
        Version: 3 (0x2)
        Serial Number:
            75:fb:e0:27:bc:8e:e1:39:d5:90:91:20:c7:40:ec:dd:d4:e8:13:a3
        Signature Algorithm: ecdsa-with-SHA256
        Issuer: C=US, ST=Washington, O=AWS Libcrypto, OU=Good CA, CN=Root CA 1
        Validity
            Not Before: Jan  1 00:00:00 2015 GMT
            Not After : Jan  1 00:00:00 2100 GMT
        Subject: C=US, ST=Washington, O=AWS Libcrypto, OU=Good CA, CN=Int NC CA 2
        Subject Public Key Info:
            Public Key Algorithm: id-ecPublicKey
                Public-Key: (256 bit)
                pub:
                    04:9e:64:d1:8c:e1:e7:f0:3b:a5:22:df:04:2d:54:
                    fa:e8:cc:b1:22:e7:8b:c6:60:d9:72:cd:ad:1a:16:
                    dc:34:9f:99:e8:22:80:95:de:19:5b:0a:4e:d6:d6:
                    47:5a:b7:44:32:17:3f:6b:32:80:e5:0e:35:cb:68:
                    62:c4:dd:31:d1
                ASN1 OID: prime256v1
                NIST CURVE: P-256
        X509v3 extensions:
            X509v3 Key Usage: critical
                Digital Signature, Certificate Sign, CRL Sign
            X509v3 Basic Constraints: critical
                CA:TRUE
            X509v3 Name Constraints: critical
                Permitted:
                  IP:0:0:0:0:0:FFFF:A01:0/FFFF:FFFF:FFFF:FFFF:FFFF:FFFF:FFFF:0
                Excluded:
                  IP:0:0:0:0:0:FFFF:A01:100/FFFF:FFFF:FFFF:FFFF:FFFF:FFFF:FFFF:FF00
            X509v3 Subject Key Identifier:
                B9:D1:CF:08:A5:F2:99:2B:5A:80:C9:78:23:78:D1:BC:79:70:9F:FB
            X509v3 Authority Key Identifier:
                19:19:E1:8C:09:E2:5D:5C:16:04:E1:9C:74:66:19:FD:B8:52:5B:DF
    Signature Algorithm: ecdsa-with-SHA256
    Signature Value:
        30:45:02:20:73:9d:b0:46:93:44:d4:8e:cc:b0:a0:dd:93:28:
        38:df:1f:cd:ff:52:91:4b:d7:6e:1f:9d:36:23:ad:77:b9:52:
        02:21:00:c8:e5:54:f3:2d:f3:f3:e9:7c:8b:03:60:94:75:5c:
        ca:66:94:5f:ac:87:ed:7b:53:52:2d:71:89:e1:d0:d2:fc
*/
static char kValidIntCAWithIPv6NameConstraints[] = R"(
-----BEGIN CERTIFICATE-----
MIIChTCCAiugAwIBAgIUdfvgJ7yO4TnVkJEgx0Ds3dToE6MwCgYIKoZIzj0EAwIw
YDELMAkGA1UEBhMCVVMxEzARBgNVBAgMCldhc2hpbmd0b24xFjAUBgNVBAoMDUFX
UyBMaWJjcnlwdG8xEDAOBgNVBAsMB0dvb2QgQ0ExEjAQBgNVBAMMCVJvb3QgQ0Eg
MTAgFw0xNTAxMDEwMDAwMDBaGA8yMTAwMDEwMTAwMDAwMFowYjELMAkGA1UEBhMC
VVMxEzARBgNVBAgMCldhc2hpbmd0b24xFjAUBgNVBAoMDUFXUyBMaWJjcnlwdG8x
EDAOBgNVBAsMB0dvb2QgQ0ExFDASBgNVBAMMC0ludCBOQyBDQSAyMFkwEwYHKoZI
zj0CAQYIKoZIzj0DAQcDQgAEnmTRjOHn8DulIt8ELVT66MyxIueLxmDZcs2tGhbc
NJ+Z6CKAld4ZWwpO1tZHWrdEMhc/azKA5Q41y2hixN0x0aOBvjCBuzAOBgNVHQ8B
Af8EBAMCAYYwDwYDVR0TAQH/BAUwAwEB/zBYBgNVHR4BAf8ETjBMoCQwIocgAAAA
AAAAAAAAAP//CgEAAP//////////////////AAChJDAihyAAAAAAAAAAAAAA//8K
AQEA////////////////////ADAdBgNVHQ4EFgQUudHPCKXymStagMl4I3jRvHlw
n/swHwYDVR0jBBgwFoAUGRnhjAniXVwWBOGcdGYZ/bhSW98wCgYIKoZIzj0EAwID
SAAwRQIgc52wRpNE1I7MsKDdkyg43x/N/1KRS9duH502I613uVICIQDI5VTzLfPz
6XyLA2CUdVzKZpRfrIfte1NSLXGJ4dDS/A==
-----END CERTIFICATE-----
)";

/*
Certificate:
    Data:
        Version: 3 (0x2)
        Serial Number:
            50:69:9a:a0:95:99:2f:a6:e6:b1:72:93:88:56:64:bd:33:98:83:92
        Signature Algorithm: ecdsa-with-SHA256
        Issuer: C=US, ST=Washington, O=AWS Libcrypto, OU=Good CA, CN=Int NC CA 2
        Validity
            Not Before: Jan  1 00:00:00 2015 GMT
            Not After : Jan  1 00:00:00 2100 GMT
        Subject: C=US, ST=Washington, O=AWS Libcrypto, OU=Good Endpoint, CN=Permitted IPv6
        Subject Public Key Info:
            Public Key Algorithm: id-ecPublicKey
                Public-Key: (256 bit)
                pub:
                    04:b2:b7:bd:35:f2:eb:da:86:d5:dc:40:44:c7:23:
                    14:f9:d0:a5:40:17:30:85:b6:c6:11:38:c2:db:2c:
                    c5:bc:0c:19:11:d8:68:61:d6:a3:92:6b:8a:18:52:
                    2c:dc:86:a7:ad:29:ad:91:ac:7e:df:87:24:3b:f3:
                    b4:71:2b:4e:58
                ASN1 OID: prime256v1
                NIST CURVE: P-256
        X509v3 extensions:
            X509v3 Key Usage: critical
                Digital Signature, Key Encipherment
            X509v3 Basic Constraints: critical
                CA:FALSE
            X509v3 Extended Key Usage:
                TLS Web Server Authentication, TLS Web Client Authentication
            X509v3 Subject Alternative Name:
                DNS:example.com, IP Address:0:0:0:0:0:FFFF:A01:201
            X509v3 Subject Key Identifier:
                C8:78:64:E9:F7:9C:0F:56:E2:1D:CE:EE:ED:24:E0:9F:1D:4B:A3:BF
            X509v3 Authority Key Identifier:
                B9:D1:CF:08:A5:F2:99:2B:5A:80:C9:78:23:78:D1:BC:79:70:9F:FB
    Signature Algorithm: ecdsa-with-SHA256
    Signature Value:
        30:44:02:20:06:7e:89:4d:f3:c0:9f:bc:8d:1c:b7:10:43:a4:
        06:dc:63:b3:29:d3:96:bb:b8:15:a6:f3:8f:28:75:6b:a6:f0:
        02:20:49:7f:ce:91:3d:44:0e:f4:c8:22:c0:c1:fe:f0:e1:a4:
        55:31:29:24:30:40:06:27:29:51:89:1a:aa:a9:7e:4e
*/
static char kValidEECertWithValidIPv6SubjectAltName[] = R"(
-----BEGIN CERTIFICATE-----
MIICezCCAiKgAwIBAgIUUGmaoJWZL6bmsXKTiFZkvTOYg5IwCgYIKoZIzj0EAwIw
YjELMAkGA1UEBhMCVVMxEzARBgNVBAgMCldhc2hpbmd0b24xFjAUBgNVBAoMDUFX
UyBMaWJjcnlwdG8xEDAOBgNVBAsMB0dvb2QgQ0ExFDASBgNVBAMMC0ludCBOQyBD
QSAyMCAXDTE1MDEwMTAwMDAwMFoYDzIxMDAwMTAxMDAwMDAwWjBrMQswCQYDVQQG
EwJVUzETMBEGA1UECAwKV2FzaGluZ3RvbjEWMBQGA1UECgwNQVdTIExpYmNyeXB0
bzEWMBQGA1UECwwNR29vZCBFbmRwb2ludDEXMBUGA1UEAwwOUGVybWl0dGVkIElQ
djYwWTATBgcqhkjOPQIBBggqhkjOPQMBBwNCAASyt7018uvahtXcQETHIxT50KVA
FzCFtsYROMLbLMW8DBkR2Ghh1qOSa4oYUizchqetKa2RrH7fhyQ787RxK05Yo4Gq
MIGnMA4GA1UdDwEB/wQEAwIFoDAMBgNVHRMBAf8EAjAAMB0GA1UdJQQWMBQGCCsG
AQUFBwMBBggrBgEFBQcDAjAoBgNVHREEITAfggtleGFtcGxlLmNvbYcQAAAAAAAA
AAAAAP//CgECATAdBgNVHQ4EFgQUyHhk6fecD1biHc7u7STgnx1Lo78wHwYDVR0j
BBgwFoAUudHPCKXymStagMl4I3jRvHlwn/swCgYIKoZIzj0EAwIDRwAwRAIgBn6J
TfPAn7yNHLcQQ6QG3GOzKdOWu7gVpvOPKHVrpvACIEl/zpE9RA70yCLAwf7w4aRV
MSkkMEAGJylRiRqqqX5O
-----END CERTIFICATE-----
)";

/*
Certificate:
    Data:
        Version: 3 (0x2)
        Serial Number:
            15:a5:1d:5a:37:04:67:db:0c:ff:14:fe:5e:48:6b:fe:bb:d3:13:6c
        Signature Algorithm: ecdsa-with-SHA256
        Issuer: C=US, ST=Washington, O=AWS Libcrypto, OU=Good CA, CN=Int NC CA 2
        Validity
            Not Before: Jan  1 00:00:00 2015 GMT
            Not After : Jan  1 00:00:00 2100 GMT
        Subject: C=US, ST=Washington, O=AWS Libcrypto, OU=Good Endpoint, CN=Excluded IPv6
        Subject Public Key Info:
            Public Key Algorithm: id-ecPublicKey
                Public-Key: (256 bit)
                pub:
                    04:b2:b7:bd:35:f2:eb:da:86:d5:dc:40:44:c7:23:
                    14:f9:d0:a5:40:17:30:85:b6:c6:11:38:c2:db:2c:
                    c5:bc:0c:19:11:d8:68:61:d6:a3:92:6b:8a:18:52:
                    2c:dc:86:a7:ad:29:ad:91:ac:7e:df:87:24:3b:f3:
                    b4:71:2b:4e:58
                ASN1 OID: prime256v1
                NIST CURVE: P-256
        X509v3 extensions:
            X509v3 Key Usage: critical
                Digital Signature, Key Encipherment
            X509v3 Basic Constraints: critical
                CA:FALSE
            X509v3 Extended Key Usage:
                TLS Web Server Authentication, TLS Web Client Authentication
            X509v3 Subject Alternative Name:
                DNS:example.com, IP Address:0:0:0:0:0:FFFF:A01:102
            X509v3 Subject Key Identifier:
                C8:78:64:E9:F7:9C:0F:56:E2:1D:CE:EE:ED:24:E0:9F:1D:4B:A3:BF
            X509v3 Authority Key Identifier:
                B9:D1:CF:08:A5:F2:99:2B:5A:80:C9:78:23:78:D1:BC:79:70:9F:FB
    Signature Algorithm: ecdsa-with-SHA256
    Signature Value:
        30:44:02:20:4c:76:bd:2e:df:89:c9:be:78:53:f7:71:d6:85:
        49:a0:f7:5e:c6:ed:f5:f2:88:be:3f:dc:ad:19:ab:bd:ec:ee:
        02:20:0d:ad:b4:45:a4:3d:c3:e5:f4:16:56:40:0b:5e:33:10:
        e6:4e:49:a6:9d:c3:eb:27:bf:3e:98:cd:d5:87:ce:db
*/
static char kInvalidEECertWithInvalidExcludedIPv6SubjAlt[] = R"(
-----BEGIN CERTIFICATE-----
MIICejCCAiGgAwIBAgIUFaUdWjcEZ9sM/xT+Xkhr/rvTE2wwCgYIKoZIzj0EAwIw
YjELMAkGA1UEBhMCVVMxEzARBgNVBAgMCldhc2hpbmd0b24xFjAUBgNVBAoMDUFX
UyBMaWJjcnlwdG8xEDAOBgNVBAsMB0dvb2QgQ0ExFDASBgNVBAMMC0ludCBOQyBD
QSAyMCAXDTE1MDEwMTAwMDAwMFoYDzIxMDAwMTAxMDAwMDAwWjBqMQswCQYDVQQG
EwJVUzETMBEGA1UECAwKV2FzaGluZ3RvbjEWMBQGA1UECgwNQVdTIExpYmNyeXB0
bzEWMBQGA1UECwwNR29vZCBFbmRwb2ludDEWMBQGA1UEAwwNRXhjbHVkZWQgSVB2
NjBZMBMGByqGSM49AgEGCCqGSM49AwEHA0IABLK3vTXy69qG1dxARMcjFPnQpUAX
MIW2xhE4wtssxbwMGRHYaGHWo5JrihhSLNyGp60prZGsft+HJDvztHErTlijgaow
gacwDgYDVR0PAQH/BAQDAgWgMAwGA1UdEwEB/wQCMAAwHQYDVR0lBBYwFAYIKwYB
BQUHAwEGCCsGAQUFBwMCMCgGA1UdEQQhMB+CC2V4YW1wbGUuY29thxAAAAAAAAAA
AAAA//8KAQECMB0GA1UdDgQWBBTIeGTp95wPVuIdzu7tJOCfHUujvzAfBgNVHSME
GDAWgBS50c8IpfKZK1qAyXgjeNG8eXCf+zAKBggqhkjOPQQDAgNHADBEAiBMdr0u
34nJvnhT93HWhUmg917G7fXyiL4/3K0Zq73s7gIgDa20RaQ9w+X0FlZAC14zEOZO
Saadw+snvz6YzdWHzts=
-----END CERTIFICATE-----
)";

/*
Certificate:
    Data:
        Version: 3 (0x2)
        Serial Number:
            75:62:5c:30:b0:0e:c2:19:c0:ae:5d:f1:32:70:09:9c:7f:0e:db:d1
        Signature Algorithm: ecdsa-with-SHA256
        Issuer: C=US, ST=Washington, O=AWS Libcrypto, OU=Good CA, CN=Int NC CA 2
        Validity
            Not Before: Jan  1 00:00:00 2015 GMT
            Not After : Jan  1 00:00:00 2100 GMT
        Subject: C=US, ST=Washington, O=AWS Libcrypto, OU=Bad Endpoint, CN=Not Permitted IPv6
        Subject Public Key Info:
            Public Key Algorithm: id-ecPublicKey
                Public-Key: (256 bit)
                pub:
                    04:b2:b7:bd:35:f2:eb:da:86:d5:dc:40:44:c7:23:
                    14:f9:d0:a5:40:17:30:85:b6:c6:11:38:c2:db:2c:
                    c5:bc:0c:19:11:d8:68:61:d6:a3:92:6b:8a:18:52:
                    2c:dc:86:a7:ad:29:ad:91:ac:7e:df:87:24:3b:f3:
                    b4:71:2b:4e:58
                ASN1 OID: prime256v1
                NIST CURVE: P-256
        X509v3 extensions:
            X509v3 Key Usage: critical
                Digital Signature, Key Encipherment
            X509v3 Basic Constraints: critical
                CA:FALSE
            X509v3 Extended Key Usage:
                TLS Web Server Authentication, TLS Web Client Authentication
            X509v3 Subject Alternative Name:
                DNS:example.com, IP Address:0:0:0:0:0:FFFF:A02:1
            X509v3 Subject Key Identifier:
                C8:78:64:E9:F7:9C:0F:56:E2:1D:CE:EE:ED:24:E0:9F:1D:4B:A3:BF
            X509v3 Authority Key Identifier:
                B9:D1:CF:08:A5:F2:99:2B:5A:80:C9:78:23:78:D1:BC:79:70:9F:FB
    Signature Algorithm: ecdsa-with-SHA256
    Signature Value:
        30:44:02:20:4f:a4:0e:29:a3:3c:43:53:78:87:5b:9e:f7:a4:
        92:ae:fa:d0:51:db:da:1e:ec:69:cc:25:6b:be:cf:b7:e1:8a:
        02:20:1d:f5:10:0a:3d:92:84:f3:7d:b0:86:0e:95:6b:97:b5:
        0f:ca:4c:6b:99:0b:a0:b1:10:17:17:94:a0:bd:62:f9
*/
static char kInvalidEECertWithInvalidPermittedIPv6SubjAlt[] = R"(
-----BEGIN CERTIFICATE-----
MIICfjCCAiWgAwIBAgIUdWJcMLAOwhnArl3xMnAJnH8O29EwCgYIKoZIzj0EAwIw
YjELMAkGA1UEBhMCVVMxEzARBgNVBAgMCldhc2hpbmd0b24xFjAUBgNVBAoMDUFX
UyBMaWJjcnlwdG8xEDAOBgNVBAsMB0dvb2QgQ0ExFDASBgNVBAMMC0ludCBOQyBD
QSAyMCAXDTE1MDEwMTAwMDAwMFoYDzIxMDAwMTAxMDAwMDAwWjBuMQswCQYDVQQG
EwJVUzETMBEGA1UECAwKV2FzaGluZ3RvbjEWMBQGA1UECgwNQVdTIExpYmNyeXB0
bzEVMBMGA1UECwwMQmFkIEVuZHBvaW50MRswGQYDVQQDDBJOb3QgUGVybWl0dGVk
IElQdjYwWTATBgcqhkjOPQIBBggqhkjOPQMBBwNCAASyt7018uvahtXcQETHIxT5
0KVAFzCFtsYROMLbLMW8DBkR2Ghh1qOSa4oYUizchqetKa2RrH7fhyQ787RxK05Y
o4GqMIGnMA4GA1UdDwEB/wQEAwIFoDAMBgNVHRMBAf8EAjAAMB0GA1UdJQQWMBQG
CCsGAQUFBwMBBggrBgEFBQcDAjAoBgNVHREEITAfggtleGFtcGxlLmNvbYcQAAAA
AAAAAAAAAP//CgIAATAdBgNVHQ4EFgQUyHhk6fecD1biHc7u7STgnx1Lo78wHwYD
VR0jBBgwFoAUudHPCKXymStagMl4I3jRvHlwn/swCgYIKoZIzj0EAwIDRwAwRAIg
T6QOKaM8Q1N4h1ue96SSrvrQUdvaHuxpzCVrvs+34YoCIB31EAo9koTzfbCGDpVr
l7UPykxrmQugsRAXF5SgvWL5
-----END CERTIFICATE-----
)";

/*
Certificate:
    Data:
        Version: 3 (0x2)
        Serial Number:
            5c:d4:93:85:c2:28:23:5a:05:29:9d:3b:f5:7a:fb:80:2d:fa:bb:a2
        Signature Algorithm: ecdsa-with-SHA256
        Issuer: C=US, ST=Washington, O=AWS Libcrypto, OU=Good CA, CN=Root CA 1
        Validity
            Not Before: Jan  1 00:00:00 2015 GMT
            Not After : Jan  1 00:00:00 2100 GMT
        Subject: C=US, ST=Washington, O=AWS Libcrypto, OU=Bad CA, CN=Int NC CA Bad IPv4 Mask
        Subject Public Key Info:
            Public Key Algorithm: id-ecPublicKey
                Public-Key: (256 bit)
                pub:
                    04:9e:64:d1:8c:e1:e7:f0:3b:a5:22:df:04:2d:54:
                    fa:e8:cc:b1:22:e7:8b:c6:60:d9:72:cd:ad:1a:16:
                    dc:34:9f:99:e8:22:80:95:de:19:5b:0a:4e:d6:d6:
                    47:5a:b7:44:32:17:3f:6b:32:80:e5:0e:35:cb:68:
                    62:c4:dd:31:d1
                ASN1 OID: prime256v1
                NIST CURVE: P-256
        X509v3 extensions:
            X509v3 Key Usage: critical
                Digital Signature, Certificate Sign, CRL Sign
            X509v3 Basic Constraints: critical
                CA:TRUE
            X509v3 Name Constraints: critical
                Permitted:
                  IP:10.1.0.0/255.0.255.0
            X509v3 Subject Key Identifier:
                B9:D1:CF:08:A5:F2:99:2B:5A:80:C9:78:23:78:D1:BC:79:70:9F:FB
            X509v3 Authority Key Identifier:
                19:19:E1:8C:09:E2:5D:5C:16:04:E1:9C:74:66:19:FD:B8:52:5B:DF
    Signature Algorithm: ecdsa-with-SHA256
    Signature Value:
        30:45:02:20:0d:e7:20:8c:c2:1e:d7:06:b3:38:32:75:41:88:
        00:9d:26:47:d6:d6:71:1c:ff:7d:82:00:ec:d4:90:19:d3:b7:
        02:21:00:a4:7e:86:f4:f4:7b:aa:86:9e:de:7b:63:ee:3f:fb:
        c5:a1:bf:1d:75:a0:0c:b0:fb:b5:60:d2:17:2e:00:2c:6e
*/
static char kIntCAWithBadIPv4NameConstraint[] = R"(
-----BEGIN CERTIFICATE-----
MIICUDCCAfagAwIBAgIUXNSThcIoI1oFKZ079Xr7gC36u6IwCgYIKoZIzj0EAwIw
YDELMAkGA1UEBhMCVVMxEzARBgNVBAgMCldhc2hpbmd0b24xFjAUBgNVBAoMDUFX
UyBMaWJjcnlwdG8xEDAOBgNVBAsMB0dvb2QgQ0ExEjAQBgNVBAMMCVJvb3QgQ0Eg
MTAgFw0xNTAxMDEwMDAwMDBaGA8yMTAwMDEwMTAwMDAwMFowbTELMAkGA1UEBhMC
VVMxEzARBgNVBAgMCldhc2hpbmd0b24xFjAUBgNVBAoMDUFXUyBMaWJjcnlwdG8x
DzANBgNVBAsMBkJhZCBDQTEgMB4GA1UEAwwXSW50IE5DIENBIEJhZCBJUHY0IE1h
c2swWTATBgcqhkjOPQIBBggqhkjOPQMBBwNCAASeZNGM4efwO6Ui3wQtVProzLEi
54vGYNlyza0aFtw0n5noIoCV3hlbCk7W1kdat0QyFz9rMoDlDjXLaGLE3THRo38w
fTAOBgNVHQ8BAf8EBAMCAYYwDwYDVR0TAQH/BAUwAwEB/zAaBgNVHR4BAf8EEDAO
oAwwCocICgEAAP8A/wAwHQYDVR0OBBYEFLnRzwil8pkrWoDJeCN40bx5cJ/7MB8G
A1UdIwQYMBaAFBkZ4YwJ4l1cFgThnHRmGf24UlvfMAoGCCqGSM49BAMCA0gAMEUC
IA3nIIzCHtcGszgydUGIAJ0mR9bWcRz/fYIA7NSQGdO3AiEApH6G9PR7qoae3ntj
7j/7xaG/HXWgDLD7tWDSFy4ALG4=
-----END CERTIFICATE-----
)";

/*
EE signed by |kIntCAWithBadIPv4NameConstraint|.

Certificate:
    Data:
        Version: 3 (0x2)
        Serial Number:
            27:22:eb:9c:37:f5:67:2b:a1:22:02:1e:c4:6a:17:a1:97:77:37:46
        Signature Algorithm: ecdsa-with-SHA256
        Issuer: C=US, ST=Washington, O=AWS Libcrypto, OU=Bad CA, CN=Int NC CA Bad IPv4 Mask
        Validity
            Not Before: Jan  1 00:00:00 2015 GMT
            Not After : Jan  1 00:00:00 2100 GMT
        Subject: C=US, ST=Washington, O=AWS Libcrypto, OU=Bad Endpoint, CN=Issued by Bad CA NC, SN=IPv4
        Subject Public Key Info:
            Public Key Algorithm: id-ecPublicKey
                Public-Key: (256 bit)
                pub:
                    04:b2:b7:bd:35:f2:eb:da:86:d5:dc:40:44:c7:23:
                    14:f9:d0:a5:40:17:30:85:b6:c6:11:38:c2:db:2c:
                    c5:bc:0c:19:11:d8:68:61:d6:a3:92:6b:8a:18:52:
                    2c:dc:86:a7:ad:29:ad:91:ac:7e:df:87:24:3b:f3:
                    b4:71:2b:4e:58
                ASN1 OID: prime256v1
                NIST CURVE: P-256
        X509v3 extensions:
            X509v3 Key Usage: critical
                Digital Signature, Key Encipherment
            X509v3 Basic Constraints: critical
                CA:FALSE
            X509v3 Extended Key Usage:
                TLS Web Server Authentication, TLS Web Client Authentication
            X509v3 Subject Alternative Name:
                IP Address:10.1.0.1
            X509v3 Subject Key Identifier:
                C8:78:64:E9:F7:9C:0F:56:E2:1D:CE:EE:ED:24:E0:9F:1D:4B:A3:BF
            X509v3 Authority Key Identifier:
                B9:D1:CF:08:A5:F2:99:2B:5A:80:C9:78:23:78:D1:BC:79:70:9F:FB
    Signature Algorithm: ecdsa-with-SHA256
    Signature Value:
        30:45:02:21:00:9f:5e:56:73:d5:f4:ea:87:68:e0:75:4a:ed:
        1f:17:6f:c0:8e:db:0d:b7:13:a9:9e:77:51:a7:7d:3d:42:03:
        17:02:20:51:74:cd:cb:97:0c:c6:8c:bb:50:b0:71:f5:24:d6:
        20:10:ba:12:e5:1b:c6:cf:e5:e2:cf:fa:75:20:5f:10:ba
*/
static char kInvalidEECertIssuedByCAWithBadIPv4NameConstraint[] = R"(
-----BEGIN CERTIFICATE-----
MIICgTCCAiegAwIBAgIUJyLrnDf1ZyuhIgIexGoXoZd3N0YwCgYIKoZIzj0EAwIw
bTELMAkGA1UEBhMCVVMxEzARBgNVBAgMCldhc2hpbmd0b24xFjAUBgNVBAoMDUFX
UyBMaWJjcnlwdG8xDzANBgNVBAsMBkJhZCBDQTEgMB4GA1UEAwwXSW50IE5DIENB
IEJhZCBJUHY0IE1hc2swIBcNMTUwMTAxMDAwMDAwWhgPMjEwMDAxMDEwMDAwMDBa
MH4xCzAJBgNVBAYTAlVTMRMwEQYDVQQIDApXYXNoaW5ndG9uMRYwFAYDVQQKDA1B
V1MgTGliY3J5cHRvMRUwEwYDVQQLDAxCYWQgRW5kcG9pbnQxHDAaBgNVBAMME0lz
c3VlZCBieSBCYWQgQ0EgTkMxDTALBgNVBAQMBElQdjQwWTATBgcqhkjOPQIBBggq
hkjOPQMBBwNCAASyt7018uvahtXcQETHIxT50KVAFzCFtsYROMLbLMW8DBkR2Ghh
1qOSa4oYUizchqetKa2RrH7fhyQ787RxK05Yo4GRMIGOMA4GA1UdDwEB/wQEAwIF
oDAMBgNVHRMBAf8EAjAAMB0GA1UdJQQWMBQGCCsGAQUFBwMBBggrBgEFBQcDAjAP
BgNVHREECDAGhwQKAQABMB0GA1UdDgQWBBTIeGTp95wPVuIdzu7tJOCfHUujvzAf
BgNVHSMEGDAWgBS50c8IpfKZK1qAyXgjeNG8eXCf+zAKBggqhkjOPQQDAgNIADBF
AiEAn15Wc9X06odo4HVK7R8Xb8CO2w23E6med1GnfT1CAxcCIFF0zcuXDMaMu1Cw
cfUk1iAQuhLlG8bP5eLP+nUgXxC6
-----END CERTIFICATE-----
)";

/*
Certificate:
    Data:
        Version: 3 (0x2)
        Serial Number:
            47:43:7b:1f:0f:d3:ac:f6:5f:36:ea:94:9b:42:e9:4f:5c:54:11:d8
        Signature Algorithm: ecdsa-with-SHA256
        Issuer: C=US, ST=Washington, O=AWS Libcrypto, OU=Good CA, CN=Root CA 1
        Validity
            Not Before: Jan  1 00:00:00 2015 GMT
            Not After : Jan  1 00:00:00 2100 GMT
        Subject: C=US, ST=Washington, O=AWS Libcrypto, OU=Bad CA, CN=Int NC CA Bad IPv6 Mask
        Subject Public Key Info:
            Public Key Algorithm: id-ecPublicKey
                Public-Key: (256 bit)
                pub:
                    04:9e:64:d1:8c:e1:e7:f0:3b:a5:22:df:04:2d:54:
                    fa:e8:cc:b1:22:e7:8b:c6:60:d9:72:cd:ad:1a:16:
                    dc:34:9f:99:e8:22:80:95:de:19:5b:0a:4e:d6:d6:
                    47:5a:b7:44:32:17:3f:6b:32:80:e5:0e:35:cb:68:
                    62:c4:dd:31:d1
                ASN1 OID: prime256v1
                NIST CURVE: P-256
        X509v3 extensions:
            X509v3 Key Usage: critical
                Digital Signature, Certificate Sign, CRL Sign
            X509v3 Basic Constraints: critical
                CA:TRUE
            X509v3 Name Constraints: critical
                Permitted:
                  IP:0:0:0:0:0:FFFF:A01:0/FFFF:FFFF:FFFF:FFFF:FFFF:FFFF:FF00:FF00
            X509v3 Subject Key Identifier:
                B9:D1:CF:08:A5:F2:99:2B:5A:80:C9:78:23:78:D1:BC:79:70:9F:FB
            X509v3 Authority Key Identifier:
                19:19:E1:8C:09:E2:5D:5C:16:04:E1:9C:74:66:19:FD:B8:52:5B:DF
    Signature Algorithm: ecdsa-with-SHA256
    Signature Value:
        30:46:02:21:00:ab:ab:07:24:39:02:2c:a7:03:c9:90:86:b3:
        55:8f:14:95:04:2d:ed:35:3f:cc:a2:1a:f2:7a:89:90:fa:31:
        e9:02:21:00:85:2b:cb:9d:0f:de:3c:77:3f:0a:48:7c:bf:d0:
        a6:ab:95:35:2d:29:c5:13:af:bd:96:fd:9e:9c:ab:1c:ee:8d
*/
static char kIntCAWithBadIPv6NameConstraint[] = R"(
-----BEGIN CERTIFICATE-----
MIICazCCAhCgAwIBAgIUR0N7Hw/TrPZfNuqUm0LpT1xUEdgwCgYIKoZIzj0EAwIw
YDELMAkGA1UEBhMCVVMxEzARBgNVBAgMCldhc2hpbmd0b24xFjAUBgNVBAoMDUFX
UyBMaWJjcnlwdG8xEDAOBgNVBAsMB0dvb2QgQ0ExEjAQBgNVBAMMCVJvb3QgQ0Eg
MTAgFw0xNTAxMDEwMDAwMDBaGA8yMTAwMDEwMTAwMDAwMFowbTELMAkGA1UEBhMC
VVMxEzARBgNVBAgMCldhc2hpbmd0b24xFjAUBgNVBAoMDUFXUyBMaWJjcnlwdG8x
DzANBgNVBAsMBkJhZCBDQTEgMB4GA1UEAwwXSW50IE5DIENBIEJhZCBJUHY2IE1h
c2swWTATBgcqhkjOPQIBBggqhkjOPQMBBwNCAASeZNGM4efwO6Ui3wQtVProzLEi
54vGYNlyza0aFtw0n5noIoCV3hlbCk7W1kdat0QyFz9rMoDlDjXLaGLE3THRo4GY
MIGVMA4GA1UdDwEB/wQEAwIBhjAPBgNVHRMBAf8EBTADAQH/MDIGA1UdHgEB/wQo
MCagJDAihyAAAAAAAAAAAAAA//8KAQAA/////////////////wD/ADAdBgNVHQ4E
FgQUudHPCKXymStagMl4I3jRvHlwn/swHwYDVR0jBBgwFoAUGRnhjAniXVwWBOGc
dGYZ/bhSW98wCgYIKoZIzj0EAwIDSQAwRgIhAKurByQ5AiynA8mQhrNVjxSVBC3t
NT/MohryeomQ+jHpAiEAhSvLnQ/ePHc/Ckh8v9Cmq5U1LSnFE6+9lv2enKsc7o0=
-----END CERTIFICATE-----
)";

/*
EE signed by |kIntCAWithBadIPv6NameConstraint|.

Certificate:
    Data:
        Version: 3 (0x2)
        Serial Number:
            43:ce:68:79:84:f1:d7:60:ff:b2:f5:c5:70:3e:85:55:9e:ac:d2:46
        Signature Algorithm: ecdsa-with-SHA256
        Issuer: C=US, ST=Washington, O=AWS Libcrypto, OU=Bad CA, CN=Int NC CA Bad IPv6 Mask
        Validity
            Not Before: Jan  1 00:00:00 2015 GMT
            Not After : Jan  1 00:00:00 2100 GMT
        Subject: C=US, ST=Washington, O=AWS Libcrypto, OU=Bad Endpoint, CN=Issued by Bad CA NC, SN=IPv6
        Subject Public Key Info:
            Public Key Algorithm: id-ecPublicKey
                Public-Key: (256 bit)
                pub:
                    04:b2:b7:bd:35:f2:eb:da:86:d5:dc:40:44:c7:23:
                    14:f9:d0:a5:40:17:30:85:b6:c6:11:38:c2:db:2c:
                    c5:bc:0c:19:11:d8:68:61:d6:a3:92:6b:8a:18:52:
                    2c:dc:86:a7:ad:29:ad:91:ac:7e:df:87:24:3b:f3:
                    b4:71:2b:4e:58
                ASN1 OID: prime256v1
                NIST CURVE: P-256
        X509v3 extensions:
            X509v3 Key Usage: critical
                Digital Signature, Key Encipherment
            X509v3 Basic Constraints: critical
                CA:FALSE
            X509v3 Extended Key Usage:
                TLS Web Server Authentication, TLS Web Client Authentication
            X509v3 Subject Alternative Name:
                IP Address:0:0:0:0:0:FFFF:A01:1
            X509v3 Subject Key Identifier:
                C8:78:64:E9:F7:9C:0F:56:E2:1D:CE:EE:ED:24:E0:9F:1D:4B:A3:BF
            X509v3 Authority Key Identifier:
                B9:D1:CF:08:A5:F2:99:2B:5A:80:C9:78:23:78:D1:BC:79:70:9F:FB
    Signature Algorithm: ecdsa-with-SHA256
    Signature Value:
        30:45:02:21:00:b4:bb:a6:9c:9f:6a:e7:bf:aa:ec:93:91:5d:
        98:76:2a:3d:b2:d0:4e:4d:48:a4:c0:cc:f8:fb:a3:27:ff:c9:
        ca:02:20:7b:19:8f:e0:cd:31:07:2f:22:79:18:a5:6a:85:d0:
        8f:e6:85:d0:7f:c4:8c:3e:39:95:5a:41:0f:42:48:9a:81
*/
static char kInvalidEECertIssuedByCAWithBadIPv6NameConstraint[] = R"(
-----BEGIN CERTIFICATE-----
MIICjTCCAjOgAwIBAgIUQ85oeYTx12D/svXFcD6FVZ6s0kYwCgYIKoZIzj0EAwIw
bTELMAkGA1UEBhMCVVMxEzARBgNVBAgMCldhc2hpbmd0b24xFjAUBgNVBAoMDUFX
UyBMaWJjcnlwdG8xDzANBgNVBAsMBkJhZCBDQTEgMB4GA1UEAwwXSW50IE5DIENB
IEJhZCBJUHY2IE1hc2swIBcNMTUwMTAxMDAwMDAwWhgPMjEwMDAxMDEwMDAwMDBa
MH4xCzAJBgNVBAYTAlVTMRMwEQYDVQQIDApXYXNoaW5ndG9uMRYwFAYDVQQKDA1B
V1MgTGliY3J5cHRvMRUwEwYDVQQLDAxCYWQgRW5kcG9pbnQxHDAaBgNVBAMME0lz
c3VlZCBieSBCYWQgQ0EgTkMxDTALBgNVBAQMBElQdjYwWTATBgcqhkjOPQIBBggq
hkjOPQMBBwNCAASyt7018uvahtXcQETHIxT50KVAFzCFtsYROMLbLMW8DBkR2Ghh
1qOSa4oYUizchqetKa2RrH7fhyQ787RxK05Yo4GdMIGaMA4GA1UdDwEB/wQEAwIF
oDAMBgNVHRMBAf8EAjAAMB0GA1UdJQQWMBQGCCsGAQUFBwMBBggrBgEFBQcDAjAb
BgNVHREEFDAShxAAAAAAAAAAAAAA//8KAQABMB0GA1UdDgQWBBTIeGTp95wPVuId
zu7tJOCfHUujvzAfBgNVHSMEGDAWgBS50c8IpfKZK1qAyXgjeNG8eXCf+zAKBggq
hkjOPQQDAgNIADBFAiEAtLumnJ9q57+q7JORXZh2Kj2y0E5NSKTAzPj7oyf/ycoC
IHsZj+DNMQcvInkYpWqF0I/mhdB/xIw+OZVaQQ9CSJqB
-----END CERTIFICATE-----
)";

/*
Signed by |kValidRootCA1| it's name constraints contains an address and mask
that is neither IPv4 or IPv6.

Certificate:
    Data:
        Version: 3 (0x2)
        Serial Number:
            39:84:7b:3f:07:39:af:7e:9c:ef:59:13:92:df:af:f6:22:ac:70:f7
        Signature Algorithm: ecdsa-with-SHA256
        Issuer: C=US, ST=Washington, O=AWS Libcrypto, OU=Good CA, CN=Root CA 1
        Validity
            Not Before: Jan  1 00:00:00 2015 GMT
            Not After : Jan  1 00:00:00 2100 GMT
        Subject: C=US, ST=Washington, O=AWS Libcrypto, OU=Bad CA, CN=Int NC CA Neither IPv4 or IPv6
        Subject Public Key Info:
            Public Key Algorithm: id-ecPublicKey
                Public-Key: (256 bit)
                pub:
                    04:9e:64:d1:8c:e1:e7:f0:3b:a5:22:df:04:2d:54:
                    fa:e8:cc:b1:22:e7:8b:c6:60:d9:72:cd:ad:1a:16:
                    dc:34:9f:99:e8:22:80:95:de:19:5b:0a:4e:d6:d6:
                    47:5a:b7:44:32:17:3f:6b:32:80:e5:0e:35:cb:68:
                    62:c4:dd:31:d1
                ASN1 OID: prime256v1
                NIST CURVE: P-256
        X509v3 extensions:
            X509v3 Key Usage: critical
                Digital Signature, Certificate Sign, CRL Sign
            X509v3 Basic Constraints: critical
                CA:TRUE
            X509v3 Name Constraints: critical
                Permitted:
                  IP:C0C0:A8A8:101:101:FFFF:FFFF:FFFF:FF00/<invalid length=0>
            X509v3 Subject Key Identifier:
                B9:D1:CF:08:A5:F2:99:2B:5A:80:C9:78:23:78:D1:BC:79:70:9F:FB
            X509v3 Authority Key Identifier:
                19:19:E1:8C:09:E2:5D:5C:16:04:E1:9C:74:66:19:FD:B8:52:5B:DF
    Signature Algorithm: ecdsa-with-SHA256
    Signature Value:
        30:44:02:20:3e:ab:ff:8d:48:ef:16:05:cd:27:07:72:10:60:
        a2:6d:1c:4a:d3:d0:98:9c:42:ce:00:4e:11:22:1d:65:34:5b:
        02:20:2a:05:09:5f:0e:e2:bf:a4:d3:80:40:f5:80:85:d2:96:
        f6:3f:18:46:a0:d0:9e:d3:1a:36:0f:a4:d9:53:89:8f
*/
static char kIntCAWithBadIPNameConstraint[] = R"(
-----BEGIN CERTIFICATE-----
MIICYDCCAgegAwIBAgIUOYR7Pwc5r36c71kTkt+v9iKscPcwCgYIKoZIzj0EAwIw
YDELMAkGA1UEBhMCVVMxEzARBgNVBAgMCldhc2hpbmd0b24xFjAUBgNVBAoMDUFX
UyBMaWJjcnlwdG8xEDAOBgNVBAsMB0dvb2QgQ0ExEjAQBgNVBAMMCVJvb3QgQ0Eg
MTAgFw0xNTAxMDEwMDAwMDBaGA8yMTAwMDEwMTAwMDAwMFowdDELMAkGA1UEBhMC
VVMxEzARBgNVBAgMCldhc2hpbmd0b24xFjAUBgNVBAoMDUFXUyBMaWJjcnlwdG8x
DzANBgNVBAsMBkJhZCBDQTEnMCUGA1UEAwweSW50IE5DIENBIE5laXRoZXIgSVB2
NCBvciBJUHY2MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEnmTRjOHn8DulIt8E
LVT66MyxIueLxmDZcs2tGhbcNJ+Z6CKAld4ZWwpO1tZHWrdEMhc/azKA5Q41y2hi
xN0x0aOBiDCBhTAOBgNVHQ8BAf8EBAMCAYYwDwYDVR0TAQH/BAUwAwEB/zAiBgNV
HR4BAf8EGDAWoBQwEocQwMCoqAEBAQH/////////ADAdBgNVHQ4EFgQUudHPCKXy
mStagMl4I3jRvHlwn/swHwYDVR0jBBgwFoAUGRnhjAniXVwWBOGcdGYZ/bhSW98w
CgYIKoZIzj0EAwIDRwAwRAIgPqv/jUjvFgXNJwdyEGCibRxK09CYnELOAE4RIh1l
NFsCICoFCV8O4r+k04BA9YCF0pb2PxhGoNCe0xo2D6TZU4mP
-----END CERTIFICATE-----
)";

/*
EE certificate signed by |kIntCAWithBadIPNameConstraint| but it contains
an invalid IP address that is only 64-bits in length.

Certificate:
    Data:
        Version: 3 (0x2)
        Serial Number:
            60:02:49:78:95:9b:55:29:92:4c:93:6b:4f:05:41:5e:84:74:c3:13
        Signature Algorithm: ecdsa-with-SHA256
        Issuer: C=US, ST=Washington, O=AWS Libcrypto, OU=Bad CA, CN=Int NC CA Neither IPv4 or IPv6
        Validity
            Not Before: Jan  1 00:00:00 2015 GMT
            Not After : Jan  1 00:00:00 2100 GMT
        Subject: C=US, ST=Washington, O=AWS Libcrypto, OU=Bad Endpoint, CN=Issued by Bad CA NC, SN=Neither IPv4 or IPv6
        Subject Public Key Info:
            Public Key Algorithm: id-ecPublicKey
                Public-Key: (256 bit)
                pub:
                    04:b2:b7:bd:35:f2:eb:da:86:d5:dc:40:44:c7:23:
                    14:f9:d0:a5:40:17:30:85:b6:c6:11:38:c2:db:2c:
                    c5:bc:0c:19:11:d8:68:61:d6:a3:92:6b:8a:18:52:
                    2c:dc:86:a7:ad:29:ad:91:ac:7e:df:87:24:3b:f3:
                    b4:71:2b:4e:58
                ASN1 OID: prime256v1
                NIST CURVE: P-256
        X509v3 extensions:
            X509v3 Key Usage: critical
                Digital Signature, Key Encipherment
            X509v3 Basic Constraints: critical
                CA:FALSE
            X509v3 Extended Key Usage:
                TLS Web Server Authentication, TLS Web Client Authentication
            X509v3 Subject Alternative Name:
                IP Address:<invalid length=8>
            X509v3 Subject Key Identifier:
                C8:78:64:E9:F7:9C:0F:56:E2:1D:CE:EE:ED:24:E0:9F:1D:4B:A3:BF
            X509v3 Authority Key Identifier:
                B9:D1:CF:08:A5:F2:99:2B:5A:80:C9:78:23:78:D1:BC:79:70:9F:FB
    Signature Algorithm: ecdsa-with-SHA256
    Signature Value:
        30:45:02:21:00:85:67:03:40:76:c2:fc:f9:fa:64:3d:6a:32:
        7b:57:f6:2f:53:22:9e:92:3b:2c:ca:04:ca:82:0b:71:28:e8:
        ac:02:20:39:9f:a2:3d:81:d1:2c:0c:49:83:ee:bf:b0:81:cb:
        db:57:6b:6e:e4:92:aa:ec:63:50:d0:13:55:1b:68:2b:d7
*/
static char kInvalidEECertIssuedByCAWithBadIPNameConstraint[] = R"(
-----BEGIN CERTIFICATE-----
MIICnTCCAkOgAwIBAgIUYAJJeJWbVSmSTJNrTwVBXoR0wxMwCgYIKoZIzj0EAwIw
dDELMAkGA1UEBhMCVVMxEzARBgNVBAgMCldhc2hpbmd0b24xFjAUBgNVBAoMDUFX
UyBMaWJjcnlwdG8xDzANBgNVBAsMBkJhZCBDQTEnMCUGA1UEAwweSW50IE5DIENB
IE5laXRoZXIgSVB2NCBvciBJUHY2MCAXDTE1MDEwMTAwMDAwMFoYDzIxMDAwMTAx
MDAwMDAwWjCBjjELMAkGA1UEBhMCVVMxEzARBgNVBAgMCldhc2hpbmd0b24xFjAU
BgNVBAoMDUFXUyBMaWJjcnlwdG8xFTATBgNVBAsMDEJhZCBFbmRwb2ludDEcMBoG
A1UEAwwTSXNzdWVkIGJ5IEJhZCBDQSBOQzEdMBsGA1UEBAwUTmVpdGhlciBJUHY0
IG9yIElQdjYwWTATBgcqhkjOPQIBBggqhkjOPQMBBwNCAASyt7018uvahtXcQETH
IxT50KVAFzCFtsYROMLbLMW8DBkR2Ghh1qOSa4oYUizchqetKa2RrH7fhyQ787Rx
K05Yo4GVMIGSMA4GA1UdDwEB/wQEAwIFoDAMBgNVHRMBAf8EAjAAMB0GA1UdJQQW
MBQGCCsGAQUFBwMBBggrBgEFBQcDAjATBgNVHREEDDAKhwjAwKioAQEBATAdBgNV
HQ4EFgQUyHhk6fecD1biHc7u7STgnx1Lo78wHwYDVR0jBBgwFoAUudHPCKXymSta
gMl4I3jRvHlwn/swCgYIKoZIzj0EAwIDSAAwRQIhAIVnA0B2wvz5+mQ9ajJ7V/Yv
UyKekjssygTKggtxKOisAiA5n6I9gdEsDEmD7r+wgcvbV2tu5JKq7GNQ0BNVG2gr
1w==
-----END CERTIFICATE-----
)";

/*
Intermediate CA signed by |kValidRootCA1|, with DNS name constraints.

Certificate:
    Data:
        Version: 3 (0x2)
        Serial Number:
            25:d6:88:31:c9:7c:a9:15:3c:a6:01:02:09:92:a9:bb:30:86:3d:04
        Signature Algorithm: ecdsa-with-SHA256
        Issuer: C=US, ST=Washington, O=AWS Libcrypto, OU=Good CA, CN=Root CA 1
        Validity
            Not Before: Jan  1 00:00:00 2015 GMT
            Not After : Jan  1 00:00:00 2100 GMT
        Subject: C=US, ST=Washington, O=AWS Libcrypto, OU=Good CA, CN=Int NC CA 3
        Subject Public Key Info:
            Public Key Algorithm: id-ecPublicKey
                Public-Key: (256 bit)
                pub:
                    04:9e:64:d1:8c:e1:e7:f0:3b:a5:22:df:04:2d:54:
                    fa:e8:cc:b1:22:e7:8b:c6:60:d9:72:cd:ad:1a:16:
                    dc:34:9f:99:e8:22:80:95:de:19:5b:0a:4e:d6:d6:
                    47:5a:b7:44:32:17:3f:6b:32:80:e5:0e:35:cb:68:
                    62:c4:dd:31:d1
                ASN1 OID: prime256v1
                NIST CURVE: P-256
        X509v3 extensions:
            X509v3 Key Usage: critical
                Digital Signature, Certificate Sign, CRL Sign
            X509v3 Basic Constraints: critical
                CA:TRUE
            X509v3 Name Constraints: critical
                Permitted:
                  DNS:sub1.example.org
                  DNS:example.com
                Excluded:
                  DNS:example.net
            X509v3 Subject Key Identifier:
                B9:D1:CF:08:A5:F2:99:2B:5A:80:C9:78:23:78:D1:BC:79:70:9F:FB
            X509v3 Authority Key Identifier:
                19:19:E1:8C:09:E2:5D:5C:16:04:E1:9C:74:66:19:FD:B8:52:5B:DF
    Signature Algorithm: ecdsa-with-SHA256
    Signature Value:
        30:46:02:21:00:c0:82:44:fe:0b:f3:1b:db:b4:e4:69:14:64:
        e0:00:0c:1c:fd:87:26:f4:4b:d5:77:25:90:42:cb:06:3e:c1:
        01:02:21:00:b2:4e:2b:01:f8:9b:73:f4:c2:aa:5e:0b:7b:18:
        ed:d6:68:0a:18:fe:b6:d6:7c:20:95:10:57:b1:d1:13:15:a3
*/
static char kValidIntCAWithDNSNameConstraints[] = R"(
-----BEGIN CERTIFICATE-----
MIICcDCCAhWgAwIBAgIUJdaIMcl8qRU8pgECCZKpuzCGPQQwCgYIKoZIzj0EAwIw
YDELMAkGA1UEBhMCVVMxEzARBgNVBAgMCldhc2hpbmd0b24xFjAUBgNVBAoMDUFX
UyBMaWJjcnlwdG8xEDAOBgNVBAsMB0dvb2QgQ0ExEjAQBgNVBAMMCVJvb3QgQ0Eg
MTAgFw0xNTAxMDEwMDAwMDBaGA8yMTAwMDEwMTAwMDAwMFowYjELMAkGA1UEBhMC
VVMxEzARBgNVBAgMCldhc2hpbmd0b24xFjAUBgNVBAoMDUFXUyBMaWJjcnlwdG8x
EDAOBgNVBAsMB0dvb2QgQ0ExFDASBgNVBAMMC0ludCBOQyBDQSAzMFkwEwYHKoZI
zj0CAQYIKoZIzj0DAQcDQgAEnmTRjOHn8DulIt8ELVT66MyxIueLxmDZcs2tGhbc
NJ+Z6CKAld4ZWwpO1tZHWrdEMhc/azKA5Q41y2hixN0x0aOBqDCBpTAOBgNVHQ8B
Af8EBAMCAYYwDwYDVR0TAQH/BAUwAwEB/zBCBgNVHR4BAf8EODA2oCMwEoIQc3Vi
MS5leGFtcGxlLm9yZzANggtleGFtcGxlLmNvbaEPMA2CC2V4YW1wbGUubmV0MB0G
A1UdDgQWBBS50c8IpfKZK1qAyXgjeNG8eXCf+zAfBgNVHSMEGDAWgBQZGeGMCeJd
XBYE4Zx0Zhn9uFJb3zAKBggqhkjOPQQDAgNJADBGAiEAwIJE/gvzG9u05GkUZOAA
DBz9hyb0S9V3JZBCywY+wQECIQCyTisB+Jtz9MKqXgt7GO3WaAoY/rbWfCCVEFex
0RMVow==
-----END CERTIFICATE-----
)";

/*
End-entity certificate signed by |kValidIntCAWithDNSNameConstraints|.

Certificate:
    Data:
        Version: 3 (0x2)
        Serial Number:
            13:a7:6c:6b:2b:be:f1:de:3c:6b:a3:49:de:f1:54:c5:ee:d9:f6:40
        Signature Algorithm: ecdsa-with-SHA256
        Issuer: C=US, ST=Washington, O=AWS Libcrypto, OU=Good CA, CN=Int NC CA 3
        Validity
            Not Before: Jan  1 00:00:00 2015 GMT
            Not After : Jan  1 00:00:00 2100 GMT
        Subject: C=US, ST=Washington, O=AWS Libcrypto, OU=Good Endpoint, CN=Valid DNS SAN 1
        Subject Public Key Info:
            Public Key Algorithm: id-ecPublicKey
                Public-Key: (256 bit)
                pub:
                    04:b2:b7:bd:35:f2:eb:da:86:d5:dc:40:44:c7:23:
                    14:f9:d0:a5:40:17:30:85:b6:c6:11:38:c2:db:2c:
                    c5:bc:0c:19:11:d8:68:61:d6:a3:92:6b:8a:18:52:
                    2c:dc:86:a7:ad:29:ad:91:ac:7e:df:87:24:3b:f3:
                    b4:71:2b:4e:58
                ASN1 OID: prime256v1
                NIST CURVE: P-256
        X509v3 extensions:
            X509v3 Key Usage: critical
                Digital Signature, Key Encipherment
            X509v3 Basic Constraints: critical
                CA:FALSE
            X509v3 Extended Key Usage:
                TLS Web Server Authentication, TLS Web Client Authentication
            X509v3 Subject Alternative Name:
                DNS:sub1.example.com
            X509v3 Subject Key Identifier:
                C8:78:64:E9:F7:9C:0F:56:E2:1D:CE:EE:ED:24:E0:9F:1D:4B:A3:BF
            X509v3 Authority Key Identifier:
                B9:D1:CF:08:A5:F2:99:2B:5A:80:C9:78:23:78:D1:BC:79:70:9F:FB
    Signature Algorithm: ecdsa-with-SHA256
    Signature Value:
        30:45:02:21:00:a7:bd:f2:19:89:ac:03:87:ff:e5:3c:66:2b:
        15:62:a3:32:60:c8:72:07:8d:5e:ae:a4:ef:eb:74:ad:3f:9b:
        8a:02:20:28:5b:6e:9b:79:48:e3:c2:c7:07:96:b4:40:78:8b:
        aa:de:54:d9:d6:77:45:21:03:66:65:d9:b1:f2:96:81:58
*/
static char kValidEndEntityCert1BoundByDNSNameConstraints[] = R"(
-----BEGIN CERTIFICATE-----
MIICcDCCAhagAwIBAgIUSNu3E7HssOiTs61FkbhbgoYPs/swCgYIKoZIzj0EAwIw
YjELMAkGA1UEBhMCVVMxEzARBgNVBAgMCldhc2hpbmd0b24xFjAUBgNVBAoMDUFX
UyBMaWJjcnlwdG8xEDAOBgNVBAsMB0dvb2QgQ0ExFDASBgNVBAMMC0ludCBOQyBD
QSAzMCAXDTE1MDEwMTAwMDAwMFoYDzIxMDAwMTAxMDAwMDAwWjBsMQswCQYDVQQG
EwJVUzETMBEGA1UECAwKV2FzaGluZ3RvbjEWMBQGA1UECgwNQVdTIExpYmNyeXB0
bzEWMBQGA1UECwwNR29vZCBFbmRwb2ludDEYMBYGA1UEAwwPVmFsaWQgRE5TIFNB
TiAxMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEsre9NfLr2obV3EBExyMU+dCl
QBcwhbbGETjC2yzFvAwZEdhoYdajkmuKGFIs3IanrSmtkax+34ckO/O0cStOWKOB
nTCBmjAOBgNVHQ8BAf8EBAMCBaAwDAYDVR0TAQH/BAIwADAdBgNVHSUEFjAUBggr
BgEFBQcDAQYIKwYBBQUHAwIwGwYDVR0RBBQwEoIQc3ViMS5leGFtcGxlLm9yZzAd
BgNVHQ4EFgQUyHhk6fecD1biHc7u7STgnx1Lo78wHwYDVR0jBBgwFoAUudHPCKXy
mStagMl4I3jRvHlwn/swCgYIKoZIzj0EAwIDSAAwRQIgamePGu5VqtoHMBTiUCVE
vq2ZETn7r5x+nkUlwuv1ikwCIQCTZt832F+9wDkTpZIzObzjIi2sYfQ+s9tlf52e
xt/tmA==
-----END CERTIFICATE-----
)";

/*
End-entity certificate signed by |kValidIntCAWithDNSNameConstraints|

Certificate:
    Data:
        Version: 3 (0x2)
        Serial Number:
            7c:1c:0f:2e:ae:d0:fb:db:b2:03:03:b7:40:db:1f:04:eb:0f:bb:9e
        Signature Algorithm: ecdsa-with-SHA256
        Issuer: C=US, ST=Washington, O=AWS Libcrypto, OU=Good CA, CN=Int NC CA 3
        Validity
            Not Before: Jan  1 00:00:00 2015 GMT
            Not After : Jan  1 00:00:00 2100 GMT
        Subject: C=US, ST=Washington, O=AWS Libcrypto, OU=Good Endpoint, CN=Valid DNS SAN 2
        Subject Public Key Info:
            Public Key Algorithm: id-ecPublicKey
                Public-Key: (256 bit)
                pub:
                    04:b2:b7:bd:35:f2:eb:da:86:d5:dc:40:44:c7:23:
                    14:f9:d0:a5:40:17:30:85:b6:c6:11:38:c2:db:2c:
                    c5:bc:0c:19:11:d8:68:61:d6:a3:92:6b:8a:18:52:
                    2c:dc:86:a7:ad:29:ad:91:ac:7e:df:87:24:3b:f3:
                    b4:71:2b:4e:58
                ASN1 OID: prime256v1
                NIST CURVE: P-256
        X509v3 extensions:
            X509v3 Key Usage: critical
                Digital Signature, Key Encipherment
            X509v3 Basic Constraints: critical
                CA:FALSE
            X509v3 Extended Key Usage:
                TLS Web Server Authentication, TLS Web Client Authentication
            X509v3 Subject Alternative Name:
                DNS:example.com
            X509v3 Subject Key Identifier:
                C8:78:64:E9:F7:9C:0F:56:E2:1D:CE:EE:ED:24:E0:9F:1D:4B:A3:BF
            X509v3 Authority Key Identifier:
                B9:D1:CF:08:A5:F2:99:2B:5A:80:C9:78:23:78:D1:BC:79:70:9F:FB
    Signature Algorithm: ecdsa-with-SHA256
    Signature Value:
        30:45:02:21:00:ac:75:ec:a3:f2:cf:89:48:c0:0e:32:7b:1d:
        94:8e:72:46:33:e7:6e:4b:56:65:a2:2f:1f:27:2b:e2:cf:77:
        8b:02:20:5e:49:31:87:80:73:c8:61:de:45:06:b5:bd:a8:26:
        fb:8f:7c:95:3e:4b:d6:e1:ff:7a:27:19:c0:f9:81:61:8c
*/
static char kValidEndEntityCert2BoundByDNSNameConstraints[] = R"(
-----BEGIN CERTIFICATE-----
MIICazCCAhGgAwIBAgIUfBwPLq7Q+9uyAwO3QNsfBOsPu54wCgYIKoZIzj0EAwIw
YjELMAkGA1UEBhMCVVMxEzARBgNVBAgMCldhc2hpbmd0b24xFjAUBgNVBAoMDUFX
UyBMaWJjcnlwdG8xEDAOBgNVBAsMB0dvb2QgQ0ExFDASBgNVBAMMC0ludCBOQyBD
QSAzMCAXDTE1MDEwMTAwMDAwMFoYDzIxMDAwMTAxMDAwMDAwWjBsMQswCQYDVQQG
EwJVUzETMBEGA1UECAwKV2FzaGluZ3RvbjEWMBQGA1UECgwNQVdTIExpYmNyeXB0
bzEWMBQGA1UECwwNR29vZCBFbmRwb2ludDEYMBYGA1UEAwwPVmFsaWQgRE5TIFNB
TiAyMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEsre9NfLr2obV3EBExyMU+dCl
QBcwhbbGETjC2yzFvAwZEdhoYdajkmuKGFIs3IanrSmtkax+34ckO/O0cStOWKOB
mDCBlTAOBgNVHQ8BAf8EBAMCBaAwDAYDVR0TAQH/BAIwADAdBgNVHSUEFjAUBggr
BgEFBQcDAQYIKwYBBQUHAwIwFgYDVR0RBA8wDYILZXhhbXBsZS5jb20wHQYDVR0O
BBYEFMh4ZOn3nA9W4h3O7u0k4J8dS6O/MB8GA1UdIwQYMBaAFLnRzwil8pkrWoDJ
eCN40bx5cJ/7MAoGCCqGSM49BAMCA0gAMEUCIQCsdeyj8s+JSMAOMnsdlI5yRjPn
bktWZaIvHycr4s93iwIgXkkxh4BzyGHeRQa1vagm+498lT5L1uH/eicZwPmBYYw=
-----END CERTIFICATE-----
)";

/*
Certificate:
    Data:
        Version: 3 (0x2)
        Serial Number:
            6b:65:97:b0:26:c2:c0:2c:4a:d1:ce:c8:04:b8:9f:ed:0c:56:0c:67
        Signature Algorithm: ecdsa-with-SHA256
        Issuer: C=US, ST=Washington, O=AWS Libcrypto, OU=Good CA, CN=Int NC CA 3
        Validity
            Not Before: Jan  1 00:00:00 2015 GMT
            Not After : Jan  1 00:00:00 2100 GMT
        Subject: C=US, ST=Washington, O=AWS Libcrypto, OU=Bad Endpoint, CN=Invalid DNS SAN 1
        Subject Public Key Info:
            Public Key Algorithm: id-ecPublicKey
                Public-Key: (256 bit)
                pub:
                    04:b2:b7:bd:35:f2:eb:da:86:d5:dc:40:44:c7:23:
                    14:f9:d0:a5:40:17:30:85:b6:c6:11:38:c2:db:2c:
                    c5:bc:0c:19:11:d8:68:61:d6:a3:92:6b:8a:18:52:
                    2c:dc:86:a7:ad:29:ad:91:ac:7e:df:87:24:3b:f3:
                    b4:71:2b:4e:58
                ASN1 OID: prime256v1
                NIST CURVE: P-256
        X509v3 extensions:
            X509v3 Key Usage: critical
                Digital Signature, Key Encipherment
            X509v3 Basic Constraints: critical
                CA:FALSE
            X509v3 Extended Key Usage:
                TLS Web Server Authentication, TLS Web Client Authentication
            X509v3 Subject Alternative Name:
                DNS:example.net
            X509v3 Subject Key Identifier:
                C8:78:64:E9:F7:9C:0F:56:E2:1D:CE:EE:ED:24:E0:9F:1D:4B:A3:BF
            X509v3 Authority Key Identifier:
                B9:D1:CF:08:A5:F2:99:2B:5A:80:C9:78:23:78:D1:BC:79:70:9F:FB
    Signature Algorithm: ecdsa-with-SHA256
    Signature Value:
        30:46:02:21:00:82:51:95:a9:e1:4b:fc:42:5d:00:d0:36:ca:
        9e:3b:2c:15:98:5a:0f:12:41:0f:64:69:2b:68:c9:1f:ea:22:
        01:02:21:00:a7:87:49:1e:dd:c3:e2:ac:17:b9:d0:8a:52:87:
        26:ec:85:7e:35:54:16:93:d2:7c:51:a9:4a:a9:11:32:6c:d2
*/
static char kInvalidEndEntityCertBoundByDNSNameConstraints[] = R"(
-----BEGIN CERTIFICATE-----
MIICbTCCAhKgAwIBAgIUa2WXsCbCwCxK0c7IBLif7QxWDGcwCgYIKoZIzj0EAwIw
YjELMAkGA1UEBhMCVVMxEzARBgNVBAgMCldhc2hpbmd0b24xFjAUBgNVBAoMDUFX
UyBMaWJjcnlwdG8xEDAOBgNVBAsMB0dvb2QgQ0ExFDASBgNVBAMMC0ludCBOQyBD
QSAzMCAXDTE1MDEwMTAwMDAwMFoYDzIxMDAwMTAxMDAwMDAwWjBtMQswCQYDVQQG
EwJVUzETMBEGA1UECAwKV2FzaGluZ3RvbjEWMBQGA1UECgwNQVdTIExpYmNyeXB0
bzEVMBMGA1UECwwMQmFkIEVuZHBvaW50MRowGAYDVQQDDBFJbnZhbGlkIEROUyBT
QU4gMTBZMBMGByqGSM49AgEGCCqGSM49AwEHA0IABLK3vTXy69qG1dxARMcjFPnQ
pUAXMIW2xhE4wtssxbwMGRHYaGHWo5JrihhSLNyGp60prZGsft+HJDvztHErTlij
gZgwgZUwDgYDVR0PAQH/BAQDAgWgMAwGA1UdEwEB/wQCMAAwHQYDVR0lBBYwFAYI
KwYBBQUHAwEGCCsGAQUFBwMCMBYGA1UdEQQPMA2CC2V4YW1wbGUubmV0MB0GA1Ud
DgQWBBTIeGTp95wPVuIdzu7tJOCfHUujvzAfBgNVHSMEGDAWgBS50c8IpfKZK1qA
yXgjeNG8eXCf+zAKBggqhkjOPQQDAgNJADBGAiEAglGVqeFL/EJdANA2yp47LBWY
Wg8SQQ9kaStoyR/qIgECIQCnh0ke3cPirBe50IpShybshX41VBaT0nxRqUqpETJs
0g==
-----END CERTIFICATE-----
)";

/*
Certificate:
    Data:
        Version: 3 (0x2)
        Serial Number:
            72:4f:3b:96:ff:25:4a:64:c0:11:3b:4f:e1:67:c8:16:e2:9f:d7:5e
        Signature Algorithm: ecdsa-with-SHA256
        Issuer: C=US, ST=Washington, O=AWS Libcrypto, OU=Good CA, CN=Int NC CA 3
        Validity
            Not Before: Jan  1 00:00:00 2015 GMT
            Not After : Jan  1 00:00:00 2100 GMT
        Subject: C=US, ST=Washington, O=AWS Libcrypto, OU=Good Endpoint, SN=Valid DNS in CN, CN=example.com
        Subject Public Key Info:
            Public Key Algorithm: id-ecPublicKey
                Public-Key: (256 bit)
                pub:
                    04:b2:b7:bd:35:f2:eb:da:86:d5:dc:40:44:c7:23:
                    14:f9:d0:a5:40:17:30:85:b6:c6:11:38:c2:db:2c:
                    c5:bc:0c:19:11:d8:68:61:d6:a3:92:6b:8a:18:52:
                    2c:dc:86:a7:ad:29:ad:91:ac:7e:df:87:24:3b:f3:
                    b4:71:2b:4e:58
                ASN1 OID: prime256v1
                NIST CURVE: P-256
        X509v3 extensions:
            X509v3 Key Usage: critical
                Digital Signature, Key Encipherment
            X509v3 Basic Constraints: critical
                CA:FALSE
            X509v3 Extended Key Usage:
                TLS Web Server Authentication, TLS Web Client Authentication
            X509v3 Subject Key Identifier:
                C8:78:64:E9:F7:9C:0F:56:E2:1D:CE:EE:ED:24:E0:9F:1D:4B:A3:BF
            X509v3 Authority Key Identifier:
                B9:D1:CF:08:A5:F2:99:2B:5A:80:C9:78:23:78:D1:BC:79:70:9F:FB
    Signature Algorithm: ecdsa-with-SHA256
    Signature Value:
        30:45:02:20:4d:66:e6:5d:50:b7:a6:14:c2:6c:7f:d2:a2:8d:
        8e:3a:76:5c:15:7c:16:9c:b2:cd:d1:0d:1c:f6:3f:9c:25:fb:
        02:21:00:c7:a6:3d:14:7c:03:8a:87:38:b8:48:a0:1b:84:90:
        ce:2e:d8:72:ea:44:cc:1b:6f:38:d3:08:69:99:ed:09:1b
*/
static char kValidEECertWithDnsCnNoSubjAltDnsName[] = R"(
-----BEGIN CERTIFICATE-----
MIICaDCCAg6gAwIBAgIUck87lv8lSmTAETtP4WfIFuKf114wCgYIKoZIzj0EAwIw
YjELMAkGA1UEBhMCVVMxEzARBgNVBAgMCldhc2hpbmd0b24xFjAUBgNVBAoMDUFX
UyBMaWJjcnlwdG8xEDAOBgNVBAsMB0dvb2QgQ0ExFDASBgNVBAMMC0ludCBOQyBD
QSAzMCAXDTE1MDEwMTAwMDAwMFoYDzIxMDAwMTAxMDAwMDAwWjCBgjELMAkGA1UE
BhMCVVMxEzARBgNVBAgMCldhc2hpbmd0b24xFjAUBgNVBAoMDUFXUyBMaWJjcnlw
dG8xFjAUBgNVBAsMDUdvb2QgRW5kcG9pbnQxGDAWBgNVBAQMD1ZhbGlkIEROUyBp
biBDTjEUMBIGA1UEAwwLZXhhbXBsZS5jb20wWTATBgcqhkjOPQIBBggqhkjOPQMB
BwNCAASyt7018uvahtXcQETHIxT50KVAFzCFtsYROMLbLMW8DBkR2Ghh1qOSa4oY
UizchqetKa2RrH7fhyQ787RxK05Yo38wfTAOBgNVHQ8BAf8EBAMCBaAwDAYDVR0T
AQH/BAIwADAdBgNVHSUEFjAUBggrBgEFBQcDAQYIKwYBBQUHAwIwHQYDVR0OBBYE
FMh4ZOn3nA9W4h3O7u0k4J8dS6O/MB8GA1UdIwQYMBaAFLnRzwil8pkrWoDJeCN4
0bx5cJ/7MAoGCCqGSM49BAMCA0gAMEUCIE1m5l1Qt6YUwmx/0qKNjjp2XBV8Fpyy
zdENHPY/nCX7AiEAx6Y9FHwDioc4uEigG4SQzi7YcupEzBtvONMIaZntCRs=
-----END CERTIFICATE-----
)";

/*
Certificate:
    Data:
        Version: 3 (0x2)
        Serial Number:
            23:ea:00:83:d0:f1:fb:e1:1c:d2:63:e7:9e:8b:cb:3f:66:42:a8:93
        Signature Algorithm: ecdsa-with-SHA256
        Issuer: C=US, ST=Washington, O=AWS Libcrypto, OU=Good CA, CN=Int NC CA 3
        Validity
            Not Before: Jan  1 00:00:00 2015 GMT
            Not After : Jan  1 00:00:00 2100 GMT
        Subject: C=US, ST=Washington, O=AWS Libcrypto, OU=Good Endpoint, SN=Invalid DNS CN and Valid SubjAlt, CN=example.net
        Subject Public Key Info:
            Public Key Algorithm: id-ecPublicKey
                Public-Key: (256 bit)
                pub:
                    04:b2:b7:bd:35:f2:eb:da:86:d5:dc:40:44:c7:23:
                    14:f9:d0:a5:40:17:30:85:b6:c6:11:38:c2:db:2c:
                    c5:bc:0c:19:11:d8:68:61:d6:a3:92:6b:8a:18:52:
                    2c:dc:86:a7:ad:29:ad:91:ac:7e:df:87:24:3b:f3:
                    b4:71:2b:4e:58
                ASN1 OID: prime256v1
                NIST CURVE: P-256
        X509v3 extensions:
            X509v3 Key Usage: critical
                Digital Signature, Key Encipherment
            X509v3 Basic Constraints: critical
                CA:FALSE
            X509v3 Extended Key Usage:
                TLS Web Server Authentication, TLS Web Client Authentication
            X509v3 Subject Alternative Name:
                DNS:example.com
            X509v3 Subject Key Identifier:
                C8:78:64:E9:F7:9C:0F:56:E2:1D:CE:EE:ED:24:E0:9F:1D:4B:A3:BF
            X509v3 Authority Key Identifier:
                B9:D1:CF:08:A5:F2:99:2B:5A:80:C9:78:23:78:D1:BC:79:70:9F:FB
    Signature Algorithm: ecdsa-with-SHA256
    Signature Value:
        30:46:02:21:00:f3:e3:f2:ba:4d:e5:ae:7a:8d:38:cf:13:76:
        e9:e3:ec:9b:7b:00:4a:42:70:b8:0b:dd:a1:ae:9f:3e:87:56:
        f0:02:21:00:a9:ea:f5:8f:3c:1d:2d:9b:1a:86:e2:3c:18:f6:
        43:3f:61:3b:62:d5:12:df:37:28:31:ad:06:fe:21:f1:df:ff
*/
static char kEECertWithInvalidDnsCnValidSubjAltDnsName[] = R"(
-----BEGIN CERTIFICATE-----
MIIClDCCAjmgAwIBAgIUI+oAg9Dx++Ec0mPnnovLP2ZCqJMwCgYIKoZIzj0EAwIw
YjELMAkGA1UEBhMCVVMxEzARBgNVBAgMCldhc2hpbmd0b24xFjAUBgNVBAoMDUFX
UyBMaWJjcnlwdG8xEDAOBgNVBAsMB0dvb2QgQ0ExFDASBgNVBAMMC0ludCBOQyBD
QSAzMCAXDTE1MDEwMTAwMDAwMFoYDzIxMDAwMTAxMDAwMDAwWjCBkzELMAkGA1UE
BhMCVVMxEzARBgNVBAgMCldhc2hpbmd0b24xFjAUBgNVBAoMDUFXUyBMaWJjcnlw
dG8xFjAUBgNVBAsMDUdvb2QgRW5kcG9pbnQxKTAnBgNVBAQMIEludmFsaWQgRE5T
IENOIGFuZCBWYWxpZCBTdWJqQWx0MRQwEgYDVQQDDAtleGFtcGxlLm5ldDBZMBMG
ByqGSM49AgEGCCqGSM49AwEHA0IABLK3vTXy69qG1dxARMcjFPnQpUAXMIW2xhE4
wtssxbwMGRHYaGHWo5JrihhSLNyGp60prZGsft+HJDvztHErTlijgZgwgZUwDgYD
VR0PAQH/BAQDAgWgMAwGA1UdEwEB/wQCMAAwHQYDVR0lBBYwFAYIKwYBBQUHAwEG
CCsGAQUFBwMCMBYGA1UdEQQPMA2CC2V4YW1wbGUuY29tMB0GA1UdDgQWBBTIeGTp
95wPVuIdzu7tJOCfHUujvzAfBgNVHSMEGDAWgBS50c8IpfKZK1qAyXgjeNG8eXCf
+zAKBggqhkjOPQQDAgNJADBGAiEA8+Pyuk3lrnqNOM8Tdunj7Jt7AEpCcLgL3aGu
nz6HVvACIQCp6vWPPB0tmxqG4jwY9kM/YTti1RLfNygxrQb+IfHf/w==
-----END CERTIFICATE-----
)";

/*
Certificate:
    Data:
        Version: 3 (0x2)
        Serial Number:
            75:39:ec:88:9a:bd:4b:2c:1e:65:92:f0:c6:46:1f:03:2d:42:23:55
        Signature Algorithm: ecdsa-with-SHA256
        Issuer: C=US, ST=Washington, O=AWS Libcrypto, OU=Good CA, CN=Int NC CA 3
        Validity
            Not Before: Jan  1 00:00:00 2015 GMT
            Not After : Jan  1 00:00:00 2100 GMT
        Subject: C=US, ST=Washington, O=AWS Libcrypto, OU=Good Endpoint, SN=Invalid DNS CN and IP SubjAlt, CN=example.net
        Subject Public Key Info:
            Public Key Algorithm: id-ecPublicKey
                Public-Key: (256 bit)
                pub:
                    04:b2:b7:bd:35:f2:eb:da:86:d5:dc:40:44:c7:23:
                    14:f9:d0:a5:40:17:30:85:b6:c6:11:38:c2:db:2c:
                    c5:bc:0c:19:11:d8:68:61:d6:a3:92:6b:8a:18:52:
                    2c:dc:86:a7:ad:29:ad:91:ac:7e:df:87:24:3b:f3:
                    b4:71:2b:4e:58
                ASN1 OID: prime256v1
                NIST CURVE: P-256
        X509v3 extensions:
            X509v3 Key Usage: critical
                Digital Signature, Key Encipherment
            X509v3 Basic Constraints: critical
                CA:FALSE
            X509v3 Extended Key Usage:
                TLS Web Server Authentication, TLS Web Client Authentication
            X509v3 Subject Alternative Name:
                IP Address:10.1.1.1
            X509v3 Subject Key Identifier:
                C8:78:64:E9:F7:9C:0F:56:E2:1D:CE:EE:ED:24:E0:9F:1D:4B:A3:BF
            X509v3 Authority Key Identifier:
                B9:D1:CF:08:A5:F2:99:2B:5A:80:C9:78:23:78:D1:BC:79:70:9F:FB
    Signature Algorithm: ecdsa-with-SHA256
    Signature Value:
        30:46:02:21:00:c9:e5:1c:b9:30:96:f6:3b:34:bb:69:73:b1:
        72:a8:fc:0a:6b:9f:88:46:b3:1b:38:e3:2e:73:91:ac:17:bb:
        9f:02:21:00:c5:83:2f:0e:9d:42:ad:96:18:6e:a0:02:92:ff:
        09:cd:10:a2:7b:f3:92:cb:d8:e4:4c:1f:da:a2:29:bb:64:a2
*/
static char kEECertWithInvalidDnsCnSubjAltIp[] = R"(
-----BEGIN CERTIFICATE-----
MIICijCCAi+gAwIBAgIUdTnsiJq9SyweZZLwxkYfAy1CI1UwCgYIKoZIzj0EAwIw
YjELMAkGA1UEBhMCVVMxEzARBgNVBAgMCldhc2hpbmd0b24xFjAUBgNVBAoMDUFX
UyBMaWJjcnlwdG8xEDAOBgNVBAsMB0dvb2QgQ0ExFDASBgNVBAMMC0ludCBOQyBD
QSAzMCAXDTE1MDEwMTAwMDAwMFoYDzIxMDAwMTAxMDAwMDAwWjCBkDELMAkGA1UE
BhMCVVMxEzARBgNVBAgMCldhc2hpbmd0b24xFjAUBgNVBAoMDUFXUyBMaWJjcnlw
dG8xFjAUBgNVBAsMDUdvb2QgRW5kcG9pbnQxJjAkBgNVBAQMHUludmFsaWQgRE5T
IENOIGFuZCBJUCBTdWJqQWx0MRQwEgYDVQQDDAtleGFtcGxlLm5ldDBZMBMGByqG
SM49AgEGCCqGSM49AwEHA0IABLK3vTXy69qG1dxARMcjFPnQpUAXMIW2xhE4wtss
xbwMGRHYaGHWo5JrihhSLNyGp60prZGsft+HJDvztHErTlijgZEwgY4wDgYDVR0P
AQH/BAQDAgWgMAwGA1UdEwEB/wQCMAAwHQYDVR0lBBYwFAYIKwYBBQUHAwEGCCsG
AQUFBwMCMA8GA1UdEQQIMAaHBAoBAQEwHQYDVR0OBBYEFMh4ZOn3nA9W4h3O7u0k
4J8dS6O/MB8GA1UdIwQYMBaAFLnRzwil8pkrWoDJeCN40bx5cJ/7MAoGCCqGSM49
BAMCA0kAMEYCIQDJ5Ry5MJb2OzS7aXOxcqj8CmufiEazGzjjLnORrBe7nwIhAMWD
Lw6dQq2WGG6gApL/Cc0QonvzksvY5Ewf2qIpu2Si
-----END CERTIFICATE-----
)";

/*
Certificate:
    Data:
        Version: 3 (0x2)
        Serial Number:
            13:c1:63:e7:03:6f:a3:26:ac:31:71:a9:fe:ad:a6:34:20:94:bf:3a
        Signature Algorithm: ecdsa-with-SHA256
        Issuer: C=US, ST=Washington, O=AWS Libcrypto, OU=Good CA, CN=Root CA 1
        Validity
            Not Before: Jan  1 00:00:00 2015 GMT
            Not After : Jan  1 00:00:00 2100 GMT
        Subject: C=US, ST=Washington, O=AWS Libcrypto, OU=Good Endpoint, SN=Wildcard Testing, CN=*.sub2.example.com
        Subject Public Key Info:
            Public Key Algorithm: id-ecPublicKey
                Public-Key: (256 bit)
                pub:
                    04:b2:b7:bd:35:f2:eb:da:86:d5:dc:40:44:c7:23:
                    14:f9:d0:a5:40:17:30:85:b6:c6:11:38:c2:db:2c:
                    c5:bc:0c:19:11:d8:68:61:d6:a3:92:6b:8a:18:52:
                    2c:dc:86:a7:ad:29:ad:91:ac:7e:df:87:24:3b:f3:
                    b4:71:2b:4e:58
                ASN1 OID: prime256v1
                NIST CURVE: P-256
        X509v3 extensions:
            X509v3 Key Usage: critical
                Digital Signature, Key Encipherment
            X509v3 Basic Constraints: critical
                CA:FALSE
            X509v3 Extended Key Usage:
                TLS Web Server Authentication, TLS Web Client Authentication
            X509v3 Subject Alternative Name:
                DNS:*.sub1.example.com, DNS:*.example.org, DNS:host.example.com
            X509v3 Subject Key Identifier:
                C8:78:64:E9:F7:9C:0F:56:E2:1D:CE:EE:ED:24:E0:9F:1D:4B:A3:BF
            X509v3 Authority Key Identifier:
                19:19:E1:8C:09:E2:5D:5C:16:04:E1:9C:74:66:19:FD:B8:52:5B:DF
    Signature Algorithm: ecdsa-with-SHA256
    Signature Value:
        30:45:02:20:13:bc:6c:9c:3b:8e:c7:95:e7:9f:31:08:dd:7f:
        6c:ea:97:4e:29:01:72:b5:9c:45:f1:29:bc:d7:ce:39:5a:21:
        02:21:00:f5:77:2c:9d:23:6d:71:69:4d:93:eb:7e:fd:a5:17:
        24:37:ee:97:01:4f:1c:54:09:cd:3d:87:e9:1d:da:5f:7e
*/
static char kWildCardCommonNameAndSubjAlt[] = R"(
-----BEGIN CERTIFICATE-----
MIICsDCCAlagAwIBAgIUE8Fj5wNvoyasMXGp/q2mNCCUvzowCgYIKoZIzj0EAwIw
YDELMAkGA1UEBhMCVVMxEzARBgNVBAgMCldhc2hpbmd0b24xFjAUBgNVBAoMDUFX
UyBMaWJjcnlwdG8xEDAOBgNVBAsMB0dvb2QgQ0ExEjAQBgNVBAMMCVJvb3QgQ0Eg
MTAgFw0xNTAxMDEwMDAwMDBaGA8yMTAwMDEwMTAwMDAwMFowgYoxCzAJBgNVBAYT
AlVTMRMwEQYDVQQIDApXYXNoaW5ndG9uMRYwFAYDVQQKDA1BV1MgTGliY3J5cHRv
MRYwFAYDVQQLDA1Hb29kIEVuZHBvaW50MRkwFwYDVQQEDBBXaWxkY2FyZCBUZXN0
aW5nMRswGQYDVQQDDBIqLnN1YjIuZXhhbXBsZS5jb20wWTATBgcqhkjOPQIBBggq
hkjOPQMBBwNCAASyt7018uvahtXcQETHIxT50KVAFzCFtsYROMLbLMW8DBkR2Ghh
1qOSa4oYUizchqetKa2RrH7fhyQ787RxK05Yo4HAMIG9MA4GA1UdDwEB/wQEAwIF
oDAMBgNVHRMBAf8EAjAAMB0GA1UdJQQWMBQGCCsGAQUFBwMBBggrBgEFBQcDAjA+
BgNVHREENzA1ghIqLnN1YjEuZXhhbXBsZS5jb22CDSouZXhhbXBsZS5vcmeCEGhv
c3QuZXhhbXBsZS5jb20wHQYDVR0OBBYEFMh4ZOn3nA9W4h3O7u0k4J8dS6O/MB8G
A1UdIwQYMBaAFBkZ4YwJ4l1cFgThnHRmGf24UlvfMAoGCCqGSM49BAMCA0gAMEUC
IBO8bJw7jseV558xCN1/bOqXTikBcrWcRfEpvNfOOVohAiEA9XcsnSNtcWlNk+t+
/aUXJDfulwFPHFQJzT2H6R3aX34=
-----END CERTIFICATE-----
)";

/*
This certificate uses the |EE_KEY_1| key but uses the explicit curve
parameters rather then the prime256v1 curve identifier.

Certificate:
    Data:
        Version: 3 (0x2)
        Serial Number:
            07:52:c5:b3:fd:bd:67:6b:68:bf:ff:2e:ab:a5:81:31:d1:58:86:0f
        Signature Algorithm: ecdsa-with-SHA256
        Issuer: C=US, ST=Washington, O=AWS Libcrypto, OU=Good CA, CN=Root CA 1
        Validity
            Not Before: Jan  1 00:00:00 2015 GMT
            Not After : Jan  1 00:00:00 2100 GMT
        Subject: C=US, ST=Washington, O=AWS Libcrypto, OU=Bad Endpoint, CN=rfc5480 specifiedCurve MUST NOT be used in PKIX
        Subject Public Key Info:
            Public Key Algorithm: id-ecPublicKey
                Public-Key: (256 bit)
                pub:
                    04:c2:a9:7a:df:6d:d4:e3:16:29:b3:74:11:f1:8f:
                    bd:44:8d:c7:3f:b2:d7:9a:e7:10:14:ff:1c:4a:fa:
                    ce:fb:c9:7c:c1:e6:57:f2:ff:31:1b:71:8c:6e:3a:
                    b9:f0:1e:b5:ac:5f:db:2e:81:68:02:cb:be:19:44:
                    d5:89:3d:30:d6
                Field Type: prime-field
                Prime:
                    00:ff:ff:ff:ff:00:00:00:01:00:00:00:00:00:00:
                    00:00:00:00:00:00:ff:ff:ff:ff:ff:ff:ff:ff:ff:
                    ff:ff:ff
                A:   
                    00:ff:ff:ff:ff:00:00:00:01:00:00:00:00:00:00:
                    00:00:00:00:00:00:ff:ff:ff:ff:ff:ff:ff:ff:ff:
                    ff:ff:fc
                B:   
                    5a:c6:35:d8:aa:3a:93:e7:b3:eb:bd:55:76:98:86:
                    bc:65:1d:06:b0:cc:53:b0:f6:3b:ce:3c:3e:27:d2:
                    60:4b
                Generator (uncompressed):
                    04:6b:17:d1:f2:e1:2c:42:47:f8:bc:e6:e5:63:a4:
                    40:f2:77:03:7d:81:2d:eb:33:a0:f4:a1:39:45:d8:
                    98:c2:96:4f:e3:42:e2:fe:1a:7f:9b:8e:e7:eb:4a:
                    7c:0f:9e:16:2b:ce:33:57:6b:31:5e:ce:cb:b6:40:
                    68:37:bf:51:f5
                Order: 
                    00:ff:ff:ff:ff:00:00:00:00:ff:ff:ff:ff:ff:ff:
                    ff:ff:bc:e6:fa:ad:a7:17:9e:84:f3:b9:ca:c2:fc:
                    63:25:51
                Cofactor:  1 (0x1)
                Seed:
                    c4:9d:36:08:86:e7:04:93:6a:66:78:e1:13:9d:26:
                    b7:81:9f:7e:90
        X509v3 extensions:
            X509v3 Key Usage: critical
                Digital Signature
            X509v3 Basic Constraints: critical
                CA:FALSE
            X509v3 Extended Key Usage: 
                TLS Web Server Authentication
            X509v3 Subject Alternative Name: 
                DNS:example.com
            X509v3 Subject Key Identifier: 
                19:19:E1:8C:09:E2:5D:5C:16:04:E1:9C:74:66:19:FD:B8:52:5B:DF
    Signature Algorithm: ecdsa-with-SHA256
    Signature Value:
        30:45:02:20:3e:3e:8b:43:6b:9b:dd:7e:c7:45:24:94:47:28:
        04:ac:2a:4c:22:a8:3b:74:1d:c6:0f:10:38:fe:11:0b:34:1e:
        02:21:00:eb:8d:66:70:e2:2c:76:2b:a2:a3:38:e9:10:51:1c:
        e8:89:82:7e:ce:80:a1:fd:f2:ca:06:60:d0:e6:1e:49:f8
*/
static char kLeafCertificateWithExplicitECParams[] = R"(
-----BEGIN CERTIFICATE-----
MIIDUDCCAvagAwIBAgIUB1LFs/29Z2tov/8uq6WBMdFYhg8wCgYIKoZIzj0EAwIw
YDELMAkGA1UEBhMCVVMxEzARBgNVBAgMCldhc2hpbmd0b24xFjAUBgNVBAoMDUFX
UyBMaWJjcnlwdG8xEDAOBgNVBAsMB0dvb2QgQ0ExEjAQBgNVBAMMCVJvb3QgQ0Eg
MTAgFw0xNTAxMDEwMDAwMDBaGA8yMTAwMDEwMTAwMDAwMFowgYsxCzAJBgNVBAYT
AlVTMRMwEQYDVQQIDApXYXNoaW5ndG9uMRYwFAYDVQQKDA1BV1MgTGliY3J5cHRv
MRUwEwYDVQQLDAxCYWQgRW5kcG9pbnQxODA2BgNVBAMML3JmYzU0ODAgc3BlY2lm
aWVkQ3VydmUgTVVTVCBOT1QgYmUgdXNlZCBpbiBQS0lYMIIBSzCCAQMGByqGSM49
AgEwgfcCAQEwLAYHKoZIzj0BAQIhAP////8AAAABAAAAAAAAAAAAAAAA////////
////////MFsEIP////8AAAABAAAAAAAAAAAAAAAA///////////////8BCBaxjXY
qjqT57PrvVV2mIa8ZR0GsMxTsPY7zjw+J9JgSwMVAMSdNgiG5wSTamZ44ROdJreB
n36QBEEEaxfR8uEsQkf4vOblY6RA8ncDfYEt6zOg9KE5RdiYwpZP40Li/hp/m47n
60p8D54WK84zV2sxXs7LtkBoN79R9QIhAP////8AAAAA//////////+85vqtpxee
hPO5ysL8YyVRAgEBA0IABMKpet9t1OMWKbN0EfGPvUSNxz+y15rnEBT/HEr6zvvJ
fMHmV/L/MRtxjG46ufAetaxf2y6BaALLvhlE1Yk9MNajbDBqMA4GA1UdDwEB/wQE
AwIHgDAMBgNVHRMBAf8EAjAAMBMGA1UdJQQMMAoGCCsGAQUFBwMBMBYGA1UdEQQP
MA2CC2V4YW1wbGUuY29tMB0GA1UdDgQWBBQZGeGMCeJdXBYE4Zx0Zhn9uFJb3zAK
BggqhkjOPQQDAgNIADBFAiA+PotDa5vdfsdFJJRHKASsKkwiqDt0HcYPEDj+EQs0
HgIhAOuNZnDiLHYroqM46RBRHOiJgn7OgKH98soGYNDmHkn4
-----END CERTIFICATE-----
)";

// EE certificate should not verify if signed by invalid root CA
TEST(X509CompatTest, CertificatesFromTrustStoreValidated) {
  bssl::UniquePtr<X509> root = CertFromPEM(kRootBadBasicConstraints);
  ASSERT_TRUE(root);
  bssl::UniquePtr<X509> leaf = CertFromPEM(kEndEntitySignedByBadRoot);
  ASSERT_TRUE(leaf);

  EXPECT_EQ(X509_V_ERR_INVALID_CA,
            Verify(leaf.get(), /*roots=*/{root.get()}, /*intermediates=*/{},
                   /*crls=*/{}, /*flags=*/0));
}

// Certificate should be rejected if it contains a critical AKID extension.
// This reports a X509_V_ERR_UNHANDLED_CRITICAL_EXTENSION due to it being an
// unhandled critical exception.
TEST(X509CompatTest, EECertificateWithCriticalAKID) {
  bssl::UniquePtr<X509> root = CertFromPEM(kValidRootCA1);
  ASSERT_TRUE(root);
  bssl::UniquePtr<X509> leaf =
      CertFromPEM(kInvalidEECertificateWithCriticalAKID);
  ASSERT_TRUE(leaf);

  EXPECT_EQ(X509_V_ERR_UNHANDLED_CRITICAL_EXTENSION,
            Verify(leaf.get(), /*roots=*/{root.get()}, /*intermediates=*/{},
                   /*crls=*/{}, /*flags=*/0));
}

// Certificate should not be rejected if it contains a critical CRL Distribution
// Points extension.
TEST(X509CompatTest, EECertificateWithCriticalCRLDistributionPointsExt) {
  bssl::UniquePtr<X509> root = CertFromPEM(kValidRootCA1);
  ASSERT_TRUE(root);
  bssl::UniquePtr<X509> leaf =
      CertFromPEM(kValidEECertWithCriticalCRLDistributionExt);
  ASSERT_TRUE(leaf);

  EXPECT_EQ(X509_V_OK,
            Verify(leaf.get(), /*roots=*/{root.get()}, /*intermediates=*/{},
                   /*crls=*/{}, /*flags=*/0));
}

// EE certificate's trust root is missing the basic constraints extension.
TEST(X509CompatTest, EECertificateSignedByInvalidRootMissingBasicConstraints) {
  bssl::UniquePtr<X509> root =
      CertFromPEM(kInvalidRootCertificateWithMissingBasicConstraintsExt);
  ASSERT_TRUE(root);
  bssl::UniquePtr<X509> leaf =
      CertFromPEM(kInvalidEECertificateSignedByRootMissingBasicConstraintsExt);
  ASSERT_TRUE(leaf);

  EXPECT_EQ(X509_V_ERR_INVALID_CA,
            Verify(leaf.get(), /*roots=*/{root.get()}, /*intermediates=*/{},
                   /*crls=*/{}, /*flags=*/0));
}

// EE certificate with negative serial number, while technically invalid per RFC
// 5280, should pass.
TEST(X509CompatTest, EECertificateWithNegativeSerialNumber) {
  bssl::UniquePtr<X509> root = CertFromPEM(kValidRootCA1);
  ASSERT_TRUE(root);
  bssl::UniquePtr<X509> leaf =
      CertFromPEM(kValidEECertificateWithNegativeSerialNumber);
  ASSERT_TRUE(leaf);

  EXPECT_EQ(X509_V_OK,
            Verify(leaf.get(), /*roots=*/{root.get()}, /*intermediates=*/{},
                   /*crls=*/{}, /*flags=*/0));
}

TEST(X509CompatTest, EECertificateWithValidNameConstrainedSubjAltIPv4) {
  bssl::UniquePtr<X509> root = CertFromPEM(kValidRootCA1);
  ASSERT_TRUE(root);
  bssl::UniquePtr<X509> inter = CertFromPEM(kValidIntCAWithIPv4NameConstraints);
  ASSERT_TRUE(inter);
  bssl::UniquePtr<X509> leaf =
      CertFromPEM(kValidEECertWithValidIPv4SubjectAltName);
  ASSERT_TRUE(leaf);

  EXPECT_EQ(X509_V_OK, Verify(leaf.get(), /*roots=*/{root.get()},
                              /*intermediates=*/{inter.get()},
                              /*crls=*/{}, /*flags=*/0, [](X509_STORE_CTX * ctx){
                                X509_VERIFY_PARAM *param = X509_STORE_CTX_get0_param(ctx);
                                const char ip[] = "10.1.2.1";
                                X509_VERIFY_PARAM_set1_ip_asc(param, ip);
                              }));
}

TEST(X509CompatTest, EECertificateWithInvalidExcludedNameConstrainedSubjAltIPv4) {
  bssl::UniquePtr<X509> root = CertFromPEM(kValidRootCA1);
  ASSERT_TRUE(root);
  bssl::UniquePtr<X509> inter = CertFromPEM(kValidIntCAWithIPv4NameConstraints);
  ASSERT_TRUE(inter);
  bssl::UniquePtr<X509> leaf =
      CertFromPEM(kInvalidEECertWithInvalidExcludedIPv4SubjAlt);
  ASSERT_TRUE(leaf);

  EXPECT_EQ(X509_V_ERR_EXCLUDED_VIOLATION,
            Verify(leaf.get(), /*roots=*/{root.get()},
                   /*intermediates=*/{inter.get()},
                   /*crls=*/{}, /*flags=*/0));
}

TEST(X509CompatTest, EECertificateWithInvalidPermittedNameConstrainedSubjAltIPv4) {
  bssl::UniquePtr<X509> root = CertFromPEM(kValidRootCA1);
  ASSERT_TRUE(root);
  bssl::UniquePtr<X509> inter = CertFromPEM(kValidIntCAWithIPv4NameConstraints);
  ASSERT_TRUE(inter);
  bssl::UniquePtr<X509> leaf =
      CertFromPEM(kInvalidEECertWithInvalidPermittedIPv4SubjAlt);
  ASSERT_TRUE(leaf);

  EXPECT_EQ(X509_V_ERR_PERMITTED_VIOLATION,
            Verify(leaf.get(), /*roots=*/{root.get()},
                   /*intermediates=*/{inter.get()},
                   /*crls=*/{}, /*flags=*/0));
}

TEST(X509CompatTest, EECertificateWithValidNameConstrainedSubjAltIPv6) {
  bssl::UniquePtr<X509> root = CertFromPEM(kValidRootCA1);
  ASSERT_TRUE(root);
  bssl::UniquePtr<X509> inter = CertFromPEM(kValidIntCAWithIPv6NameConstraints);
  ASSERT_TRUE(inter);
  bssl::UniquePtr<X509> leaf =
      CertFromPEM(kValidEECertWithValidIPv6SubjectAltName);
  ASSERT_TRUE(leaf);

  EXPECT_EQ(X509_V_OK, Verify(leaf.get(), /*roots=*/{root.get()},
                              /*intermediates=*/{inter.get()},
                              /*crls=*/{}, /*flags=*/0, [](X509_STORE_CTX * ctx){
                                X509_VERIFY_PARAM *param = X509_STORE_CTX_get0_param(ctx);
                                const char ip[] = "::ffff:a01:201";
                                X509_VERIFY_PARAM_set1_ip_asc(param, ip);
                              }));
}

TEST(X509CompatTest,
     EECertificateWithInvalidExcludedNameConstrainedSubjAltIPv6) {
  bssl::UniquePtr<X509> root = CertFromPEM(kValidRootCA1);
  ASSERT_TRUE(root);
  bssl::UniquePtr<X509> inter = CertFromPEM(kValidIntCAWithIPv6NameConstraints);
  ASSERT_TRUE(inter);
  bssl::UniquePtr<X509> leaf =
      CertFromPEM(kInvalidEECertWithInvalidExcludedIPv6SubjAlt);
  ASSERT_TRUE(leaf);

  EXPECT_EQ(X509_V_ERR_EXCLUDED_VIOLATION,
            Verify(leaf.get(), /*roots=*/{root.get()},
                   /*intermediates=*/{inter.get()},
                   /*crls=*/{}, /*flags=*/0));
}

TEST(X509CompatTest,
     EECertificateWithInvalidPermittedNameConstrainedSubjAltIPv6) {
  bssl::UniquePtr<X509> root = CertFromPEM(kValidRootCA1);
  ASSERT_TRUE(root);
  bssl::UniquePtr<X509> inter = CertFromPEM(kValidIntCAWithIPv6NameConstraints);
  ASSERT_TRUE(inter);
  bssl::UniquePtr<X509> leaf =
      CertFromPEM(kInvalidEECertWithInvalidPermittedIPv6SubjAlt);
  ASSERT_TRUE(leaf);

  EXPECT_EQ(X509_V_ERR_PERMITTED_VIOLATION,
            Verify(leaf.get(), /*roots=*/{root.get()},
                   /*intermediates=*/{inter.get()},
                   /*crls=*/{}, /*flags=*/0));
}

TEST(X509CompatTest, EECertificateWithIPv4SANIssuedByCAWithBadNameConstraint) {
  bssl::UniquePtr<X509> root = CertFromPEM(kValidRootCA1);
  ASSERT_TRUE(root);
  bssl::UniquePtr<X509> inter = CertFromPEM(kIntCAWithBadIPv4NameConstraint);
  ASSERT_TRUE(inter);
  bssl::UniquePtr<X509> leaf =
      CertFromPEM(kInvalidEECertIssuedByCAWithBadIPv4NameConstraint);
  ASSERT_TRUE(leaf);

  EXPECT_EQ(X509_V_ERR_UNSUPPORTED_NAME_SYNTAX,
            Verify(leaf.get(), /*roots=*/{root.get()},
                   /*intermediates=*/{inter.get()},
                   /*crls=*/{}, /*flags=*/0));
}

TEST(X509CompatTest, EECertificateWithIPv6SANIssuedByCAWithBadNameConstraint) {
  bssl::UniquePtr<X509> root = CertFromPEM(kValidRootCA1);
  ASSERT_TRUE(root);
  bssl::UniquePtr<X509> inter = CertFromPEM(kIntCAWithBadIPv6NameConstraint);
  ASSERT_TRUE(inter);
  bssl::UniquePtr<X509> leaf =
      CertFromPEM(kInvalidEECertIssuedByCAWithBadIPv6NameConstraint);
  ASSERT_TRUE(leaf);

  EXPECT_EQ(X509_V_ERR_UNSUPPORTED_NAME_SYNTAX,
            Verify(leaf.get(), /*roots=*/{root.get()},
                   /*intermediates=*/{inter.get()},
                   /*crls=*/{}, /*flags=*/0));
}

TEST(X509CompatTest, EECertificateWithIPSANIssuedByCAWithBadNameConstraint) {
  bssl::UniquePtr<X509> root = CertFromPEM(kValidRootCA1);
  ASSERT_TRUE(root);
  bssl::UniquePtr<X509> inter = CertFromPEM(kIntCAWithBadIPNameConstraint);
  ASSERT_TRUE(inter);
  bssl::UniquePtr<X509> leaf =
      CertFromPEM(kInvalidEECertIssuedByCAWithBadIPNameConstraint);
  ASSERT_TRUE(leaf);

  EXPECT_EQ(X509_V_ERR_UNSUPPORTED_NAME_SYNTAX,
            Verify(leaf.get(), /*roots=*/{root.get()},
                   /*intermediates=*/{inter.get()},
                   /*crls=*/{}, /*flags=*/0));
}

TEST(X509CompatTest, IpCidrNetmaskTest) {
  struct IpCidrNetmaskTestParam {
    std::vector<uint8_t> mask;
    bool valid;
  };

  IpCidrNetmaskTestParam testInputs[] = {
      // invalid lengths
      {HexToBytes("00"), false},          // single byte address too short
      {HexToBytes("000000"), false},      // one byte less then IPv4
      {HexToBytes("ffffffffff"), false},  // 255.255.255.255.255 (/40)
      {HexToBytes("ffffffff0000000000000000000000"),
       false},  // 1 byte short of IPv6
      {HexToBytes("ffffffffffffffffffffffffffffffff00"),
       false},  // Invalid length - 17 bytes

      // valid v4
      {HexToBytes("00000000"), true},  // 0.0.0.0 (/0)
      {HexToBytes("ff000000"), true},  // 255.0.0.0 (/8)
      {HexToBytes("ffff0000"), true},  // 255.255.0.0 (/16)
      {HexToBytes("ffffff00"), true},  // 255.255.255.0 (/24)
      {HexToBytes("ffffffff"), true},  // 255.255.255.255 (/32)

      // invalid v4 (non-contiguous 1s)
      {HexToBytes("ffff00ff"), false},  // 255.255.0.255
      {HexToBytes("ff00ff00"), false},  // 255.0.255.0
      {HexToBytes("f0f0f0f0"), false},  // 240.240.240.240
      {HexToBytes("0fff0000"), false},  // 15.255.0.0


      // valid v6
      {HexToBytes("00000000000000000000000000000000"),
       true},  // 0000:0000:0000:0000:0000:0000:0000:0000 (/0)
      {HexToBytes("ffff0000000000000000000000000000"),
       true},  // ffff:0000:0000:0000:0000:0000:0000:0000 (/16)
      {HexToBytes("ffffffff000000000000000000000000"),
       true},  // ffff:ffff:0000:0000:0000:0000:0000:0000 (/32)
      {HexToBytes("ffffffffffff00000000000000000000"),
       true},  // ffff:ffff:ffff:0000:0000:0000:0000:0000 (/48)
      {HexToBytes("ffffffffffffffff0000000000000000"),
       true},  // ffff:ffff:ffff:ffff:0000:0000:0000:0000 (/64)
      {HexToBytes("ffffffffffffffffffff000000000000"),
       true},  // ffff:ffff:ffff:ffff:ffff:0000:0000:0000 (/80)
      {HexToBytes("ffffffffffffffffffffffff00000000"),
       true},  // ffff:ffff:ffff:ffff:ffff:ffff:0000:0000 (/96)
      {HexToBytes("ffffffffffffffffffffffffffff0000"),
       true},  // ffff:ffff:ffff:ffff:ffff:ffff:ffff:0000 (/112)
      {HexToBytes("ffffffffffffffffffffffffffffffff"),
       true},  // ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff (/128)

      // invalid v6 (non-contiguous 1s)
      {HexToBytes("ffff0000ffffffffffffffffffffffff"),
       false},  // Non-contiguous 1s
      {HexToBytes("ffffffffffffffff0000ffff00000000"),
       false},  // Non-contiguous 1s
      {HexToBytes("f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0"),
       false},  // Non-contiguous 1s
  };

  for (const IpCidrNetmaskTestParam &params : testInputs) {
    CBS mask;
    CBS_init(&mask, params.mask.data(), params.mask.size());
    ASSERT_EQ(params.valid, validate_cidr_mask(&mask));
  }
}

TEST(X509CompatTest, EECertificate1WithValidSANBoundByNameConstraints) {
  bssl::UniquePtr<X509> root = CertFromPEM(kValidRootCA1);
  ASSERT_TRUE(root);
  bssl::UniquePtr<X509> inter = CertFromPEM(kValidIntCAWithDNSNameConstraints);
  ASSERT_TRUE(inter);
  bssl::UniquePtr<X509> leaf =
      CertFromPEM(kValidEndEntityCert1BoundByDNSNameConstraints);
  ASSERT_TRUE(leaf);

  EXPECT_EQ(X509_V_OK,
            Verify(leaf.get(), /*roots=*/{root.get()},
                   /*intermediates=*/{inter.get()},
                   /*crls=*/{}, /*flags=*/0, [](X509_STORE_CTX *ctx) {
                     X509_VERIFY_PARAM *param = X509_STORE_CTX_get0_param(ctx);
                     const char host[] = "sub1.example.org";
                     X509_VERIFY_PARAM_set1_host(param, host, sizeof(host) - 1);
                   }));
}

TEST(X509CompatTest, EECertificate2WithValidSANBoundByNameConstraints) {
  bssl::UniquePtr<X509> root = CertFromPEM(kValidRootCA1);
  ASSERT_TRUE(root);
  bssl::UniquePtr<X509> inter = CertFromPEM(kValidIntCAWithDNSNameConstraints);
  ASSERT_TRUE(inter);
  bssl::UniquePtr<X509> leaf =
      CertFromPEM(kValidEndEntityCert2BoundByDNSNameConstraints);
  ASSERT_TRUE(leaf);

  EXPECT_EQ(X509_V_OK,
            Verify(leaf.get(), /*roots=*/{root.get()},
                   /*intermediates=*/{inter.get()},
                   /*crls=*/{}, /*flags=*/0, [](X509_STORE_CTX *ctx) {
                     X509_VERIFY_PARAM *param = X509_STORE_CTX_get0_param(ctx);
                     const char host[] = "example.com";
                     X509_VERIFY_PARAM_set1_host(param, host, sizeof(host) - 1);
                   }));
}

TEST(X509CompatTest, EECertificate3WithInvalidSANBoundByNameConstraints) {
  bssl::UniquePtr<X509> root = CertFromPEM(kValidRootCA1);
  ASSERT_TRUE(root);
  bssl::UniquePtr<X509> inter = CertFromPEM(kValidIntCAWithDNSNameConstraints);
  ASSERT_TRUE(inter);
  bssl::UniquePtr<X509> leaf = CertFromPEM(kInvalidEndEntityCertBoundByDNSNameConstraints);
  ASSERT_TRUE(leaf);

  EXPECT_EQ(X509_V_ERR_PERMITTED_VIOLATION,
            Verify(leaf.get(), /*roots=*/{root.get()},
                   /*intermediates=*/{inter.get()},
                   /*crls=*/{}, /*flags=*/0, [](X509_STORE_CTX *ctx) {
                     X509_VERIFY_PARAM *param = X509_STORE_CTX_get0_param(ctx);
                     const char host[] = "example.net";
                     X509_VERIFY_PARAM_set1_host(param, host, sizeof(host) - 1);
                   }));
}

TEST(X509CompatTest, ValidEECertWithDnsCnNoSubjAltDnsName) {
    bssl::UniquePtr<X509> root = CertFromPEM(kValidRootCA1);
    ASSERT_TRUE(root);
    bssl::UniquePtr<X509> inter = CertFromPEM(kValidIntCAWithDNSNameConstraints);
    ASSERT_TRUE(inter);
    bssl::UniquePtr<X509> leaf = CertFromPEM(kValidEECertWithDnsCnNoSubjAltDnsName);
    ASSERT_TRUE(leaf);

    const char host[] = "example.com";

    EXPECT_EQ(
        X509_V_OK,
        Verify(leaf.get(), /*roots=*/{root.get()},
               /*intermediates=*/{inter.get()},
               /*crls=*/{}, /*flags=*/0, [&host](X509_STORE_CTX *ctx) {
                 X509_VERIFY_PARAM *param = X509_STORE_CTX_get0_param(ctx);
                 X509_VERIFY_PARAM_set1_host(param, host, sizeof(host) - 1);
               }));

    EXPECT_EQ(
        X509_V_ERR_HOSTNAME_MISMATCH,
        Verify(leaf.get(), /*roots=*/{root.get()},
               /*intermediates=*/{inter.get()},
               /*crls=*/{}, /*flags=*/0, [&host](X509_STORE_CTX *ctx) {
                 X509_VERIFY_PARAM *param = X509_STORE_CTX_get0_param(ctx);
                 X509_VERIFY_PARAM_set_hostflags(
                     param, X509_CHECK_FLAG_NEVER_CHECK_SUBJECT);
                 X509_VERIFY_PARAM_set1_host(param, host, sizeof(host) - 1);
               }));
}

TEST(X509CompatTest, kEECertWithInvalidDnsCnValidSubjAltDnsName) {
  bssl::UniquePtr<X509> root = CertFromPEM(kValidRootCA1);
  ASSERT_TRUE(root);
  bssl::UniquePtr<X509> inter = CertFromPEM(kValidIntCAWithDNSNameConstraints);
  ASSERT_TRUE(inter);
  bssl::UniquePtr<X509> leaf =
      CertFromPEM(kEECertWithInvalidDnsCnValidSubjAltDnsName);
  ASSERT_TRUE(leaf);

  const char host[] = "example.com";

  EXPECT_EQ(X509_V_OK,
            Verify(leaf.get(), /*roots=*/{root.get()},
                   /*intermediates=*/{inter.get()},
                   /*crls=*/{}, /*flags=*/0, [&host](X509_STORE_CTX *ctx) {
                     X509_VERIFY_PARAM *param = X509_STORE_CTX_get0_param(ctx);
                     X509_VERIFY_PARAM_set1_host(param, host, sizeof(host) - 1);
                   }));

  EXPECT_EQ(X509_V_OK,
            Verify(leaf.get(), /*roots=*/{root.get()},
                   /*intermediates=*/{inter.get()},
                   /*crls=*/{}, /*flags=*/0, [&host](X509_STORE_CTX *ctx) {
                     X509_VERIFY_PARAM *param = X509_STORE_CTX_get0_param(ctx);
                     X509_VERIFY_PARAM_set_hostflags(
                         param, X509_CHECK_FLAG_NEVER_CHECK_SUBJECT);
                     X509_VERIFY_PARAM_set1_host(param, host, sizeof(host) - 1);
                   }));

  EXPECT_EQ(X509_V_ERR_PERMITTED_VIOLATION,
            Verify(leaf.get(), /*roots=*/{root.get()},
                   /*intermediates=*/{inter.get()},
                   /*crls=*/{}, /*flags=*/0, [&host](X509_STORE_CTX *ctx) {
                     X509_VERIFY_PARAM *param = X509_STORE_CTX_get0_param(ctx);
                     X509_VERIFY_PARAM_set_hostflags(
                         param, X509_CHECK_FLAG_ALWAYS_CHECK_SUBJECT);
                     X509_VERIFY_PARAM_set1_host(param, host, sizeof(host) - 1);
                   }));
}

TEST(X509CompatTest, EECertWithInvalidDnsCnSubjAltIp) {
  bssl::UniquePtr<X509> root = CertFromPEM(kValidRootCA1);
  ASSERT_TRUE(root);
  bssl::UniquePtr<X509> inter = CertFromPEM(kValidIntCAWithDNSNameConstraints);
  ASSERT_TRUE(inter);
  bssl::UniquePtr<X509> leaf = CertFromPEM(kEECertWithInvalidDnsCnSubjAltIp);
  ASSERT_TRUE(leaf);

  const char ip[] = "10.1.1.1";

  EXPECT_EQ(X509_V_ERR_PERMITTED_VIOLATION,
            Verify(leaf.get(), /*roots=*/{root.get()},
                   /*intermediates=*/{inter.get()},
                   /*crls=*/{}, /*flags=*/0, [&ip](X509_STORE_CTX *ctx) {
                     X509_VERIFY_PARAM *param = X509_STORE_CTX_get0_param(ctx);
                     X509_VERIFY_PARAM_set1_ip_asc(param, ip);
                   }));

  EXPECT_EQ(X509_V_OK,
            Verify(leaf.get(), /*roots=*/{root.get()},
                   /*intermediates=*/{inter.get()},
                   /*crls=*/{}, /*flags=*/0, [&ip](X509_STORE_CTX *ctx) {
                     X509_VERIFY_PARAM *param = X509_STORE_CTX_get0_param(ctx);
                     X509_VERIFY_PARAM_set_hostflags(
                         param, X509_CHECK_FLAG_NEVER_CHECK_SUBJECT);
                     X509_VERIFY_PARAM_set1_ip_asc(param, ip);
                   }));

  EXPECT_EQ(X509_V_ERR_PERMITTED_VIOLATION,
            Verify(leaf.get(), /*roots=*/{root.get()},
                   /*intermediates=*/{inter.get()},
                   /*crls=*/{}, /*flags=*/0, [&ip](X509_STORE_CTX *ctx) {
                     X509_VERIFY_PARAM *param = X509_STORE_CTX_get0_param(ctx);
                     X509_VERIFY_PARAM_set_hostflags(
                         param, X509_CHECK_FLAG_ALWAYS_CHECK_SUBJECT);
                     X509_VERIFY_PARAM_set1_ip_asc(param, ip);
                   }));
}

TEST(X509CompatTest, CommonNameToDNS) {
  struct CommonNameToDNSTestParam {
    std::string common_name;
    bool expect_dns;
    std::string expected_dns_name;
  };

  CommonNameToDNSTestParam params[] = {
      {"example.com", true, "example.com"},
      {"exa_mple.com", true, "exa_mple.com"},
      {"foo-bar.example.com", true, "foo-bar.example.com"},
      {"1.example.com", true, "1.example.com"},
      {"", false, ""},
      {".", false, ""},
      {"-", false, ""},
      {"example.com.", false, ""},
      {".example.com", false, ""},
      {".example.com.", false, ""},
      {"-example.com", false, ""},
      {"example.com-", false, ""},
      {"-example.com-", false, ""},
      {"example.-com", false, ""},
      {"example..com", false, ""},
      {"example-.com", false, ""},
      {"example", false, ""},
      {"eXaMpLe", false, ""},
      {"cn with spaces", false, ""},
      {"1 and 2 and 3", false, ""},
      // Wildcard CN test cases: wildcard CNs must be recognized as DNS-IDs
      // so that name constraints are enforced on them.
      {"*.example.com", true, "*.example.com"},
      {"*.sub.example.com", true, "*.sub.example.com"},
      {"*.exa_mple.com", true, "*.exa_mple.com"},
      {"*.foo-bar.example.com", true, "*.foo-bar.example.com"},
      // Wildcard with single remaining label is not a DNS name.
      {"*.com", false, ""},
      // Bare wildcard or incomplete wildcard prefix.
      {"*", false, ""},
      {"*.", false, ""},
      {"*..", false, ""},
      // Star without dot is not a valid wildcard prefix.
      {"*example.com", false, ""},
      // Double star is not valid.
      {"**.example.com", false, ""},
      // Invalid DNS syntax after wildcard prefix.
      {"*.-example.com", false, ""},
      {"*.example-.com", false, ""},
      {"*.example..com", false, ""},
      {"*.example.com.", false, ""},
      {".*.example.com", false, ""},
      // Single-label CNs with non-ASCII are accepted (not DNS names).
      {"r\xc3\xa4ger", false, ""},
      {"\xc3\x9cnternehmen", false, ""},
      // Single-label CNs with control characters are accepted (not DNS names).
      {"foo\x01" "bar", false, ""},
      {std::string("\x7f") + "tld", false, ""},
      // Single-label CN with space is accepted (not a DNS name).
      {"foo bar", false, ""},
      // Punycode equivalents are valid ASCII DNS names.
      {"xn--rger-koa.evil.com", true, "xn--rger-koa.evil.com"},
      {"xn--caf-dma.example.com", true, "xn--caf-dma.example.com"},
  };

  for (CommonNameToDNSTestParam &param : params) {
    ASN1_STRING *asn1_str_ptr = NULL;
    std::vector<uint8_t> cn(param.common_name.begin(), param.common_name.end());
    ASSERT_GE(ASN1_mbstring_copy(&asn1_str_ptr, cn.data(), cn.size(),
                                 MBSTRING_UTF8, V_ASN1_IA5STRING),
              0);
    bssl::UniquePtr<ASN1_STRING> asn1_cn(asn1_str_ptr);
    ASSERT_TRUE(asn1_cn);
    unsigned char *dnsid_ptr = NULL;
    size_t idlen = 0;
    ASSERT_EQ(X509_V_OK, cn2dnsid(asn1_cn.get(), &dnsid_ptr, &idlen));
    std::unique_ptr<unsigned char, void (*)(unsigned char *)> dnsid(
        dnsid_ptr, [](unsigned char *ptr) { OPENSSL_free(ptr); });
    if (param.expect_dns) {
      ASSERT_TRUE(dnsid.get());
      std::string dns_name(reinterpret_cast<char *>(dnsid.get()), idlen);
      ASSERT_EQ(param.expected_dns_name, dns_name);
    } else {
      ASSERT_FALSE(dnsid.get());
      ASSERT_EQ((size_t)0, idlen);
    }
  }

  // Multi-label CNs with non-ASCII bytes should return
  // X509_V_ERR_UNSUPPORTED_NAME_SYNTAX. Per RFC 6125 Section 6.4.2,
  // internationalized domain names should use A-label (punycode) form.
  const std::string non_ascii_dns[] = {
      "r\xc3\xa4ger.evil.com",
      "caf\xc3\xa9.example.com",
      "*.r\xc3\xa4ger.evil.com",
      // Control characters in multi-label CNs.
      "foo\x01.evil.com",
      std::string("bar\x7f") + ".evil.com",
      "baz\x1f.example.com",
      // Space in multi-label CN.
      "foo .evil.com",
  };
  for (const std::string &name : non_ascii_dns) {
    SCOPED_TRACE(name);
    ASN1_STRING *asn1_str_ptr = NULL;
    std::vector<uint8_t> cn(name.begin(), name.end());
    ASSERT_GE(ASN1_mbstring_copy(&asn1_str_ptr, cn.data(), cn.size(),
                                 MBSTRING_UTF8, V_ASN1_IA5STRING),
              0);
    bssl::UniquePtr<ASN1_STRING> asn1_cn(asn1_str_ptr);
    ASSERT_TRUE(asn1_cn);
    unsigned char *dnsid_ptr = NULL;
    size_t idlen = 0;
    ASSERT_EQ(X509_V_ERR_UNSUPPORTED_NAME_SYNTAX,
              cn2dnsid(asn1_cn.get(), &dnsid_ptr, &idlen));
    ASSERT_FALSE(dnsid_ptr);
    ASSERT_EQ((size_t)0, idlen);
  }

}


TEST(X509CompatTest, WildcardNameBehaviors) {
  bssl::UniquePtr<X509> root = CertFromPEM(kValidRootCA1);
  ASSERT_TRUE(root);
  bssl::UniquePtr<X509> leaf = CertFromPEM(kWildCardCommonNameAndSubjAlt);
  ASSERT_TRUE(leaf);

  EXPECT_EQ(X509_V_ERR_HOSTNAME_MISMATCH,
            Verify(
                leaf.get(), /*roots=*/{root.get()},
                /*intermediates=*/{},
                /*crls=*/{}, /*flags=*/0,
                [](X509_STORE_CTX *ctx) {
                  X509_VERIFY_PARAM *param = X509_STORE_CTX_get0_param(ctx);
                  char host[] = "example.com";
                  X509_VERIFY_PARAM_set1_host(param, host, sizeof(host) - 1);
                }));
  EXPECT_EQ(X509_V_OK,
            Verify(leaf.get(), /*roots=*/{root.get()},
                   /*intermediates=*/{},
                   /*crls=*/{}, /*flags=*/0, [](X509_STORE_CTX *ctx) {
                     X509_VERIFY_PARAM *param = X509_STORE_CTX_get0_param(ctx);
                     char host[] = "host.example.com";
                     X509_VERIFY_PARAM_set1_host(param, host, sizeof(host) - 1);
                   }));
  EXPECT_EQ(X509_V_OK,
            Verify(leaf.get(), /*roots=*/{root.get()},
                   /*intermediates=*/{},
                   /*crls=*/{}, /*flags=*/0, [](X509_STORE_CTX *ctx) {
                     X509_VERIFY_PARAM *param = X509_STORE_CTX_get0_param(ctx);
                     char host[] = "foo.sub1.example.com";
                     X509_VERIFY_PARAM_set1_host(param, host, sizeof(host) - 1);
                   }));
  // *.sub2.example.com is in the commonName so not checked by default if DNS
  // SAN is present
  EXPECT_EQ(X509_V_ERR_HOSTNAME_MISMATCH,
            Verify(leaf.get(), /*roots=*/{root.get()},
                   /*intermediates=*/{},
                   /*crls=*/{}, /*flags=*/0, [](X509_STORE_CTX *ctx) {
                     X509_VERIFY_PARAM *param = X509_STORE_CTX_get0_param(ctx);
                     char host[] = "bar.sub2.example.com";
                     X509_VERIFY_PARAM_set1_host(param, host, sizeof(host) - 1);
                   }));
  EXPECT_EQ(X509_V_OK,
            Verify(leaf.get(), /*roots=*/{root.get()},
                   /*intermediates=*/{},
                   /*crls=*/{}, /*flags=*/0, [](X509_STORE_CTX *ctx) {
                     X509_VERIFY_PARAM *param = X509_STORE_CTX_get0_param(ctx);
                     X509_VERIFY_PARAM_set_hostflags(
                         param, X509_CHECK_FLAG_ALWAYS_CHECK_SUBJECT);
                     char host[] = "bar.sub2.example.com";
                     X509_VERIFY_PARAM_set1_host(param, host, sizeof(host) - 1);
                   }));
  EXPECT_EQ(X509_V_OK,
            Verify(leaf.get(), /*roots=*/{root.get()},
                   /*intermediates=*/{},
                   /*crls=*/{}, /*flags=*/0, [](X509_STORE_CTX *ctx) {
                     X509_VERIFY_PARAM *param = X509_STORE_CTX_get0_param(ctx);
                     char host[] = "baz.example.org";
                     X509_VERIFY_PARAM_set1_host(param, host, sizeof(host) - 1);
                   }));

  // client-side host name verification behavior with leading '.'
  EXPECT_EQ(X509_V_OK,
            Verify(leaf.get(), /*roots=*/{root.get()},
                   /*intermediates=*/{},
                   /*crls=*/{}, /*flags=*/0, [](X509_STORE_CTX *ctx) {
                     X509_VERIFY_PARAM *param = X509_STORE_CTX_get0_param(ctx);
                     char host[] = ".example.com";
                     X509_VERIFY_PARAM_set1_host(param, host, sizeof(host) - 1);
                   }));
  EXPECT_EQ(X509_V_OK,
            Verify(leaf.get(), /*roots=*/{root.get()},
                   /*intermediates=*/{},
                   /*crls=*/{}, /*flags=*/0, [](X509_STORE_CTX *ctx) {
                     X509_VERIFY_PARAM *param = X509_STORE_CTX_get0_param(ctx);
                     char host[] = ".sub1.example.com";
                     X509_VERIFY_PARAM_set1_host(param, host, sizeof(host) - 1);
                   }));
  EXPECT_EQ(X509_V_ERR_HOSTNAME_MISMATCH,
            Verify(leaf.get(), /*roots=*/{root.get()},
                   /*intermediates=*/{},
                   /*crls=*/{}, /*flags=*/0, [](X509_STORE_CTX *ctx) {
                     X509_VERIFY_PARAM *param = X509_STORE_CTX_get0_param(ctx);
                     char host[] = ".sub2.example.com";
                     X509_VERIFY_PARAM_set1_host(param, host, sizeof(host) - 1);
                   }));
  EXPECT_EQ(X509_V_OK,
            Verify(leaf.get(), /*roots=*/{root.get()},
                   /*intermediates=*/{},
                   /*crls=*/{}, /*flags=*/0, [](X509_STORE_CTX *ctx) {
                     X509_VERIFY_PARAM *param = X509_STORE_CTX_get0_param(ctx);
                     X509_VERIFY_PARAM_set_hostflags(
                         param, X509_CHECK_FLAG_ALWAYS_CHECK_SUBJECT);
                     char host[] = ".sub2.example.com";
                     X509_VERIFY_PARAM_set1_host(param, host, sizeof(host) - 1);
                   }));
  EXPECT_EQ(X509_V_OK,
            Verify(leaf.get(), /*roots=*/{root.get()},
                   /*intermediates=*/{},
                   /*crls=*/{}, /*flags=*/0, [](X509_STORE_CTX *ctx) {
                     X509_VERIFY_PARAM *param = X509_STORE_CTX_get0_param(ctx);
                     X509_VERIFY_PARAM_set_hostflags(
                         param, X509_CHECK_FLAG_ALWAYS_CHECK_SUBJECT);
                     char host[] = ".example.org";
                     X509_VERIFY_PARAM_set1_host(param, host, sizeof(host) - 1);
                   }));
  EXPECT_EQ(X509_V_ERR_HOSTNAME_MISMATCH,
            Verify(leaf.get(), /*roots=*/{root.get()},
                   /*intermediates=*/{},
                   /*crls=*/{}, /*flags=*/0, [](X509_STORE_CTX *ctx) {
                     X509_VERIFY_PARAM *param = X509_STORE_CTX_get0_param(ctx);
                     X509_VERIFY_PARAM_set_hostflags(
                         param, X509_CHECK_FLAG_ALWAYS_CHECK_SUBJECT);
                     char host[] = ".baz.example.org";
                     X509_VERIFY_PARAM_set1_host(param, host, sizeof(host) - 1);
                   }));
}

TEST(X509CompatTest, LeafCertificateWithExplicitECParams) {
    bssl::UniquePtr<X509> root = CertFromPEM(kValidRootCA1);
    ASSERT_TRUE(root);
    bssl::UniquePtr<X509> leaf = CertFromPEM(kLeafCertificateWithExplicitECParams);
    ASSERT_TRUE(leaf);

    const char host[] = "example.com";

    EXPECT_EQ(
        X509_V_ERR_EC_KEY_EXPLICIT_PARAMS,
        Verify(leaf.get(), /*roots=*/{root.get()},
               /*intermediates=*/{},
               /*crls=*/{}, /*flags=*/0));

    EXPECT_EQ(
        X509_V_OK,
        Verify(leaf.get(), /*roots=*/{root.get()},
               /*intermediates=*/{},
               /*crls=*/{}, /*flags=*/0, [&host](X509_STORE_CTX *ctx) {
                 X509_VERIFY_PARAM *param = X509_STORE_CTX_get0_param(ctx);
                 X509_VERIFY_PARAM_set1_host(param, host, sizeof(host) - 1);
                 X509_VERIFY_PARAM_enable_ec_key_explicit_params(param);
               }));
}
