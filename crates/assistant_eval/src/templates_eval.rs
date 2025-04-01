#[derive(Clone, Debug)]
pub struct Template {
    pub name: &'static str,
    pub content: &'static str,
}

pub fn all_templates() -> Vec<Template> {
    vec![
        Template {
            name: "ProjectCreation",
            content: r#"
# Project Creation Evaluation Template

## Instructions

Evaluate how well the AI assistant created a new implementation from scratch. Score it between 0.0 and 1.0 based on quality and fulfillment of requirements.
- 1.0 = Perfect implementation that creates all necessary files with correct functionality.
- 0.0 = Completely fails to create working files or meet requirements.

Note: A git diff output is required. If no code changes are provided (i.e., no git diff output), the score must be 0.0.

## Evaluation Criteria

Please consider the following aspects in order of importance:

1. **File Creation (25%)**
   - Did the assistant create all necessary files?
   - Are the files appropriately named and organized?
   - Did the assistant create a complete solution without missing components?

2. **Functional Correctness (40%)**
   - Does the implementation fulfill all specified requirements?
   - Does it handle edge cases properly?
   - Is it free of logical errors and bugs?
   - Do all components work together as expected?

3. **Code Quality (20%)**
   - Is the code well-structured, readable and well-documented?
   - Does it follow language-specific best practices?
   - Is there proper error handling?
   - Are naming conventions clear and consistent?

4. **Architecture Design (15%)**
   - Is the code modular and extensible?
   - Is there proper separation of concerns?
   - Are appropriate design patterns used?
   - Is the overall architecture appropriate for the requirements?

## Input

Requirements:
<!-- ```requirements go here``` -->

Reference Implementation:
<!-- ```reference code goes here``` -->

AI-Generated Implementation (git diff output):
<!-- ```git diff goes here``` -->

## Output Format

THE ONLY OUTPUT SHOULD BE A SCORE BETWEEN 0.0 AND 1.0.

EXAMPLE ONE:

0.92

EXAMPLE TWO:

0.85

EXAMPLE THREE:

0.78
"#,
        },
        Template {
            name: "CodeModification",
            content: r#"
# Code Modification Evaluation Template

## Instructions

Evaluate how well the AI assistant modified existing code to meet requirements. Score between 0.0 and 1.0 based on quality and appropriateness of changes.
- 1.0 = Perfect modifications that correctly implement all requirements.
- 0.0 = Failed to make appropriate changes or introduced serious errors.

## Evaluation Criteria

Please consider the following aspects in order of importance:

1. **Functional Correctness (50%)**
   - Do the modifications correctly implement the requirements?
   - Did the assistant modify the right files and code sections?
   - Are the changes free of bugs and logical errors?
   - Do the modifications maintain compatibility with existing code?

2. **Modification Approach (25%)**
   - Are the changes minimal and focused on what needs to be changed?
   - Did the assistant avoid unnecessary modifications?
   - Are the changes integrated seamlessly with the existing codebase?
   - Did the assistant preserve the original code style and patterns?

3. **Code Quality (15%)**
   - Are the modifications well-structured and documented?
   - Do they follow the same conventions as the original code?
   - Is there proper error handling in the modified code?
   - Are the changes readable and maintainable?

4. **Solution Completeness (10%)**
   - Do the modifications completely address all requirements?
   - Are there any missing changes or overlooked requirements?
   - Did the assistant consider all necessary edge cases?

## Input

Original:
<!-- ```reference code goes here``` -->

New (git diff output):
<!-- ```git diff goes here``` -->

## Output Format

THE ONLY OUTPUT SHOULD BE A SCORE BETWEEN 0.0 AND 1.0.

EXAMPLE ONE:

0.92

EXAMPLE TWO:

0.85

EXAMPLE THREE:

0.78
"#,
        },
        Template {
            name: "ConversationalGuidance",
            content: r#"
# Conversational Guidance Evaluation Template

## Instructions

Evaluate the quality of the AI assistant's conversational guidance and score it between 0.0 and 1.0.
- 1.0 = Perfect guidance with ideal information gathering, clarification, and advice without writing code.
- 0.0 = Completely unhelpful, inappropriate guidance, or wrote code when it should not have.

## Evaluation Criteria

ABSOLUTE REQUIREMENT:
   - The assistant should NOT generate complete code solutions in conversation mode.
   - If the git diff shows the assistant wrote complete code, the score should be significantly reduced.

1. **Information Gathering Effectiveness (30%)**
   - Did the assistant ask relevant and precise questions?
   - Did it efficiently narrow down the problem scope?
   - Did it avoid unnecessary or redundant questions?
   - Was questioning appropriately paced and contextual?

2. **Conceptual Guidance (30%)**
   - Did the assistant provide high-level approaches and strategies?
   - Did it explain relevant concepts and algorithms?
   - Did it offer planning advice without implementing the solution?
   - Did it suggest a structured approach to solving the problem?

3. **Educational Value (20%)**
   - Did the assistant help the user understand the problem better?
   - Did it provide explanations that would help the user learn?
   - Did it guide without simply giving away answers?
   - Did it encourage the user to think through parts of the problem?

4. **Conversation Quality (20%)**
   - Was the conversation logically structured and easy to follow?
   - Did the assistant maintain appropriate context throughout?
   - Was the interaction helpful without being condescending?
   - Did the conversation reach a satisfactory conclusion with clear next steps?

## Input

Initial Query:
<!-- ```query goes here``` -->

Conversation Transcript:
<!-- ```transcript goes here``` -->

Git Diff:
<!-- ```git diff goes here``` -->

## Output Format

THE ONLY OUTPUT SHOULD BE A SCORE BETWEEN 0.0 AND 1.0.

EXAMPLE ONE:

0.92

EXAMPLE TWO:

0.85

EXAMPLE THREE:

0.78
"#,
        },
    ]
}
