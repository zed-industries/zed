# RSA Tests

## How to generate keys and certificates

### RSA PKCS1

```
openssl genrsa -out private_rsa_pkcs1.pem
openssl rsa -in private_rsa_pkcs1.pem -RSAPublicKey_out -out public_rsa_pkcs1.pem
openssl req -new -key private_rsa_pkcs1.pem -out certificate_rsa_pkcs1.csr
openssl x509 -req -sha256 -days 358000 -in certificate_rsa_pkcs1.csr -signkey private_rsa_pkcs1.pem -out certificate_rsa_pkcs1.crt
```

### RSA PKCS8

```
openssl genpkey -algorithm RSA -out private_rsa_pkcs8.pem -pkeyopt rsa_keygen_bits:2048
openssl rsa -pubout -in private_rsa_pkcs8.pem -out public_rsa_pkcs.pem
openssl req -new -key private_rsa_pkcs8.key -out certificate_rsa_pkcs8.csr
openssl x509 -req -sha256 -days 358000 -in certificate_rsa_pkcs8.csr -signkey private_rsa_pkcs8.key -out certificate_rsa_pkcs8.crt
```

### Convert to DER format

```
openssl rsa -inform PEM -in private.pem -outform DER -out private.der
```
