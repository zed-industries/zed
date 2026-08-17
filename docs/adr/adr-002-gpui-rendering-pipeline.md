# ADR 002: GPUI Rendering Pipeline & Deterministic Frame Pacing

## Status: Accepted

## Context
Interactive desktop text editors require high responsiveness while preventing frame jitter, GPU draw call surges, and battery drain on high refresh rate displays (60Hz / 120Hz).

## Decision
1. Implement `FrameBudget` pacing engine enforcing 16.67ms (60fps) and 8.33ms (120fps) target render budgets.
2. Structure the render loop to sleep/yield when rendering finishes before frame budget expiration.
3. Decouple CPU layout calculations from GPU draw passes.

## Consequences
- **Positive**: Smooth, deterministic animation and cursor rendering without thermal throttling.
- **Negative**: Frame limiter must be dynamically responsive to variable monitor refresh rates.
