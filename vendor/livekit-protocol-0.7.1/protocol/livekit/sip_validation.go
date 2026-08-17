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
	"errors"
	fmt "fmt"
	"strconv"
	"strings"
)

// RFC 3261 compliant validation functions for SIP headers and messages

type allowedCharacters struct {
	ascii [128]bool
	utf8  bool
}

func NewAllowedCharacters() *allowedCharacters {
	return &allowedCharacters{}
}

func (a *allowedCharacters) AddUTF8() error {
	a.utf8 = true
	return nil
}

func (a *allowedCharacters) AddNumbers() error {
	for r := '0'; r <= '9'; r++ {
		a.ascii[r] = true
	}
	return nil
}

func (a *allowedCharacters) AddLowercaseASCII() error {
	for r := 'a'; r <= 'z'; r++ {
		a.ascii[r] = true
	}
	return nil
}

func (a *allowedCharacters) AddUppercaseASCII() error {
	for r := 'A'; r <= 'Z'; r++ {
		a.ascii[r] = true
	}
	return nil
}

func (a *allowedCharacters) AddPrintableLienarASCII() {
	// Anything between 0x20 and 0x7E
	for i := 0x20; i <= 0x7E; i++ {
		a.ascii[i] = true
	}
	a.ascii[0x09] = true // \t or HTAB
}

func (a *allowedCharacters) Add(chars string) error {
	for _, char := range chars {
		if int(char) >= len(a.ascii) {
			return fmt.Errorf("char %d out of range, consider explicilty adding utf8 characters", char)
		}
		a.ascii[char] = true
	}
	return nil
}

func (a *allowedCharacters) Remove(chars string) error {
	for _, char := range chars {
		if int(char) >= len(a.ascii) {
			return fmt.Errorf("char %d out of range, consider explicilty adding utf8 characters", char)
		}
		a.ascii[char] = false
	}
	return nil
}

func (a *allowedCharacters) Copy() *allowedCharacters {
	return &allowedCharacters{
		ascii: a.ascii,
		utf8:  a.utf8,
	}
}

func (a *allowedCharacters) Validate(target string) error {
	for _, char := range target {
		if int(char) >= len(a.ascii) {
			if a.utf8 {
				continue
			}
			return fmt.Errorf("char %d out of range, consider explicilty adding utf8 characters", char)
		}
		if !a.ascii[char] {
			return fmt.Errorf("char %d not allowed", char)
		}
	}
	return nil
}

var tokenCharacters *allowedCharacters
var displayNameCharacters *allowedCharacters
var headerValuesCharacters *allowedCharacters

func init() {
	// Per RFC 3261 Section 25.1
	//	SIP-message    =  Request / Response
	//	Request        =  Request-Line	*( message-header )	CRLF	[ message-body ]
	//	Response       =  Status-Line	*( message-header )	CRLF	[ message-body ]
	//	Request-Line   =  Method SP Request-URI SP SIP-Version CRLF
	//	Method         =  (CAPITAL ASCII)
	//	Request-URI    =  SIP-URI / SIPS-URI / absoluteURI
	//	SIP-Version    =  "SIP" "/" 1*DIGIT "." 1*DIGIT (CAPITAL ASCII, DIGITS, "/.")
	//	Status-Line    =  SIP-Version SP Status-Code SP Reason-Phrase CRLF
	//	Status-Code    =  (Alphanum + "-")
	//	Reason-Phrase  =  (Basically whatever...)
	//	extension-header =  header-name (token) ":" header-value (Basically whatever...)

	// URIs
	//	SIP-URI        =  "sip:" [ userinfo ] hostport uri-parameters [ headers ]
	//	SIPS-URI       =  "sips:" [ userinfo ] hostport uri-parameters [ headers ]

	// One specific header form we care about:
	//	name-addr      =  [ display-name ] LAQUOT addr-spec RAQUOT
	//	display-name   =  *(token LWS)/ quoted-string
	//	addr-spec      =  SIP-URI / SIPS-URI / absoluteURI

	tokenCharacters = NewAllowedCharacters()
	tokenCharacters.AddNumbers()
	tokenCharacters.AddLowercaseASCII()
	tokenCharacters.AddUppercaseASCII()
	tokenCharacters.Add("-.!%*_+`'~")

	displayNameCharacters = tokenCharacters.Copy()
	displayNameCharacters.Add(" \t")

	headerValuesCharacters = NewAllowedCharacters()
	headerValuesCharacters.AddPrintableLienarASCII()
	headerValuesCharacters.AddUTF8()
}

// Required headers for SIP requests per RFC 3261 Section 8.1.1
var RequiredRequestHeaders = map[string]bool{
	"via":          true,
	"from":         true,
	"to":           true,
	"call-id":      true,
	"cseq":         true,
	"max-forwards": true,
}

// Required headers for SIP responses per RFC 3261 Section 8.2.1
var RequiredResponseHeaders = map[string]bool{
	"via":     true,
	"from":    true,
	"to":      true,
	"call-id": true,
	"cseq":    true,
}

// Crucial headers that can't be overridden by the user, and their shorthands
var FrobiddenSipHeaderNames = map[string]bool{
	"accept":           true,
	"accept-encoding":  true,
	"accept-language":  true,
	"allow":            true,
	"allow-events":     true, // rfc3903
	"call-id":          true,
	"contact":          true,
	"content-encoding": true,
	"content-length":   true,
	"content-type":     true,
	"cseq":             true,
	"event":            true, // rfc3903
	"expires":          true,
	"from":             true, // We might allow this in the future, but for now we're printing
	"max-forwards":     true,
	"record-route":     true,
	"refer-to":         true, // rfc3515
	"reply-to":         true,
	"k":                true, // Supported
	"l":                true, // Content-Length
	"m":                true, // Contact
	"o":                true, // Event; rfc3903
	"r":                true, // Refer-To; rfc3515
	"t":                true, // To
	"u":                true, // Allow-Events; rfc3903
	"v":                true, // Via
}

