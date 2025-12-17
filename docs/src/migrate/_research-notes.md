<!--
  TEMPORARY RESEARCH FILE - Delete when migration guides are complete

  This file contains external community insights used to add "flair" to migration guides.
  These are NOT the template or backbone—use intellij.md as the structural template.

  STATUS:
  ✅ PyCharm guide - COMPLETE
  ✅ WebStorm guide - COMPLETE
  ✅ RustRover guide - COMPLETE
-->

# Migration Research Notes

## Completed Guides

All three JetBrains migration guides have been populated with full content:

1. **pycharm.md** - Python development, virtual environments, Ruff/Pyright, Django/Flask workflows
2. **webstorm.md** - JavaScript/TypeScript development, npm workflows, framework considerations
3. **rustrover.md** - Rust development, rust-analyzer parity, Cargo workflows, licensing notes

## Key Sources Used

- IntelliJ IDEA migration doc (structural template)
- JetBrains PyCharm Getting Started docs
- JetBrains WebStorm Getting Started docs
- JetBrains RustRover Quick Start Guide
- External community feedback (Reddit, Hacker News, Medium)

## External Quotes Incorporated

### WebStorm Guide

> "I work for AWS and the applications I deal with are massive. Often I need to keep many projects open due to tight dependencies. I'm talking about complex microservices and micro frontend infrastructure which oftentimes lead to 2-15 minutes of indexing wait time whenever I open a project or build the system locally."

### RustRover Guide

- Noted rust-analyzer shared foundation between RustRover and Zed
- Addressed licensing/telemetry concerns that motivate some users to switch
- Included debugger caveats based on community feedback

## Cross-Cutting Themes Applied to All Guides

### Universal Pain Points Addressed

1. Indexing (instant in Zed)
2. Resource usage (Zed is lightweight)
3. Startup time (Zed is near-instant)
4. UI clutter (Zed is minimal by design)

### Universal Missing Features Documented

- No project model / SDK management
- No database tools
- No framework-specific integration
- No visual run configurations (use tasks)
- No built-in HTTP client

### JetBrains Keymap Emphasized

All three guides emphasize:

- Select JetBrains keymap during onboarding or in settings
- `Shift Shift` for Search Everywhere works
- Most familiar shortcuts preserved

## Next Steps (Optional Enhancements)

- [ ] Cross-link guides to JetBrains docs for users who want to reference original IDE features
- [ ] Add a consolidated "hub page" linking to all migration guides
- [ ] Consider adding VS Code migration guide using similar structure
- [ ] Review for tone consistency against Zed Documentation Guidelines
