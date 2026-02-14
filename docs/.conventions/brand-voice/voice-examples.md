# Voice Transformation Examples

Ten before/after transformations demonstrating Zed's brand voice. Use these as calibration for diagnosis and reconstruction.

---

## 1. Hype to Specifics

**Before (Score: 2/5 Technical Grounding)**

> Zed delivers blazingly fast performance that will revolutionize your coding experience. Our cutting-edge technology ensures you never wait again.

**After (Score: 5/5)**

> Zed is written in Rust with GPU-accelerated rendering. Keystrokes register in under 8ms. Scrolling stays at 120fps even in large files.

**Transformation notes:**

- "blazingly fast" → specific latency numbers
- "revolutionize" → removed entirely
- "cutting-edge technology" → actual tech stack
- "never wait again" → measurable claim

---

## 2. Marketing to Technical

**Before (Score: 2/5 Developer Respect)**

> Don't worry about the complicated stuff — Zed handles it all for you! Just focus on what you do best: writing amazing code.

**After (Score: 5/5)**

> Zed runs language servers in separate processes with automatic crash recovery. If a language server fails, you keep editing while it restarts.

**Transformation notes:**

- Removed patronizing tone ("don't worry")
- Removed enthusiasm ("amazing")
- Added technical mechanism
- Treats reader as capable of understanding

---

## 3. Abstract to Concrete

**Before (Score: 2/5 Specificity)**

> Zed provides a seamless collaborative experience that brings your team together in powerful new ways.

**After (Score: 5/5)**

> Share your workspace with `cmd+shift+c`. Collaborators see your cursor, selections, and edits in real time. Voice chat is built in — no separate call needed.

**Transformation notes:**

- "seamless" → actual UX flow
- "powerful new ways" → specific features
- Added keyboard shortcut (concrete entry point)
- Described what collaboration actually looks like

---

## 4. Em Dash Chains to Natural Flow

**Before (Score: 2/5 Natural Syntax)**

> Zed is fast — really fast — and built for the way developers actually work — not how tools think they should work.

**After (Score: 5/5)**

> Zed is built for speed. We optimized for the workflows developers actually use: jumping between files, searching across projects, editing multiple cursors at once.

**Transformation notes:**

