# Zed Docs Automation

## Overview

GitHub Action that analyzes PRs and suggests documentation updates when warranted. Replaces the existing incomplete `docs_automation.yml` workflow.

## Status

- **Phase:** Foundation Complete, Action Development Next
- **Progress:** 65%
- **Started:** 2026-02-01
- **Target:** TBD

## Goal

Make documentation happen automatically when code ships, with high-quality suggestions that match Zed's voice and conventions.

## Approach: Hybrid Conventions + Smart Retrieval

Combine static conventions with dynamic context retrieval to give the GitHub Action enough information to generate genuinely useful doc suggestions.

---

## Components

### 1. Static Context (Committed to Repo) ✓ COMPLETE

```
docs/
  .conventions/
    CONVENTIONS.md            # Rules, tone, structure
    brand-voice/
      SKILL.md                # Core voice principles
      rubric.md               # 8-point scoring criteria
      taboo-phrases.md        # Patterns to avoid
      voice-examples.md       # Before/after transformations
    REMEDIATION-PLAN.md       # Tracking doc for ongoing work
    AUTOMATION-PLAN.md        # This file
  .doc-examples/              # Curated gold standard examples
    simple-feature.md         # Overview/navigation docs
    complex-feature.md        # Comprehensive feature docs
    configuration.md          # Settings documentation
    reference.md              # API/tool reference
  .scripts/
    add-frontmatter.py        # Batch frontmatter utility
```

### 2. GitHub Action Workflow (Next Phase)

Triggers on PR, builds context, generates suggestions as review comments.

### 3. Feedback Mechanism (Future)

Track when suggestions are modified vs applied directly to improve over time.

---

## Implementation Plan

### Phase 1: Foundation ✓ COMPLETE

- [x] Audit existing docs to identify patterns and conventions
- [x] Write `docs/.conventions/CONVENTIONS.md` covering:
  - Document structure (sections, ordering)
  - What requires documentation vs what doesn't
  - Formatting rules (headers, code blocks, links, anchor IDs)
  - Keybinding and action syntax (`{#kb}`, `{#action}`)
  - JSON example annotations (`[settings]`, `[keymap]`)
  - Callout format (`> **Note:**`)
  - Version-specific note patterns
- [x] Write brand voice guidelines in `docs/.conventions/brand-voice/`:
  - 8-point quality rubric (must score 4+ on all criteria)
  - Taboo phrases list (hype words, AI patterns, empty enthusiasm)
  - Voice transformation examples
- [x] Curate 4 gold standard examples into `docs/.doc-examples/`:
  - `simple-feature.md` — Overview/navigation docs
  - `complex-feature.md` — Comprehensive feature docs
  - `configuration.md` — Settings documentation
  - `reference.md` — API/tool reference

### Phase 1.5: Documentation Remediation ✓ COMPLETE

Applied conventions to existing docs:

**P0 Critical:**

- [x] Rewrite `getting-started.md` — Remove taboo phrases, convert hardcoded keybindings to `{#kb}` syntax, add anchor IDs
- [x] Remove empty `quick-start.md` stub
- [x] Add YAML frontmatter to 118 docs missing it (batch script)

**P1 High:**

- [x] Fix `collaboration/` brand voice — Rewrite overview.md opening, improve channels.md and contacts docs
- [x] Standardize callout format to `> **Note:**` across 9 files
- [x] Improve extension docs openings (installing, developing)

**P2 Medium:**

- [x] Convert keybindings in `running-testing.md` to `{#kb}` syntax
- [x] Add anchor IDs to key reference docs (configuring-zed, key-bindings, vim)
- [ ] Add "See Also" sections where missing (lower priority)

**P3 Lower:**

- [ ] Version callouts where behavior differs
- [ ] Remove TBD HTML comments

### Phase 2: Settings UI References (NEXT)

Apply Settings UI research findings across all docs:

- [ ] Update docs to mention Settings UI before JSON examples where applicable
- [ ] Use pattern: "Open Settings ({#kb zed::OpenSettings}) and search for X, or set in JSON"
- [ ] For JSON-only settings, note: "(this setting requires manual JSON editing)"
- [ ] Consider adding `zed://settings/` deep links

**High-priority files (most settings.json refs):**

