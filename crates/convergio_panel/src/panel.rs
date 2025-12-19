//! Convergio Panel - displays list of AI agents organized by category

use crate::ConvergioPanelSettings;
use agent_ui::{AgentPanel, ExternalAgent, NewExternalAgentThread};
use anyhow::Result;
use collections::{HashMap, HashSet};
use db::kvp::KEY_VALUE_STORE;
use editor::Editor;
use gpui::{
    actions, div, prelude::*, Action, App, AsyncWindowContext, Context, Entity,
    EventEmitter, FocusHandle, Focusable, InteractiveElement, ParentElement, Pixels,
    Render, SharedString, Styled, Subscription, WeakEntity, Window,
};
use project::Fs;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ui::{prelude::*, checkbox, Disclosure, Icon, IconName, Label, ListHeader, ListItem, ToggleState};
use util::ResultExt;
use workspace::{
    Workspace,
    dock::{DockPosition, Panel, PanelEvent},
};

const CONVERGIO_PANEL_KEY: &str = "ConvergioPanel";

actions!(
    convergio_panel,
    [
        ToggleFocus,
    ]
);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AgentCategory {
    Leadership,
    Finance,
    Technology,
    Security,
    Product,
    Data,
    Strategy,
    Marketing,
    ProjectManagement,
    HrPeople,
    Operations,
    External,
    AiCoordination,
    Executive,
}

impl AgentCategory {
    fn all() -> Vec<Self> {
        vec![
            Self::Leadership,
            Self::Finance,
            Self::Technology,
            Self::Security,
            Self::Product,
            Self::Data,
            Self::Strategy,
            Self::Marketing,
            Self::ProjectManagement,
            Self::HrPeople,
            Self::Operations,
            Self::External,
            Self::AiCoordination,
            Self::Executive,
        ]
    }

    fn display_name(&self) -> &'static str {
        match self {
            Self::Leadership => "Leadership & Orchestration",
            Self::Finance => "Finance & Business",
            Self::Technology => "Technology & Architecture",
            Self::Security => "Security & Compliance",
            Self::Product => "Product & Design",
            Self::Data => "Data & Analytics",
            Self::Strategy => "Strategy & Decisions",
            Self::Marketing => "Marketing & Sales",
            Self::ProjectManagement => "Project Management",
            Self::HrPeople => "HR & People",
            Self::Operations => "Operations & Process",
            Self::External => "Government & External",
            Self::AiCoordination => "AI & Coordination",
            Self::Executive => "Executive Support",
        }
    }

    fn icon(&self) -> IconName {
        match self {
            Self::Leadership => IconName::ZedAgent,
            Self::Finance => IconName::Hash,
            Self::Technology => IconName::Code,
            Self::Security => IconName::ShieldCheck,
            Self::Product => IconName::SwatchBook,
            Self::Data => IconName::DatabaseZap,
            Self::Strategy => IconName::Crosshair,
            Self::Marketing => IconName::Sparkle,
            Self::ProjectManagement => IconName::ListTodo,
            Self::HrPeople => IconName::UserGroup,
            Self::Operations => IconName::Cog,
            Self::External => IconName::Public,
            Self::AiCoordination => IconName::Ai,
            Self::Executive => IconName::Bell,
        }
    }

    /// Returns the accent color for this category (HSLA format)
    fn color(&self) -> gpui::Hsla {
        use gpui::hsla;
        match self {
            Self::Leadership => hsla(0.08, 0.9, 0.55, 1.0),      // Gold/Orange - leadership
            Self::Finance => hsla(0.33, 0.7, 0.45, 1.0),         // Green - money
            Self::Technology => hsla(0.58, 0.8, 0.50, 1.0),      // Blue - tech
            Self::Security => hsla(0.0, 0.75, 0.50, 1.0),        // Red - security
            Self::Product => hsla(0.83, 0.7, 0.55, 1.0),         // Purple - design
            Self::Data => hsla(0.50, 0.7, 0.50, 1.0),            // Cyan - data
            Self::Strategy => hsla(0.75, 0.6, 0.50, 1.0),        // Violet - strategy
            Self::Marketing => hsla(0.92, 0.8, 0.55, 1.0),       // Pink - marketing
            Self::ProjectManagement => hsla(0.17, 0.7, 0.50, 1.0), // Yellow - PM
            Self::HrPeople => hsla(0.42, 0.6, 0.50, 1.0),        // Teal - people
            Self::Operations => hsla(0.67, 0.5, 0.50, 1.0),      // Indigo - ops
            Self::External => hsla(0.25, 0.6, 0.45, 1.0),        // Lime - external
            Self::AiCoordination => hsla(0.55, 0.9, 0.60, 1.0),  // Bright blue - AI
            Self::Executive => hsla(0.08, 0.8, 0.60, 1.0),       // Amber - exec
        }
    }
}

