//! Agent Invocation Module
//!
//! Handles invoking Convergio agents via the Anthropic API
//! and streaming responses back to the database.

use anyhow::{anyhow, Result};
use anthropic::{
    Event, Message, Request, RequestContent, Role, ANTHROPIC_API_URL,
};
use futures::StreamExt;
use gpui::{BackgroundExecutor, Task};
use http_client::HttpClient;
use std::sync::Arc;

use crate::convergio_db::{ChatMessage, ConvergioDb, MessageType};

/// Default model for Convergio agents
const DEFAULT_MODEL: &str = "claude-sonnet-4-5";
const MAX_TOKENS: u64 = 8192;

/// Agent definition with name and system prompt
pub struct AgentDefinition {
    pub name: &'static str,
    pub display_name: &'static str,
    pub system_prompt: &'static str,
}

/// Get the system prompt for an agent by name
/// Matches by exact name or by prefix (e.g., "ali" matches "ali-chief-of-staff")
pub fn get_agent_prompt(agent_name: &str) -> Option<&'static AgentDefinition> {
    // Try exact match first
    if let Some(agent) = AGENTS.iter().find(|a| a.name == agent_name) {
        return Some(agent);
    }

    // Try prefix match (e.g., "ali" matches "ali-chief-of-staff")
    AGENTS.iter().find(|a| a.name.starts_with(&format!("{}-", agent_name)))
}

/// Get a generic system prompt for unknown agents
pub fn get_generic_prompt(agent_name: &str) -> String {
    format!(
        "You are {}, an AI assistant in the Convergio ecosystem. \
         Provide helpful, accurate, and professional responses. \
         Be concise but thorough. If you don't know something, say so.",
        agent_name
    )
}

/// Invoke an agent with a user message
/// Returns a task that streams the response and updates the database
pub fn invoke_agent(
    db: Arc<ConvergioDb>,
    http_client: Arc<dyn HttpClient>,
    api_key: String,
    session_id: String,
    agent_name: String,
    messages: Vec<ChatMessage>,
    executor: BackgroundExecutor,
) -> Task<Result<()>> {
    executor.spawn(async move {
        // Get agent definition or use generic prompt
        let system_prompt = match get_agent_prompt(&agent_name) {
            Some(agent) => agent.system_prompt.to_string(),
            None => {
                log::warn!("No specific prompt for agent '{}', using generic", agent_name);
                get_generic_prompt(&agent_name)
            }
        };

        // Build message history for API
        let api_messages: Vec<Message> = messages
            .iter()
            .filter(|m| m.message_type == MessageType::User || m.message_type == MessageType::Assistant)
            .map(|m| Message {
                role: if m.message_type == MessageType::User {
                    Role::User
                } else {
                    Role::Assistant
                },
                content: vec![RequestContent::Text {
                    text: m.content.clone(),
                    cache_control: None,
                }],
            })
            .collect();

        // Build request
        let request = Request {
            model: DEFAULT_MODEL.to_string(),
            max_tokens: MAX_TOKENS,
            messages: api_messages,
            tools: vec![],
            thinking: None,
            tool_choice: None,
            system: Some(anthropic::StringOrContents::String(system_prompt)),
            metadata: None,
            stop_sequences: vec![],
            temperature: Some(0.7),
            top_k: None,
            top_p: None,
        };

        // Stream completion
        let mut stream = anthropic::stream_completion(
            http_client.as_ref(),
            ANTHROPIC_API_URL,
            &api_key,
            request,
            None,
        )
        .await
        .map_err(|e| anyhow!("Failed to start stream: {:?}", e))?;

        // Collect response text
        let mut response_text = String::new();
        let mut input_tokens: i64 = 0;
        let mut output_tokens: i64 = 0;

        while let Some(event_result) = stream.next().await {
            match event_result {
                Ok(event) => match event {
                    Event::ContentBlockDelta { delta, .. } => {
                        if let anthropic::ContentDelta::TextDelta { text } = delta {
                            response_text.push_str(&text);
                        }
                    }
                    Event::MessageDelta { usage, .. } => {
                        if let Some(tokens) = usage.output_tokens {
                            output_tokens = tokens as i64;
                        }
                    }
                    Event::MessageStart { message } => {
                        if let Some(tokens) = message.usage.input_tokens {
                            input_tokens = tokens as i64;
                        }
                    }
                    Event::MessageStop => {
                        // Done streaming
                        break;
                    }
                    Event::Error { error } => {
                        log::error!("API error: {:?}", error);
                        return Err(anyhow!("API error: {:?}", error));
                    }
                    _ => {}
                },
                Err(e) => {
                    log::error!("Stream error: {:?}", e);
                    return Err(anyhow!("Stream error: {:?}", e));
                }
            }
        }

        // Insert assistant message into database
        if !response_text.is_empty() {
            // Calculate approximate cost (Claude Sonnet 4.5 pricing)
            let cost_usd = (input_tokens as f64 * 0.003 / 1000.0)
                + (output_tokens as f64 * 0.015 / 1000.0);

            db.insert_assistant_message(
                &session_id,
                &agent_name,
                &response_text,
                input_tokens,
                output_tokens,
                cost_usd,
            )?;
        }

        Ok(())
    })
}

