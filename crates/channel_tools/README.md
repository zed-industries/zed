# Channel Tools for AI Agents

This module provides AI agent tools for interacting with Zed's channel system. These tools enable agents to manage channels programmatically, including creating channels, organizing them, and editing channel notes collaboratively.

## Overview

The channel tools allow AI agents to:
- Create new channels and subchannels
- Move channels to different parent channels
- Reorder channels within their parent
- Edit channel notes using collaborative editing

## Tools

### CreateChannelTool

Creates a new channel in the workspace.

**Input Schema:**
```json
{
  "name": "channel-name",
  "parent": "parent-channel-name",  // optional
  "visibility": "members"  // or "public"
}
```

**Features:**
- Creates root channels when no parent is specified
- Supports creating subchannels under existing channels
- Allows setting channel visibility (members-only or public)
- Does not require user confirmation

### MoveChannelTool

Moves a channel to a different parent channel.

**Input Schema:**
```json
{
  "channel": "channel-to-move",
  "to": "new-parent-channel"  // or null for root
}
```

**Features:**
- Moves channels between different parents
- Currently does not support moving channels to root (limitation)
- Validates against circular dependencies (can't move to descendants)
- Requires user confirmation before executing

### ReorderChannelTool

Changes the order of a channel among its siblings.

**Input Schema:**
```json
{
  "channel": "channel-name",
  "direction": "up"  // or "down"
}
```

**Features:**
- Moves channels up or down within their sibling list
- Uses the native channel reordering API
- Does not require user confirmation

### EditChannelNotesTool

Edits channel notes using collaborative editing to avoid conflicts.

**Input Schema:**
```json
{
  "channel": "channel-name",
  "edits": [
    {
      "kind": "create",  // or "edit" or "append"
      "content": "Note content",
      "range": {  // optional, for "edit" kind
        "start_line": 0,
        "start_column": 0,
        "end_line": 10,
        "end_column": 0
      }
    }
  ]
}
```

**Features:**
- Supports creating new notes, editing existing content, or appending
- Uses collaborative editing through channel buffers
- Automatically handles buffer synchronization
- Supports multiple edits in a single operation
- Does not require user confirmation

## Collaborative Editing

The EditChannelNotesTool uses Zed's collaborative editing infrastructure:

1. Opens a channel buffer (same as when users edit notes)
2. Applies edits through the buffer's collaborative editing system
3. Acknowledges buffer versions to ensure synchronization
4. Avoids conflicts with other users editing simultaneously

This approach ensures that agent edits integrate seamlessly with human edits and maintain consistency across all connected clients.

## Implementation Details

### Architecture
- Tools implement the `Tool` trait from the assistant_tool crate
- Each tool maintains a reference to the global ChannelStore
- Operations are performed asynchronously using GPUI's task system
- Channel lookups are done by name for user-friendliness

### Error Handling
- Invalid channel names result in descriptive error messages
- Network failures are propagated as errors
- Validation prevents invalid operations (e.g., circular moves)

### Testing
All tools have comprehensive test coverage including:
- Schema validation
- UI text generation
- Confirmation requirements
- Basic operation validation

## Limitations

1. **Moving to Root**: The MoveChannelTool cannot currently move channels to the root level due to API limitations
2. **Channel Deletion**: No tool for deleting channels (intentional safety measure)
3. **Permissions**: Tools operate with the current user's permissions
4. **Name Conflicts**: No automatic handling of duplicate channel names

## Usage Example

An agent might use these tools in sequence to organize a project:

```
1. Create main channels:
   - "frontend" (public)
   - "backend" (members)
   - "docs" (public)

2. Create subchannels:
   - "frontend/components"
   - "frontend/styles"
   - "backend/api"
   - "backend/database"

3. Edit channel notes:
   - Add README content to each channel
   - Include guidelines and links

4. Reorder for clarity:
   - Move "docs" to the top
   - Organize subchannels alphabetically
```

## Future Enhancements

Potential improvements could include:
- Channel deletion tool (with strong safety measures)
- Bulk operations for efficiency
- Channel templates for common structures
- Integration with channel permissions/roles
- Search functionality for finding channels