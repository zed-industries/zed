#!/usr/bin/env -S uvx --from fastmcp fastmcp run --no-banner -t stdio
"""
Test MCP server for audience annotations.

Exposes tools that return content blocks with different annotations.audience
values so you can verify Zed's filtering behaviour:

  - user-only blocks appear in the tool call card but are excluded from model context
  - model-facing blocks go to both
  - mixed responses are partitioned correctly

Configure in Zed settings under context_servers:

  "context_servers": {
    "test-audience": {
      "command": "uvx",
      "args": ["--from", "fastmcp", "fastmcp", "run", "<path-to-zed>/script/test-audience-mcp.py:mcp", "-t", "stdio"]
    }
  }

Or run directly:  uvx --from fastmcp fastmcp run script/test-audience-mcp.py:mcp -t stdio
"""

from __future__ import annotations

from fastmcp import FastMCP
from mcp.types import Annotations, TextContent

mcp = FastMCP("test-audience")


@mcp.tool()
def mixed_audience(query: str = "hello") -> list[TextContent]:
    """Return a mix of user-only and model-facing content blocks.

    The first block is user-only (detailed explanation). The second block
    has no audience annotation, so it goes to both user and model.
    """
    return [
        TextContent(
            type="text",
            text=(
                f"## Detailed results for '{query}'\n\n"
                "This block is annotated with `audience: [\"user\"]` so it "
                "should appear in the tool call card but **not** be sent to "
                "the model.\n\n"
                "- Item A: 42\n"
                "- Item B: 97\n"
                "- Item C: 13\n"
            ),
            annotations=Annotations(audience=["user"]),
        ),
        TextContent(
            type="text",
            text=f"Summary for '{query}': found 3 items totalling 152.",
        ),
    ]


@mcp.tool()
def user_only() -> list[TextContent]:
    """Return content that is entirely user-only.

    Every block has audience: ["user"]. The model should receive the
    placeholder text "[output displayed to user]" instead.
    """
    return [
        TextContent(
            type="text",
            text=(
                "This is a rich, formatted explanation meant only for the "
                "human reading the tool call card.\n\n"
                "The model should NOT see this text. Instead it should "
                "receive a short placeholder."
            ),
            annotations=Annotations(audience=["user"]),
        ),
    ]


@mcp.tool()
def model_only(question: str = "What is 2+2?") -> list[TextContent]:
    """Return content annotated for the model only.

    The block has audience: ["assistant"]. It should go to the model but
    not appear in the tool call card displayed to the user.
    """
    return [
        TextContent(
            type="text",
            text=f"The answer to '{question}' is 4.",
            annotations=Annotations(audience=["assistant"]),
        ),
    ]


@mcp.tool()
def no_annotations() -> list[TextContent]:
    """Return content with no audience annotations at all.

    This is the baseline -- content goes to both user and model, same as
    any normal MCP tool.
    """
    return [
        TextContent(
            type="text",
            text="This content has no annotations. It should appear everywhere.",
        ),
    ]