// Headers that must comply with name-addr specification per RFC 3261 Section 20.10
// name-addr = [display-name] <addr-spec>
// addr-spec = SIP-URI / SIPS-URI / absoluteURI
var nameAddrHeaders = map[string]bool{
	"from":                true,
	"to":                  true,
	"contact":             true,
	"route":               true,
	"record-route":        true,
	"reply-to":            true,
	"p-asserted-identity": true, // RFC 3325 Section 9.1
}

// ValidateHeaderName validates a SIP header name per RFC 3261 Section 25.1
func ValidateHeaderName(name string, restrictNames bool) error {
	if name == "" {
		return errors.New("header name cannot be empty")
	}

	if len(name) > 255 {
		return errors.New("header name too long (max 255 characters)")
	}

	if err := tokenCharacters.Validate(name); err != nil {
		return fmt.Errorf("header name %s contains invalid characters: %w", name, err)
	}

	// Convert to lowercase for case-insensitive comparison
	if restrictNames {
		lowerName := strings.ToLower(name)
		if forbidden, exists := FrobiddenSipHeaderNames[lowerName]; exists && forbidden {
			return fmt.Errorf("header name %s not supported", name)
		}
	}

	return nil
}

// ValidateHeaderValue validates a SIP header value per RFC 3261 Section 25.1
func ValidateHeaderValue(name, value string) error {
	if value == "" {
		return nil
	}

	if len(value) > 1024 {
		return fmt.Errorf("header %s: value too long (max 1024 characters)", name)
	}

	// Basic character validation - printable ASCII. We're stricter than the spec here - no UTF-8 for now
	if err := headerValuesCharacters.Validate(value); err != nil {
		return fmt.Errorf("header %s: value: %w", name, err)
	}

	// Convert to lowercase for case-insensitive comparison
	lowerName := strings.ToLower(name)
	if _, exists := nameAddrHeaders[lowerName]; exists && false {
		// TODO: Disabled since all supported headers are forbidden, re-enable when we allow some
		if err := validateNameAddrHeader(value); err != nil {
			return fmt.Errorf("header %s: value: %w", name, err)
		}
	}

	return nil
}

// findAngleBrackets efficiently finds angle brackets in a single scan
// Returns: start, end positions (-1 = missing), and error status
func findAngleBrackets(value string) (int, int, error) {
	start := -1
	end := -1

	for i, r := range value {
		switch r {
		case '<':
			if start != -1 {
				return -1, -1, errors.New("multiple opening brackets")
			}
			start = i
		case '>':
			if end != -1 {
				return -1, -1, errors.New("multiple closing brackets")
			}
			end = i
		}
	}

	// Check for mismatched brackets
	if (start == -1) != (end == -1) {
		return -1, -1, errors.New("mismatched angle brackets")
	}

	// Check that < comes before >
	if start > end {
		return -1, -1, errors.New("malformed angle brackets")
	}

	return start, end, nil
}

// validateNameAddrHeader validates headers that use name-addr format per RFC 3261 Section 20.10
func validateNameAddrHeader(value string) error {
	// RFC 3261 Section 20.10 - name-addr format
	// name-addr = [display-name] <addr-spec>
	// addr-spec = SIP-URI / SIPS-URI / absoluteURI

	uri := value
	start, end, err := findAngleBrackets(value)
	if err != nil {
		return err
	}
	if start >= 0 || end >= 0 {
		uri = value[start+1 : end]
		if err := validateDisplayName(strings.TrimSpace(value[:start])); err != nil {
			return err
		}
	} else {
		// This is a bare URI, and should comply with addr-spec, no special characters
		if strings.ContainsAny(value, ";,? ") {
			return errors.New("bare URI with special characters")
		}
	}
	return validateURI(uri)
}

// validateDisplayName validates a display name in name-addr format
func validateDisplayName(displayName string) error {
	if displayName == "" {
		return nil
	}

	// Check if display name is quoted
	if strings.HasPrefix(displayName, `"`) && strings.HasSuffix(displayName, `"`) {
		// Quoted display name - use strconv.Unquote to validate proper escaping
		_, err := strconv.Unquote(displayName)
		if err != nil {
			return fmt.Errorf("display name: %w", err)
		}
		return nil
	}

	// Unquoted display name - must not contain special characters
	if err := displayNameCharacters.Validate(displayName); err != nil {
		return fmt.Errorf("display name: %w", err)
	}

	return nil
}

// validateURI validates URIs that can appear in name-addr format
func validateURI(uri string) error {
	// Just do the basics, full validation should be done by sip service
	scheme := strings.SplitN(uri, ":", 2)[0]
	if scheme != "sip" && scheme != "sips" && scheme != "tel" {
		// Technically, it either needs to be sip/s: or scheme://...
		// Thus, tel: uri should not be supported here... but we allow it because of de-facto usage.
		return errors.New("uri: scheme not one of sip, sips, or tel")
	}

	// Just no spaces, proper validation should be done in sip service
	if strings.Contains(uri, " ") {
		return errors.New("uri: contains spaces")
	}

	return nil
}
