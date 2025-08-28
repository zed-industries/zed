# Linear - Ticket Management

You are tasked with managing Linear tickets, including creating tickets from thoughts documents, updating existing tickets, and following the team's specific workflow patterns.

## Initial Setup

First, verify that Linear MCP tools are available by checking if any `mcp__linear__` tools exist. If not, respond:
```
I need access to Linear tools to help with ticket management. Please run the `/mcp` command to enable the Linear MCP server, then try again.
```

If tools are available, respond based on the user's request:

### For general requests:
```
I can help you with Linear tickets. What would you like to do?
1. Create a new ticket from a thoughts document
2. Add a comment to a ticket (I'll use our conversation context)
3. Search for tickets
4. Update ticket status or details
```

### For specific create requests:
```
I'll help you create a Linear ticket from your thoughts document. Please provide:
1. The path to the thoughts document (or topic to search for)
2. Any specific focus or angle for the ticket (optional)
```

Then wait for the user's input.

## Team Workflow & Status Progression

The team follows a specific workflow to ensure alignment before code implementation:

1. **Triage** → All new tickets start here for initial review
2. **Spec Needed** → More detail is needed - problem to solve and solution outline necessary
3. **Research Needed** → Ticket requires investigation before plan can be written
4. **Research in Progress** → Active research/investigation underway
5. **Research in Review** → Research findings under review (optional step)
6. **Ready for Plan** → Research complete, ticket needs an implementation plan
7. **Plan in Progress** → Actively writing the implementation plan
8. **Plan in Review** → Plan is written and under discussion
9. **Ready for Dev** → Plan approved, ready for implementation
10. **In Dev** → Active development
11. **Code Review** → PR submitted
12. **Done** → Completed

**Key principle**: Review and alignment happen at the plan stage (not PR stage) to move faster and avoid rework.

## Important Conventions

### URL Mapping for Thoughts Documents
When referencing thoughts documents, always provide GitHub links using the `links` parameter:
- `thoughts/shared/...` → `https://github.com/humanlayer/thoughts/blob/main/repos/humanlayer/shared/...`
- `thoughts/allison/...` → `https://github.com/humanlayer/thoughts/blob/main/repos/humanlayer/allison/...`
- `thoughts/global/...` → `https://github.com/humanlayer/thoughts/blob/main/global/...`

### Default Values
- **Status**: Always create new tickets in "Triage" status
- **Project**: For new tickets, default to "M U L T I C L A U D E" (ID: f11c8d63-9120-4393-bfae-553da0b04fd8) unless told otherwise
- **Priority**: Default to Medium (3) for most tasks, use best judgment or ask user
  - Urgent (1): Critical blockers, security issues
  - High (2): Important features with deadlines, major bugs
  - Medium (3): Standard implementation tasks (default)
  - Low (4): Nice-to-haves, minor improvements
- **Links**: Use the `links` parameter to attach URLs (not just markdown links in description)

### Automatic Label Assignment
Automatically apply labels based on the ticket content:
- **hld**: For tickets about the `hld/` directory (the daemon)
- **wui**: For tickets about `humanlayer-wui/`
- **meta**: For tickets about `hlyr` commands, thoughts tool, or `thoughts/` directory

Note: meta is mutually exclusive with hld/wui. Tickets can have both hld and wui, but not meta with either.

## Action-Specific Instructions

### 1. Creating Tickets from Thoughts

#### Steps to follow after receiving the request:

1. **Locate and read the thoughts document:**
   - If given a path, read the document directly
   - If given a topic/keyword, search thoughts/ directory using Grep to find relevant documents
   - If multiple matches found, show list and ask user to select
   - Create a TodoWrite list to track: Read document → Analyze content → Draft ticket → Get user input → Create ticket

2. **Analyze the document content:**
   - Identify the core problem or feature being discussed
   - Extract key implementation details or technical decisions
   - Note any specific code files or areas mentioned
   - Look for action items or next steps
   - Identify what stage the idea is at (early ideation vs ready to implement)
   - Take time to ultrathink about distilling the essence of this document into a clear problem statement and solution approach

3. **Check for related context (if mentioned in doc):**
   - If the document references specific code files, read relevant sections
   - If it mentions other thoughts documents, quickly check them
   - Look for any existing Linear tickets mentioned

4. **Get Linear workspace context:**
   - List teams: `mcp__linear__list_teams`
   - If multiple teams, ask user to select one
   - List projects for selected team: `mcp__linear__list_projects`

5. **Draft the ticket summary:**
   Present a draft to the user:
   ```
   ## Draft Linear Ticket

   **Title**: [Clear, action-oriented title]

   **Description**:
   [2-3 sentence summary of the problem/goal]

   ## Key Details
   - [Bullet points of important details from thoughts]
   - [Technical decisions or constraints]
   - [Any specific requirements]

   ## Implementation Notes (if applicable)
   [Any specific technical approach or steps outlined]

   ## References
   - Source: `thoughts/[path/to/document.md]` ([View on GitHub](converted GitHub URL))
   - Related code: [any file:line references]
   - Parent ticket: [if applicable]

   ---
   Based on the document, this seems to be at the stage of: [ideation/planning/ready to implement]
   ```