/// Get the Anthropic API key from environment
pub fn get_api_key() -> Option<String> {
    std::env::var("ANTHROPIC_API_KEY").ok()
}

// Agent definitions with system prompts
// Complete list of all 62 Convergio agents
static AGENTS: &[AgentDefinition] = &[
    // Core Leadership
    AgentDefinition {
        name: "ali-chief-of-staff",
        display_name: "Ali - Chief of Staff",
        system_prompt: include_str!("prompts/ali-chief-of-staff.md"),
    },
    AgentDefinition {
        name: "amy-cfo",
        display_name: "Amy - CFO",
        system_prompt: include_str!("prompts/amy-cfo.md"),
    },
    AgentDefinition {
        name: "satya-board-of-directors",
        display_name: "Satya - Board Advisor",
        system_prompt: include_str!("prompts/satya-board-of-directors.md"),
    },
    AgentDefinition {
        name: "dan-engineering-gm",
        display_name: "Dan - Engineering GM",
        system_prompt: include_str!("prompts/dan-engineering-gm.md"),
    },

    // Engineering & Development
    AgentDefinition {
        name: "rex-code-reviewer",
        display_name: "Rex - Code Reviewer",
        system_prompt: include_str!("prompts/rex-code-reviewer.md"),
    },
    AgentDefinition {
        name: "dario-debugger",
        display_name: "Dario - Debugger",
        system_prompt: include_str!("prompts/dario-debugger.md"),
    },
    AgentDefinition {
        name: "baccio-tech-architect",
        display_name: "Baccio - Tech Architect",
        system_prompt: include_str!("prompts/baccio-tech-architect.md"),
    },
    AgentDefinition {
        name: "paolo-best-practices-enforcer",
        display_name: "Paolo - Best Practices",
        system_prompt: include_str!("prompts/paolo-best-practices-enforcer.md"),
    },
    AgentDefinition {
        name: "marco-devops-engineer",
        display_name: "Marco - DevOps Engineer",
        system_prompt: include_str!("prompts/marco-devops-engineer.md"),
    },
    AgentDefinition {
        name: "otto-performance-optimizer",
        display_name: "Otto - Performance Optimizer",
        system_prompt: include_str!("prompts/otto-performance-optimizer.md"),
    },

    // Product Management
    AgentDefinition {
        name: "marcello-pm",
        display_name: "Marcello - Product Manager",
        system_prompt: include_str!("prompts/marcello-pm.md"),
    },
    AgentDefinition {
        name: "oliver-pm",
        display_name: "Oliver - Senior PM",
        system_prompt: include_str!("prompts/oliver-pm.md"),
    },

    // Security & Compliance
    AgentDefinition {
        name: "luca-security-expert",
        display_name: "Luca - Security Expert",
        system_prompt: include_str!("prompts/luca-security-expert.md"),
    },
    AgentDefinition {
        name: "elena-legal-compliance-expert",
        display_name: "Elena - Legal & Compliance",
        system_prompt: include_str!("prompts/elena-legal-compliance-expert.md"),
    },
    AgentDefinition {
        name: "dr-enzo-healthcare-compliance-manager",
        display_name: "Dr. Enzo - Healthcare Compliance",
        system_prompt: include_str!("prompts/dr-enzo-healthcare-compliance-manager.md"),
    },
    AgentDefinition {
        name: "guardian-ai-security-validator",
        display_name: "Guardian - AI Security",
        system_prompt: include_str!("prompts/guardian-ai-security-validator.md"),
    },
    AgentDefinition {
        name: "thor-quality-assurance-guardian",
        display_name: "Thor - QA Guardian",
        system_prompt: include_str!("prompts/thor-quality-assurance-guardian.md"),
    },

    // Design & UX
    AgentDefinition {
        name: "sara-ux-ui-designer",
        display_name: "Sara - UX/UI Designer",
        system_prompt: include_str!("prompts/sara-ux-ui-designer.md"),
    },
    AgentDefinition {
        name: "jenny-inclusive-accessibility-champion",
        display_name: "Jenny - Accessibility Champion",
        system_prompt: include_str!("prompts/jenny-inclusive-accessibility-champion.md"),
    },
    AgentDefinition {
        name: "jony-creative-director",
        display_name: "Jony - Creative Director",
        system_prompt: include_str!("prompts/jony-creative-director.md"),
    },
    AgentDefinition {
        name: "stefano-design-thinking-facilitator",
        display_name: "Stefano - Design Thinking",
        system_prompt: include_str!("prompts/stefano-design-thinking-facilitator.md"),
    },

    // Data & Analytics
    AgentDefinition {
        name: "omri-data-scientist",
        display_name: "Omri - Data Scientist",
        system_prompt: include_str!("prompts/omri-data-scientist.md"),
    },
    AgentDefinition {
        name: "ava-analytics-insights-virtuoso",
        display_name: "Ava - Analytics Virtuoso",
        system_prompt: include_str!("prompts/ava-analytics-insights-virtuoso.md"),
    },
    AgentDefinition {
        name: "diana-performance-dashboard",
        display_name: "Diana - Performance Dashboard",
        system_prompt: include_str!("prompts/diana-performance-dashboard.md"),
    },

    // Finance & Investment
    AgentDefinition {
        name: "fiona-market-analyst",
        display_name: "Fiona - Market Analyst",
        system_prompt: include_str!("prompts/fiona-market-analyst.md"),
    },
    AgentDefinition {
        name: "wiz-investor-venture-capital",
        display_name: "Wiz - VC Investor",
        system_prompt: include_str!("prompts/wiz-investor-venture-capital.md"),
    },
    AgentDefinition {
        name: "michael-vc",
        display_name: "Michael - VC Analyst",
        system_prompt: include_str!("prompts/michael-vc.md"),
    },

    // Strategy & Decision Making
    AgentDefinition {
        name: "angela-da",
        display_name: "Angela - Decision Architect",
        system_prompt: include_str!("prompts/angela-da.md"),
    },
    AgentDefinition {
        name: "ethan-da",
        display_name: "Ethan - Senior DA",
        system_prompt: include_str!("prompts/ethan-da.md"),
    },
    AgentDefinition {
        name: "evan-ic6da",
        display_name: "Evan - Principal DA (IC6)",
        system_prompt: include_str!("prompts/evan-ic6da.md"),
    },
    AgentDefinition {
        name: "domik-mckinsey-strategic-decision-maker",
        display_name: "Domik - McKinsey Strategist",
        system_prompt: include_str!("prompts/domik-mckinsey-strategic-decision-maker.md"),
    },
    AgentDefinition {
        name: "matteo-strategic-business-architect",
        display_name: "Matteo - Business Architect",
        system_prompt: include_str!("prompts/matteo-strategic-business-architect.md"),
    },
    AgentDefinition {
        name: "antonio-strategy-expert",
        display_name: "Antonio - Strategy Expert",
        system_prompt: include_str!("prompts/antonio-strategy-expert.md"),
    },
    AgentDefinition {
        name: "socrates-first-principles-reasoning",
        display_name: "Socrates - First Principles",
        system_prompt: include_str!("prompts/socrates-first-principles-reasoning.md"),
    },

    // Marketing & Sales
    AgentDefinition {
        name: "sofia-marketing-strategist",
        display_name: "Sofia - Marketing Strategist",
        system_prompt: include_str!("prompts/sofia-marketing-strategist.md"),
    },
    AgentDefinition {
        name: "fabio-sales-business-development",
        display_name: "Fabio - Sales & BD",
        system_prompt: include_str!("prompts/fabio-sales-business-development.md"),
    },
    AgentDefinition {
        name: "riccardo-storyteller",
        display_name: "Riccardo - Storyteller",
        system_prompt: include_str!("prompts/riccardo-storyteller.md"),
    },
    AgentDefinition {
        name: "steve-executive-communication-strategist",
        display_name: "Steve - Exec Communications",
        system_prompt: include_str!("prompts/steve-executive-communication-strategist.md"),
    },

    // Project & Program Management
    AgentDefinition {
        name: "davide-project-manager",
        display_name: "Davide - Project Manager",
        system_prompt: include_str!("prompts/davide-project-manager.md"),
    },
    AgentDefinition {
        name: "luke-program-manager",
        display_name: "Luke - Program Manager",
        system_prompt: include_str!("prompts/luke-program-manager.md"),
    },
    AgentDefinition {
        name: "taskmaster-strategic-task-decomposition-master",
        display_name: "Taskmaster - Task Decomposition",
        system_prompt: include_str!("prompts/taskmaster-strategic-task-decomposition-master.md"),
    },
    AgentDefinition {
        name: "strategic-planner",
        display_name: "Strategic Planner",
        system_prompt: include_str!("prompts/strategic-planner.md"),
    },

    // HR & Team
    AgentDefinition {
        name: "giulia-hr-talent-acquisition",
        display_name: "Giulia - HR & Talent",
        system_prompt: include_str!("prompts/giulia-hr-talent-acquisition.md"),
    },
    AgentDefinition {
        name: "coach-team-coach",
        display_name: "Coach - Team Coach",
        system_prompt: include_str!("prompts/coach-team-coach.md"),
    },
    AgentDefinition {
        name: "behice-cultural-coach",
        display_name: "Behice - Cultural Coach",
        system_prompt: include_str!("prompts/behice-cultural-coach.md"),
    },

    // Operations & Process
    AgentDefinition {
        name: "enrico-business-process-engineer",
        display_name: "Enrico - Process Engineer",
        system_prompt: include_str!("prompts/enrico-business-process-engineer.md"),
    },
    AgentDefinition {
        name: "dave-change-management-specialist",
        display_name: "Dave - Change Management",
        system_prompt: include_str!("prompts/dave-change-management-specialist.md"),
    },
    AgentDefinition {
        name: "andrea-customer-success-manager",
        display_name: "Andrea - Customer Success",
        system_prompt: include_str!("prompts/andrea-customer-success-manager.md"),
    },

    // Government & Legal
    AgentDefinition {
        name: "sophia-govaffairs",
        display_name: "Sophia - Government Affairs",
        system_prompt: include_str!("prompts/sophia-govaffairs.md"),
    },

    // Startup & VC
    AgentDefinition {
        name: "sam-startupper",
        display_name: "Sam - Startup Expert",
        system_prompt: include_str!("prompts/sam-startupper.md"),
    },

    // AI & Prompts
    AgentDefinition {
        name: "po-prompt-optimizer",
        display_name: "PO - Prompt Optimizer",
        system_prompt: include_str!("prompts/po-prompt-optimizer.md"),
    },

    // Orchestration & Coordination
    AgentDefinition {
        name: "wanda-workflow-orchestrator",
        display_name: "Wanda - Workflow Orchestrator",
        system_prompt: include_str!("prompts/wanda-workflow-orchestrator.md"),
    },
    AgentDefinition {
        name: "xavier-coordination-patterns",
        display_name: "Xavier - Coordination Patterns",
        system_prompt: include_str!("prompts/xavier-coordination-patterns.md"),
    },
    AgentDefinition {
        name: "marcus-context-memory-keeper",
        display_name: "Marcus - Memory Keeper",
        system_prompt: include_str!("prompts/marcus-context-memory-keeper.md"),
    },
    AgentDefinition {
        name: "anna-executive-assistant",
        display_name: "Anna - Executive Assistant",
        system_prompt: include_str!("prompts/anna-executive-assistant.md"),
    },

    // System Agents
    AgentDefinition {
        name: "general-purpose",
        display_name: "General Purpose Agent",
        system_prompt: include_str!("prompts/general-purpose.md"),
    },
    AgentDefinition {
        name: "explore",
        display_name: "Explore Agent",
        system_prompt: include_str!("prompts/explore.md"),
    },
    AgentDefinition {
        name: "plan",
        display_name: "Plan Agent",
        system_prompt: include_str!("prompts/plan.md"),
    },
    AgentDefinition {
        name: "feature-release-manager",
        display_name: "Feature Release Manager",
        system_prompt: include_str!("prompts/feature-release-manager.md"),
    },
    AgentDefinition {
        name: "app-release-manager",
        display_name: "App Release Manager",
        system_prompt: include_str!("prompts/app-release-manager.md"),
    },
    AgentDefinition {
        name: "mckinsey-strategic-consultant",
        display_name: "McKinsey Consultant",
        system_prompt: include_str!("prompts/mckinsey-strategic-consultant.md"),
    },
    AgentDefinition {
        name: "claude-code-guide",
        display_name: "Claude Code Guide",
        system_prompt: include_str!("prompts/claude-code-guide.md"),
    },
    AgentDefinition {
        name: "statusline-setup",
        display_name: "Statusline Setup",
        system_prompt: include_str!("prompts/statusline-setup.md"),
    },
];
