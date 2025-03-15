# Tool Evals

A framework for evaluating and benchmarking AI assistant performance in the Zed editor.

## Overview

Tool Evals provides a headless environment for running assistants evaluations on code repositories. It automates the process of:

1. Cloning and setting up test repositories
2. Sending prompts to language models
3. Allowing the assistant to use tools to modify code
4. Collecting metrics on performance
5. Evaluating results against known good solutions

## How It Works

The system consists of several key components:

- **Eval**: Loads test cases from the evaluation_data directory, clones repos, and executes evaluations
- **HeadlessAssistant**: Provides a headless environment for running the AI assistant
- **Judge**: Compares AI-generated diffs with reference solutions and scores their functional similarity

The evaluation flow:
1. An evaluation is loaded from the evaluation_data directory
2. The target repository is cloned and checked out at a specific commit
3. A HeadlessAssistant instance is created with the specified language model
4. The user prompt is sent to the assistant
5. The assistant responds and uses tools to modify code
6. Upon completion, a diff is generated from the changes
7. Results are saved including the diff, assistant's response, and performance metrics
8. If a reference solution exists, a Judge evaluates the similarity of the solution

## Setup Requirements

### Prerequisites

- Rust and Cargo
- Git
- Network access to clone repositories
- Appropriate API keys for language models and git services (Anthropic, GitHub, etc.)

### Environment Variables

Ensure you have the required API keys set, either from a dev run of Zed or via these environment variables:
- `ZED_ANTHROPIC_API_KEY` for Claude models
- `ZED_OPENAI_API_KEY` for OpenAI models
- `ZED_GITHUB_API_KEY` for GitHub API (or similar)

## Usage

### Running a Single Evaluation

To run a specific evaluation:

```bash
cargo run -p assistant_eval -- bubbletea-add-set-window-title
```

The arguments are regex patterns for the evaluation names to run, so to run all evaluations that contain `bubbletea`, run:

```bash
cargo run -p assistant_eval -- bubbletea
```

To run all evaluations:

```bash
cargo run -p assistant_eval -- --all
```

## Evaluation Data Structure

Each evaluation should be placed in the `evaluation_data` directory with the following structure:

* `prompt.txt`: The user's prompt.
* `original.diff`: The `git diff` of the change anticipated for this prompt.
* `setup.json`: Information about the repo used for the evaluation.