6. **Interactive refinement:**
   Ask the user:
   - Does this summary capture the ticket accurately?
   - Which project should this go in? [show list]
   - What priority? (Default: Medium/3)
   - Any additional context to add?
   - Should we include more/less implementation detail?
   - Do you want to assign it to yourself?

   Note: Ticket will be created in "Triage" status by default.

7. **Create the Linear ticket:**
   ```
   mcp__linear__create_issue with:
   - title: [refined title]
   - description: [final description in markdown]
   - teamId: [selected team]
   - projectId: [use default project from above unless user specifies]
   - priority: [selected priority number, default 3]
   - stateId: [Triage status ID]
   - assigneeId: [if requested]
   - labelIds: [apply automatic label assignment from above]
   - links: [{url: "GitHub URL", title: "Document Title"}]
   ```

8. **Post-creation actions:**
   - Show the created ticket URL
   - Ask if user wants to:
     - Add a comment with additional implementation details
     - Create sub-tasks for specific action items
     - Update the original thoughts document with the ticket reference
   - If yes to updating thoughts doc:
     ```
     Add at the top of the document:
     ---
     linear_ticket: [URL]
     created: [date]
     ---
     ```

## Example transformations:

### From verbose thoughts:
```
"I've been thinking about how our resumed sessions don't inherit permissions properly.
This is causing issues where users have to re-specify everything. We should probably
store all the config in the database and then pull it when resuming. Maybe we need
new columns for permission_prompt_tool and allowed_tools..."
```

### To concise ticket:
```
Title: Fix resumed sessions to inherit all configuration from parent

Description:

## Problem to solve
Currently, resumed sessions only inherit Model and WorkingDir from parent sessions,
causing all other configuration to be lost. Users must re-specify permissions and
settings when resuming.

## Solution
Store all session configuration in the database and automatically inherit it when
resuming sessions, with support for explicit overrides.
```

### 2. Adding Comments and Links to Existing Tickets

When user wants to add a comment to a ticket:

1. **Determine which ticket:**
   - Use context from the current conversation to identify the relevant ticket
   - If uncertain, use `mcp__linear__get_issue` to show ticket details and confirm with user
   - Look for ticket references in recent work discussed

2. **Format comments for clarity:**
   - Attempt to keep comments concise (~10 lines) unless more detail is needed
   - Focus on the key insight or most useful information for a human reader
   - Not just what was done, but what matters about it
   - Include relevant file references with backticks and GitHub links

3. **File reference formatting:**
   - Wrap paths in backticks: `thoughts/allison/example.md`
   - Add GitHub link after: `([View](url))`
   - Do this for both thoughts/ and code files mentioned

4. **Comment structure example:**
   ```markdown
   Implemented retry logic in webhook handler to address rate limit issues.

   Key insight: The 429 responses were clustered during batch operations,
   so exponential backoff alone wasn't sufficient - added request queuing.

   Files updated:
   - `hld/webhooks/handler.go` ([GitHub](link))
   - `thoughts/shared/rate_limit_analysis.md` ([GitHub](link))
   ```

5. **Handle links properly:**
   - If adding a link with a comment: Update the issue with the link AND mention it in the comment
   - If only adding a link: Still create a comment noting what link was added for posterity
   - Always add links to the issue itself using the `links` parameter

6. **For comments with links:**
   ```
   # First, update the issue with the link
   mcp__linear__update_issue with:
   - id: [ticket ID]
   - links: [existing links + new link with proper title]

   # Then, create the comment mentioning the link
   mcp__linear__create_comment with:
   - issueId: [ticket ID]
   - body: [formatted comment with key insights and file references]
   ```

7. **For links only:**
   ```
   # Update the issue with the link
   mcp__linear__update_issue with:
   - id: [ticket ID]
   - links: [existing links + new link with proper title]

   # Add a brief comment for posterity
   mcp__linear__create_comment with:
   - issueId: [ticket ID]
   - body: "Added link: `path/to/document.md` ([View](url))"
   ```

### 3. Searching for Tickets

When user wants to find tickets:

1. **Gather search criteria:**
   - Query text
   - Team/Project filters
   - Status filters
   - Date ranges (createdAt, updatedAt)

2. **Execute search:**
   ```
   mcp__linear__list_issues with:
   - query: [search text]
   - teamId: [if specified]
   - projectId: [if specified]
   - stateId: [if filtering by status]
   - limit: 20
   ```

3. **Present results:**
   - Show ticket ID, title, status, assignee
   - Group by project if multiple projects
   - Include direct links to Linear

### 4. Updating Ticket Status

When moving tickets through the workflow:

1. **Get current status:**
   - Fetch ticket details
   - Show current status in workflow