- Removed all em dashes
- Split into two clear sentences
- Abstract claim → specific examples
- "really fast" → removed (show, don't tell)

---

## 5. Enthusiasm to Confidence

**Before (Score: 1/5 Quiet Confidence)**

> We're thrilled to announce Zed 1.0! After years of hard work, we can't wait for you to experience what we've built. You're going to love it!

**After (Score: 5/5)**

> Zed 1.0 is available today. This release includes GPU text rendering, multi-buffer editing, and native collaboration. Download it at zed.dev.

**Transformation notes:**

- Removed all emotional language
- "thrilled" → deleted
- "can't wait" → deleted
- "You're going to love it" → deleted
- Added substance instead of feelings

---

## 6. "It's Not X, It's Y" Fix

**Before (Score: 2/5 Natural Syntax)**

> Zed isn't just an editor — it's a complete development environment. It's not about features — it's about flow. And it's not slow — it's instant.

**After (Score: 5/5)**

> Zed combines editing, debugging, collaboration, and AI assistance in one application. Everything runs in the same process, so switching between tasks has no context-switch overhead.

**Transformation notes:**

- Eliminated all "it's not X, it's Y" patterns
- Replaced negation with positive statements
- Added technical reasoning
- One clear sentence instead of three choppy ones

---

## 7. Vague Benefits to Specific Outcomes

**Before (Score: 2/5 Specificity)**

> Zed's AI integration enhances your productivity and streamlines your workflow, helping you code smarter and ship faster.

**After (Score: 5/5)**

> Zed runs AI completions inline as you type. Suggestions appear in 200ms. Accept with Tab, reject by continuing to type. The model runs locally or connects to your preferred API.

**Transformation notes:**

- "enhances productivity" → specific UX
- "streamlines workflow" → actual interaction model
- "code smarter" → deleted (meaningless)
- Added technical options (local vs API)

---

## 8. Social Media Cleanup

**Before (Score: 1/5 across multiple criteria)**

> 🚀 Big news! Zed just dropped MASSIVE updates! Multi-file editing, insane AI features, and SO much more. This is a game-changer, folks! Try it now! 🔥

**After (Score: 4/5)**

> Zed 0.150: Multi-buffer editing is here. Edit across files in a single view. AI completions now stream inline. Full changelog at zed.dev/releases.

**Transformation notes:**

- Removed all emoji
- Removed exclamation points
- "MASSIVE" → specific features
- "game-changer" → deleted
- "SO much more" → link to changelog
- Added version number for precision

---

## 9. Feature Announcement

**Before (Score: 2/5 Information Priority)**

> We've been listening to your feedback, and after months of development, our incredible team has built something truly special. Today, we're excited to finally share our new terminal integration!

**After (Score: 5/5)**

> Zed now includes a built-in terminal. Open it with `ctrl+\``. Terminals run in splits alongside your editor panes and share the same working directory as your project.

**Transformation notes:**

- Lead with the feature, not the backstory
- Removed emotional buildup
- Added keyboard shortcut
- Described actual behavior

---

## 10. Philosophy Statement

**Before (Score: 3/5 Quiet Confidence)**

> At Zed, we believe that developers deserve better tools. We're passionate about creating the best possible coding experience because we know how frustrating slow, bloated editors can be.

**After (Score: 5/5)**

> Developer tools should be fast, understandable, and collaborative. We built Zed to meet that standard. It's open source so you can verify our work and extend it.

**Transformation notes:**

- "We believe" → direct statement
- "passionate about" → deleted
- "best possible" → specific standard
- "frustrating, slow, bloated" → removed comparison
- Added concrete proof point (open source)

---

## Fact Preservation Rules

When transforming copy, certain elements must survive unchanged:

### Mark During Diagnosis

Tag factual claims with `[FACT]` during diagnosis phase:

```
Zed is written in [FACT: Rust] with [FACT: GPU-accelerated rendering].
It was built by [FACT: the team behind Atom and Tree-sitter].
```

### Never Change

| Category           | Examples                                   |
| ------------------ | ------------------------------------------ |
| Technical specs    | "120fps", "8ms latency", "Rust"            |
| Proper nouns       | "Tree-sitter", "Anthropic", "Claude"       |
| Version numbers    | "Zed 1.0", "v0.150"                        |
| Keyboard shortcuts | "cmd+shift+c", "ctrl+\`"                   |
| URLs               | "zed.dev/releases"                         |
| Attribution        | "built by the team behind Atom"            |
| Dates              | "available today", "released January 2024" |
| Quotes             | Any attributed quotation                   |

### Verification Step

After reconstruction, diff against original `[FACT]` markers:

1. List all facts from original
2. Confirm each appears in final copy
3. If a fact was removed, justify why (e.g., not relevant to new scope)
4. If a fact was changed, flag as error

### Example Verification

**Original with markers:**

> Zed is [FACT: written in Rust] with [FACT: GPU-accelerated rendering at 120fps]. Built by [FACT: the team behind Atom and Tree-sitter].

**Reconstruction:**

> Zed renders every frame on the GPU at 120fps. The Rust codebase prioritizes memory safety without garbage collection pauses. The same engineers who built Atom and Tree-sitter lead development.

**Verification:**

- ✅ "Rust" preserved
- ✅ "GPU-accelerated" preserved
- ✅ "120fps" preserved
- ✅ "team behind Atom and Tree-sitter" preserved
- **Pass**

---

## Transformation Patterns Summary

| Problem              | Solution                     |
| -------------------- | ---------------------------- |
| Hype words           | Replace with measurements    |
| Em dash chains       | Split into sentences         |
| "It's not X, it's Y" | State positively what it is  |
| Enthusiasm           | Delete; add substance        |
| Vague benefits       | Name specific features       |
| Buried lede          | Lead with the news           |
| Rhetorical questions | Make declarative statements  |
| Abstract claims      | Add mechanism or measurement |
