# 🔍 Complete Analysis: Issue #[NUMBER] - [TITLE]

**Analysis Date**: [DATE]  
**Analyzed by**: [ANALYST NAME]  
**Issue**: [#NUMBER](https://github.com/zed-industries/zed/issues/[NUMBER])  
**Status**: [🔴 OPEN | 🟢 CLOSED | 🟡 IN PROGRESS]  
**Priority**: [🚨 P0 - CRITICAL | ⚠️ P1 - HIGH | 📌 P2 - MEDIUM | 📋 P3 - LOW]

---

## 📋 Table of Contents

1. [Executive Summary](#executive-summary)
2. [Issue Information](#issue-information)
3. [Collected Data](#collected-data)
4. [Missing Information](#missing-information)
5. [Validation Checklist](#validation-checklist)
6. [Technical Analysis](#technical-analysis)
7. [Recommended Actions](#recommended-actions)
8. [Timeline and Next Steps](#timeline-and-next-steps)
9. [References](#references)

---

## 📊 Executive Summary

### Problem Overview

**One-Line Summary**: [Brief description of the issue]

**Key Metrics**:
- **Severity**: [Critical/High/Medium/Low]
- **Impact**: [Number of users affected or potential impact]
- **Platform**: [macOS/Windows/Linux/All]
- **Component**: [Editor/LSP/UI/Performance/etc]

### Impact Assessment

```
USER IMPACT:
□ Blocks core functionality
□ Causes data loss
□ Severe performance degradation
□ UI/UX inconvenience
□ Minor cosmetic issue

SCOPE:
□ All users
□ Specific platform (specify)
□ Specific configuration
□ Edge case
```

### Urgency Classification

```
PRIORITY MATRIX:

Severity:    [1-5] ████░
Frequency:   [1-5] ███░░
User Impact: [1-5] █████

OVERALL: [P0/P1/P2/P3]
```

---

## 🐛 Issue Information

### Basic Details

| Field | Value |
|-------|-------|
| **Issue Number** | #[NUMBER] |
| **Title** | [TITLE] |
| **Reporter** | [@USERNAME] ([Display Name]) |
| **Date Reported** | [DATE] |
| **Labels** | [label1, label2, label3] |
| **Assignees** | [None / @assignee1, @assignee2] |
| **Status** | [OPEN/CLOSED/IN_PROGRESS] |
| **Milestone** | [None / Milestone name] |
| **Related Issues** | [#123, #456] |

### System Information

```yaml
# Fill in based on issue report
Operating System: [macOS/Windows/Linux] [VERSION]
Architecture: [x86_64/aarch64/other]
Memory: [XX GB]
CPU: [Model and core count]
GPU: [If relevant]

Zed Version: [vX.X.X]
Commit Hash: [hash]
Release Channel: [stable/preview/nightly]
Installation Method: [dmg/homebrew/apt/build from source]
```

### Problem Description

**Original Report**:
> [Quote the user's original problem description]

**Steps to Reproduce**:
1. [Step 1]
2. [Step 2]
3. [Step 3]
4. [...]

**Expected Behavior**: 
[What should happen]

**Actual Behavior**:
[What actually happens]

**Frequency**:
- [ ] Always (100%)
- [ ] Often (>50%)
- [ ] Sometimes (10-50%)
- [ ] Rarely (<10%)
- [ ] Once

### Evidence Provided

#### Visual Evidence
- [ ] Screenshots attached
- [ ] Video demonstration
- [ ] GIF/animated demonstration
- [ ] Activity Monitor/Task Manager capture
- [ ] Process sample/dump

#### Logs and Diagnostics
- [ ] Application logs
- [ ] System logs
- [ ] Crash reports
- [ ] Debug output
- [ ] Network traces

#### Configuration
- [ ] settings.json provided
- [ ] Extension list
- [ ] Keymap configuration
- [ ] Environment variables

---

## ✅ Collected Data

### Information Present

Use this checklist to mark what information HAS been provided:

#### 1. System Information ⬜
- [ ] Operating system and version
- [ ] System architecture
- [ ] RAM amount
- [ ] CPU model and cores
- [ ] GPU information
- [ ] Disk space
- [ ] Display configuration

**Status**: [0-100]% Complete  
**Quality**: [🟢 Excellent | 🟡 Adequate | 🔴 Insufficient]

---

#### 2. Application Information ⬜
- [ ] Exact Zed version
- [ ] Commit hash
- [ ] Release channel
- [ ] Installation method
- [ ] Update history
- [ ] Previous working version

**Status**: [0-100]% Complete  
**Quality**: [🟢 Excellent | 🟡 Adequate | 🔴 Insufficient]

---

#### 3. Problem Reproduction ⬜
- [ ] Clear steps to reproduce
- [ ] Expected behavior described
- [ ] Actual behavior described
- [ ] Minimal reproducible example
- [ ] Frequency information
- [ ] Specific conditions required
- [ ] Reproducible by others

**Status**: [0-100]% Complete  
**Quality**: [🟢 Excellent | 🟡 Adequate | 🔴 Insufficient]

---

#### 4. Evidence and Documentation ⬜
- [ ] Screenshots
- [ ] Videos
- [ ] Logs
- [ ] Error messages
- [ ] Stack traces
- [ ] Performance metrics
- [ ] Network captures

**Status**: [0-100]% Complete  
**Quality**: [🟢 Excellent | 🟡 Adequate | 🔴 Insufficient]

---

#### 5. Configuration Details ⬜
- [ ] settings.json
- [ ] Installed extensions
- [ ] Active theme
- [ ] Keymap customizations
- [ ] Language servers
- [ ] Feature flags
- [ ] Environment variables

**Status**: [0-100]% Complete  
**Quality**: [🟢 Excellent | 🟡 Adequate | 🔴 Insufficient]

---

#### 6. Workspace/Project Context ⬜
- [ ] Project size (file count)
- [ ] Total disk usage
- [ ] Programming languages
- [ ] Git repository info
- [ ] Directory structure
- [ ] Build system
- [ ] Dependencies (node_modules, etc)

**Status**: [0-100]% Complete  
**Quality**: [🟢 Excellent | 🟡 Adequate | 🔴 Insufficient]

---

#### 7. Diagnostic Tests Performed ⬜
- [ ] Tested with clean config
- [ ] Tested without extensions
- [ ] Tested with minimal project
- [ ] Tested in safe mode
- [ ] Version comparison
- [ ] Fresh installation test
- [ ] Different user account test

**Status**: [0-100]% Complete  
**Quality**: [🟢 Excellent | 🟡 Adequate | 🔴 Insufficient]

---

#### 8. Community Interaction ⬜
- [ ] Reporter responsive
- [ ] Additional info provided
- [ ] Maintainer acknowledged
- [ ] Other users confirmed
- [ ] Workarounds shared
- [ ] Similar issues linked

**Status**: [0-100]% Complete  
**Quality**: [🟢 Excellent | 🟡 Adequate | 🔴 Insufficient]

---

### Overall Data Quality Score

```
┌─────────────────────────────────────────┐
│ DATA COMPLETENESS DASHBOARD             │
├─────────────────────────────────────────┤
│                                         │
│  System Info:          [█████░░░░░] 50% │
│  App Info:             [███████░░░] 70% │
│  Reproduction:         [████░░░░░░] 40% │
│  Evidence:             [███░░░░░░░] 30% │
│  Configuration:        [█░░░░░░░░░] 10% │
│  Project Context:      [░░░░░░░░░░]  0% │
│  Diagnostic Tests:     [░░░░░░░░░░]  0% │
│  Community:            [██████░░░░] 60% │
│                                         │
│  OVERALL SCORE:        [███░░░░░░░] 33% │
│                                         │
│  STATUS: 🔴 INSUFFICIENT                │
│  ACTION: COLLECT MORE DATA              │
│                                         │
└─────────────────────────────────────────┘
```

---

## ❌ Missing Information

### Critical Missing Data

List everything that is MISSING and CRITICAL for diagnosis:

#### 1. Essential System Information
```markdown
MISSING:
- [ ] [Specific missing item 1]
- [ ] [Specific missing item 2]
- [ ] [Specific missing item 3]

WHY IMPORTANT:
[Explain why this information is critical]

HOW TO COLLECT:
[Provide specific instructions]
```

#### 2. Application Configuration
```markdown
MISSING:
- [ ] [Specific missing item 1]
- [ ] [Specific missing item 2]
- [ ] [Specific missing item 3]

WHY IMPORTANT:
[Explain why this information is critical]

HOW TO COLLECT:
```bash
# Example commands
cat ~/.config/zed/settings.json
```

#### 3. Diagnostic Data
```markdown
MISSING:
- [ ] [Specific missing item 1]
- [ ] [Specific missing item 2]
- [ ] [Specific missing item 3]

WHY IMPORTANT:
[Explain why this information is critical]

HOW TO COLLECT:
[Provide specific instructions]
```

#### 4. Reproduction Context
```markdown
MISSING:
- [ ] [Specific missing item 1]
- [ ] [Specific missing item 2]
- [ ] [Specific missing item 3]

WHY IMPORTANT:
[Explain why this information is critical]

HOW TO COLLECT:
[Provide specific instructions]
```

### Important But Not Critical

List information that would be helpful but isn't blocking:

```markdown
WOULD BE HELPFUL:
- [ ] [Item 1]
- [ ] [Item 2]
- [ ] [Item 3]

REASON:
[Why this would help]
```

---

## ✅ Validation Checklist

### Complete Bug Report Validation

Use this comprehensive checklist to validate the issue report:

#### Category 1: System Information
- [ ] 1.1. Operating system name and version
- [ ] 1.2. System architecture (x86_64/aarch64)
- [ ] 1.3. Total RAM
- [ ] 1.4. CPU model and specifications
- [ ] 1.5. GPU information (if relevant)
- [ ] 1.6. Available disk space
- [ ] 1.7. Display resolution and DPI
- [ ] 1.8. System locale and language

**Score**: ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜ [X/8]  
**Status**: [🟢 Complete | 🟡 Partial | 🔴 Missing]

---

#### Category 2: Application Information
- [ ] 2.1. Exact Zed version number
- [ ] 2.2. Commit hash or build number
- [ ] 2.3. Release channel (stable/preview/nightly)
- [ ] 2.4. Installation method
- [ ] 2.5. Installation date
- [ ] 2.6. Update history
- [ ] 2.7. Previous working version (if regression)
- [ ] 2.8. Installation location

**Score**: ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜ [X/8]  
**Status**: [🟢 Complete | 🟡 Partial | 🔴 Missing]

---

#### Category 3: Problem Reproduction
- [ ] 3.1. Clear, numbered steps to reproduce
- [ ] 3.2. Expected behavior explicitly stated
- [ ] 3.3. Actual behavior explicitly stated
- [ ] 3.4. Frequency of occurrence
- [ ] 3.5. Minimal reproducible example
- [ ] 3.6. Specific conditions required
- [ ] 3.7. Confirmed by other users
- [ ] 3.8. Works in different environment

**Score**: ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜ [X/8]  
**Status**: [🟢 Complete | 🟡 Partial | 🔴 Missing]

---

#### Category 4: Visual Evidence
- [ ] 4.1. Screenshot of the problem
- [ ] 4.2. Video demonstration (if applicable)
- [ ] 4.3. Before/after comparison
- [ ] 4.4. UI state captured
- [ ] 4.5. Error messages visible
- [ ] 4.6. System monitors (CPU/Memory)
- [ ] 4.7. Timeline or sequence diagram
- [ ] 4.8. Annotated screenshots

**Score**: ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜ [X/8]  
**Status**: [🟢 Complete | 🟡 Partial | 🔴 Missing]

---

#### Category 5: Configuration
- [ ] 5.1. Complete settings.json
- [ ] 5.2. List of installed extensions
- [ ] 5.3. Extension versions
- [ ] 5.4. Custom keymap (if any)
- [ ] 5.5. Active theme
- [ ] 5.6. Configured language servers
- [ ] 5.7. Feature flags enabled
- [ ] 5.8. Environment variables

**Score**: ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜ [X/8]  
**Status**: [🟢 Complete | 🟡 Partial | 🔴 Missing]

---

#### Category 6: Workspace/Project
- [ ] 6.1. Project size (file count)
- [ ] 6.2. Total disk usage
- [ ] 6.3. Primary programming languages
- [ ] 6.4. Git repository details
- [ ] 6.5. Directory structure overview
- [ ] 6.6. Build system used
- [ ] 6.7. Dependencies (node_modules, vendor, etc)
- [ ] 6.8. Remote connections active

**Score**: ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜ [X/8]  
**Status**: [🟢 Complete | 🟡 Partial | 🔴 Missing]

---

#### Category 7: Isolation Tests
- [ ] 7.1. Tested with empty/minimal project
- [ ] 7.2. Tested without extensions
- [ ] 7.3. Tested with default configuration
- [ ] 7.4. Tested in safe mode
- [ ] 7.5. Tested with fresh installation
- [ ] 7.6. Tested in different user account
- [ ] 7.7. Compared with previous version
- [ ] 7.8. Tested on different machine

**Score**: ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜ [X/8]  
**Status**: [🟢 Complete | 🟡 Partial | 🔴 Missing]

---

#### Category 8: Logs and Diagnostics
- [ ] 8.1. Application logs attached
- [ ] 8.2. System logs captured
- [ ] 8.3. Crash reports (if any)
- [ ] 8.4. Debug output included
- [ ] 8.5. Performance profiling data
- [ ] 8.6. Network traces (if relevant)
- [ ] 8.7. LSP logs (if relevant)
- [ ] 8.8. Extension logs

**Score**: ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜ [X/8]  
**Status**: [🟢 Complete | 🟡 Partial | 🔴 Missing]

---

#### Category 9: Error Messages
- [ ] 9.1. Complete error message text
- [ ] 9.2. Error codes or identifiers
- [ ] 9.3. Stack traces (if any)
- [ ] 9.4. Context when error occurred
- [ ] 9.5. Error frequency
- [ ] 9.6. Related warnings
- [ ] 9.7. Console output
- [ ] 9.8. Popup messages

**Score**: ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜ [X/8]  
**Status**: [🟢 Complete | 🟡 Partial | 🔴 Missing]

---

#### Category 10: Attempted Solutions
- [ ] 10.1. Workarounds tested
- [ ] 10.2. Version rollback attempted
- [ ] 10.3. Configuration reset tried
- [ ] 10.4. Reinstallation attempted
- [ ] 10.5. Cache/data cleared
- [ ] 10.6. Searched existing issues
- [ ] 10.7. Checked documentation
- [ ] 10.8. Results documented

**Score**: ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜ [X/8]  
**Status**: [🟢 Complete | 🟡 Partial | 🔴 Missing]

---

### Validation Summary

```
┌─────────────────────────────────────────────────────┐
│         VALIDATION CHECKLIST SUMMARY                │
├─────────────────────────────────────────────────────┤
│                                                     │
│  Category 1: System Info           [X/8] [███░░░░] │
│  Category 2: App Info              [X/8] [███░░░░] │
│  Category 3: Reproduction          [X/8] [███░░░░] │
│  Category 4: Visual Evidence       [X/8] [███░░░░] │
│  Category 5: Configuration         [X/8] [███░░░░] │
│  Category 6: Workspace             [X/8] [███░░░░] │
│  Category 7: Isolation Tests       [X/8] [███░░░░] │
│  Category 8: Logs                  [X/8] [███░░░░] │
│  Category 9: Error Messages        [X/8] [███░░░░] │
│  Category 10: Solutions Tried      [X/8] [███░░░░] │
│                                                     │
│  ─────────────────────────────────────────────────  │
│  TOTAL SCORE:                    [XX/80]           │
│  PERCENTAGE:                     XX%               │
│                                                     │
│  GRADE: [🟢 A | 🟢 B | 🟡 C | 🔴 D | 🔴 F]        │
│                                                     │
│  RECOMMENDATION:                                    │
│  [Action needed based on score]                    │
│                                                     │
└─────────────────────────────────────────────────────┘

GRADING SCALE:
90-100%: 🟢 A - Excellent, ready for investigation
75-89%:  🟢 B - Good, minor gaps acceptable
60-74%:  🟡 C - Adequate, some critical info missing
45-59%:  🔴 D - Poor, major gaps in information
0-44%:   🔴 F - Insufficient, cannot proceed
```

---

## 🔬 Technical Analysis

### Problem Classification

```
TYPE OF ISSUE:
□ Bug (unexpected behavior)
□ Performance problem
□ Crash/stability issue
□ Feature request
□ Enhancement
□ Documentation
□ UI/UX issue
□ Compatibility problem
□ Security concern
□ Data loss/corruption

COMPONENT:
□ Core editor
□ Language server integration
□ UI/rendering
□ File system operations
□ Git integration
□ Extension system
□ Settings/configuration
□ Build/packaging
□ Network/remote
□ Other: [specify]
```

### Hypotheses and Root Causes

List potential causes ordered by probability:

#### Hypothesis 1: [Most Likely Cause] 🥇
```
PROBABILITY: [High/Medium/Low]

EVIDENCE SUPPORTING:
- [Evidence point 1]
- [Evidence point 2]
- [Evidence point 3]

POSSIBLE ROOT CAUSES:
□ [Specific cause 1]
□ [Specific cause 2]
□ [Specific cause 3]

SUGGESTED TESTS:
→ [Test 1]
→ [Test 2]
→ [Test 3]

EXPECTED OUTCOME:
[What would confirm this hypothesis]
```

#### Hypothesis 2: [Second Most Likely] 🥈
```
PROBABILITY: [High/Medium/Low]

EVIDENCE SUPPORTING:
- [Evidence point 1]
- [Evidence point 2]

POSSIBLE ROOT CAUSES:
□ [Specific cause 1]
□ [Specific cause 2]

SUGGESTED TESTS:
→ [Test 1]
→ [Test 2]

EXPECTED OUTCOME:
[What would confirm this hypothesis]
```

#### Hypothesis 3: [Third Possibility] 🥉
```
PROBABILITY: [High/Medium/Low]

EVIDENCE SUPPORTING:
- [Evidence point 1]

POSSIBLE ROOT CAUSES:
□ [Specific cause 1]

SUGGESTED TESTS:
→ [Test 1]

EXPECTED OUTCOME:
[What would confirm this hypothesis]
```

### Related Issues and PRs

```markdown
SIMILAR ISSUES:
- #[NUMBER]: [Brief description]
- #[NUMBER]: [Brief description]

RELATED PRs:
- #[NUMBER]: [Brief description]
- #[NUMBER]: [Brief description]

DUPLICATES:
- #[NUMBER]: [If this is a duplicate]

DEPENDENCIES:
- Blocks: #[NUMBER]
- Blocked by: #[NUMBER]
```

### Technical Details

```markdown
AFFECTED CODE AREAS:
- [File/module 1]
- [File/module 2]
- [File/module 3]

POTENTIAL COMMITS:
- [commit hash]: [description]
- [commit hash]: [description]

REGRESSION INTRODUCED IN:
- Version: [vX.X.X]
- Commit: [hash]
- PR: #[NUMBER]
```

---

## 🎯 Recommended Actions

### For the Issue Reporter

#### 🚨 Immediate Actions (0-24h)

##### 1. Provide Missing Critical Information

```markdown
PLEASE PROVIDE:

[ ] System Information
    - Exact OS version
    - Hardware specifications
    - [Other specific items]

[ ] Configuration Files
    ```bash
    # Export your settings
    cat ~/.config/zed/settings.json > zed_settings.json
    # Attach zed_settings.json to issue
    ```

[ ] Reproduction Steps
    - Detailed step-by-step instructions
    - Minimal example if possible

[ ] Logs and Diagnostics
    ```bash
    # Collect logs (platform-specific commands)
    [Commands to collect logs]
    ```
```

##### 2. Isolation Tests

```markdown
PLEASE TEST AND REPORT RESULTS:

Test 1: Clean Configuration
□ Backup config: mv ~/.config/zed ~/.config/zed.backup
□ Start Zed fresh
□ Does problem persist? [Yes/No]
□ Result: [Describe]

Test 2: Minimal Project
□ Create empty project
□ Open in Zed
□ Does problem persist? [Yes/No]
□ Result: [Describe]

Test 3: Disable Extensions
□ Disable all extensions
□ Restart Zed
□ Does problem persist? [Yes/No]
□ Result: [Describe]

Test 4: [Specific to issue]
□ [Test steps]
□ Does problem persist? [Yes/No]
□ Result: [Describe]
```

##### 3. Document Workarounds

```markdown
IF YOU FIND A WORKAROUND:

Please share:
1. What worked
2. Steps to apply workaround
3. Any limitations
4. Temporary or permanent solution

This helps other users affected by same issue!
```

#### 📋 Template for Additional Information

Copy and fill this in the issue:

```markdown
### Additional Information Requested

#### System Details
- OS: [e.g., macOS 14.5, Windows 11, Ubuntu 24.04]
- Hardware: [e.g., MacBook Pro M2, Dell XPS 15]
- CPU: [Model and cores]
- RAM: [Amount]
- Disk: [Type and available space]

#### Zed Configuration
- Installation: [dmg/homebrew/apt/flatpak/source]
- Extensions: [List or "none"]
- Theme: [Name]
- Language Servers: [List active ones]

#### Project Context
- Size: [~X files, X MB]
- Languages: [Primary languages used]
- Git: [Yes/No, if yes: repository size]
- Special: [node_modules, large binaries, etc.]

#### Test Results

**Test 1: Clean Config**
- Steps: [What I did]
- Result: [What happened]
- Problem persists: [Yes/No]

**Test 2: [Another test]**
- Steps: [What I did]
- Result: [What happened]
- Problem persists: [Yes/No]

#### Logs
- Attached: [List files attached]
- Available upon request: [What else you have]

#### Screenshots/Videos
[Attach or link]
```

---

### For the Zed Team

#### 🚨 Critical Actions (0-48h)

##### 1. Initial Triage

```markdown
TASK: Initial issue assessment

CHECKLIST:
□ Verify issue is reproducible
□ Check for duplicates
□ Assess severity and priority
□ Assign appropriate labels
□ Assign to team member/milestone

RESPONSIBLE: [Name/Role]
DEADLINE: 24 hours from report
```

##### 2. Request Missing Information

```markdown
TASK: Identify and request critical missing data

TEMPLATE RESPONSE:
"Thank you for reporting this issue. To help us investigate,
could you please provide:

1. [Missing item 1]
2. [Missing item 2]
3. [Missing item 3]

[Specific instructions on how to collect]

This information will help us reproduce and fix the issue faster."

RESPONSIBLE: [Name/Role]
DEADLINE: 24 hours from report
```

##### 3. Reproduction Attempt

```markdown
TASK: Attempt to reproduce the issue internally

ENVIRONMENT:
- Platform: [Match reporter's platform]
- Version: [Match reported version]
- Configuration: [Match reported config]

STEPS:
1. Setup matching environment
2. Follow reproduction steps
3. Document results
4. Take screenshots/recordings
5. Collect diagnostics

SUCCESS CRITERIA:
□ Issue reproduced
□ Reproduction documented
□ Diagnostics collected

RESPONSIBLE: [Name/Role]
DEADLINE: 48 hours from report
```

#### 📊 Investigation Phase (48-96h)

##### 4. Technical Analysis

```markdown
TASK: Deep dive technical investigation

ACTIVITIES:
□ Code review of relevant areas
□ Git bisect if regression
□ Profiling/debugging
□ Log analysis
□ Compare with working version

DELIVERABLES:
□ Root cause identified
□ Technical write-up
□ Reproduction rate documented
□ Impact assessment

RESPONSIBLE: [Name/Role]
DEADLINE: 96 hours from report
```

##### 5. Solution Design

```markdown
TASK: Design fix or mitigation

CONSIDERATIONS:
□ Root cause addressed
□ No regressions introduced
□ Performance impact
□ Backwards compatibility
□ Test coverage

DELIVERABLES:
□ Fix approach documented
□ Implementation plan
□ Test plan
□ Timeline estimate

RESPONSIBLE: [Name/Role]
DEADLINE: 96 hours from report
```

#### 🔧 Implementation Phase (Variable)

##### 6. Fix Implementation

```markdown
TASK: Implement and test fix

CHECKLIST:
□ Code implemented
□ Unit tests added
□ Integration tests added
□ Manual testing completed
□ Documentation updated
□ Changelog entry

RESPONSIBLE: [Name/Role]
DEADLINE: [Based on priority]
```

##### 7. Code Review

```markdown
TASK: Review and approve fix

CHECKLIST:
□ Code quality verified
□ Tests adequate
□ No regressions introduced
□ Documentation clear
□ Approved for merge

RESPONSIBLE: [Reviewer name]
DEADLINE: [24-48h after implementation]
```

#### 🚀 Release Phase

##### 8. Release Planning

```markdown
TASK: Plan fix release

DECISION:
□ Hotfix (immediate)
□ Next minor release
□ Next major release
□ Backport to stable

COMMUNICATION:
□ Update issue
□ Notify affected users
□ Changelog entry
□ Release notes

RESPONSIBLE: [Release manager]
```

##### 9. Post-Release Monitoring

```markdown
TASK: Monitor fix effectiveness

CHECKLIST:
□ Verify fix in release
□ Monitor crash reports
□ Monitor user feedback
□ Check for regressions
□ Close issue when confirmed

RESPONSIBLE: [Name/Role]
DURATION: 7 days post-release
```

---

## 📅 Timeline and Next Steps

### Suggested Resolution Timeline

```
┌─────────────────────────────────────────────────────────┐
│                RESOLUTION TIMELINE                      │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  DAY 0: Issue Reported                                 │
│  ├─ ✓ Issue created                                    │
│  ├─ ⏳ Initial triage                                  │
│  ├─ ⏳ Labels assigned                                 │
│  └─ ⏳ Acknowledge receipt                             │
│                                                         │
│  DAY 1: Information Gathering                          │
│  ├─ ⏳ Request missing info                            │
│  ├─ ⏳ Attempt reproduction                            │
│  ├─ ⏳ Check for duplicates                            │
│  └─ ⏳ Assign priority                                 │
│                                                         │
│  DAY 2-3: Initial Investigation                        │
│  ├─ ⏳ Reproduce issue                                 │
│  ├─ ⏳ Collect diagnostics                             │
│  ├─ ⏳ Review code                                     │
│  └─ ⏳ Form hypotheses                                 │
│                                                         │
│  DAY 4-7: Deep Dive Analysis                           │
│  ├─ ⏳ Root cause analysis                             │
│  ├─ ⏳ Git bisect (if regression)                      │
│  ├─ ⏳ Design solution                                 │
│  └─ ⏳ Create implementation plan                      │
│                                                         │
│  DAY 8-14: Implementation                              │
│  ├─ ⏳ Implement fix                                   │
│  ├─ ⏳ Write tests                                     │
│  ├─ ⏳ Code review                                     │
│  └─ ⏳ Integration testing                             │
│                                                         │
│  DAY 15+: Release                                      │
│  ├─ ⏳ Merge to main                                   │
│  ├─ ⏳ Include in release                              │
│  ├─ ⏳ Monitor deployment                              │
│  └─ ⏳ Verify and close issue                          │
│                                                         │
└─────────────────────────────────────────────────────────┘

Note: Timeline varies based on priority and complexity
- P0 (Critical): 0-7 days
- P1 (High): 7-14 days  
- P2 (Medium): 14-30 days
- P3 (Low): 30+ days or future release
```

### Phase Breakdown

#### Phase 1: Intake and Triage (Days 0-1)

**Objective**: Assess and categorize the issue

**Activities**:
- ✅ Issue created by reporter
- ⏳ Maintainer reviews and acknowledges
- ⏳ Labels assigned (bug/feature/priority/component)
- ⏳ Duplicate check performed
- ⏳ Initial priority assigned
- ⏳ Missing information requested

**Success Criteria**:
- Issue properly categorized
- Priority and severity assessed
- Assigned to appropriate team/person
- Reporter knows issue is being looked at

**Blockers**:
- Insufficient information
- Unclear reproduction steps
- Cannot determine severity

---

#### Phase 2: Investigation (Days 2-7)

**Objective**: Understand the problem and identify root cause

**Activities**:
- ⏳ Reproduce issue internally
- ⏳ Collect diagnostics and logs
- ⏳ Review relevant code areas
- ⏳ Git bisect if regression
- ⏳ Profiling/debugging
- ⏳ Form hypotheses
- ⏳ Test hypotheses

**Success Criteria**:
- Issue reproducible internally
- Root cause identified
- Technical understanding documented
- Solution approach defined

**Blockers**:
- Cannot reproduce
- Missing critical information
- Complex/intermittent issue
- Requires specific hardware/setup

---

#### Phase 3: Solution Design (Days 5-10)

**Objective**: Design appropriate fix or mitigation

**Activities**:
- ⏳ Evaluate solution approaches
- ⏳ Consider edge cases
- ⏳ Plan test coverage
- ⏳ Assess performance impact
- ⏳ Check backwards compatibility
- ⏳ Design API changes (if needed)
- ⏳ Document approach

**Success Criteria**:
- Solution approach agreed upon
- Implementation plan created
- Test plan defined
- Timeline estimated

**Blockers**:
- Multiple conflicting solutions
- Breaking changes required
- Unclear requirements
- Needs architectural decision

---

#### Phase 4: Implementation (Days 8-14)

**Objective**: Implement and test the fix

**Activities**:
- ⏳ Code the solution
- ⏳ Write unit tests
- ⏳ Write integration tests
- ⏳ Manual testing
- ⏳ Performance testing
- ⏳ Update documentation
- ⏳ Create changelog entry

**Success Criteria**:
- Fix implemented
- Tests passing
- No regressions
- Documentation updated
- Ready for review

**Blockers**:
- Technical complications
- Test failures
- Performance regressions
- Scope creep

---

#### Phase 5: Review and Merge (Days 12-16)

**Objective**: Review and approve the fix

**Activities**:
- ⏳ Code review
- ⏳ Address feedback
- ⏳ CI/CD validation
- ⏳ Final approval
- ⏳ Merge to main branch

**Success Criteria**:
- Code reviewed and approved
- All checks passing
- Merged to main
- Ready for release

**Blockers**:
- Review feedback requires changes
- CI failures
- Merge conflicts
- Missing approvals

---

#### Phase 6: Release and Verification (Days 15+)

**Objective**: Deploy fix and verify effectiveness

**Activities**:
- ⏳ Include in release
- ⏳ Update release notes
- ⏳ Notify affected users
- ⏳ Monitor deployment
- ⏳ Verify fix with reporter
- ⏳ Watch for regressions
- ⏳ Close issue

**Success Criteria**:
- Fix deployed in release
- Reporter confirms fix
- No new related issues
- Issue closed

**Blockers**:
- Release schedule
- Last-minute issues found
- Reporter cannot verify
- New regressions discovered

---

### Milestone Tracking

```markdown
MILESTONES:

□ Issue Acknowledged (Day 0-1)
  - Issue reviewed by maintainer
  - Initial response to reporter
  - Labels and priority assigned

□ Reproducible (Day 1-3)
  - Issue reproduced internally
  - Diagnostics collected
  - Environment documented

□ Root Cause Identified (Day 3-7)
  - Technical investigation complete
  - Root cause documented
  - Solution approach defined

□ Fix Implemented (Day 7-14)
  - Code written and tested
  - Documentation updated
  - Ready for review

□ Fix Merged (Day 10-16)
  - Code reviewed and approved
  - Merged to main branch
  - Included in release plan

□ Fix Released (Day 14-21)
  - Deployed in release
  - Release notes updated
  - Users notified

□ Verified and Closed (Day 15-28)
  - Reporter confirms fix
  - No regressions observed
  - Issue closed
```

---

### Communication Plan

#### Updates to Issue Reporter

```markdown
COMMUNICATION SCHEDULE:

Initial Response (Day 0-1):
"Thank you for reporting this issue. We're looking into it.
[Request any missing critical information]"

Investigation Update (Day 3-5):
"We've been able to [reproduce/investigate] the issue.
[Share findings or request additional tests]"

Solution Update (Day 7-10):
"We've identified the root cause and are working on a fix.
Expected timeline: [estimate]"

Implementation Update (Day 10-14):
"Fix has been implemented and is under review.
Should be in next [release/hotfix]"

Release Update (Day 14+):
"This has been fixed in version X.X.X.
Please update and let us know if issue persists."

Closure (Day 15-28):
"Closing as fixed. Please reopen if you still experience issues.
Thanks for your report!"
```

#### Internal Team Updates

```markdown
INTERNAL COMMUNICATION:

Daily (for P0):
- Status in standup
- Blocker identification
- Help needed

Weekly (for P1-P2):
- Progress update
- Timeline check
- Resource needs

Monthly (for P3):
- Status review
- Reprioritization decision
```

---

## 📚 References and Resources

### Related Documentation

```markdown
OFFICIAL DOCS:
- [Zed Documentation](https://zed.dev/docs)
- [Contributing Guide](https://github.com/zed-industries/zed/blob/main/CONTRIBUTING.md)
- [Issue Templates](https://github.com/zed-industries/zed/tree/main/.github/ISSUE_TEMPLATE)

PLATFORM-SPECIFIC:
- [macOS Performance Guide](https://developer.apple.com/documentation/xcode/improving-your-app-s-performance)
- [Windows Debugging](https://docs.microsoft.com/en-us/windows/win32/debug/debugging)
- [Linux Performance Tools](https://www.brendangregg.com/linuxperf.html)

ZEDRELATED:
- [Extension API](https://zed.dev/docs/extensions)
- [Theme Guide](https://zed.dev/docs/themes)
- [Language Server Protocol](https://microsoft.github.io/language-server-protocol/)
```

### Similar Issues

```markdown
SEARCH QUERIES:
- [Relevant label combinations]
- [Keywords from this issue]
- [Error messages or symptoms]

KNOWN SIMILAR ISSUES:
- #[NUMBER]: [Brief description]
- #[NUMBER]: [Brief description]
- #[NUMBER]: [Brief description]
```

### Diagnostic Tools

#### macOS
```bash
# System profiling
system_profiler SPHardwareDataType
system_profiler SPSoftwareDataType

# CPU profiling
sudo dtrace -n 'profile-997 /execname == "Zed"/ { @[ustack()] = count(); }'

# Process sampling
sudo sample Zed 30 -file zed-sample.txt

# System calls
sudo dtruss -p [Zed PID]

# File system activity
sudo fs_usage -w -f filesys Zed

# Network activity
sudo nettop -p [Zed PID]

# Memory analysis
leaks Zed
vmmap [Zed PID]

# Crash logs
open ~/Library/Logs/DiagnosticReports/

# Application logs
open ~/Library/Logs/Zed/
```

#### Windows
```powershell
# System information
systeminfo
Get-ComputerInfo

# Process monitoring
Get-Process Zed | Format-List *

# Resource Monitor
resmon.exe

# Performance Monitor
perfmon.exe

# Process Explorer (Sysinternals)
procexp.exe

# Logs
Get-EventLog -LogName Application -Source Zed

# Application data
cd $env:APPDATA\Zed
```

#### Linux
```bash
# System info
uname -a
lsb_release -a
cat /proc/cpuinfo
cat /proc/meminfo

# Process info
ps aux | grep zed
top -p [Zed PID]
htop

# System calls
strace -p [Zed PID]

# File access
lsof -p [Zed PID]

# Performance profiling
perf record -p [Zed PID]
perf report

# Logs
journalctl -u zed
~/.local/share/zed/logs/
```

### Community Resources

```markdown
GETTING HELP:
- [Zed Community Discord](https://discord.gg/zed)
- [GitHub Discussions](https://github.com/zed-industries/zed/discussions)
- [Zed Twitter/X](https://twitter.com/zed)

CONTRIBUTING:
- [Good First Issues](https://github.com/zed-industries/zed/labels/good%20first%20issue)
- [Help Wanted](https://github.com/zed-industries/zed/labels/help%20wanted)
```

---

## 📌 Appendix

### Issue Report Quality Rubric

Use this to assess the overall quality of the issue report:

| Criteria | Weight | Score | Points |
|----------|--------|-------|--------|
| **Problem Description** | 15% | [1-10] | [X/1.5] |
| **Reproduction Steps** | 20% | [1-10] | [X/2.0] |
| **System Information** | 15% | [1-10] | [X/1.5] |
| **Evidence (screenshots/logs)** | 15% | [1-10] | [X/1.5] |
| **Configuration Details** | 10% | [1-10] | [X/1.0] |
| **Diagnostic Tests** | 10% | [1-10] | [X/1.0] |
| **Clarity and Organization** | 10% | [1-10] | [X/1.0] |
| **Responsiveness** | 5% | [1-10] | [X/0.5] |
| **Total** | 100% | — | **[X/10]** |

**Grade**: [🟢 A | 🟢 B | 🟡 C | 🔴 D | 🔴 F]

**Recommendation**:
- **9.0-10.0 (A)**: Excellent report, ready for immediate investigation
- **7.5-8.9 (B)**: Good report, minor clarifications needed
- **6.0-7.4 (C)**: Acceptable, some important information missing
- **4.5-5.9 (D)**: Poor, critical information missing
- **0-4.4 (F)**: Insufficient, cannot proceed without major additions

---

### Template Change Log

| Version | Date | Changes | Author |
|---------|------|---------|--------|
| 1.0 | [DATE] | Initial template creation | [AUTHOR] |
| | | | |

---

### Notes and Observations

```markdown
ADDITIONAL NOTES:

[Any additional observations, context, or notes about this specific issue]

LESSONS LEARNED:

[What did we learn from this issue that could improve future processes?]

PROCESS IMPROVEMENTS:

[Suggested improvements to templates, workflows, or documentation]
```

---

## 📞 Contact and Escalation

### For Questions About This Analysis

- **Analyst**: [NAME]
- **Email**: [EMAIL]
- **GitHub**: [@USERNAME]

### For the Original Issue

- **Issue**: [#NUMBER](https://github.com/zed-industries/zed/issues/[NUMBER])
- **Reporter**: [@USERNAME]
- **Assignee**: [@USERNAME] or [Unassigned]

### Escalation Path

```
Level 1: Issue Reporter ↔ Community
Level 2: Maintainer/Contributor
Level 3: Core Team Member
Level 4: Project Lead
```

**When to Escalate**:
- No response for >7 days (by priority)
- Critical security issue
- Data loss potential
- Large number of affected users
- Blocker for release

---

**🚨 This is a living document. Update as investigation progresses.**

---

**Document Version**: 1.0  
**Template Version**: 1.0  
**Last Updated**: [DATE]  
**Generated by**: [ANALYST/TOOL]
