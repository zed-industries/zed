"""
Minimal MCP server using FastMCP.

This server exposes exactly one prompt with exactly one argument.
Run it over stdio (recommended for editor integration).

Requirements:
  pip install "mcp[cli]"

Run:
  python server.py
"""

from __future__ import annotations

from mcp.server.fastmcp import FastMCP

mcp = FastMCP("fastmcp-test")


@mcp.prompt("test_prompt")
def test_prompt(name: str):
    """
    A simple prompt that accepts one argument: `name`.

    Return a structured prompt containing multiple chat messages.
    """
    return [
        {
            "role": "assistant",
            "content": "Hello! This is a test prompt that returns assistant messages.",
        },
        {
            "role": "user",
            "content": f"My name is {name}. Please confirm you received it.",
        },
        {
            "role": "assistant",
            "content": f"Confirmed — I received your name: {name}.",
        },
    ]


if __name__ == "__main__":
    # Expose the MCP server over stdio so editors (like Zed) can spawn it.
    mcp.run()
