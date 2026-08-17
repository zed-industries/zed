# Performance

## "Reflected" vs "Forward"

There is a significant performance penalty for the "forward" implementations, due to the extra shuffle-masking. I've
noted the different variants in the table below.

## Aarch64

Also note that, on Aarch64 NEON platforms, there is a measurable difference between some of the target implementations
for the major CRC-32 variants depending on typical payload size. You may wish to tune for your specific environment and
use cases using the target feature when building (we will certainly do this on our production systems).

## Blending

For some of the CRC-32 implementations, a blended approach where "large" (>1KiB) payloads are processed by a different
implementation than "small" payloads, is supplied.

Particularly for x86_64, the blended approach yields the best result on CRC-32/ISO-HDLC, where hardware support lags
behind.

On aarch64, the blended approach is the ideal sweet spot for AWS Graviton4 instances, which is probably the primary
production deployment.

In cases where it makes sense, the blended approaches are the default. You can fine-tune this for your deployment
strategy using feature flags.

### CRC-32/AUTOSAR (reflected)

| Arch    | Brand | CPU             | System                    | Target          | Throughput (1 KiB) | Throughput (1 MiB) |
|:--------|:------|:----------------|:--------------------------|:----------------|-------------------:|-------------------:|
| x86_64  | Intel | Sapphire Rapids | EC2 c7i.metal-48xl        | avx2_vpclmulqdq |      ~16.204 GiB/s |      ~55.921 GiB/s |
| x86_64  | Intel | Sapphire Rapids | EC2 c7i.metal-48xl        | sse_pclmulqdq   |      ~14.243 GiB/s |      ~28.148 GiB/s |
| x86_64  | AMD   | Genoa           | EC2 c7a.metal-48xl        | avx2_vpclmulqdq |      ~17.135 GiB/s |      ~27.378 GiB/s |
| x86_64  | AMD   | Genoa           | EC2 c7a.metal-48xl        | sse_pclmulqdq   |      ~10.237 GiB/s |      ~13.708 GiB/s |
| aarch64 | AWS   | Graviton4       | EC2 c8g.metal-48xl        | neon_pclmulqdq  |      ~15.061 GiB/s |      ~24.871 GiB/s |
| aarch64 | Apple | M4 Max          | MacBook Pro 16" (16 core) | neon_pclmulqdq  |      ~39.201 GiB/s |      ~72.359 GiB/s | 
| aarch64 | Apple | M2 Ultra        | Mac Studio (24 core)      | neon_pclmulqdq  |      ~36.917 GiB/s |      ~64.916 GiB/s |
| aarch64 | Apple | M3 Ultra        | Mac Studio (32 core)      | neon_pclmulqdq  |      ~38.245 GiB/s |      ~71.382 GiB/s |

### CRC-32/BZIP2 (forward)

| Arch    | Brand | CPU             | System                    | Target          | Throughput (1 KiB) | Throughput (1 MiB) |
|:--------|:------|:----------------|:--------------------------|:----------------|-------------------:|-------------------:|
| x86_64  | Intel | Sapphire Rapids | EC2 c7i.metal-48xl        | avx2_vpclmulqdq |      ~15.613 GiB/s |      ~27.997 GiB/s |
| x86_64  | Intel | Sapphire Rapids | EC2 c7i.metal-48xl        | sse_pclmulqdq   |      ~13.369 GiB/s |      ~28.142 GiB/s |
| x86_64  | AMD   | Genoa           | EC2 c7a.metal-48xl        | avx2_vpclmulqdq |      ~14.100 GiB/s |      ~25.755 GiB/s |
| x86_64  | AMD   | Genoa           | EC2 c7a.metal-48xl        | sse_pclmulqdq   |      ~9.8876 GiB/s |      ~13.293 GiB/s |
| aarch64 | AWS   | Graviton4       | EC2 c8g.metal-48xl        | neon_pclmulqdq  |      ~14.040 GiB/s |      ~21.014 GiB/s |
| aarch64 | Apple | M4 Max          | MacBook Pro 16" (16 core) | neon_pclmulqdq  |      ~37.052 GiB/s |      ~58.513 GiB/s | 
| aarch64 | Apple | M2 Ultra        | Mac Studio (24 core)      | neon_pclmulqdq  |       ~34.099 GiB/ |      ~53.448 GiB/s |
| aarch64 | Apple | M3 Ultra        | Mac Studio (32 core)      | neon_pclmulqdq  |      ~38.424 GiB/s |      ~59.369 GiB/s |

