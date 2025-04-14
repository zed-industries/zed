
Add validation support for Employer Identification Numbers (EIN) to the Go validator library

I need to implement a new validator function for US Employer Identification Numbers (EIN) in this Go validation library. The EIN validator should:

1. Create a new tag called "ein" that validates if a string is a valid US Employer Identification Number
2. Follow the pattern of ##-#######, where # is a digit (regex pattern would be ^(\d{2}-\d{7})$)
3. Ensure the field contains exactly 10 characters (including the hyphen)
4. Document the new validator in the README.md and doc.go files
5. Add proper unit tests to verify validation works correctly for valid and invalid EINs
