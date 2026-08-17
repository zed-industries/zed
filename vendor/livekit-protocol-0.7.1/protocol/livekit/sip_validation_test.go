// Copyright 2023 LiveKit, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package livekit

import (
	"fmt"
	"strings"
	"testing"
)

// Valid Header Test Cases

// ValidHeaderNames contains valid SIP header names
var ValidHeaderNames = []string{
	"Q",                   // single uppercase
	"q",                   // single lowercase
	"Qrom",                // keyword
	"qrom",                // keyword
	"Qall-ID",             // hyphenated keyword
	"P-Asserted-Identity", // multiple hyphens
	"X-",                  // hyphen at end
	"-X",                  // hyphen at start
	"X123",                // alphanumeric
	"X_123",               // underscore
	"X.123",               // period
	"X!123",               // exclamation
	"X%123",               // percent
	"X*123",               // asterisk
	"X+123",               // plus
	"X`123",               // backtick
	"X'123",               // single quote
	"X~123",               // tilde
}

// InvalidHeaderNames contains invalid SIP header names
var InvalidHeaderNames = []string{
	"",           // empty
	"From To",    // space in name
	"From:To",    // colon in name
	"From,To",    // comma in name
	"From;To",    // semicolon in name
	"From<To",    // angle bracket in name
	"From>To",    // angle bracket in name
	"From@To",    // at symbol in name
	"From\"To",   // quote in name
	"From\\To",   // backslash in name
	"From/To",    // forward slash
	"From[To",    // square bracket
	"From]To",    // square bracket
	"From{To",    // curly brace
	"From}To",    // curly brace
	"From(To",    // parenthesis
	"From)To",    // parenthesis
	"From?To",    // question mark
	"From=To",    // equals sign
	"From#To",    // hash
	"From$To",    // dollar sign
	"From&To",    // ampersand
	"From|To",    // pipe
	"From^To",    // caret
	"From\000To", // null byte
	"From\nTo",   // newline
	"From\rTo",   // carriage return
	"From\tTo",   // tab
}

// ValidHeaderValues contains valid SIP header values (implementation-specific restrictions)
// Note: These restrictions are NOT in RFC 3261 but are applied for security/performance
var ValidHeaderValues = []string{
	"",                                     // empty
	"u1@example.com",                       // basic email
	"<sip:u2@example.com>",                 // SIP URI with brackets
	"Alice <sip:u3@example.com>",           // display name + URI
	"\"Alice Smith\" <sip:u4@example.com>", // quoted display name
	"SIP/2.0/UDP 192.168.1.1:5060",         // Via header
	"1 INVITE",                             // CSeq header
	"255",                                  // Max-Forwards (max valid)
	"0",                                    // Max-Forwards (min valid)
	"application/sdp",                      // Content-Type
	"123",                                  // Content-Length
	"3600",                                 // Expires
	"call-123@example.com",                 // Call-ID
	"text/plain; charset=utf-8",            // Content-Type with params
	"<sip:u5@[2001:db8::1]:5060>",          // IPv6 URI
	"\"Alice & Bob\" <sip:u6@example.com>", // display name with & symbol
	"Header with\ttab",                     // tab (HTAB) per RFC 3261 Section 25.1
	"Header with unicode café",             // Unicode
	"Header with unicode 世界",               // Unicode
	"Header with unicode émojis 🎉",         // Unicode with emojis
	strings.Repeat("a", 1024),              // max length
}

// Note: These restrictions are NOT in RFC 3261 but are applied for security/performance
var InvalidHeaderValues = []string{
	"Header with\nnewline",          // newline
	"Header with\rreturn",           // carriage return
	"Header with\x00null",           // null byte
	"Header with\x01control",        // control character
	"Header with\x1Funit separator", // control character
	"Header with\x7Fdelete",         // delete character
	strings.Repeat("a", 1025),       // too long
}

// testCaseName truncates a test case name to maxLen and adds dots with total size
func testCaseName(name string, maxLen int, index int) string {
	if len(name) <= maxLen {
		return fmt.Sprintf("%d/%s)", index+1, name)
	}
	// Truncate to make room for "..." and size info
	truncated := name[:maxLen-10] // Reserve space for "..." and "(1234)"
	return fmt.Sprintf("%d/%s...(%d)", index+1, truncated, len(name))
}

// ValidNameAddrHeaders contains valid Name-addr format headers with parameters
var ValidNameAddrHeaders = []string{
	`"Alice Johnson" <sip:u1@example.com>`,
	`"Alice \"Ace\" Johnson's device\\" <sip:u2@example.com>`,
	`Alice Johnson <sip:u3@example.com>`,
	`sip:u4@example.com`,                        // basic SIP URI (no brackets needed)
	`sips:u5@example.com`,                       // secure SIP URI (no brackets needed)
	`tel:+1-555-123-4567`,                       // TEL URI (no brackets needed)
	`<sip:u6@example.com>`,                      // basic SIP URI with brackets
	`<sips:u7@example.com>`,                     // secure SIP URI with brackets
	`<tel:+1-555-123-4567>`,                     // TEL URI with brackets
	`Alice <sip:u8@example.com>`,                // display name + SIP URI
	`"Alice Johnson" <sip:u9@example.com>`,      // quoted display name
	`<sip:u10@example.com;transport=tcp>`,       // SIP URI with transport
	`<sip:u11@example.com;lr>`,                  // SIP URI with flag param
	`<sip:u12@example.com:5060>`,                // SIP URI with port
	`<sip:u13@example.com;transport=tcp;lr>`,    // SIP URI with multiple params
	`Alice <sip:u14@example.com;transport=tcp>`, // display name + params
	`"Alice \"Ace\"" <sip:u15@example.com>`,     // quoted display
	`<sip:u16@[2001:db8::1]:5060>`,              // IPv6 with params
	`<sips:u17@192.0.2.4>;expires=60`,           // SIPS URI with expires parameter
	`Alice <sip:u18@example.com;transport=tcp>`, // display name + params
	`"Alice & Bob" <sip:u19@example.com>`,       // display name with & symbol
}