### CRC-32/ISCSI (reflected) [aka "crc32c" in many, but not all, implementations]

| Arch    | Brand | CPU             | System                    | Target                     | Throughput (1 KiB) | Throughput (1 MiB) |
|:--------|:------|:----------------|:--------------------------|:---------------------------|-------------------:|-------------------:|
| x86_64  | Intel | Sapphire Rapids | EC2 c7i.metal-48xl        | **avx512_vpclmulqdq_v3x2** |      ~38.013 GiB/s |      ~111.72 GiB/s |
| x86_64  | Intel | Sapphire Rapids | EC2 c7i.metal-48xl        | avx512_v4s3x3              |      ~18.684 GiB/s |      ~43.461 GiB/s |
| x86_64  | Intel | Sapphire Rapids | EC2 c7i.metal-48xl        | sse_v4s3x3                 |      ~16.780 GiB/s |      ~43.471 GiB/s |
| x86_64  | Intel | Sapphire Rapids | EC2 c7i.metal-48xl        | sse_pclmulqdq              |      ~14.050 GiB/s |      ~28.045 GiB/s |
| x86_64  | Intel | Sapphire Rapids | EC2 c7i.metal-48xl        | avx2_vpclmulqdq            |      ~16.597 GiB/s |      ~56.318 GiB/s |
| x86_64  | AMD   | Genoa           | EC2 c7a.metal-48xl        | **avx512_vpclmulqdq_v3x2** |      ~21.125 GiB/s |      ~54.638 GiB/s |
| x86_64  | AMD   | Genoa           | EC2 c7a.metal-48xl        | avx512_v4s3x3              |      ~12.549 GiB/s |      ~29.019 GiB/s |
| x86_64  | AMD   | Genoa           | EC2 c7a.metal-48xl        | sse_v4s3x3                 |      ~11.786 GiB/s |      ~24.365 GiB/s |
| x86_64  | AMD   | Genoa           | EC2 c7a.metal-48xl        | sse_pclmulqdq              |      ~10.690 GiB/s |      ~13.656 GiB/s |
| x86_64  | AMD   | Genoa           | EC2 c7a.metal-48xl        | avx2_vpclmulqdq            |      ~16.625 GiB/s |      ~27.308 GiB/s |
| aarch64 | AWS   | Graviton4       | EC2 c8g.metal-48xl        | neon_eor3_v9s3x2e_s3       |      ~12.182 GiB/s |      ~31.463 GiB/s |
| aarch64 | AWS   | Graviton4       | EC2 c8g.metal-48xl        | neon_v12e_v1               |      ~19.076 GiB/s |      ~21.931 GiB/s |
| aarch64 | AWS   | Graviton4       | EC2 c8g.metal-48xl        | neon_v3s4x2e_v2            |      ~13.123 GiB/s |      ~28.757 GiB/s |
| aarch64 | AWS   | Graviton4       | EC2 c8g.metal-48xl        | **neon_blended**           |      ~18.530 GiB/s |      ~31.598 GiB/s |
| aarch64 | AWS   | Graviton4       | EC2 c8g.metal-48xl        | neon_pclmulqdq             |      ~14.685 GiB/s |      ~24.676 GiB/s |
| aarch64 | Apple | M3 Max          | MacBook Pro 16"           | neon_eor3_v9s3x2e_s3       |      ~19.848 GiB/s |      ~95.026 GiB/s | 
| aarch64 | Apple | M3 Max          | MacBook Pro 16"           | neon_v12e_v1               |      ~50.518 GiB/s |      ~94.455 GiB/s | 
| aarch64 | Apple | M3 Max          | MacBook Pro 16"           | neon_v3s4x2e_v2            |      ~14.499 GiB/s |      ~48.604 GiB/s | 
| aarch64 | Apple | M3 Max          | MacBook Pro 16"           | neon_pclmulqdq             |      ~25.107 GiB/s |      ~67.141 GiB/s | 
| aarch64 | Apple | M4 Max          | MacBook Pro 16" (16 core) | neon_eor3_v9s3x2e_s3       |      ~29.130 GiB/s |      ~96.865 GiB/s | 
| aarch64 | Apple | M4 Max          | MacBook Pro 16" (16 core) | neon_v12e_v1               |      ~59.834 GiB/s |      ~105.31 GiB/s | 
| aarch64 | Apple | M4 Max          | MacBook Pro 16" (16 core) | neon_v3s4x2e_v2            |      ~22.472 GiB/s |      ~54.195 GiB/s | 
| aarch64 | Apple | M4 Max          | MacBook Pro 16" (16 core) | **neon_blended**           |      ~60.791 GiB/s |      ~96.310 GiB/s | 
| aarch64 | Apple | M4 Max          | MacBook Pro 16" (16 core) | neon_pclmulqdq             |      ~39.322 GiB/s |      ~72.366 GiB/s |
| aarch64 | Apple | M2 Ultra        | Mac Studio (24 core)      | neon_eor3_v9s3x2e_s3       |      ~20.183 GiB/s |      ~87.630 GiB/s |
| aarch64 | Apple | M2 Ultra        | Mac Studio (24 core)      | neon_v12e_v1               |      ~50.776 GiB/s |      ~82.354 GiB/s |
| aarch64 | Apple | M2 Ultra        | Mac Studio (24 core)      | neon_v3s4x2e_v2            |      ~19.021 GiB/s |      ~44.149 GiB/s |
| aarch64 | Apple | M2 Ultra        | Mac Studio (24 core)      | **neon_blended**           |      ~50.260 GiB/s |      ~87.601 GiB/s |
| aarch64 | Apple | M2 Ultra        | Mac Studio (24 core)      | neon_pclmulqdq             |      ~34.384 GiB/s |      ~65.159 GiB/s |
| aarch64 | Apple | M3 Ultra        | Mac Studio (32 core)      | neon_eor3_v9s3x2e_s3       |      ~23.398 GiB/s |      ~97.221 GiB/s |
| aarch64 | Apple | M3 Ultra        | Mac Studio (32 core)      | **neon_v12e_v1**           |      ~54.760 GiB/s |      ~99.616 GiB/s |
| aarch64 | Apple | M3 Ultra        | Mac Studio (32 core)      | neon_v3s4x2e_v2            |      ~18.425 GiB/s |      ~52.132 GiB/s |
| aarch64 | Apple | M3 Ultra        | Mac Studio (32 core)      | neon_blended               |      ~55.806 GiB/s |      ~95.427 GiB/s |
| aarch64 | Apple | M3 Ultra        | Mac Studio (32 core)      | neon_pclmulqdq             |      ~40.545 GiB/s |      ~72.045 GiB/s |

