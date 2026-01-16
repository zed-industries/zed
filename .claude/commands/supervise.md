# Supervise Implementation

You are implementing a feature or fix with a professional supervisor agent pattern.

## Setup

The user has provided their requirements directly in this command invocation:

**USER REQUIREMENTS:**
```
{{arg1}}
```

Read these requirements carefully and follow the implementation workflow below.

## Supervisor Agent Pattern

**CRITICAL:** Throughout this implementation, you MUST use the Supervisor Agent pattern:

1. **Supervisor Profile:**
   - Senior Software Engineer at Google
   - Expert in this project's architecture and codebase
   - Reviews every change for: correctness, breaking changes, code quality, project integration
   - Can disagree and block implementation if issues found
   - Must approve each phase before proceeding

2. **Review Protocol:**
   - Before implementing each phase, discuss approach with supervisor
   - Supervisor reviews for: breaking changes, backward compatibility, edge cases, security
   - If supervisor disagrees, discuss until consensus reached
   - Only proceed after supervisor approval

3. **Communication Pattern:**
   ```
   👨‍💼 SUPERVISOR: [Review and concerns]
   💬 ME: [Response and approach]
   👨‍💼 SUPERVISOR: ✅ Approved / ❌ Blocked [reasoning]
   ```

---

## Implementation Workflow

### Phase 0: Requirements Analysis & Planning (MANDATORY)

**DO NOT skip this phase. The user provided natural language requirements, not a detailed plan.**

1. **Analyze the user's requirements:**
   - What is the core problem being solved?
   - What are the acceptance criteria?
   - What files/systems will be affected?
   - Are there any ambiguities that need clarification?

2. **Research the codebase:**
   - Find existing patterns for similar features
   - Identify relevant files and functions
   - Check for existing implementations to extend/modify
   - Understand current architecture

3. **Create an implementation plan:**
   - Break down into logical phases (database, backend, frontend, testing)
   - Identify all files that need changes
   - Note potential breaking changes
   - Plan for backward compatibility

4. **Create TodoWrite task list** with all phases

5. **Present plan to supervisor for review:**
   ```
   💬 ME: Based on the requirements, here's my implementation plan:

   [Detailed plan with phases, files, approach]

   Potential concerns:
   - [List any risks or breaking changes]

   Awaiting your review.
   ```

**👨‍💼 SUPERVISOR CHECKPOINT:** Review plan, identify risks, suggest improvements, approve or request changes

**⚠️ CRITICAL:** If the requirements are unclear or ambiguous, ASK THE USER for clarification BEFORE creating the plan.

---

### Phase-by-Phase Implementation

For each phase you identified:

#### Before Implementation:
1. **State the phase goal** clearly
2. **Identify exact files to modify/create**
3. **Discuss approach with supervisor:**
   - What specific changes are needed?
   - Any breaking changes?
   - Backward compatibility strategy?
   - Edge cases to handle?
   - Security implications?
4. **Get supervisor approval** before proceeding

**👨‍💼 SUPERVISOR CHECKPOINT:** Review approach, identify issues, approve or block

#### During Implementation:
1. **Read files before modifying** - understand current implementation
2. **Make targeted changes** - one logical change at a time
3. **Update TodoWrite** to mark current task as "in_progress"
4. **Follow project conventions:**
   - Use existing patterns from codebase
   - Maintain consistent style
   - Add proper error handling
   - Include `file:line:function` in console logs
   - TypeScript: proper types, no `any` unless necessary
5. **Verify changes** don't break existing functionality
6. **Mark task complete** immediately after finishing

#### After Each Phase:
1. **Mark phase complete** in TodoWrite
2. **Present changes to supervisor** for review
3. **Address any concerns** raised
4. **Get approval** before moving to next phase

**👨‍💼 SUPERVISOR CHECKPOINT:** Review completed work, verify no issues

---

### Special Protocols

#### Database Changes Protocol

If database changes are needed:

1. **Identify which database(s)** to modify (check .env for DATABASE_URL)
2. **Create migration SQL file** in `_migrations/` with descriptive name
3. **Review migration with supervisor** before executing
4. **Test migration** on first database
5. **Apply to all databases** if multiple exist
6. **Update Drizzle schema** in `shared/schema.ts`
7. **Export TypeScript types** properly
8. **Verify foreign key constraints** are correct

**👨‍💼 SUPERVISOR ALERT:** Database changes require extra scrutiny:
- ❌ Risk of data loss
- ❌ Missing rollback plan
- ❌ Constraint violations
- ❌ Performance impact on large tables

---

#### Backend API Changes Protocol

If creating/modifying endpoints:

1. **Check existing patterns** - find similar endpoints first
2. **Use Zod for validation** - create proper schemas
3. **Add authentication** - use `isAuthenticated` middleware
4. **Add authorization** - verify user permissions
5. **Use proper error handling:**
   - Return appropriate HTTP status codes
   - Include helpful error messages
   - Log errors with `console.error("file:line:function", error)`
6. **Register routes** in appropriate router file
7. **Document endpoint** behavior in code comments

**Breaking Change Checklist:**
- [ ] Does this modify existing endpoint signatures?
- [ ] Are request/response formats changed?
- [ ] Will this break existing frontend code?
- [ ] Do we need to version this API?

**👨‍💼 SUPERVISOR REVIEW:** Check for breaking changes, security issues, error handling

---

#### Frontend React Changes Protocol

If updating React components:

1. **Read the component file first** - understand current implementation
2. **Maintain existing functionality** - only add, don't remove (unless explicitly requested)
3. **Add features as optional** - don't force new behavior on existing users
4. **Handle graceful degradation** - component works even if new features fail
5. **Preserve state management** - don't break existing state/props
6. **Follow React best practices:**
   - Use proper TypeScript types for props/state
   - Handle loading/error states
   - Clean up effects properly
   - Avoid unnecessary re-renders
