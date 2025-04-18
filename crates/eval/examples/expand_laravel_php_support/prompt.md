
I'd like to update our Laravel package's CI workflow and dependencies to ensure compatibility with the upcoming Laravel 12 release and PHP 8.4. Currently, our package supports Laravel versions 5.8 through 11 and PHP versions 7.1 through 8.3, and we'll need to extend this support while maintaining backward compatibility.

**Key Changes Needed:**
First, we'll need to update composer.json to explicitly support Laravel 12. The CI test matrix should also be expanded to include PHP 8.4 testing across all supported Laravel versions. The workflow configuration will require adjustments to properly handle these new version combinations.

There are some test compatibility issues we'll need to address - particularly around how we check string column types in Laravel 11+ (where 'string' was changed to 'varchar'), and we should add conditional skipping for tests that depend on traits that might not be available in all test environments.

While making these changes, we could also implement some workflow improvements: enabling the fail-fast: false option to get complete test results even with individual failures, modernizing our dependency installation approach using the newer composer update syntax, and making the DBAL dependency installation conditional since it's not needed for all test cases.

Would you be able to help review these changes or suggest any additional considerations we should keep in mind for this compatibility update? I want to make sure we maintain stability while expanding our support coverage.