### CRC-32/ISO-HDLC (reflected) [aka "crc32" in many, but not all, implementations]

| Arch    | Brand | CPU             | System                    | Target                 | Throughput (1 KiB) | Throughput (1 MiB) |
|:--------|:------|:----------------|:--------------------------|:-----------------------|-------------------:|-------------------:|
| x86_64  | Intel | Sapphire Rapids | EC2 c7i.metal-48xl        | avx512_vpclmulqdq_v3x2 |      ~8.1734 GiB/s |      ~111.60 GiB/s |
| x86_64  | Intel | Sapphire Rapids | EC2 c7i.metal-48xl        | avx512_v4s3x3          |      ~7.2054 GiB/s |      ~11.953 GiB/s |
| x86_64  | Intel | Sapphire Rapids | EC2 c7i.metal-48xl        | sse_v4s3x3             |      ~7.1272 GiB/s |      ~11.883 GiB/s |
| x86_64  | Intel | Sapphire Rapids | EC2 c7i.metal-48xl        | sse_pclmulqdq          |      ~14.050 GiB/s |      ~28.045 GiB/s |
| x86_64  | Intel | Sapphire Rapids | EC2 c7i.metal-48xl        | avx2_vpclmulqdq        |      ~16.615 GiB/s |      ~56.090 GiB/s |
| x86_64  | Intel | Sapphire Rapids | EC2 c7i.metal-48xl        | **avx2_blended**       |      ~16.563 GiB/s |      ~110.41 GiB/s |
| x86_64  | AMD   | Genoa           | EC2 c7a.metal-48xl        | avx512_vpclmulqdq_v3x2 |      ~6.8854 GiB/s |      ~53.906 GiB/s |
| x86_64  | AMD   | Genoa           | EC2 c7a.metal-48xl        | avx512_v4s3x3          |      ~4.4371 GiB/s |      ~8.9257 GiB/s |
| x86_64  | AMD   | Genoa           | EC2 c7a.metal-48xl        | sse_v4s3x3             |      ~4.1876 GiB/s |      ~8.2890 GiB/s |
| x86_64  | AMD   | Genoa           | EC2 c7a.metal-48xl        | sse_pclmulqdq          |      ~10.690 GiB/s |      ~13.656 GiB/s |
| x86_64  | AMD   | Genoa           | EC2 c7a.metal-48xl        | avx2_vpclmulqdq        |      ~17.806 GiB/s |      ~27.312 GiB/s |
| x86_64  | AMD   | Genoa           | EC2 c7a.metal-48xl        | **avx2_blended**       |      ~17.231 GiB/s |      ~53.796 GiB/s |
| aarch64 | AWS   | Graviton4       | EC2 c8g.metal-48xl        | neon_eor3_v9s3x2e_s3   |      ~12.187 GiB/s |      ~31.140 GiB/s |
| aarch64 | AWS   | Graviton4       | EC2 c8g.metal-48xl        | neon_v12e_v1           |      ~18.965 GiB/s |      ~22.512 GiB/s |
| aarch64 | AWS   | Graviton4       | EC2 c8g.metal-48xl        | neon_v3s4x2e_v2        |      ~13.093 GiB/s |      ~28.971 GiB/s |
| aarch64 | AWS   | Graviton4       | EC2 c8g.metal-48xl        | **neon_blended**       |      ~18.470 GiB/s |      ~31.536 GiB/s |
| aarch64 | AWS   | Graviton4       | EC2 c8g.metal-48xl        | neon_pclmulqdq         |      ~14.654 GiB/s |      ~24.264 GiB/s |
| aarch64 | Apple | M3 Max          | MacBook Pro 16"           | neon_eor3_v9s3x2e_s3   |      ~21.994 GiB/s |      ~95.329 GiB/s | 
| aarch64 | Apple | M3 Max          | MacBook Pro 16"           | neon_v12e_v1           |      ~40.351 GiB/s |      ~95.176 GiB/s | 
| aarch64 | Apple | M3 Max          | MacBook Pro 16"           | neon_v3s4x2e_v2        |      ~17.035 GiB/s |      ~48.650 GiB/s | 
| aarch64 | Apple | M3 Max          | MacBook Pro 16"           | neon_pclmulqdq         |      ~39.514 GiB/s |      ~67.118 GiB/s | 
| aarch64 | Apple | M4 Max          | MacBook Pro 16" (16 core) | neon_eor3_v9s3x2e_s3   |      ~29.186 GiB/s |      ~96.635 GiB/s | 
| aarch64 | Apple | M4 Max          | MacBook Pro 16" (16 core) | **neon_v12e_v1**       |      ~59.174 GiB/s |      ~105.28 GiB/s | 
| aarch64 | Apple | M4 Max          | MacBook Pro 16" (16 core) | neon_v3s4x2e_v2        |      ~22.576 GiB/s |      ~54.050 GiB/s | 
| aarch64 | Apple | M4 Max          | MacBook Pro 16" (16 core) | neon_blended           |      ~59.331 GiB/s |      ~96.238 GiB/s | 
| aarch64 | Apple | M4 Max          | MacBook Pro 16" (16 core) | neon_pclmulqdq         |      ~39.546 GiB/s |      ~72.143 GiB/s |
| aarch64 | Apple | M2 Ultra        | Mac Studio (24 core)      | neon_eor3_v9s3x2e_s3   |      ~20.433 GiB/s |      ~87.812 GiB/s |
| aarch64 | Apple | M2 Ultra        | Mac Studio (24 core)      | neon_v12e_v1           |      ~50.557 GiB/s |      ~82.379 GiB/s |
| aarch64 | Apple | M2 Ultra        | Mac Studio (24 core)      | neon_v3s4x2e_v2        |      ~18.785 GiB/s |       ~44.212 GiB/ |
| aarch64 | Apple | M2 Ultra        | Mac Studio (24 core)      | **neon_blended**       |      ~50.106 GiB/s |      ~86.992 GiB/s |
| aarch64 | Apple | M2 Ultra        | Mac Studio (24 core)      | neon_pclmulqdq         |      ~33.112 GiB/s |      ~64.870 GiB/s |
| aarch64 | Apple | M3 Ultra        | Mac Studio (32 core)      | neon_eor3_v9s3x2e_s3   |      ~23.382 GiB/s |      ~97.845 GiB/s |
| aarch64 | Apple | M3 Ultra        | Mac Studio (32 core)      | **neon_v12e_v1**       |      ~56.451 GiB/s |      ~98.768 GiB/s |
| aarch64 | Apple | M3 Ultra        | Mac Studio (32 core)      | neon_v3s4x2e_v2        |      ~18.083 GiB/s |      ~52.059 GiB/s |
| aarch64 | Apple | M3 Ultra        | Mac Studio (32 core)      | neon_blended           |      ~53.937 GiB/s |      ~97.331 GiB/s |
| aarch64 | Apple | M3 Ultra        | Mac Studio (32 core)      | neon_pclmulqdq         |      ~40.117 GiB/s |      ~71.966 GiB/s |