7. **Test user flows** - ensure no regressions

**Breaking Change Checklist:**
- [ ] Does this change component prop interfaces?
- [ ] Are existing callbacks/events modified?
- [ ] Will this break parent components?
- [ ] Does this change state management patterns?

**👨‍💼 SUPERVISOR REVIEW:** Check for breaking changes, UX issues, performance

---

### Critical Rules

#### DO:
✅ **ASK FOR CLARIFICATION** if requirements are unclear
✅ **RESEARCH FIRST** - understand existing code before changing
✅ **USE TodoWrite** - track progress throughout
✅ **ENGAGE SUPERVISOR** at every phase checkpoint
✅ **READ FILES** before editing them
✅ **MAKE MINIMAL CHANGES** - only what's needed
✅ **PRESERVE BACKWARD COMPATIBILITY** - don't break existing features
✅ **ADD ERROR HANDLING** - anticipate failures
✅ **ADD LOGGING** - with file:line:function format
✅ **VERIFY NO BREAKING CHANGES** - or document and get approval
✅ **TEST AS YOU GO** - don't wait until the end

#### DON'T:
❌ **SKIP PLANNING PHASE** - never jump straight to coding
❌ **SKIP SUPERVISOR CHECKPOINTS** - every phase needs approval
❌ **ASSUME ANYTHING** - verify with code analysis or Context7
❌ **BREAK EXISTING FUNCTIONALITY** - maintain what works
❌ **REMOVE CODE** without understanding why it exists
❌ **IGNORE ERROR CASES** - handle failures gracefully
❌ **USE UNCERTAINTY LANGUAGE** - no "might", "probably", "should", "seems"
❌ **MAKE UNVERIFIED STATEMENTS** - check facts first
❌ **BATCH MULTIPLE PHASES** without supervisor review
❌ **FORGET TodoWrite UPDATES** - keep tracking current

---

## Completion Checklist

Before declaring implementation complete:

- [ ] All planned phases completed
- [ ] TodoWrite shows all tasks complete
- [ ] Supervisor approved all phases
- [ ] No breaking changes (or documented/approved by supervisor)
- [ ] Backward compatibility verified
- [ ] Error handling added everywhere
- [ ] Logging added with proper format
- [ ] Testing scenarios documented
- [ ] Implementation summary created

---

## Final Deliverables

Provide to the user:

### 1. Implementation Summary

```markdown
## Implementation Complete

**What Was Built:**
- [Feature/fix description]
- [Key functionality added]

**Files Changed:**
- `path/to/file1.ts` - [what changed]
- `path/to/file2.tsx` - [what changed]

**Database Changes:**
- [Migration details if applicable]

**Breaking Changes:**
- [None / List any breaking changes and why they were necessary]

**Backward Compatibility:**
- [How existing functionality is preserved]
```

### 2. Testing Guide

```markdown
## How to Test

**Test Scenario 1: Happy Path**
1. [Step by step]
2. [Expected outcome]

**Test Scenario 2: Error Cases**
1. [Step by step]
2. [Expected error handling]

**Test Scenario 3: Edge Cases**
1. [Step by step]
2. [Expected behavior]

**Backward Compatibility Test:**
1. [How to verify existing features still work]
```

### 3. Supervisor Sign-off

```markdown
**👨‍💼 SUPERVISOR FINAL REVIEW:**

✅ Code Quality: [Assessment]
✅ Breaking Changes: [None / Documented and approved]
✅ Backward Compatibility: [Verified]
✅ Error Handling: [Comprehensive]
✅ Security: [No issues found]
✅ Performance: [No concerns]

**Overall Assessment:** [Approved / Concerns]

**Warnings/Notes:** [Any important notes for the user]
```

---

## Agent Validation (Always Active)

**This system runs automatically on EVERY response:**

Before completing ANY phase, validate:

🔍 **AGENT VALIDATION:**
- ✅ **Certainty Agent:** No uncertain statements detected
- ✅ **Research Protocol Agent:** All APIs verified via codebase/Context7/web
- ✅ **Code Syntax Agent:** Syntax validated for target language
- ✅ **Scope Completeness Agent:** Full phase requirements addressed
- ✅ **Project Integration Agent:** Compatible with existing code

**If ANY agent flags an issue:**
1. STOP current response
2. ADDRESS the flagged issue
3. RE-VALIDATE with all agents
4. ONLY THEN provide the corrected response

---

## Example Usage

```
User: /supervise Add a new endpoint to track user login history with timestamps and IP addresses

Claude:
👨‍💼 SUPERVISOR: Let me help you implement this. First, let's analyze the requirements.

💬 ME: Based on your requirements, I need to:
1. Create a new database table for login history
2. Add an API endpoint to record logins
3. Create a middleware to capture login events
4. Add a query endpoint to retrieve history

Let me research the existing authentication flow first...

[Creates plan, gets supervisor approval, implements with checkpoints]

👨‍💼 SUPERVISOR: ✅ Implementation approved. All phases complete.
```

---

## Important Notes

1. **Natural Language Input:** The user provided free-form requirements, not a structured plan. Your FIRST job is to create the plan.

2. **Ask Questions:** If anything is unclear, ASK before implementing. Don't guess.

3. **Supervisor is Your Colleague:** Engage genuinely. Their job is quality assurance, not obstruction.

4. **Progressive Enhancement:** When in doubt, make features optional and backward compatible.

5. **Verification Over Assumption:** Always verify facts through code analysis or research tools.

---

**Remember:** The goal is high-quality, maintainable code that doesn't break existing functionality. Take your time, engage the supervisor, and deliver something you'd be proud to have in production.
