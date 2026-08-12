# OMP-only, local-only first release

The first release ships a single OMP profile and local terminals only. Remote/WSL sessions, inferred recovery from arbitrary foreground programs, and other harness adapters (Claude Code, Codex, Gemini) are explicit non-goals. The resume-argv builder is structured as a per-agent function so more adapters can be added later.

**Considered:** a full adapter registry and remote support in the first release — deferred because each harness needs a reliable session-identity source before it can be added; resume flags alone are insufficient, and remote complicates resume-path existence across filesystems.