package model

import "errors"

var (
	// TODO: consider about erro category and boundary.

	// -------- domain level errors.
	ErrMessageKeyNotFound = errors.New("message key not found")
	ErrPortNotFound       = errors.New("message key not found")
	ErrInvalidComponent   = errors.New("message key not found")

	// -------- infrastructure level errors.
	ErrWindowNotFound = errors.New("message key not found")
)
