# AWS SigV4 and SigV4A Signing Test Suite

This test suite is taken from the [CRT test suite](https://github.com/awslabs/aws-c-auth/tree/v0.9.0/tests/aws-signing-test-suite).

We added the following changes:

* Migrated old format tests `double-url-encode` and `double-encode-path` not in the new suite as we use these in many tests.