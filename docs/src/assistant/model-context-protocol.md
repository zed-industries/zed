# Model Context Protocol

## Overview

The Model Context Protocol (MCP) is a JSON-RPC based protocol for communication between a client (e.g., Zed) and context servers. It enables context-aware development assistance through various features like prompts, resources, and tools.

Currently, Zed's client only implements a subset of the protocol required to support custom prompt insertions and manipulations. This is likely to be expanded in the future.

## Protocol Basics

- Communication: JSON-RPC 2.0 over stdio
- Versioning: Protocol version negotiated during initialization

## Message Types

1. Requests: Client-to-server method calls
2. Responses: Server-to-client replies to requests
3. Notifications: Unidirectional messages (no response expected)

## Lifecycle

1. Client sends `initialize` request
2. Server responds with capabilities
3. Client sends `initialized` notification
4. Normal operation begins

### Initialize Request

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "initialize",
  "params": {
    "protocolVersion": 1,
    "capabilities": {
      "experimental": {},
      "sampling": {}
    },
    "clientInfo": {
      "name": "Zed",
      "version": "1.0.0"
    }
  }
}
```

### Initialize Response

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "protocolVersion": 1,
    "capabilities": {
      "experimental": {},
      "logging": {},
      "prompts": {},
      "resources": {
        "subscribe": true
      },
      "tools": {}
    },
    "serverInfo": {
      "name": "ExampleServer",
      "version": "1.0.0"
    }
  }
}
```

### Initialized Notification

```json
{
  "jsonrpc": "2.0",
  "method": "notifications/initialized",
  "params": {}
}
```

## Features

### Prompts

#### List Prompts

Request:

```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "prompts/list",
  "params": {}
}
```

Response:

```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "result": {
    "prompts": [
      {
        "name": "examplePrompt",
        "arguments": [
          {
            "name": "arg1",
            "description": "Description of arg1",
            "required": true
          }
        ]
      }
    ]
  }
}
```

#### Execute Prompt

Request:

```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "method": "prompts/get",
  "params": {
    "name": "examplePrompt",
    "arguments": {
      "arg1": "value1"
    }
  }
}
```

Response:

```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "result": {
    "prompt": "Generated prompt text"
  }
}
```

### Resources

#### List Resources

Request:

```json
{
  "jsonrpc": "2.0",
  "id": 4,
  "method": "resources/list",
  "params": {}
}
```

Response:

```json
{
  "jsonrpc": "2.0",
  "id": 4,
  "result": {
    "resourceTemplates": [
      {
        "uriTemplate": "template://example/{param}",
        "name": "Example Template",
        "description": "Description of the template"
      }
    ],
    "resources": [
      {
        "uri": "https://example.com/resource",
        "mimeType": "text/plain"
      }
    ]
  }
}
```

#### Read Resource

Request:

```json
{
  "jsonrpc": "2.0",
  "id": 5,
  "method": "resources/read",
  "params": {
    "uri": "https://example.com/resource"
  }
}
```

Response:

```json
{
  "jsonrpc": "2.0",
  "id": 5,
  "result": {
    "contents": [
      {
        "uri": "https://example.com/resource",
        "mimeType": "text/plain",
        "contentType": "text",
        "text": "Resource content"
      }
    ]
  }
}
```

#### Subscribe to Resource

Request:

```json
{
  "jsonrpc": "2.0",
  "id": 6,
  "method": "resources/subscribe",
  "params": {
    "uri": "https://example.com/resource"
  }
}
```

Response:

```json
{
  "jsonrpc": "2.0",
  "id": 6,
  "result": null
}
```

#### Unsubscribe from Resource

Request:

```json
{
  "jsonrpc": "2.0",
  "id": 7,
  "method": "resources/unsubscribe",
  "params": {
    "uri": "https://example.com/resource"
  }
}
```

Response:

```json
{
  "jsonrpc": "2.0",
  "id": 7,
  "result": null
}
```

### Tools

#### Call Tool

Request:

```json
{
  "jsonrpc": "2.0",
  "id": 8,
  "method": "tools/call",
  "params": {
    "name": "exampleTool",
    "arguments": {
      "key": "value"
    }
  }
}
```

Response:

```json
{
  "jsonrpc": "2.0",
  "id": 8,
  "result": {
    "output": "Tool execution result"
  }
}
```

### Logging

#### Set Logging Level

Request:

```json
{
  "jsonrpc": "2.0",
  "id": 9,
  "method": "logging/setLevel",
  "params": {
    "level": "info"
  }
}
```

Response:

```json
{
  "jsonrpc": "2.0",
  "id": 9,
  "result": null
}
```

### Notifications

#### Progress

```json
{
  "jsonrpc": "2.0",
  "method": "notifications/progress",
  "params": {
    "progressToken": "operation1",
    "progress": 50.0,
    "total": 100.0
  }
}
```

## Error Handling

Errors should be returned as standard JSON-RPC 2.0 error objects:

```json
{
  "jsonrpc": "2.0",
  "id": null,
  "error": {
    "code": -32000,
    "message": "Error message"
  }
}
```
