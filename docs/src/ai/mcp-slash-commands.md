# MCP Tools as Slash Commands

Zed automatically converts MCP (Model Context Protocol) tools into slash commands, making them easily accessible in text threads and assistant conversations.

## Overview

When an MCP server provides tools, Zed automatically creates corresponding slash commands using the server name and tool name. Server names are automatically cleaned by removing common prefixes like "mcp-server-" and "mcp-". This allows you to invoke MCP tools directly from the chat interface using familiar slash command syntax.

## How It Works

1. **Automatic Detection**: When MCP servers are running and provide tools, Zed automatically discovers them
2. **Slash Command Creation**: Each MCP tool becomes a slash command with the name format `/{server-name}-{tool-name}`
3. **Name Cleaning**: Server names have prefixes like "mcp-server-" and "mcp-" automatically removed
4. **Name Formatting**: Both server names and tool names with underscores are converted to hyphens
5. **Argument Support**: Tools support multiple argument formats including key:value and key=value syntax

## Example Usage

If you have a GitHub MCP server (named "mcp-server-github" or just "github") running with tools like:
- `get_me` - Get information about the authenticated user
- `create_issue` - Create a new GitHub issue
- `search_code` - Search for code across repositories

These become available as:
- `/github-get-me` - No arguments required
- `/github-create-issue title:"Bug report" body:"Description"` - Key:value arguments
- `/github-search-code query="React hooks"` - Key=value arguments

Note: A server named "mcp-server-github" becomes "github" in the command names.

## Example Output

When you run these commands, the output is automatically formatted for readability:

### JSON Response (formatted as text):
```
Output from github tool 'get-me':

  login: octocat
  name: The Octocat
  email: octocat@github.com
  public_repos: 8
  followers: 4000
```

### Simple Text Response:
```
Output from github tool 'search-code':

Found 15 results for "React hooks" in public repositories:
- useState in react/examples/hooks-basic.js
- useEffect in facebook/react/packages/react/src/ReactHooks.js
- Custom hooks in awesome-react/hooks-collection.md
```

### No Output Response:
```
✓ Tool 'create-issue' from github executed successfully.

No output was returned by the tool.
```

## Argument Formats

MCP tool slash commands support multiple argument formats:

### JSON Format
```
/github-create-issue {"title": "New feature", "body": "Feature description", "labels": ["enhancement"]}
```

### Key:Value Format
```
/github-create-issue title:"Bug report" body:"This is a detailed description" priority:high
/github-get-issue owner:zed-industries repo:zed issue_number:35574
```

### Key=Value Format
```
/github-create-issue title="Bug report" body="This is a detailed description" priority=high
/github-get-issue owner=zed-industries repo=zed issue_number=35574
```

### Simple String
```
/github-search-code React hooks
```

### Auto-mapping
For tools with a single parameter, simple strings are automatically mapped to the first property in the tool's schema.

### Automatic Type Conversion
Values are automatically converted to the appropriate type:
- Numbers: `count=42`, `price=19.99` → converted to numeric values
- Booleans: `active=true`, `enabled=false` → converted to boolean values
- Quoted strings: `name="John Doe"` → removes quotes, keeps as string
- Null: `value=null` → converted to null value
- Regular text: `status=pending` → kept as string

## Server Name Processing

Zed automatically cleans server names to create user-friendly command names:

- `mcp-server-github` → `/github-{tool-name}`
- `mcp-server-slack-bot` → `/slack-bot-{tool-name}`
- `mcp-custom-tool` → `/custom-tool-{tool-name}`
- `my_server_name` → `/my-server-name-{tool-name}`

## Tool Discovery

MCP tools are automatically loaded as slash commands when:
1. An MCP server is running and connected
2. The server supports the `tools` capability  
3. The server provides a list of available tools
4. Tools are registered with cleaned server names for better usability

## Completion and Help

- Slash commands appear in the command picker (triggered by typing `/`)
- Tool descriptions from the MCP server are shown as help text
- Arguments are validated based on the tool's input schema

## Benefits

- **Unified Interface**: Use the same slash command syntax for both built-in Zed commands and MCP tools
- **Discoverability**: All available tools are visible in the command picker
- **Type Safety**: Arguments are automatically converted to correct types (numbers, booleans, strings)
- **Readable Output**: JSON responses are automatically formatted as readable text
- **Seamless Integration**: No additional configuration required

## Troubleshooting

### Commands Not Appearing
- Ensure your MCP server is running and properly configured
- Check that the server supports the `tools` capability
- Verify the server is providing tools in its tools list

### Argument Errors
- Check the tool's input schema for required parameters
- Ensure JSON format is valid when using complex arguments
- Use simple strings for tools with single parameters

## Related

- [MCP Configuration](./mcp.md)
- [Slash Commands](../assistant/slash-commands.md)
- [Text Threads](../text-threads.md)