#[derive(Clone, Debug)]
pub struct ConvergioAgent {
    pub name: String,
    pub server_name: String,
    pub display_name: String,
    pub description: String,
    pub icon: IconName,
    pub category: AgentCategory,
    pub skills: Vec<String>,
}

impl ConvergioAgent {
    fn all_agents() -> Vec<Self> {
        vec![
            // Leadership & Orchestration
            Self::new("ali", "Ali - Chief of Staff", "Master orchestrator, coordinates all agents", IconName::ZedAgent, AgentCategory::Leadership, vec!["orchestration", "coordination", "strategy"]),
            Self::new("satya-board-of-directors", "Satya - Board", "System-thinking strategist, transformation", IconName::Star, AgentCategory::Leadership, vec!["strategy", "transformation", "leadership"]),

            // Finance & Business
            Self::new("amy-cfo", "Amy - CFO", "Financial strategy, ROI analysis", IconName::Hash, AgentCategory::Finance, vec!["finance", "roi", "investment"]),
            Self::new("fiona-market-analyst", "Fiona - Markets", "Financial markets, stock research", IconName::ArrowUpRight, AgentCategory::Finance, vec!["markets", "stocks", "analysis"]),
            Self::new("wiz-investor-venture-capital", "Wiz - VC", "Investment strategy, portfolio", IconName::BoltOutlined, AgentCategory::Finance, vec!["investment", "vc", "startups"]),
            Self::new("michael-vc", "Michael - Corporate VC", "Corporate ventures, M&A", IconName::StarFilled, AgentCategory::Finance, vec!["corporate", "ma", "ventures"]),

            // Technology & Architecture
            Self::new("baccio-tech-architect", "Baccio - Architect", "System design, DDD, Clean Architecture", IconName::Blocks, AgentCategory::Technology, vec!["architecture", "design", "ddd"]),
            Self::new("dario-debugger", "Dario - Debugger", "Root cause analysis, troubleshooting", IconName::Debug, AgentCategory::Technology, vec!["debugging", "troubleshooting", "analysis"]),
            Self::new("rex-code-reviewer", "Rex - Reviewer", "Code quality, design patterns", IconName::Code, AgentCategory::Technology, vec!["code review", "quality", "patterns"]),
            Self::new("paolo-best-practices-enforcer", "Paolo - Standards", "Coding standards, best practices", IconName::Check, AgentCategory::Technology, vec!["standards", "best practices", "quality"]),
            Self::new("marco-devops-engineer", "Marco - DevOps", "CI/CD, Kubernetes, IaC", IconName::Terminal, AgentCategory::Technology, vec!["devops", "cicd", "kubernetes"]),
            Self::new("otto-performance-optimizer", "Otto - Performance", "Profiling, bottleneck analysis", IconName::BoltFilled, AgentCategory::Technology, vec!["performance", "optimization", "profiling"]),
            Self::new("dan-engineering-gm", "Dan - Eng GM", "Engineering leadership, team building", IconName::Server, AgentCategory::Technology, vec!["engineering", "leadership", "teams"]),

            // Security & Compliance
            Self::new("luca-security-expert", "Luca - Security", "Penetration testing, Zero-Trust", IconName::ShieldCheck, AgentCategory::Security, vec!["security", "penetration", "zero-trust"]),
            Self::new("elena-legal-compliance-expert", "Elena - Legal", "GDPR, contracts, risk management", IconName::FileDoc, AgentCategory::Security, vec!["legal", "gdpr", "compliance"]),
            Self::new("dr-enzo-healthcare-compliance-manager", "Enzo - Healthcare", "HIPAA, FDA, medical devices", IconName::Plus, AgentCategory::Security, vec!["hipaa", "fda", "healthcare"]),
            Self::new("guardian-ai-security-validator", "Guardian - AI Security", "AI/ML security, bias detection", IconName::AiClaude, AgentCategory::Security, vec!["ai security", "bias", "ethics"]),
            Self::new("thor-quality-assurance-guardian", "Thor - QA", "Quality watchdog, ISE testing", IconName::TodoComplete, AgentCategory::Security, vec!["qa", "testing", "quality"]),

            // Product & Design
            Self::new("marcello-pm", "Marcello - Product", "Product strategy, roadmap", IconName::FileTextOutlined, AgentCategory::Product, vec!["product", "roadmap", "strategy"]),
            Self::new("oliver-pm", "Oliver - Sr PM", "Product leadership, vision", IconName::FileTextFilled, AgentCategory::Product, vec!["product", "leadership", "vision"]),
            Self::new("sara-ux-ui-designer", "Sara - UX/UI", "User experience, WCAG", IconName::SwatchBook, AgentCategory::Product, vec!["ux", "ui", "design"]),
            Self::new("jenny-inclusive-accessibility-champion", "Jenny - A11y", "Accessibility, inclusive design", IconName::Person, AgentCategory::Product, vec!["accessibility", "wcag", "inclusive"]),
            Self::new("jony-creative-director", "Jony - Creative", "Creative strategy, brand", IconName::Sparkle, AgentCategory::Product, vec!["creative", "brand", "design"]),
            Self::new("stefano-design-thinking-facilitator", "Stefano - Design", "Design thinking, workshops", IconName::Pencil, AgentCategory::Product, vec!["design thinking", "workshops", "ideation"]),

            // Data & Analytics
            Self::new("omri-data-scientist", "Omri - Data", "ML, predictive modeling, AI", IconName::Ai, AgentCategory::Data, vec!["ml", "data science", "ai"]),
            Self::new("angela-da", "Angela - Analytics", "Decision frameworks, analytics", IconName::DatabaseZap, AgentCategory::Data, vec!["analytics", "decisions", "data"]),
            Self::new("ethan-da", "Ethan - Analytics", "Strategic analytics, trade-offs", IconName::ArrowRightLeft, AgentCategory::Data, vec!["analytics", "strategy", "analysis"]),
            Self::new("evan-ic6da", "Evan - Sr Analytics", "Enterprise decisions, alignment", IconName::Crosshair, AgentCategory::Data, vec!["enterprise", "decisions", "alignment"]),
            Self::new("ava-analytics-insights-virtuoso", "Ava - Insights", "Ecosystem intelligence, patterns", IconName::Eye, AgentCategory::Data, vec!["insights", "patterns", "intelligence"]),

            // Strategy & Decision Making
            Self::new("matteo-strategic-business-architect", "Matteo - Strategy", "Business model, positioning", IconName::Crosshair, AgentCategory::Strategy, vec!["business", "strategy", "positioning"]),
            Self::new("antonio-strategy-expert", "Antonio - Strategy", "OKR, Lean, Agile, SWOT", IconName::ListTodo, AgentCategory::Strategy, vec!["okr", "lean", "agile"]),
            Self::new("domik-mckinsey-strategic-decision-maker", "Domik - McKinsey", "Quantitative analysis, ISE", IconName::Hash, AgentCategory::Strategy, vec!["mckinsey", "analysis", "decisions"]),
            Self::new("socrates-first-principles-reasoning", "Socrates - Reasoning", "First principles, Socratic method", IconName::CircleHelp, AgentCategory::Strategy, vec!["first principles", "reasoning", "analysis"]),

            // Marketing & Sales
            Self::new("sofia-marketing-strategist", "Sofia - Marketing", "Digital marketing, growth", IconName::Sparkle, AgentCategory::Marketing, vec!["marketing", "digital", "growth"]),
            Self::new("fabio-sales-business-development", "Fabio - Sales", "Revenue growth, partnerships", IconName::ArrowUpRight, AgentCategory::Marketing, vec!["sales", "partnerships", "revenue"]),
            Self::new("riccardo-storyteller", "Riccardo - Stories", "Brand narratives, content", IconName::Quote, AgentCategory::Marketing, vec!["storytelling", "content", "brand"]),
            Self::new("steve-executive-communication-strategist", "Steve - Comms", "C-suite messaging, stakeholders", IconName::Envelope, AgentCategory::Marketing, vec!["communication", "executive", "stakeholders"]),

            // Project & Program Management
            Self::new("davide-project-manager", "Davide - PM", "Agile, Scrum, Waterfall", IconName::ListTodo, AgentCategory::ProjectManagement, vec!["project", "agile", "scrum"]),
            Self::new("luke-program-manager", "Luke - Program", "Portfolio management, coordination", IconName::Blocks, AgentCategory::ProjectManagement, vec!["program", "portfolio", "coordination"]),
            Self::new("taskmaster-strategic-task-decomposition-master", "Taskmaster", "Task decomposition, OKR", IconName::TodoProgress, AgentCategory::ProjectManagement, vec!["tasks", "decomposition", "planning"]),

            // HR & People
            Self::new("giulia-hr-talent-acquisition", "Giulia - HR", "Recruitment, talent strategy", IconName::Person, AgentCategory::HrPeople, vec!["hr", "recruitment", "talent"]),
            Self::new("coach-team-coach", "Coach - Teams", "Team building, performance", IconName::UserGroup, AgentCategory::HrPeople, vec!["coaching", "teams", "performance"]),
            Self::new("behice-cultural-coach", "Behice - Culture", "Cross-cultural, global teams", IconName::Public, AgentCategory::HrPeople, vec!["culture", "global", "diversity"]),

            // Operations & Process
            Self::new("enrico-business-process-engineer", "Enrico - Process", "Workflow, automation", IconName::Cog, AgentCategory::Operations, vec!["process", "workflow", "automation"]),
            Self::new("dave-change-management-specialist", "Dave - Change", "Transformation, adoption", IconName::RotateCw, AgentCategory::Operations, vec!["change", "transformation", "adoption"]),
            Self::new("andrea-customer-success-manager", "Andrea - CS", "Customer lifecycle, retention", IconName::UserCheck, AgentCategory::Operations, vec!["customer", "success", "retention"]),

            // Government & External
            Self::new("sophia-govaffairs", "Sophia - GovAffairs", "Regulatory, policy advocacy", IconName::Public, AgentCategory::External, vec!["government", "regulatory", "policy"]),
            Self::new("sam-startupper", "Sam - Startup", "Y Combinator, PMF, fundraising", IconName::BoltOutlined, AgentCategory::External, vec!["startup", "yc", "fundraising"]),

            // AI & Coordination
            Self::new("po-prompt-optimizer", "PO - Prompts", "Prompt engineering, LLM", IconName::TextSnippet, AgentCategory::AiCoordination, vec!["prompts", "llm", "ai"]),
            Self::new("wanda-workflow-orchestrator", "Wanda - Workflows", "Multi-agent workflows", IconName::Blocks, AgentCategory::AiCoordination, vec!["workflows", "orchestration", "agents"]),
            Self::new("xavier-coordination-patterns", "Xavier - Coordination", "Swarm intelligence, patterns", IconName::GitBranch, AgentCategory::AiCoordination, vec!["coordination", "swarm", "patterns"]),
            Self::new("marcus-context-memory-keeper", "Marcus - Memory", "Context persistence, memory", IconName::DatabaseZap, AgentCategory::AiCoordination, vec!["memory", "context", "persistence"]),
            Self::new("diana-performance-dashboard", "Diana - Dashboard", "Agent analytics, KPIs", IconName::Screen, AgentCategory::AiCoordination, vec!["dashboard", "analytics", "kpis"]),

            // Executive Support
            Self::new("anna-executive-assistant", "Anna - EA", "Task management, scheduling", IconName::Bell, AgentCategory::Executive, vec!["assistant", "scheduling", "tasks"]),
        ]
    }

