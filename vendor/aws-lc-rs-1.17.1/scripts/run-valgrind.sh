#!/usr/bin/env bash
# Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0 OR ISC

# # Helper script for running aws-lc-rs tests under Valgrind
#
# Usage:
#   ./scripts/run-valgrind.sh [OPTIONS] [TEST_NAME]
#
# Examples:
#   ./scripts/run-valgrind.sh                    # Run all tests
#   ./scripts/run-valgrind.sh pqdsa_test         # Run specific test
#   ./scripts/run-valgrind.sh --no-suppress      # Run without suppressions
#   ./scripts/run-valgrind.sh --release          # Run release build
#   ./scripts/run-valgrind.sh --strict-leaks     # Only check for real leaks (definite/indirect)

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default configuration
USE_SUPPRESSIONS=1
BUILD_MODE="debug"
LEAK_CHECK="full"
SHOW_LEAK_KINDS="all"
ERROR_EXITCODE=1
TEST_THREADS=1
FEATURES="unstable"
PACKAGE="aws-lc-rs"
VALGRIND_EXTRA_ARGS=""
GEN_SUPPRESSIONS=0
STRICT_LEAKS=0
export AWS_LC_RS_DISABLE_SLOW_TESTS=1

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --no-suppress)
            USE_SUPPRESSIONS=0
            shift
            ;;
        --gen-suppressions)
            GEN_SUPPRESSIONS=1
            shift
            ;;
        --strict-leaks)
            STRICT_LEAKS=1
            shift
            ;;
        --release)
            BUILD_MODE="release"
            shift
            ;;
        --debug)
            BUILD_MODE="debug"
            shift
            ;;
        --threads)
            TEST_THREADS="$2"
            shift 2
            ;;
        --features)
            FEATURES="$2"
            shift 2
            ;;
        --package|-p)
            PACKAGE="$2"
            shift 2
            ;;
        --help|-h)
            echo "Usage: $0 [OPTIONS] [TEST_NAME]"
            echo ""
            echo "Options:"
            echo "  --no-suppress      Disable Valgrind suppressions (show all warnings)"
            echo "  --gen-suppressions Generate suppression rules for errors found"
            echo "  --strict-leaks     Only report real leaks (definite/indirect), ignores"
            echo "                     possibly lost and still reachable. Use this to verify"
            echo "                     suppressions aren't masking actual memory leaks."
            echo "  --release          Use release build (faster but less debug info)"
            echo "  --debug            Use debug build (default)"
            echo "  --threads N        Number of test threads (default: 1)"
            echo "  --features FEATS   Cargo features to enable (default: unstable)"
            echo "  --package PKG      Package to test (default: aws-lc-rs)"
            echo "  --help, -h         Show this help message"
            echo ""
            echo "Examples:"
            echo "  $0                         # Run all tests"
            echo "  $0 pqdsa_test              # Run specific test"
            echo "  $0 --no-suppress           # Run without suppressions"
            echo "  $0 --gen-suppressions      # Generate suppression rules"
            echo "  $0 --strict-leaks          # Verify no real leaks (ignores false positives)"
            echo "  $0 --release pqdsa_test    # Run specific test in release mode"
            exit 0
            ;;
        --*)
            echo -e "${RED}Error: Unknown option $1${NC}"
            exit 1
            ;;
        *)
            # Assume it's a test name
            TEST_NAME="$1"
            shift
            ;;
    esac
done

# Get the repository root directory
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$REPO_ROOT/aws-lc-rs"

# Check if Valgrind is installed
if ! command -v valgrind &> /dev/null; then
    echo -e "${RED}Error: Valgrind is not installed${NC}"
    echo "Install it with:"
    echo "  Ubuntu/Debian: sudo apt-get install valgrind"
    echo "  macOS: brew install valgrind"
    exit 1
fi

# Handle strict-leaks mode - only show definite and indirect leaks (real leaks)
if [ $STRICT_LEAKS -eq 1 ]; then
    SHOW_LEAK_KINDS="definite,indirect"
    USE_SUPPRESSIONS=0  # No need for suppressions in strict mode
    echo -e "${YELLOW}=== STRICT LEAKS MODE ===${NC}"
    echo -e "${YELLOW}Only checking for real memory leaks (definite/indirect).${NC}"
    echo -e "${YELLOW}Possibly lost and still reachable are IGNORED.${NC}"
    echo -e "${YELLOW}If this passes, your suppressions are NOT masking real leaks.${NC}"
    echo ""
