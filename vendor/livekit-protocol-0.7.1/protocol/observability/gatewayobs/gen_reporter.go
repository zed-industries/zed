// Code generated; DO NOT EDIT.

package gatewayobs

import (
	"time"
)

const Version_J2LFGS0 = true

type KeyResolver interface {
	Resolve(string)
	Reset()
}

type Reporter interface {
	WithProject(id string) ProjectReporter
	WithDeferredProject() (ProjectReporter, KeyResolver)
}

type ProjectTx interface{}

type ProjectReporter interface {
	RegisterFunc(func(ts time.Time, tx ProjectTx) bool)
	Tx(func(tx ProjectTx))
	TxAt(time.Time, func(tx ProjectTx))
	WithProvider(name string) ProviderReporter
	WithDeferredProvider() (ProviderReporter, KeyResolver)
	ProjectTx
}

type ProviderTx interface{}

type ProviderReporter interface {
	RegisterFunc(func(ts time.Time, tx ProviderTx) bool)
	Tx(func(tx ProviderTx))
	TxAt(time.Time, func(tx ProviderTx))
	WithModel(name string) ModelReporter
	WithDeferredModel() (ModelReporter, KeyResolver)
	ProviderTx
}

type ModelTx interface {
	ReportInferencePromptTokens(v uint64)
	ReportInferencePromptCacheTokens(v uint64)
	ReportInferenceCompletionTokens(v uint64)
	ReportInferenceTotalTokens(v uint64)
	ReportInferenceCacheCreateTokens(v uint64)
	ReportInferenceCacheReadTokens(v uint64)
	ReportSttDuration(v uint32)
	ReportTtsChars(v uint32)
}

type ModelReporter interface {
	RegisterFunc(func(ts time.Time, tx ModelTx) bool)
	Tx(func(tx ModelTx))
	TxAt(time.Time, func(tx ModelTx))
	ModelTx
}