    fn new(name: &str, display_name: &str, description: &str, icon: IconName, category: AgentCategory, skills: Vec<&str>) -> Self {
        let server_part = display_name.split(" - ").next().unwrap_or(name);
        let server_name = format!("Convergio-{}", server_part);
        Self {
            name: name.to_string(),
            server_name,
            display_name: display_name.to_string(),
            description: description.to_string(),
            icon,
            category,
            skills: skills.into_iter().map(|s| s.to_string()).collect(),
        }
    }

    fn matches_query(&self, query: &str) -> bool {
        if query.is_empty() {
            return true;
        }
        let query_lower = query.to_lowercase();
        self.display_name.to_lowercase().contains(&query_lower)
            || self.description.to_lowercase().contains(&query_lower)
            || self.skills.iter().any(|s| s.to_lowercase().contains(&query_lower))
    }
}

#[derive(Serialize, Deserialize)]
struct SerializedConvergioPanel {
    width: Option<f32>,
    collapsed_categories: Vec<String>,
    #[serde(default)]
    has_seen_onboarding: bool,
}

const AGENT_SESSIONS_KEY: &str = "ConvergioPanelAgentSessions";

#[derive(Serialize, Deserialize, Default)]
struct SerializedAgentSessions {
    sessions: HashMap<String, String>,
}

