I’m working on improvements to a Jetty OSGi application’s file path handling and deployment logic. The changes focus on two main areas: URI normalization and WAR file extraction.

First, the URI handling logic needs updates to ensure consistent formatting, particularly when dealing with file paths. Currently, there are cases where paths aren’t properly normalized, especially when converting between file URIs and URLs. This affects both core OSGi resource resolution and utility methods that process path strings. The goal is to apply systematic corrections so that paths are reliably formatted across different scenarios.

Second, the WAR file extraction process requires refinement to make it more robust. The current implementation checks for pre-extracted sibling directories, but the logic could be strengthened by using the resolved webApp path directly rather than reconstructing it from strings. Additionally, the code would benefit from clearer documentation and added safeguards to handle edge cases gracefully. These changes will apply to both the EE9 and EE10 WebApp configurations, ensuring consistent behavior across versions.

The overarching aim is to reduce deployment failures and improve maintainability while keeping the changes backward-compatible.
