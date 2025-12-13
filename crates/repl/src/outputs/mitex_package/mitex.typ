// Modified mitex.typ - we only need the scope, not the WASM plugin
// The Rust mitex crate does the LaTeX->Typst conversion for us
#import "specs/mod.typ": mitex-scope

// Re-export the scope so helper functions like mitexsqrt are available
#let scope = mitex-scope