pub struct ConvergioPanel {
    focus_handle: FocusHandle,
    workspace: WeakEntity<Workspace>,
    _fs: Arc<dyn Fs>,
    width: Option<Pixels>,
    agents: Vec<ConvergioAgent>,
    selected_index: Option<usize>,
    collapsed_categories: HashSet<AgentCategory>,
    filter_editor: Entity<Editor>,
    filter_query: String,
    active_agents: HashSet<String>,
    show_active_only: bool,
    // Track session IDs for conversation persistence
    // Maps agent_name -> last_session_id (for future use when thread creation events are available)
    _agent_sessions: HashMap<String, String>,
    _filter_editor_subscription: Subscription,
    // Onboarding state
    show_onboarding: bool,
}

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace.register_action(|workspace, _: &ToggleFocus, window, cx| {
            workspace.toggle_panel_focus::<ConvergioPanel>(window, cx);
        });
    })
    .detach();
}

impl ConvergioPanel {
    pub fn new(workspace: &Workspace, workspace_handle: WeakEntity<Workspace>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let fs = workspace.app_state().fs.clone();
        let focus_handle = cx.focus_handle();
        let agents = ConvergioAgent::all_agents();

        let filter_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Search agents...", window, cx);
            editor
        });

        let subscription = cx.subscribe(&filter_editor, |this, _, event: &editor::EditorEvent, cx| {
            if let editor::EditorEvent::BufferEdited { .. } = event {
                this.on_filter_changed(cx);
            }
        });

        // Load persisted agent sessions asynchronously
        cx.spawn(async move |this, cx| {
            let key = AGENT_SESSIONS_KEY.to_string();
            let result: anyhow::Result<Option<String>> = cx.background_executor().spawn(async move {
                KEY_VALUE_STORE.read_kvp(&key)
            }).await;

            if let Ok(Some(serialized)) = result {
                if let Ok(data) = serde_json::from_str::<SerializedAgentSessions>(&serialized) {
                    let _ = this.update(cx, |this, _| {
                        this._agent_sessions = data.sessions;
                        log::info!("Loaded {} persisted agent sessions", this._agent_sessions.len());
                    });
                }
            }
        }).detach();

        // Load onboarding state
        let panel_key = CONVERGIO_PANEL_KEY.to_string();
        cx.spawn(async move |this, cx| {
            let result: anyhow::Result<Option<String>> = cx.background_executor().spawn(async move {
                KEY_VALUE_STORE.read_kvp(&panel_key)
            }).await;

            if let Ok(Some(serialized)) = result {
                if let Ok(data) = serde_json::from_str::<SerializedConvergioPanel>(&serialized) {
                    let _ = this.update(cx, |this, _| {
                        this.show_onboarding = !data.has_seen_onboarding;
                    });
                }
            }
        }).detach();

        Self {
            focus_handle,
            workspace: workspace_handle,
            _fs: fs,
            width: None,
            agents,
            selected_index: None,
            collapsed_categories: HashSet::default(),
            filter_editor,
            filter_query: String::new(),
            active_agents: HashSet::default(),
            show_active_only: false,
            _agent_sessions: HashMap::default(),
            _filter_editor_subscription: subscription,
            show_onboarding: true, // Show by default, will be updated async
        }
    }

    pub async fn load(
        workspace: WeakEntity<Workspace>,
        mut cx: AsyncWindowContext,
    ) -> Result<Entity<Self>> {
        let workspace_handle = workspace.clone();
        workspace.update_in(&mut cx, |workspace, window, cx| {
            cx.new(|cx| ConvergioPanel::new(workspace, workspace_handle, window, cx))
        })
    }

    fn on_filter_changed(&mut self, cx: &mut Context<Self>) {
        self.filter_query = self.filter_editor.read(cx).text(cx);
        cx.notify();
    }

    fn toggle_category(&mut self, category: AgentCategory, cx: &mut Context<Self>) {
        if self.collapsed_categories.contains(&category) {
            self.collapsed_categories.remove(&category);
        } else {
            self.collapsed_categories.insert(category);
        }
        cx.notify();
        self.serialize(cx);
    }

    fn toggle_active_filter(&mut self, cx: &mut Context<Self>) {
        self.show_active_only = !self.show_active_only;
        cx.notify();
    }

    /// Opens a Convergio agent thread, attempting to resume an existing conversation if available.
    fn open_agent_thread(&mut self, agent_name: &str, server_name: &str, window: &mut Window, cx: &mut Context<Self>) {
        // Mark agent as active
        self.active_agents.insert(agent_name.to_string());
        log::info!("Opening Convergio agent chat: {}", agent_name);

        // Try to find existing thread in history by agent_name
        let mut found_thread_id: Option<String> = None;

        if let Some(workspace) = self.workspace.upgrade() {
            if let Some(panel) = workspace.read(cx).panel::<AgentPanel>(cx) {
                // Search by full server name (e.g., "Convergio-Ali") as stored in DB
                if let Some(thread) = panel.read(cx).history_store.read(cx).thread_by_agent_name(server_name) {
                    found_thread_id = Some(thread.id.to_string());
                    log::info!("Found existing thread {} for agent {}", thread.id, server_name);
                }
            }
        }

        if let Some(thread_id) = found_thread_id {
            // Resume existing conversation
            log::info!("Resuming thread {} for {}", thread_id, agent_name);
            let action = NewExternalAgentThread::resume(
                ExternalAgent::Custom { name: server_name.to_string().into() },
                thread_id
            );
            window.dispatch_action(action.boxed_clone(), cx);
        } else {
            // Create new thread
            log::info!("Creating new thread for {}", agent_name);
            let action = NewExternalAgentThread::with_agent(ExternalAgent::Custom {
                name: server_name.to_string().into()
            });
            window.dispatch_action(action.boxed_clone(), cx);
        }
        cx.notify();
    }

    /// Stores a session ID for an agent and persists it to storage
    pub fn store_agent_session(&mut self, agent_name: &str, session_id: String, cx: &mut Context<Self>) {
        self._agent_sessions.insert(agent_name.to_string(), session_id);
        self.save_agent_sessions(cx);
    }

    fn save_agent_sessions(&self, cx: &mut Context<Self>) {
        let sessions = self._agent_sessions.clone();
        cx.background_executor()
            .spawn(async move {
                let data = SerializedAgentSessions { sessions };
                if let Ok(serialized) = serde_json::to_string(&data) {
                    KEY_VALUE_STORE
                        .write_kvp(AGENT_SESSIONS_KEY.to_string(), serialized)
                        .await
                        .log_err();
                }
            })
            .detach();
    }

    fn dismiss_onboarding(&mut self, cx: &mut Context<Self>) {
        self.show_onboarding = false;
        self.serialize(cx);
        cx.notify();
    }

    fn serialize(&self, cx: &mut Context<Self>) {
        let width = self.width.map(|w| f32::from(w));
        let collapsed: Vec<String> = self.collapsed_categories
            .iter()
            .map(|c| format!("{:?}", c))
            .collect();
        let has_seen_onboarding = !self.show_onboarding;
        cx.background_executor()
            .spawn(async move {
                let serialized = serde_json::to_string(&SerializedConvergioPanel {
                    width,
                    collapsed_categories: collapsed,
                    has_seen_onboarding,
                }).ok();
                if let Some(serialized) = serialized {
                    KEY_VALUE_STORE
                        .write_kvp(CONVERGIO_PANEL_KEY.to_string(), serialized)
                        .await
                        .log_err();
                }
            })
            .detach();
    }

    fn render_onboarding(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("convergio-onboarding")
            .size_full()
            .flex()
            .flex_col()
            .bg(cx.theme().colors().panel_background)
            .p_4()
            .gap_4()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_2()
                    .child(
                        Icon::new(IconName::ZedAgent)
                            .size(IconSize::XLarge)
                            .color(Color::Accent)
                    )
                    .child(
                        Label::new("Welcome to Convergio")
                            .size(LabelSize::Large)
                            .weight(gpui::FontWeight::BOLD)
                    )
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(
                        div()
                            .flex()
                            .items_start()
                            .gap_2()
                            .child(
                                Icon::new(IconName::UserGroup)
                                    .size(IconSize::Small)
                                    .color(Color::Accent)
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(
                                        Label::new("54 Specialized Agents")
                                            .size(LabelSize::Default)
                                            .weight(gpui::FontWeight::MEDIUM)
                                    )
                                    .child(
                                        Label::new("From CFO to DevOps, each agent brings deep expertise")
                                            .size(LabelSize::Small)
                                            .color(Color::Muted)
                                    )
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .items_start()
                            .gap_2()
                            .child(
                                Icon::new(IconName::Ai)
                                    .size(IconSize::Small)
                                    .color(Color::Accent)
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(
                                        Label::new("Ali - Your Chief of Staff")
                                            .size(LabelSize::Default)
                                            .weight(gpui::FontWeight::MEDIUM)
                                    )
                                    .child(
                                        Label::new("Orchestrates all agents and maintains context")
                                            .size(LabelSize::Small)
                                            .color(Color::Muted)
                                    )
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .items_start()
                            .gap_2()
                            .child(
                                Icon::new(IconName::Chat)
                                    .size(IconSize::Small)
                                    .color(Color::Accent)
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(
                                        Label::new("Persistent Conversations")
                                            .size(LabelSize::Default)
                                            .weight(gpui::FontWeight::MEDIUM)
                                    )
                                    .child(
                                        Label::new("Agents remember your previous discussions")
                                            .size(LabelSize::Small)
                                            .color(Color::Muted)
                                    )
                            )
                    )
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .mt_4()
                    .child(
                        Label::new("Quick Start:")
                            .size(LabelSize::Small)
                            .weight(gpui::FontWeight::MEDIUM)
                    )
                    .child(
                        Label::new("1. Click any agent to start a conversation")
                            .size(LabelSize::Small)
                            .color(Color::Muted)
                    )
                    .child(
                        Label::new("2. Use search to find agents by skill")
                            .size(LabelSize::Small)
                            .color(Color::Muted)
                    )
                    .child(
                        Label::new("3. Ali stays aware of all your conversations")
                            .size(LabelSize::Small)
                            .color(Color::Muted)
                    )
            )
            .child(
                div()
                    .mt_auto()
                    .pt_4()
                    .child(
                        div()
                            .id("get-started-btn")
                            .px_4()
                            .py_2()
                            .rounded_md()
                            .bg(cx.theme().colors().element_selected)
                            .cursor_pointer()
                            .flex()
                            .justify_center()
                            .child(
                                Label::new("Get Started")
                                    .size(LabelSize::Default)
                                    .weight(gpui::FontWeight::MEDIUM)
                            )
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.dismiss_onboarding(cx);
                            }))
                    )
            )
    }

    fn filtered_agents(&self) -> Vec<&ConvergioAgent> {
        self.agents
            .iter()
            .filter(|agent| {
                let matches_query = agent.matches_query(&self.filter_query);
                let matches_active = if self.show_active_only {
                    self.active_agents.contains(&agent.name)
                } else {
                    true
                };
                matches_query && matches_active
            })
            .collect()
    }

    fn agents_in_category(&self, category: AgentCategory) -> Vec<&ConvergioAgent> {
        self.filtered_agents()
            .into_iter()
            .filter(|agent| agent.category == category)
            .collect()
    }

    fn active_count(&self) -> usize {
        self.active_agents.len()
    }

    fn render_category_header(&self, category: AgentCategory, agent_count: usize, active_in_cat: usize, cx: &Context<Self>) -> impl IntoElement {
        let is_collapsed = self.collapsed_categories.contains(&category);
        let cat = category;
        let category_color = category.color();

        div()
            .id(SharedString::from(format!("category-{:?}", category)))
            .flex()
            .items_center()
            .gap_1()
            .px_2()
            .py_1()
            .bg(cx.theme().colors().surface_background)
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .cursor_pointer()
            .on_click(cx.listener(move |this, _, _, cx| {
                this.toggle_category(cat, cx);
            }))
            .child(
                Disclosure::new(SharedString::from(format!("disc-{:?}", category)), !is_collapsed)
            )
            .child(
                div()
                    .w_1()
                    .h_4()
                    .rounded_sm()
                    .bg(category_color)
                    .mr_1()
            )
            .child(
                Icon::new(category.icon())
                    .size(IconSize::Small)
                    .color(Color::Accent)
            )
            .child(
                Label::new(category.display_name())
                    .size(LabelSize::Small)
                    .weight(gpui::FontWeight::MEDIUM)
            )
            .child(
                Label::new(format!("({})", agent_count))
                    .size(LabelSize::XSmall)
                    .color(Color::Muted)
            )
            .when(active_in_cat > 0, |this| {
                this.child(
                    div()
                        .ml_1()
                        .px_1()
                        .rounded_sm()
                        .bg(cx.theme().status().success)
                        .child(
                            Label::new(format!("{} active", active_in_cat))
                                .size(LabelSize::XSmall)
                                .color(Color::Default)
                        )
                )
            })
    }

    fn render_agent(&self, agent: &ConvergioAgent, global_ix: usize, cx: &Context<Self>) -> impl IntoElement {
        let is_selected = self.selected_index == Some(global_ix);
        let is_active = self.active_agents.contains(&agent.name);
        let agent_name = agent.name.clone();
        let server_name = agent.server_name.clone();

        ListItem::new(global_ix)
            .inset(true)
            .spacing(ui::ListItemSpacing::Sparse)
            .toggle_state(is_selected)
            .start_slot(
                div()
                    .pl_4()
                    .flex()
                    .items_center()
                    .gap_1()
                    .when(is_active, |this| {
                        this.child(
                            div()
                                .w_2()
                                .h_2()
                                .rounded_full()
                                .bg(cx.theme().status().success)
                        )
                    })
                    .child(
                        Icon::new(agent.icon)
                            .size(IconSize::Small)
                            .color(if is_active { Color::Accent } else { Color::Muted })
                    )
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1()
                            .child(Label::new(agent.display_name.clone()).size(LabelSize::Small))
                            .when(is_active, |this| {
                                this.child(
                                    Label::new("●")
                                        .size(LabelSize::XSmall)
                                        .color(Color::Success)
                                )
                            })
                    )
                    .child(
                        Label::new(agent.description.clone())
                            .size(LabelSize::XSmall)
                            .color(Color::Muted)
                    )
            )
            .on_click(cx.listener(move |this, _, window, cx| {
                this.selected_index = Some(global_ix);
                this.open_agent_thread(&agent_name, &server_name, window, cx);
            }))
    }

    fn render_search_bar(&self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let active_count = self.active_count();

        div()
            .p_2()
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .flex()
            .flex_col()
            .gap_2()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .px_2()
                    .py_1()
                    .rounded_md()
                    .bg(cx.theme().colors().editor_background)
                    .child(
                        Icon::new(IconName::MagnifyingGlass)
                            .size(IconSize::Small)
                            .color(Color::Muted)
                    )
                    .child(
                        div()
                            .flex_1()
                            .child(self.filter_editor.clone())
                    )
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        checkbox(
                            "active-filter",
                            if self.show_active_only {
                                ToggleState::Selected
                            } else {
                                ToggleState::Unselected
                            },
                        )
                        .label("Active only")
                        .label_size(LabelSize::Small)
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.toggle_active_filter(cx);
                        }))
                    )
                    .when(active_count > 0, |this| {
                        this.child(
                            Label::new(format!("{} active", active_count))
                                .size(LabelSize::XSmall)
                                .color(Color::Success)
                        )
                    })
            )
    }
}

