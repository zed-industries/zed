// Code generated; DO NOT EDIT.

package gatewayobs

import (
	"time"
)

var (
	_ Reporter         = (*noopReporter)(nil)
	_ ProjectReporter  = (*noopProjectReporter)(nil)
	_ ProviderReporter = (*noopProviderReporter)(nil)
	_ ModelReporter    = (*noopModelReporter)(nil)
)

type noopKeyResolver struct{}

func (noopKeyResolver) Resolve(string) {}
func (noopKeyResolver) Reset()         {}

type noopReporter struct{}

func NewNoopReporter() Reporter {
	return &noopReporter{}
}

func (r *noopReporter) WithProject(id string) ProjectReporter {
	return &noopProjectReporter{}
}

func (r *noopReporter) WithDeferredProject() (ProjectReporter, KeyResolver) {
	return &noopProjectReporter{}, noopKeyResolver{}
}

type noopProjectReporter struct{}

func NewNoopProjectReporter() ProjectReporter {
	return &noopProjectReporter{}
}

func (r *noopProjectReporter) RegisterFunc(f func(ts time.Time, tx ProjectTx) bool) {}
func (r *noopProjectReporter) Tx(f func(ProjectTx))                                 {}
func (r *noopProjectReporter) TxAt(ts time.Time, f func(ProjectTx))                 {}
func (r *noopProjectReporter) WithProvider(name string) ProviderReporter {
	return &noopProviderReporter{}
}
func (r *noopProjectReporter) WithDeferredProvider() (ProviderReporter, KeyResolver) {
	return &noopProviderReporter{}, noopKeyResolver{}
}

type noopProviderReporter struct{}

func NewNoopProviderReporter() ProviderReporter {
	return &noopProviderReporter{}
}

func (r *noopProviderReporter) RegisterFunc(f func(ts time.Time, tx ProviderTx) bool) {}
func (r *noopProviderReporter) Tx(f func(ProviderTx))                                 {}
func (r *noopProviderReporter) TxAt(ts time.Time, f func(ProviderTx))                 {}
func (r *noopProviderReporter) WithModel(name string) ModelReporter {
	return &noopModelReporter{}
}
func (r *noopProviderReporter) WithDeferredModel() (ModelReporter, KeyResolver) {
	return &noopModelReporter{}, noopKeyResolver{}
}

type noopModelReporter struct{}

func NewNoopModelReporter() ModelReporter {
	return &noopModelReporter{}
}

func (r *noopModelReporter) RegisterFunc(f func(ts time.Time, tx ModelTx) bool) {}
func (r *noopModelReporter) Tx(f func(ModelTx))                                 {}
func (r *noopModelReporter) TxAt(ts time.Time, f func(ModelTx))                 {}
func (r *noopModelReporter) ReportInferencePromptTokens(v uint64)               {}
func (r *noopModelReporter) ReportInferencePromptCacheTokens(v uint64)          {}
func (r *noopModelReporter) ReportInferenceCompletionTokens(v uint64)           {}
func (r *noopModelReporter) ReportInferenceTotalTokens(v uint64)                {}
func (r *noopModelReporter) ReportInferenceCacheCreateTokens(v uint64)          {}
func (r *noopModelReporter) ReportInferenceCacheReadTokens(v uint64)            {}
func (r *noopModelReporter) ReportSttDuration(v uint32)                         {}
func (r *noopModelReporter) ReportTtsChars(v uint32)                            {}