fi

# Build Valgrind command
VALGRIND_CMD="valgrind --error-exitcode=${ERROR_EXITCODE} --leak-check=${LEAK_CHECK} --show-leak-kinds=${SHOW_LEAK_KINDS}"

# Add gen-suppressions if enabled
if [ $GEN_SUPPRESSIONS -eq 1 ]; then
    VALGRIND_CMD="${VALGRIND_CMD} --gen-suppressions=all"
    echo -e "${BLUE}Generating suppression rules for all errors${NC}"
    # Disable error exit code when generating suppressions to see all issues
    ERROR_EXITCODE=0
fi

# Add suppression file if enabled
if [ $USE_SUPPRESSIONS -eq 1 ]; then
    SUPPRESSION_FILE="${REPO_ROOT}/.valgrind/rust-test.supp"
    if [ -f "$SUPPRESSION_FILE" ]; then
        VALGRIND_CMD="${VALGRIND_CMD} --suppressions=${SUPPRESSION_FILE}"
        echo -e "${BLUE}Using suppressions from: ${SUPPRESSION_FILE}${NC}"
    else
        echo -e "${YELLOW}Warning: Suppression file not found: ${SUPPRESSION_FILE}${NC}"
    fi
else
    echo -e "${YELLOW}Running WITHOUT suppressions - expect false positives${NC}"
fi

# Add any extra Valgrind arguments
if [ -n "$VALGRIND_EXTRA_ARGS" ]; then
    VALGRIND_CMD="${VALGRIND_CMD} ${VALGRIND_EXTRA_ARGS}"
fi

# Build cargo command
CARGO_CMD="cargo test -p ${PACKAGE} --features ${FEATURES}"

if [ "$BUILD_MODE" = "release" ]; then
    CARGO_CMD="${CARGO_CMD} --release"
    echo -e "${BLUE}Using release build${NC}"
else
    echo -e "${BLUE}Using debug build${NC}"
fi

# Add test name if provided
if [ -n "$TEST_NAME" ]; then
    CARGO_CMD="${CARGO_CMD} --test ${TEST_NAME}"
    echo -e "${BLUE}Running test: ${TEST_NAME}${NC}"
else
    echo -e "${BLUE}Running all tests${NC}"
fi

# Add test arguments
CARGO_CMD="${CARGO_CMD} -- --test-threads=${TEST_THREADS}"

# Print configuration
echo -e "${GREEN}=== Valgrind Test Configuration ===${NC}"
echo "Package: ${PACKAGE}"
echo "Features: ${FEATURES}"
echo "Build: ${BUILD_MODE}"
echo "Test threads: ${TEST_THREADS}"
echo "Suppressions: $([ $USE_SUPPRESSIONS -eq 1 ] && echo 'enabled' || echo 'disabled')"
echo "Generate suppressions: $([ $GEN_SUPPRESSIONS -eq 1 ] && echo 'enabled' || echo 'disabled')"
echo "Strict leaks mode: $([ $STRICT_LEAKS -eq 1 ] && echo 'enabled (only definite/indirect)' || echo 'disabled')"
echo ""

# Export environment variables
export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUNNER="${VALGRIND_CMD}"
export AWS_LC_RS_DISABLE_SLOW_TESTS=1

echo -e "${GREEN}=== Starting Valgrind Test Run ===${NC}"
echo "Command: ${CARGO_CMD}"
echo ""

# Run the tests
if eval ${CARGO_CMD}; then
    echo ""
    echo -e "${GREEN}=== Valgrind tests PASSED ===${NC}"
    exit 0
else
    EXIT_CODE=$?
    echo ""
    echo -e "${RED}=== Valgrind tests FAILED ===${NC}"
    echo ""
    echo "Possible causes:"
    echo "  1. Memory leak detected (check output above)"
    echo "  2. Uninitialized memory usage"
    echo "  3. Invalid memory access"
    echo ""
    echo "Next steps:"
    echo "  - Review the Valgrind output above"
    echo "  - Check .valgrind/KNOWN_ISSUES.md for known issues"
    echo "  - Run with --no-suppress to see all warnings"
    echo "  - Run with --gen-suppressions to generate suppression rules"
    echo "  - For false positives in stdlib, add to .valgrind/rust-test.supp"
    exit $EXIT_CODE
fi
