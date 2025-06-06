# CodeOrbit Release Checklist

## Pre-Release
- [ ] Ensure all tests are passing
- [ ] Update version numbers in `Cargo.toml` and `package.json`
- [ ] Update `CHANGELOG.md` with release notes
- [ ] Verify all dependencies are up to date
- [ ] Run security audit (`cargo audit`, `npm audit`)

## Build Process
- [ ] Build release version: `cargo build --release`
- [ ] Run tests on release build
- [ ] Create release package
  - Windows: `zip -r CodeOrbit-v1.0.0-windows-x64.zip target/release/codeorbit.exe README.md assets/`
  - Linux: `tar -czvf CodeOrbit-v1.0.0-linux-x64.tar.gz target/release/codeorbit README.md assets/`
  - macOS: `zip -r CodeOrbit-v1.0.0-macos-x64.zip target/release/codeorbit README.md assets/`

## Release Process
- [ ] Create a new release on GitHub
- [ ] Upload release assets
- [ ] Write release notes
- [ ] Publish release

## Post-Release
- [ ] Update documentation
- [ ] Announce release
  - [ ] Blog post
  - [ ] Social media
  - [ ] Newsletter (if applicable)
- [ ] Bump version number for next development cycle

## Verification
- [ ] Test installation on clean environment
- [ ] Verify all features work as expected
- [ ] Check for any regressions
