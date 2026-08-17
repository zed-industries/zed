# BetterTLS Test Suite

Generated using the Netflix [bettertls] project.

[bettertls]: https://github.com/Netflix/bettertls

## Pathbuilding 

To regenerate pathbuilding test data:

1. Install Go
2. Generate the JSON testdata export for the path building suite:

```bash
GOBIN=$PWD go install github.com/Netflix/bettertls/test-suites/cmd/bettertls@latest
./bettertls export-tests --suite pathbuilding --out ./pathbuilding.tests.json
```