### CRC-64/ECMA-182 (forward)

| Arch    | Brand | CPU             | System                    | Target          | Throughput (1 KiB) | Throughput (1 MiB) |
|:--------|:------|:----------------|:--------------------------|:----------------|-------------------:|-------------------:|
| x86_64  | Intel | Sapphire Rapids | EC2 c7i.metal-48xl        | avx2_vpclmulqdq |      ~16.733 GiB/s |      ~27.976 GiB/s |
| x86_64  | Intel | Sapphire Rapids | EC2 c7i.metal-48xl        | sse_pclmulqdq   |      ~13.896 GiB/s |      ~28.171 GiB/s |
| x86_64  | AMD   | Genoa           | EC2 c7a.metal-48xl        | avx2_vpclmulqdq |      ~14.807 GiB/s |      ~25.764 GiB/s |
| x86_64  | AMD   | Genoa           | EC2 c7a.metal-48xl        | sse_pclmulqdq   |      ~10.145 GiB/s |      ~13.277 GiB/s |
| aarch64 | AWS   | Graviton4       | EC2 c8g.metal-48xl        | neon_pclmulqdq  |      ~14.367 GiB/s |      ~21.078 GiB/s |
| aarch64 | Apple | M4 Max          | MacBook Pro 16" (16 core) | neon_pclmulqdq  |      ~37.538 GiB/s |      ~59.511 GiB/s | 
| aarch64 | Apple | M2 Ultra        | Mac Studio (24 core)      | neon_pclmulqdq  |      ~34.098 GiB/s |      ~53.587 GiB/s |
| aarch64 | Apple | M3 Ultra        | Mac Studio (32 core)      | neon_pclmulqdq  |      ~37.525 GiB/s |      ~59.392 GiB/s |