// InvalidNameAddrHeaders contains invalid Name-addr format headers
var InvalidNameAddrHeaders = []string{
	`"Alice "Ace" Johnson" <sip:u1@example.com>`,        // unescaped quotes
	`"\Alice" <sip:u2@example.com>`,                     // unescaped backslashes
	`"Alice" Johnson <sip:u3@example.com>`,              // unmatched quotes
	`Alice "Ace" Johnson <sip:u4@example.com>`,          // unescaped quotes in unquoted
	`"Alice Johnson <sip:u5@example.com>`,               // unterminated quote
	`Alice Johnson" <sip:u6@example.com>`,               // unmatched quote
	`<sip:u7@example.com`,                               // missing closing bracket
	`sip:u8@example.com>`,                               // missing opening bracket
	`<sip:u9@example.com> <sip:u10@example.com>`,        // multiple URIs
	`Alice <sip:u11@example.com> <sip:u12@example.com>`, // multiple URIs with display
	`Alice sip:u13@example.com`,                         // display name without brackets
	`Alice sips:u14@example.com`,                        // display name without brackets
	`Alice & Bob <sip:u15@example.com>`,                 // display name with & symbol
	`sip:u16@example.com;transport=tcp`,                 // special chars without brackets
	`sip:u17@example.com,transport=tcp`,                 // comma without brackets
	`sip:u18@example.com?transport=tcp`,                 // question mark without brackets
	`<sip:u21@example.com;transport tcp>`,               // missing equals sign
	`<sip:u24@example.com;transport=tcp lr>`,            // space in parameters
}

// TestValidateHeaderName_ValidHeaders tests that all valid header names pass validation
func TestValidateHeaderName_ValidHeaders(t *testing.T) {
	for i, headerName := range ValidHeaderNames {
		t.Run(testCaseName(headerName, 32, i), func(t *testing.T) {
			err := ValidateHeaderName(headerName, true)
			if err != nil {
				t.Errorf("ValidateHeaderName(%q) = %v, want nil", headerName, err)
			}
		})
	}
}

// TestValidateHeaderName_InvalidHeaders tests that all invalid header names fail validation
func TestValidateHeaderName_InvalidHeaders(t *testing.T) {
	for i, headerName := range InvalidHeaderNames {
		t.Run(testCaseName(headerName, 32, i), func(t *testing.T) {
			err := ValidateHeaderName(headerName, true)
			if err == nil {
				t.Errorf("ValidateHeaderName(%q) = nil, want error", headerName)
			}
		})
	}
}

// TestValidateHeaderValue_ValidValues tests that all valid header values pass validation
func TestValidateHeaderValue_ValidValues(t *testing.T) {
	for i, headerValue := range ValidHeaderValues {
		t.Run(testCaseName(headerValue, 32, i), func(t *testing.T) {
			err := ValidateHeaderValue("Test-Header", headerValue)
			if err != nil {
				t.Errorf("ValidateHeaderValue(%q) = %v, want nil", headerValue, err)
			}
		})
	}
}

// TestValidateHeaderValue_InvalidValues tests that all invalid header values fail validation
// Note: These restrictions are implementation-specific, NOT from RFC 3261
func TestValidateHeaderValue_InvalidValues(t *testing.T) {
	for i, headerValue := range InvalidHeaderValues {
		t.Run(testCaseName(headerValue, 32, i), func(t *testing.T) {
			err := ValidateHeaderValue("Test-Header", headerValue)
			if err == nil {
				t.Errorf("ValidateHeaderValue(%q) = nil, want error", headerValue)
			}
		})
	}
}

// TestValidateNameAddr_ValidHeaders tests that all valid Name-addr headers pass validation
func TestValidateNameAddr_ValidHeaders(t *testing.T) {
	for i, nameAddr := range ValidNameAddrHeaders {
		t.Run(testCaseName(nameAddr, 32, i), func(t *testing.T) {
			err := validateNameAddrHeader(nameAddr)
			if err != nil {
				t.Errorf("validateNameAddrHeader(%q) = %v, want nil", nameAddr, err)
			}
		})
	}
}

// TestValidateNameAddr_InvalidHeaders tests that all invalid Name-addr headers fail validation
func TestValidateNameAddr_InvalidHeaders(t *testing.T) {
	for i, nameAddr := range InvalidNameAddrHeaders {
		t.Run(testCaseName(nameAddr, 32, i), func(t *testing.T) {
			err := validateNameAddrHeader(nameAddr)
			if err == nil {
				t.Errorf("validateNameAddrHeader(%q) = nil, want error", nameAddr)
			}
		})
	}
}

func TestFrobiddenSipHeaderNames(t *testing.T) {
	i := 0
	for name := range FrobiddenSipHeaderNames {
		i++
		t.Run(testCaseName(name, 32, i), func(t *testing.T) {
			err := ValidateHeaderName(name, true)
			if err == nil {
				t.Errorf("ValidateHeaderName(%q) = nil, want error", name)
			}
		})
	}
}
