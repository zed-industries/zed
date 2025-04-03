# Tool Evals

A framework for evaluating and benchmarking the agent panel generations.

## Overview

Tool Evals provides a headless environment for running assistants evaluations on code repositories. It automates the process of:

1. Setting up test code and repositories
2. Sending prompts to language models
3. Allowing the assistant to use tools to modify code
4. Collecting metrics on performance and tool usage
5. Evaluating results against known good solutions

## How It Works

The system consists of several key components:

- **Eval**: Loads exercises from the zed-ace-framework repository, creates temporary repos, and executes evaluations
- **HeadlessAssistant**: Provides a headless environment for running the AI assistant
- **Judge**: Evaluates AI-generated solutions against reference implementations and assigns scores
- **Templates**: Defines evaluation frameworks for different tasks (Project Creation, Code Modification, Conversational Guidance)

## Setup Requirements

### Prerequisites

- Rust and Cargo
- Git
- Python (for report generation)
- Network access to clone repositories
- Appropriate API keys for language models and git services (Anthropic, GitHub, etc.)

### Environment Variables

Ensure you have the required API keys set, either from a dev run of Zed or via these environment variables:
- `ZED_ANTHROPIC_API_KEY` for Claude models
- `ZED_GITHUB_API_KEY` for GitHub API (or similar)

## Usage

### Running Evaluations

```bash
# Run all tests
cargo run -p assistant_eval -- --all

# Run only specific languages
cargo run -p assistant_eval -- --all --languages python,rust

# Limit concurrent evaluations
cargo run -p assistant_eval -- --all --concurrency 5

# Limit number of exercises per language
cargo run -p assistant_eval -- --all --max-exercises-per-language 3
```

### Evaluation Template Types

The system supports three types of evaluation templates:

1. **ProjectCreation**: Tests the model's ability to create new implementations from scratch
2. **CodeModification**: Tests the model's ability to modify existing code to meet new requirements
3. **ConversationalGuidance**: Tests the model's ability to provide guidance without writing code

### Support Repo

The [zed-industries/zed-ace-framework](https://github.com/zed-industries/zed-ace-framework) contains the analytics and reporting scripts.