impl Focusable for ConvergioPanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<PanelEvent> for ConvergioPanel {}

impl Panel for ConvergioPanel {
    fn persistent_name() -> &'static str {
        "ConvergioPanel"
    }

    fn panel_key() -> &'static str {
        "convergio_panel"
    }

    fn position(&self, _window: &Window, cx: &App) -> DockPosition {
        ConvergioPanelSettings::get_global(cx).dock
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(&mut self, _position: DockPosition, _window: &mut Window, _cx: &mut Context<Self>) {
        // Settings update will be added later
    }

    fn size(&self, _window: &Window, cx: &App) -> Pixels {
        self.width
            .unwrap_or_else(|| ConvergioPanelSettings::get_global(cx).default_width)
    }

    fn set_size(&mut self, size: Option<Pixels>, window: &mut Window, cx: &mut Context<Self>) {
        self.width = size;
        cx.notify();
        cx.defer_in(window, |this, _, cx| {
            this.serialize(cx);
        });
    }

    fn icon(&self, _window: &Window, cx: &App) -> Option<IconName> {
        ConvergioPanelSettings::get_global(cx)
            .button
            .then_some(IconName::UserGroup)
    }

    fn icon_tooltip(&self, _window: &Window, _cx: &App) -> Option<&'static str> {
        Some("Convergio Agents")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }

    fn activation_priority(&self) -> u32 {
        3
    }
}