### CRC-64/NVME (reflected)

| Arch    | Brand | CPU             | System                    | Target          | Throughput (1 KiB) | Throughput (1 MiB) |
|:--------|:------|:----------------|:--------------------------|:----------------|-------------------:|-------------------:|
| x86_64  | Intel | Sapphire Rapids | EC2 c7i.metal-48xl        | avx2_vpclmulqdq |      ~16.967 GiB/s |      ~56.369 GiB/s |
| x86_64  | Intel | Sapphire Rapids | EC2 c7i.metal-48xl        | sse_pclmulqdq   |      ~14.082 GiB/s |      ~28.104 GiB/s |
| x86_64  | AMD   | Genoa           | EC2 c7a.metal-48xl        | avx2_vpclmulqdq |      ~17.347 GiB/s |      ~27.377 GiB/s |
| x86_64  | AMD   | Genoa           | EC2 c7a.metal-48xl        | sse_pclmulqdq   |      ~10.661 GiB/s |      ~13.664 GiB/s |
| aarch64 | AWS   | Graviton4       | EC2 c8g.metal-48xl        | neon_pclmulqdq  |      ~16.272 GiB/s |      ~16.272 GiB/s |
| aarch64 | Apple | M4 Max          | MacBook Pro 16" (16 core) | neon_pclmulqdq  |      ~40.335 GiB/s |      ~72.282 GiB/s | 
| aarch64 | Apple | M2 Ultra        | Mac Studio (24 core)      | neon_pclmulqdq  |      ~39.315 GiB/s |      ~64.987 GiB/s |
| aarch64 | Apple | M3 Ultra        | Mac Studio (32 core)      | neon_pclmulqdq  |      ~43.987 GiB/s |      ~71.891 GiB/s |
