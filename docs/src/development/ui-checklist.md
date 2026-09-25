---
title: UI/UX Checklist
description: Review UI changes for accessibility, responsiveness, consistency, and performance.
---

# UI/UX checklist

Use this checklist when your changes affect UI.

## Accessibility / Ergonomics {#accessibility-ergonomics}

- Do all keyboard shortcuts work as intended?
- Are shortcuts discoverable (tooltips, menus, docs)?
- Is it usable without a mouse (keyboard-only navigation)?
- Do all mouse actions work (drag, context menus, resizing, scrolling)?
- Does the feature look great in light and dark mode themes?
- Are hover states and focus indicators clear and consistent?

## Responsiveness {#responsiveness}

- Does the UI scale gracefully on:
  - Narrow panes (e.g., side-by-side split views)?
  - Short panes (e.g., laptops with 13" displays)?
  - High-DPI / Retina displays?
- Does resizing panes or windows keep the UI usable and attractive?
- Do dialogs or modals stay centered and within viewport bounds?

## Platform Consistency {#platform-consistency}

- Is the feature fully usable on Windows, Linux, and macOS?
- Does it respect system-level settings (fonts, scaling, input methods)?

## Performance {#performance}

- All user interactions must have instant feedback.
  - If the user requests something slow (e.g. an LLM generation) there should be some indication of the work in progress.
- Does it handle large files, big projects, or heavy workloads without degrading?
- Frames must take no more than 8ms (120fps)

## Consistency {#consistency}

- Does it match Zed’s design language (spacing, typography, icons)?
  - Make sure to visit [the icon design guidelines](https://github.com/zed-industries/zed/blob/main/crates/icons/README.md)
- Are terminology, labels, and tone consistent with the rest of Zed?
- Are interactions consistent (e.g., how tabs close, how modals dismiss, how errors show)?

## Internationalization & Text {#internationalization-text}

- Are strings concise, clear, and unambiguous?
- Do we avoid internal Zed jargon that only insiders would know?

## User Paths & Edge Cases {#user-paths-edge-cases}

- What does the happy path look like?
- What does the unhappy path look like? (errors, rejections, invalid states)
- How does it work in offline vs. online states?
- How does it work in unauthenticated vs. authenticated states?
- How does it behave if data is missing, corrupted, or delayed?
- Are error messages actionable and consistent with Zed’s voice?

## Discoverability & Learning {#discoverability-learning}

- Can a first-time user figure it out without docs?
- Is there an intuitive way to undo/redo actions?
- Are power features discoverable but not intrusive?
- Is there a path from beginner → expert usage (progressive disclosure)?

See the [contribution guidelines](https://github.com/zed-industries/zed/blob/main/CONTRIBUTING.md) for preparing your pull request.