impl Render for ConvergioPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Show onboarding if not dismissed
        if self.show_onboarding {
            return self.render_onboarding(cx).into_any_element();
        }

        let mut global_ix = 0usize;

        div()
            .id("convergio-panel")
            .size_full()
            .flex()
            .flex_col()
            .bg(cx.theme().colors().panel_background)
            .child(
                div()
                    .p_2()
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .child(
                        ListHeader::new("Convergio Agents")
                            .inset(true)
                    )
            )
            .child(self.render_search_bar(window, cx))
            .child(
                div()
                    .id("convergio-agents-list")
                    .flex_1()
                    .overflow_y_scroll()
                    .children(
                        AgentCategory::all()
                            .into_iter()
                            .filter_map(|category| {
                                let agents_in_cat = self.agents_in_category(category);
                                if agents_in_cat.is_empty() {
                                    return None;
                                }

                                let active_in_cat = agents_in_cat
                                    .iter()
                                    .filter(|a| self.active_agents.contains(&a.name))
                                    .count();

                                let is_collapsed = self.collapsed_categories.contains(&category);
                                let header = self.render_category_header(category, agents_in_cat.len(), active_in_cat, cx);

                                let agent_elements: Vec<_> = if is_collapsed {
                                    vec![]
                                } else {
                                    agents_in_cat
                                        .into_iter()
                                        .map(|agent| {
                                            let element = self.render_agent(agent, global_ix, cx);
                                            global_ix += 1;
                                            element.into_any_element()
                                        })
                                        .collect()
                                };

                                Some(
                                    div()
                                        .child(header)
                                        .children(agent_elements)
                                        .into_any_element()
                                )
                            })
                    )
            )
            .into_any_element()
    }
}