- [ ] `ai/llm-providers.md` (15+ refs)
- [ ] `ai/edit-prediction.md` (8+ refs)
- [ ] `ai/tool-permissions.md` (3 refs, explicitly mentions UI)
- [ ] `configuring-languages.md` (8+ refs)
- [ ] `languages/python.md` (10+ refs)

### Phase 3: Full Brand Voice Audit

Execute the brand voice audit defined in the audit framework below:

- [ ] Run pre-screening grep commands (exclamation points, hype words, em dashes)
- [ ] Fix quick wins (~3 exclamation points, ~10 em dash chains)
- [ ] Audit root-level docs (Priority 1)
- [ ] Audit ai/ docs (Priority 2)
- [ ] Audit migrate/ docs (Priority 3)
- [ ] Audit remaining directories as time permits

### Phase 4: GitHub Action Development

- [ ] Create new workflow: `.github/workflows/docs_suggestions.yml`
- [ ] Build context assembly script:
  - Always include `CONVENTIONS.md` and brand voice summary
  - Always include one gold standard example (matched to change type)
  - Dynamically find related existing docs based on changed paths
  - Include the PR diff
- [ ] Craft the prompt with high bar for suggestions
- [ ] Output as PR comment with clear, actionable suggestions
- [ ] Handle edge cases:
  - Internal-only changes → "No documentation changes needed"
  - Updates to existing features → suggests edits to existing docs
  - Large PRs → filters to user-facing changes

### Phase 6: Rollout