2. **Suggest next status:**
   - Triage → Spec Needed (lacks detail/problem statement)
   - Spec Needed → Research Needed (once problem/solution outlined)
   - Research Needed → Research in Progress (starting research)
   - Research in Progress → Research in Review (optional, can skip to Ready for Plan)
   - Research in Review → Ready for Plan (research approved)
   - Ready for Plan → Plan in Progress (starting to write plan)
   - Plan in Progress → Plan in Review (plan written)
   - Plan in Review → Ready for Dev (plan approved)
   - Ready for Dev → In Dev (work started)

3. **Update with context:**
   ```
   mcp__linear__update_issue with:
   - id: [ticket ID]
   - stateId: [new status ID]
   ```

   Consider adding a comment explaining the status change.

## Important Notes

- Tag users in descriptions and comments using `@[name](ID)` format, e.g., `@[dex](16765c85-2286-4c0f-ab49-0d4d79222ef5)`
- Keep tickets concise but complete - aim for scannable content
- All tickets should include a clear "problem to solve" - if the user asks for a ticket and only gives implementation details, you MUST ask "To write a good ticket, please explain the problem you're trying to solve from a user perspective"
- Focus on the "what" and "why", include "how" only if well-defined
- Always preserve links to source material using the `links` parameter
- Don't create tickets from early-stage brainstorming unless requested
- Use proper Linear markdown formatting
- Include code references as: `path/to/file.ext:linenum`
- Ask for clarification rather than guessing project/status
- Remember that Linear descriptions support full markdown including code blocks
- Always use the `links` parameter for external URLs (not just markdown links)
- remember - you must get a "Problem to solve"!

## Comment Quality Guidelines

When creating comments, focus on extracting the **most valuable information** for a human reader:

- **Key insights over summaries**: What's the "aha" moment or critical understanding?
- **Decisions and tradeoffs**: What approach was chosen and what it enables/prevents
- **Blockers resolved**: What was preventing progress and how it was addressed
- **State changes**: What's different now and what it means for next steps
- **Surprises or discoveries**: Unexpected findings that affect the work

Avoid:
- Mechanical lists of changes without context
- Restating what's obvious from code diffs
- Generic summaries that don't add value

Remember: The goal is to help a future reader (including yourself) quickly understand what matters about this update.

## Commonly Used IDs

### Engineering Team
- **Team ID**: `6b3b2115-efd4-4b83-8463-8160842d2c84`

### Label IDs
- **bug**: `ff23dde3-199b-421e-904c-4b9f9b3d452c`
- **hld**: `d28453c8-e53e-4a06-bea9-b5bbfad5f88a`
- **meta**: `7a5abaae-f343-4f52-98b0-7987048b0cfa`
- **wui**: `996deb94-ba0f-4375-8b01-913e81477c4b`

### Workflow State IDs
- **Triage**: `77da144d-fe13-4c3a-a53a-cfebd06c0cbe` (type: triage)
- **spec needed**: `274beb99-bff8-4d7b-85cf-04d18affbc82` (type: unstarted)
- **research needed**: `d0b89672-8189-45d6-b705-50afd6c94a91` (type: unstarted)
- **research in progress**: `c41c5a23-ce25-471f-b70a-eff1dca60ffd` (type: unstarted)
- **research in review**: `1a9363a7-3fae-42ee-a6c8-1fc714656f09` (type: unstarted)
- **ready for plan**: `995011dd-3e36-46e5-b776-5a4628d06cc8` (type: unstarted)
- **plan in progress**: `a52b4793-d1b6-4e5d-be79-b2254185eed0` (type: started)
- **plan in review**: `15f56065-41ea-4d9a-ab8c-ec8e1a811a7a` (type: started)
- **ready for dev**: `c25bae2f-856a-4718-aaa8-b469b7822f58` (type: started)
- **in dev**: `6be18699-18d7-496e-a7c9-37d2ddefe612` (type: started)
- **code review**: `8ca7fda1-08d4-48fb-a0cf-954246ccbe66` (type: started)
- **Ready for Deploy**: `a3ad0b54-17bf-4ad3-b1c1-2f56c1f2515a` (type: started)
- **Done**: `8159f431-fbc7-495f-a861-1ba12040f672` (type: completed)
- **Backlog**: `6cf6b25a-054a-469b-9845-9bd9ab39ad76` (type: backlog)
- **PostIts**: `a57f2ab3-c6f8-44c7-a36b-896154729338` (type: backlog)
- **Todo**: `ddf85246-3a7c-4141-a377-09069812bbc3` (type: unstarted)
- **Duplicate**: `2bc0e829-9853-4f76-ad34-e8732f062da2` (type: canceled)
- **Canceled**: `14a28d0d-c6aa-4d8e-9ff2-9801d4cc7de1` (type: canceled)


## Linear User IDs

- allison: b157f9e4-8faf-4e7e-a598-dae6dec8a584
- dex: 16765c85-2286-4c0f-ab49-0d4d79222ef5
- sundeep: 0062104d-9351-44f5-b64c-d0b59acb516b
