# Fail immediately in case of errors and/or unset variables
set -eu -o pipefail

# Echo commands so that the progress can be seen in CI server logs.
set -x

# Check the shell scripts
shellcheck .github/workflows/run_invocation_tests.sh
shellcheck test_profile
