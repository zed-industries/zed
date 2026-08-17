package utils

import "errors"

func ErrorIsOneOf(err error, targets ...error) bool {
	for _, t := range targets {
		if errors.Is(err, t) {
			return true
		}
	}
	return false
}

func ScreenError(err error, ignored ...error) error {
	if ErrorIsOneOf(err, ignored...) {
		return nil
	}
	return err
}
