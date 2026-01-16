# Implement Plan with Supervisor Agent

You are implementing a technical plan with a professional supervisor agent pattern.

## Setup

The user has provided a plan file at: {{arg1}}

Read this plan file and follow the implementation workflow below.

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

## Implementation Workflow

### Phase 0: Analysis & Planning

1. **Read the plan file** from the provided path
2. **Analyze the plan:**
   - Identify all phases/steps
   - List files that will be modified
   - Identify potential breaking changes
   - Note dependencies between phases
3. **Create TodoWrite task list** with all phases broken down
4. **Present to supervisor** for initial review

**👨‍💼 SUPERVISOR CHECKPOINT:** Review plan analysis, identify risks, approve phases

---

### Phase-by-Phase Implementation

For each phase in the plan:

#### Before Implementation:
1. **State the phase goal** clearly
2. **Identify files to modify**
3. **Discuss approach with supervisor:**
   - What changes are needed?
   - Any breaking changes?
   - Backward compatibility strategy?
   - Edge cases to handle?
4. **Get supervisor approval** before proceeding

**👨‍💼 SUPERVISOR CHECKPOINT:** Review approach, identify issues, approve or block

#### During Implementation:
1. **Make targeted changes** - one logical change at a time
2. **Update TodoWrite** to mark current task as "in_progress"
3. **Follow project conventions:**
   - Use existing patterns from codebase
   - Maintain consistent style
   - Add proper error handling
   - Include file:line:function in console logs
4. **Verify changes** don't break existing functionality

#### After Implementation:
1. **Mark phase complete** in TodoWrite
2. **Present changes to supervisor** for review
3. **Address any concerns** raised
4. **Get approval** before moving to next phase

**👨‍💼 SUPERVISOR CHECKPOINT:** Review completed work, verify no issues

---

### Database Changes Protocol

If the plan includes database changes:

1. **Check both databases** if plan mentions multiple DB connections
2. **Create migration SQL file** in `_migrations/` directory
3. **Test migration** on first database
4. **Apply to all databases** mentioned in plan
5. **Update Drizzle schema** in `shared/schema.ts`
6. **Export TypeScript types**

**⚠️ SUPERVISOR ALERT:** Database changes require extra scrutiny for:
- Data loss risks
- Migration rollback plan
- Constraint violations
- Performance impact

---

### Backend Changes Protocol

If creating new endpoints:

1. **Follow existing patterns** - check similar endpoints first
2. **Create proper validation** using Zod schemas
3. **Add authentication/authorization** where needed
4. **Use proper error handling** with descriptive messages
5. **Add logging** with file:line:function format
6. **Register routes** in appropriate router files

**Breaking Change Check:**
- Does this modify existing endpoint signatures?
- Are request/response formats changed?
- Will this break existing frontend code?

---

### Frontend Changes Protocol

If updating React components:

1. **Maintain existing functionality** - only add, don't remove
2. **Add optional features** - don't force new behavior
3. **Handle graceful degradation** - work without new features
4. **Preserve state management** - don't break existing state
5. **Test user flows** - ensure no regressions

**Breaking Change Check:**
- Does this change prop interfaces?
- Are existing callbacks modified?
- Will this break existing user flows?

---

### Testing Protocol

After all phases complete:

1. **Create test scenarios** based on plan requirements
2. **Document how to test:**
   - Happy path
   - Error cases
   - Edge cases
   - Backward compatibility
3. **Identify manual testing steps** for user
4. **Note any automated test gaps**

---

## Critical Rules

### DO:
✅ Use TodoWrite to track progress throughout
✅ Engage supervisor at EVERY phase checkpoint
✅ Read files before editing them
✅ Make targeted, minimal changes
✅ Preserve backward compatibility
✅ Add proper error handling and logging
✅ Document breaking changes clearly
✅ Test as you go

### DON'T:
❌ Skip supervisor checkpoints
❌ Make changes without reading plan
❌ Break existing functionality
❌ Remove code without understanding it
❌ Ignore error cases
❌ Skip backward compatibility checks
❌ Assume - always verify with code analysis
❌ Batch multiple phases without supervisor review

---

## Completion Checklist

Before declaring implementation complete:

- [ ] All phases from plan completed
- [ ] TodoWrite shows all tasks complete
- [ ] Supervisor approved all phases
- [ ] No breaking changes (or documented/approved)
- [ ] Backward compatibility verified
- [ ] Error handling added
- [ ] Logging added with proper format
- [ ] Testing scenarios documented
- [ ] Implementation summary created

---

## Final Deliverables

Provide to the user:

1. **Implementation Summary:**
   - What was implemented
   - Files changed
   - Breaking changes (if any)
   - How to test

2. **Testing Guide:**
   - Step-by-step test scenarios
   - Expected outcomes
   - Edge cases to verify

3. **Supervisor Sign-off:**
   - Final supervisor review
   - Any concerns or warnings
   - Approval statement

---

## Agent Validation (Always Active)

Before completing ANY phase:

🔍 **AGENT VALIDATION:**
✅ Certainty: No uncertain statements (no "might", "probably", "seems")
✅ Research: All APIs and patterns verified in codebase or via Context7/web
✅ Code Syntax: Syntax validated for target language
✅ Scope Completeness: Full phase requirements addressed
✅ Project Integration: Compatible with existing code

---

## Example Usage

User will invoke this command as:
```
/implement-plan path/to/plan.md
```

You will:
1. Read the plan at the provided path
2. Activate supervisor agent pattern
3. Work through phases with supervisor approval
4. Track progress with TodoWrite
5. Deliver comprehensive summary

**Remember:** The supervisor is your colleague, not your adversary. Their job is to ensure quality and prevent issues. Engage them actively and genuinely consider their feedback.