- [ ] Test on 5-10 recent PRs (run manually, don't post)
- [ ] Gather feedback from team on suggestion quality
- [ ] Iterate on conventions and prompt based on feedback
- [ ] Enable on all PRs to `crates/**/*.rs`
- [ ] Document the system itself

### Phase 7: Iteration (Future)

- [ ] Track acceptance rate of suggestions
- [ ] Log modifications made before applying
- [ ] Use modification patterns to improve conventions/prompt
- [ ] Consider expanding to other file types

---

## Key Conventions (Summary)

### Document Structure

1. YAML frontmatter (`title`, `description`)
2. H1 title with anchor ID
3. Opening paragraph (what + why)
4. Usage/Getting Started
5. Configuration (with JSON examples)
6. Reference tables (actions, keybindings)
7. See Also links

### Formatting

- Settings in `code`: `vim_mode`, `buffer_font_size`
- Keybindings via `{#kb action::Name}` syntax
- Actions via `{#action action::Name}` syntax
- JSON with `[settings]` or `[keymap]` annotation
- Callouts as `> **Note:**`, `> **Tip:**`, `> **Warning:**`
- Anchor IDs on linkable sections: `## Section {#section-id}`

### Brand Voice (Must Score 4+ on All)

1. Technical Grounding — Specific, verifiable claims
2. Natural Syntax — Flows like developer speech
3. Quiet Confidence — Facts without hype
4. Developer Respect — Peer-to-peer tone
5. Information Priority — Key info first
6. Specificity — Concrete, measurable claims
7. Voice Consistency — Unified tone throughout
8. Earned Claims — Assertions are supportable

### Taboo (Auto-Fail)

- Exclamation points
- "We're excited/thrilled"
- "Revolutionary" or "game-changing"
- Em dash chains (2+ per paragraph)
- "It's not X, it's Y" constructions

---

## GitHub Action Workflow (Draft)

```yaml
name: Documentation Suggestions

on:
  pull_request:
    paths:
      - "crates/**/*.rs"

jobs:
  suggest-docs:
    runs-on: ubuntu-latest
    permissions:
      pull-requests: write
      contents: read

    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - name: Get changed files
        id: changes
        run: |
          echo "files=$(git diff --name-only origin/${{ github.base_ref }}...HEAD | grep '\.rs$' | tr '\n' ' ')" >> $GITHUB_OUTPUT

      - name: Build documentation context
        id: context
        run: |
          # Start with conventions
          echo "## Documentation Conventions" > context.md
          cat docs/.conventions/CONVENTIONS.md >> context.md

          # Add brand voice summary
          echo -e "\n## Brand Voice Quick Reference" >> context.md
          echo "Score 4+ on: Technical Grounding, Natural Syntax, Quiet Confidence, Developer Respect, Information Priority, Specificity, Voice Consistency, Earned Claims" >> context.md
          echo "Never use: exclamation points, 'we're excited', 'revolutionary', em dash chains" >> context.md

          # Add a gold standard example
          echo -e "\n## Example of Well-Documented Feature" >> context.md
          cat docs/.doc-examples/simple-feature.md >> context.md

          # Find related existing docs based on changed crate names
          echo -e "\n## Related Existing Documentation" >> context.md
          for file in ${{ steps.changes.outputs.files }}; do
            crate=$(echo $file | cut -d'/' -f2)
            if ls docs/src/*${crate}*.md 2>/dev/null | head -1 | xargs cat >> context.md 2>/dev/null; then
              break
            fi
          done

          # Add the diff
          echo -e "\n## Changes in This PR" >> context.md
          git diff origin/${{ github.base_ref }}...HEAD -- ${{ steps.changes.outputs.files }} >> context.md

      - name: Analyze and suggest documentation
        uses: anthropics/claude-code-action@v1
        with:
          model: claude-sonnet-4-20250514
          prompt: |
            You are a documentation reviewer for Zed, a high-performance code editor written in Rust.

            $(cat context.md)

            ## Your Task

            Analyze these changes and decide if documentation needs to be added or updated.

            ### When to suggest documentation:
            - New user-facing features or behaviors
            - New or changed settings/configuration options
            - New or changed keybindings/commands
            - New AI capabilities, providers, or tools
            - Public extension APIs
            - Breaking changes (even if fix is simple)
            - Version-specific behavior changes

            ### When NOT to suggest documentation:
            - Internal refactors with no user-visible change
            - Bug fixes (unless existing docs were wrong)
            - Performance improvements (unless user-visible)
            - Test changes, CI changes

            ### High bar for suggestions
            Only suggest documentation when it genuinely helps users. Most PRs won't need docs changes. When in doubt, don't suggest.

            ### Response format

            If docs changes are warranted:
            1. Explain briefly why docs are needed (one sentence)
            2. Specify which doc file to update or create
            3. Provide the exact content to add/change, following conventions:
               - Include YAML frontmatter if new file
               - Use {#kb action::Name} for keybindings
               - Use > **Note:** for callouts
               - Match the brand voice (no hype, no exclamation points)

            If no docs needed, respond with just:
            "No documentation changes needed."

            Do not hedge or qualify. Either docs are needed or they're not.

          post_as: pr_comment
```

---

## Success Metrics

- **Acceptance rate**: % of suggestions applied (target: >60%)
- **Modification rate**: % of suggestions edited before applying (target: <30%)
- **Coverage**: % of PRs with new features that get docs (target: >90%)
- **Time to docs**: Days between feature merge and docs live (target: same day)

---

## Commits Made

| Commit       | Description                                              | Files |
| ------------ | -------------------------------------------------------- | ----- |
| `25a211b358` | Add documentation conventions and gold standard examples | 9     |
| `0f223962c9` | P0/P1 remediation - frontmatter, brand voice, callouts   | 127   |
| `c9246012ef` | P2 remediation - keybinding syntax and anchor IDs        | 5     |

---

## Next Actions

- [ ] Finalize GitHub Action workflow
- [ ] Test action on recent PRs manually
- [ ] Add any remaining items to the plan before going live

---

## Competitive Analysis: Cursor Docs

### What Cursor Does Well

1. **Clear opening sentences**: Every page starts with a single sentence explaining what it is

   - "Agent is Cursor's assistant that can complete complex coding tasks independently"
   - "This quickstart walks you through working with Cursor's Agent"

2. **Task-oriented structure**: Quickstart uses action verbs as headers

   - "Start with Agent", "Plan before building", "Write specific prompts"

3. **Inline keybindings**: Shows keybindings inline with actions

   - "Open the Agent panel with `Cmd I` / `Ctrl I`"

4. **Comparison tables**: Vague vs Specific prompts table is highly effective

5. **"Next steps" cards**: Visual link cards at bottom for related content

6. **Cookbook section**: Practical workflow recipes (TDD, git workflows, etc.)

### Gaps in Cursor We Should Avoid

1. No YAML frontmatter visible (may be internal)
2. Less technical depth (our language docs are more comprehensive)
3. No explicit brand voice guidelines (we have rubric)

### Action Items from Cursor Comparison

- [ ] Audit Zed quickstart/getting-started for task-oriented headers
- [ ] Add comparison tables where "vague vs specific" patterns help

---

## Mintlify Insights: AI + Human Readability

### Key Takeaways

1. **YAML frontmatter is critical for AI**: Metadata helps AI understand page purpose

   - ✓ We now have frontmatter on all docs

2. **Minimal formatting for AI extraction**:

   - Clear hierarchical headings ✓
   - Short paragraphs and lists ✓
   - Avoid hidden/interactive content (tabs, collapsibles)
   - Separate code from text with fenced blocks ✓

3. **Writing habits that help AI**:

   - Concise sentences, active voice ✓ (in brand voice guidelines)
   - Descriptive alt text for images — **Need to audit**
   - Consistent terminology — **Need to audit**
   - Minimize vague pronouns — **Add to conventions**

4. **llms.txt / llms-full.txt**: Single-file export for AI tools
   - **Consider**: Generate `docs/llms.txt` as part of build

### Action Items from Mintlify

- [ ] Add "minimize vague pronouns" to CONVENTIONS.md
- [ ] Scan all docs for vague pronouns ("it", "this", "that") and fix
- [ ] Audit images for descriptive alt text
- [ ] Consider generating `llms.txt` for AI consumption (future)
- [ ] Review docs for interactive elements that may not parse well (future)

---

## Settings UI vs settings.json Audit

Found **100+ references** to `settings.json` across docs. Many could reference the Settings UI instead.

### Files with Most settings.json References

| File                     | Count | Priority                      |
| ------------------------ | ----- | ----------------------------- |
| ai/llm-providers.md      | 15+   | High — many could use UI      |
| languages/python.md      | 10+   | Medium                        |
| ai/edit-prediction.md    | 8+    | High — has UI controls        |
| configuring-languages.md | 8+    | Medium                        |
| remote-development.md    | 8+    | Low — advanced config         |
| ai/tool-permissions.md   | 3     | High — explicitly mentions UI |

### Pattern to Fix

**Before:**

> To disable all AI features, add the following to your `settings.json`:

**After:**

> Disable AI features in the Settings Editor ({#kb zed::OpenSettings}) by searching for "ai" and toggling off, or add to your settings file:

### Settings UI Research Findings (2026-02-14)

**Key files:**

- `crates/settings_ui/src/page_data.rs` — Settings registration (~9000 lines, 362 settings)
- `crates/settings_ui/src/settings_ui.rs` — Main UI implementation

**Statistics:**

- **329 settings** have full UI support (~91%)
- **33 settings** are JSON-only (marked with `.unimplemented()`)

**JSON-only settings** (complex types the UI doesn't support):

- `private_files`, `wrap_guides` — Arrays
- `buffer_font_features`, `buffer_font_fallbacks` — Font configuration
- `lsp` — Entire LSP configuration object
- `settings_profiles` — Complex profiles system
- Various language-specific nested settings

**Deep links work:** `zed://settings/buffer_font_size` opens Settings UI to that setting

### Documentation Pattern

**For UI-supported settings (~91%):**

> Open Settings ({#kb zed::OpenSettings}) and search for "Font Size", or set `buffer_font_size` in your settings file.

**For JSON-only settings (~9%):**

> Add the following to your `settings.json` (this setting requires manual JSON editing):

### Action Items

- [x] Research Settings UI capabilities
- [ ] Audit high-priority files (llm-providers, edit-prediction, tool-permissions)
- [ ] Add Settings UI mention before JSON examples where applicable
- [ ] Update CONVENTIONS.md with "UI first" pattern
- [ ] Consider adding `zed://settings/` deep links in docs

---

## Full Brand Voice Audit

Every doc needs to be scored against the 8-point rubric. Target: **4+ on all criteria**.

### Audit Process

1. **Batch by directory**: Audit docs in groups (ai/, languages/, extensions/, etc.)
2. **Score each doc**: Use quick scoring template from rubric.md
3. **Flag failures**: Any criterion scoring 3 or below needs rewrite
4. **Track in spreadsheet or checklist**: Mark pass/fail/needs-work

### Priority Order for Audit

| Priority | Directory      | Files | Rationale                      |
| -------- | -------------- | ----- | ------------------------------ |
| 1        | Root           | ~15   | High-traffic entry points      |
| 2        | ai/            | ~17   | Core feature, brand-critical   |
| 3        | migrate/       | 5     | First impression for new users |
| 4        | collaboration/ | 3     | Already partially fixed        |
| 5        | extensions/    | ~10   | Developer-facing               |
| 6        | languages/     | ~60   | Lower priority, template-based |
| 7        | development/   | ~8    | Internal audience              |
| 8        | reference/     | 3     | Auto-generated content         |

### Quick Audit Checklist (Per Doc)

```markdown
## [filename.md]

| Criterion            | Score | Notes |
| -------------------- | ----- | ----- |
| Technical Grounding  | /5    |       |
| Natural Syntax       | /5    |       |
| Quiet Confidence     | /5    |       |
| Developer Respect    | /5    |       |
| Information Priority | /5    |       |
| Specificity          | /5    |       |
| Voice Consistency    | /5    |       |
| Earned Claims        | /5    |       |

**Result:** [ ] Pass (32+) / [ ] Rewrite needed
**Taboo phrases found:**
**Action items:**
```

### Automated Pre-Screening

Before manual audit, grep for auto-fail patterns:

````bash
# Exclamation points
grep -rn '!' docs/src/*.md | grep -v '```' | grep -v 'http'

# "We're excited/thrilled"
grep -rni "we're excited\|we're thrilled\|we are excited" docs/src/

# Hype words
grep -rni "revolutionary\|game-changing\|blazingly\|seamless" docs/src/

# Em dash chains (2+ per line)
grep -rn '—.*—' docs/src/
````

### Audit Tracking

| Directory      | Total | Audited | Pass | Needs Work | Complete |
| -------------- | ----- | ------- | ---- | ---------- | -------- |
| Root           | 15    | 0       | 0    | 0          | [ ]      |
| ai/            | 17    | 0       | 0    | 0          | [ ]      |
| migrate/       | 5     | 0       | 0    | 0          | [ ]      |
| collaboration/ | 3     | 3       | 3    | 0          | [x]      |
| extensions/    | 10    | 2       | 2    | 0          | [ ]      |
| languages/     | 60    | 0       | 0    | 0          | [ ]      |
| development/   | 8     | 0       | 0    | 0          | [ ]      |
| reference/     | 3     | 0       | 0    | 0          | [ ]      |

### Pre-Screening Results (2026-02-14)

**Exclamation points (potential issues):**

- `command-palette.md:12` — "Try it!" (casual but borderline)
- `helix.md:8` — "Work in progress!" (acceptable for WIP notice)
- `key-bindings.md:206` — "help is very much appreciated!" (remove)

**"We're excited/thrilled":** None found ✓

**Hype words:**

- `ai/mcp.md:10` — "seamless integration" (in MCP protocol description quote, may be acceptable as external quote)

**Em dash chains (2+ per paragraph):**

- `environment.md:21` — Two em dashes in one sentence
- `environment.md:97` — Two em dashes in one sentence
- `troubleshooting.md:10, 40` — Borderline
- `migrate/rustrover.md:177` — Two em dashes
- `ai/agent-panel.md:128` — Two em dashes
- Several others in extensions/, ai/, toolchains.md

**Summary:** Docs are in relatively good shape. Main issues:

- ~3 exclamation points to remove
- ~10 sentences with em dash chains to simplify
- 1 "seamless" usage (in quote context)

### Estimated Effort

- **Pre-screening (automated):** 30 min ✓ DONE
- **Quick fixes (exclamation points, em dashes):** 1-2 hours
- **Manual audit per doc:** 5-10 min
- **Rewrite per failed doc:** 15-30 min
- **Total estimate:** 15-25 hours for full audit

---

## Decisions

| Question                 | Decision        | Rationale                                                             |
| ------------------------ | --------------- | --------------------------------------------------------------------- |
| Where to post?           | PR comment      | Not inline - docs suggestions aren't tied to specific diff lines      |
| Large PRs?               | Smart filtering | Focus on user-facing changes; internal refactors naturally filter out |
| Skip mechanism?          | None            | Action should be smart enough to say "no docs needed" with high bar   |
| Existing workflow?       | Replace         | `docs_automation.yml` was half-finished; this fully ships it          |
| Brand voice enforcement? | Via conventions | Rubric in conventions, not runtime checking                           |

---

_Created: 2026-02-01_
_Last Updated: 2026-02-14_
