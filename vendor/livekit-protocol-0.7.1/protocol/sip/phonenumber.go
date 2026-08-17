package sip

import (
	"github.com/livekit/protocol/livekit"
	"github.com/nyaruka/phonenumbers"
)

// ExtractAreaCode extracts the area code from a phone number using the phonenumbers library
func ExtractAreaCode(phoneNumber string) string {
	// Parse the phone number without defaulting to any country
	num, err := phonenumbers.Parse(phoneNumber, "")
	if err != nil {
		// If parsing fails, fall back to empty string
		return ""
	}

	// Get the country code
	countryCode := phonenumbers.GetRegionCodeForNumber(num)

	// Only handle US numbers for now
	if countryCode != "US" {
		return ""
	}

	// Get the national number and extract first 3 digits (area code for US)
	nationalNumber := phonenumbers.GetNationalSignificantNumber(num)
	if len(nationalNumber) < 3 {
		return ""
	}
	return nationalNumber[:3]
}

// DetermineNumberType determines the phone number type using the phonenumbers library
func DetermineNumberType(phoneNumber string) livekit.PhoneNumberType {
	// Parse the phone number without defaulting to any country
	num, err := phonenumbers.Parse(phoneNumber, "")
	if err != nil {
		// If parsing fails, fall back to unknown
		return livekit.PhoneNumberType_PHONE_NUMBER_TYPE_UNKNOWN
	}

	numberType := phonenumbers.GetNumberType(num)

	// We are excluding a bunch of number types for now
	switch numberType {
	case phonenumbers.MOBILE:
		return livekit.PhoneNumberType_PHONE_NUMBER_TYPE_MOBILE
	case phonenumbers.FIXED_LINE:
		return livekit.PhoneNumberType_PHONE_NUMBER_TYPE_LOCAL
	case phonenumbers.FIXED_LINE_OR_MOBILE:
		return livekit.PhoneNumberType_PHONE_NUMBER_TYPE_LOCAL
	case phonenumbers.TOLL_FREE:
		return livekit.PhoneNumberType_PHONE_NUMBER_TYPE_TOLL_FREE
	default:
		return livekit.PhoneNumberType_PHONE_NUMBER_TYPE_UNKNOWN
	}
}
