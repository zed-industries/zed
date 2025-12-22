//! Convergio Panel - displays list of AI agents organized by category

use crate::chat_view::ConvergioChatView;
use crate::ConvergioSettings;
use agent::HistoryStore;
use anyhow::Result;
use collections::{HashMap, HashSet};
use db::kvp::KEY_VALUE_STORE;
use editor::Editor;
use gpui::{
    actions, div, prelude::*, Action, App, AsyncWindowContext, Context, Entity,
    EventEmitter, FocusHandle, Focusable, InteractiveElement, KeyContext, ParentElement, Pixels,
    Render, SharedString, Styled, Subscription, WeakEntity, Window,
};
use menu::{Confirm, SelectFirst, SelectLast, SelectNext, SelectPrevious};
use project::Fs;
use serde::{Deserialize, Serialize};
use settings::Settings;
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

/// Agent Pack presets for different use cases
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AgentPack {
    /// All 54 agents - full enterprise experience
    #[default]
    Enterprise,
    /// Core agents for startups: Ali, Baccio, Dario, Rex, Amy, Marcello
    Startup,
    /// Developer-focused: Ali, Baccio, Dario, Rex, Paolo, Guardian
    Developer,
    /// Education-focused: accessibility, coaching, and learning agents
    Education,
    /// Minimal: Just Ali
    Minimal,
    /// Custom selection (uses active_agents filter)
    Custom,
}

impl AgentPack {
    #[allow(dead_code)]
    fn all() -> Vec<Self> {
        vec![
            Self::Enterprise,
            Self::Startup,
            Self::Developer,
            Self::Education,
            Self::Minimal,
            Self::Custom,
        ]
    }

    fn display_name(&self) -> &'static str {
        match self {
            Self::Enterprise => "Enterprise (All 54)",
            Self::Startup => "Startup (6 core)",
            Self::Developer => "Developer (6 tech)",
            Self::Education => "Education (12 focus)",
            Self::Minimal => "Minimal (Ali only)",
            Self::Custom => "Custom",
        }
    }

    #[allow(dead_code)]
    fn description(&self) -> &'static str {
        match self {
            Self::Enterprise => "Full access to all agents for enterprise teams",
            Self::Startup => "Essential agents for fast-moving startups",
            Self::Developer => "Tech-focused agents for development workflows",
            Self::Education => "Accessibility-focused agents for educational environments",
            Self::Minimal => "Just Ali as your AI Chief of Staff",
            Self::Custom => "Your custom agent selection",
        }
    }

    /// Returns the agent names included in this pack
    fn included_agents(&self) -> Option<Vec<&'static str>> {
        match self {
            Self::Enterprise => None, // All agents
            Self::Startup => Some(vec![
                "ali", "baccio-tech-architect", "dario-debugger",
                "rex-code-reviewer", "amy-cfo", "marcello-pm"
            ]),
            Self::Developer => Some(vec![
                "ali", "baccio-tech-architect", "dario-debugger",
                "rex-code-reviewer", "paolo-best-practices-enforcer", "guardian-ai-security-validator"
            ]),
            Self::Education => Some(vec![
                "ali", "jenny-inclusive-accessibility-champion", "coach-team-coach",
                "riccardo-storyteller", "behice-cultural-coach", "socrates-first-principles-reasoning",
                "stefan-design-thinking-facilitator", "sara-ux-ui-designer", "thor-quality-assurance-guardian",
                "marcello-pm", "davide-project-manager", "anna-executive-assistant"
            ]),
            Self::Minimal => Some(vec!["ali"]),
            Self::Custom => None, // Uses active_agents filter
        }
    }

    fn icon(&self) -> IconName {
        match self {
            Self::Enterprise => IconName::UserGroup,
            Self::Startup => IconName::Sparkle,
            Self::Developer => IconName::Code,
            Self::Education => IconName::Ai,
            Self::Minimal => IconName::ConvergioAli,
            Self::Custom => IconName::Settings,
        }
    }
}

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
    pub name: SharedString,
    pub server_name: SharedString,
    pub display_name: SharedString,
    pub description: SharedString,
    pub icon: IconName,
    pub category: AgentCategory,
    pub skills: Vec<SharedString>,
}

impl ConvergioAgent {
    fn all_agents() -> Vec<Self> {
        vec![
            // Leadership & Orchestration
            Self::new("ali", "Ali - Chief of Staff", "Master orchestrator, coordinates all agents", IconName::ConvergioAli, AgentCategory::Leadership, vec!["orchestration", "coordination", "strategy"]),
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
            name: name.to_string().into(),
            server_name: server_name.into(),
            display_name: display_name.to_string().into(),
            description: description.to_string().into(),
            icon,
            category,
            skills: skills.into_iter().map(|s| s.to_string().into()).collect(),
        }
    }

    fn matches_query(&self, query: &str) -> bool {
        if query.is_empty() {
            return true;
        }
        // Optimize: only lowercase query once, reuse for all checks
        let query_lower = query.to_lowercase();
        let display_lower = self.display_name.to_lowercase();
        let desc_lower = self.description.to_lowercase();
        display_lower.contains(&query_lower)
            || desc_lower.contains(&query_lower)
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
    _workspace: WeakEntity<Workspace>,
    _fs: Arc<dyn Fs>,
    width: Option<Pixels>,
    agents: Vec<ConvergioAgent>,
    selected_index: Option<usize>,
    collapsed_categories: HashSet<AgentCategory>,
    filter_editor: Entity<Editor>,
    filter_query: SharedString,
    active_agents: HashSet<SharedString>,
    show_active_only: bool,
    // Active agent pack preset
    active_pack: AgentPack,
    // Show pack selector UI (future: expanded view with descriptions)
    #[allow(dead_code)]
    show_pack_selector: bool,
    // Track session IDs for conversation persistence
    // Maps agent_name -> last_session_id (for future use when thread creation events are available)
    _agent_sessions: HashMap<String, String>,
    _filter_editor_subscription: Subscription,
    // Onboarding state
    show_onboarding: bool,
    // History store for conversation resume (future use)
    #[allow(dead_code)]
    history_store: Option<Entity<HistoryStore>>,
    _history_subscription: Option<Subscription>,
    // Pending thread opens waiting for history to load (future use)
    #[allow(dead_code)]
    pending_thread_opens: Vec<(SharedString, SharedString)>,
    // Track agents that are currently processing in background
    processing_agents: HashSet<SharedString>,
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
    // Keyboard navigation: select next agent (↓, j)
    fn select_next(&mut self, _: &SelectNext, _window: &mut Window, cx: &mut Context<Self>) {
        let filtered_count = self.filtered_agents().len();
        if filtered_count == 0 {
            return;
        }

        self.selected_index = Some(match self.selected_index {
            Some(ix) => (ix + 1).min(filtered_count - 1),
            None => 0,
        });
        cx.notify();
    }

    // Keyboard navigation: select previous agent (↑, k)
    fn select_previous(&mut self, _: &SelectPrevious, _window: &mut Window, cx: &mut Context<Self>) {
        let filtered_count = self.filtered_agents().len();
        if filtered_count == 0 {
            return;
        }

        self.selected_index = Some(match self.selected_index {
            Some(ix) => ix.saturating_sub(1),
            None => 0,
        });
        cx.notify();
    }

    // Keyboard navigation: select first agent (gg, Home)
    fn select_first(&mut self, _: &SelectFirst, _window: &mut Window, cx: &mut Context<Self>) {
        if !self.filtered_agents().is_empty() {
            self.selected_index = Some(0);
            cx.notify();
        }
    }

    // Keyboard navigation: select last agent (G, End)
    fn select_last(&mut self, _: &SelectLast, _window: &mut Window, cx: &mut Context<Self>) {
        let count = self.filtered_agents().len();
        if count > 0 {
            self.selected_index = Some(count - 1);
            cx.notify();
        }
    }

    // Keyboard navigation: confirm selection (Enter)
    fn confirm(&mut self, _: &Confirm, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(ix) = self.selected_index {
            let filtered = self.filtered_agents();
            if let Some(agent) = filtered.get(ix) {
                let agent_name = agent.name.clone();
                let server_name = agent.server_name.clone();
                self.open_agent_thread(&agent_name, &server_name, window, cx);
            }
        }
    }

    // Provide key context for keyboard bindings
    fn dispatch_context(&self, _window: &Window, _cx: &Context<Self>) -> KeyContext {
        let mut context = KeyContext::new_with_defaults();
        context.add("ConvergioPanel");
        context.add("menu");
        context
    }
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

        // History store is optional - the panel works without it
        // This feature may be connected later when AgentPanel integration matures
        let history_store: Option<Entity<HistoryStore>> = None;
        let history_subscription: Option<Subscription> = None;

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
            _workspace: workspace_handle,
            _fs: fs,
            width: None,
            agents,
            selected_index: None,
            collapsed_categories: HashSet::default(),
            filter_editor,
            filter_query: SharedString::default(),
            active_agents: HashSet::default(),
            show_active_only: false,
            active_pack: AgentPack::default(),
            show_pack_selector: false,
            _agent_sessions: HashMap::default(),
            _filter_editor_subscription: subscription,
            show_onboarding: true, // Show by default, will be updated async
            history_store,
            _history_subscription: history_subscription,
            pending_thread_opens: Vec::new(),
            processing_agents: HashSet::default(),
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
        self.filter_query = self.filter_editor.read(cx).text(cx).into();
        cx.notify();
    }

    fn toggle_category(&mut self, category: AgentCategory, window: &mut Window, cx: &mut Context<Self>) {
        if self.collapsed_categories.contains(&category) {
            self.collapsed_categories.remove(&category);
        } else {
            self.collapsed_categories.insert(category);
        }
        cx.notify();
        cx.defer_in(window, |this, _, cx| {
            this.serialize(cx);
        });
    }

    fn toggle_active_filter(&mut self, cx: &mut Context<Self>) {
        self.show_active_only = !self.show_active_only;
        cx.notify();
    }

    fn set_active_pack(&mut self, pack: AgentPack, cx: &mut Context<Self>) {
        if self.active_pack != pack {
            self.active_pack = pack;
            log::info!("Switched to agent pack: {}", pack.display_name());
            // Reset selected index since filtered list will change
            self.selected_index = None;
            cx.notify();
            // TODO: Persist pack selection to KEY_VALUE_STORE
        }
    }

    /// Opens a Convergio agent chat using the custom ConvergioChatView.
    /// This reads directly from convergio.db for full synchronization with CLI.
    fn open_agent_thread(&mut self, agent_name: &str, server_name: &str, window: &mut Window, cx: &mut Context<Self>) {
        // Mark agent as active
        self.active_agents.insert(agent_name.to_string().into());
        log::info!("Opening Convergio agent chat: {} (custom chat view)", agent_name);

        let workspace = self._workspace.clone();
        let agent_name: SharedString = agent_name.to_string().into();
        let display_name: SharedString = server_name.to_string().into();

        // Create the custom chat view
        if let Some(workspace) = workspace.upgrade() {
            workspace.update(cx, |workspace, cx| {
                let chat_view = cx.new(|cx| {
                    ConvergioChatView::new(
                        agent_name.clone(),
                        display_name,
                        self._workspace.clone(),
                        window,
                        cx,
                    )
                });

                // Open the chat view as a workspace item
                workspace.add_item_to_active_pane(Box::new(chat_view), None, true, window, cx);
            });
        }

        cx.notify();
    }

    /// Process pending thread opens when history becomes available (future use)
    #[allow(dead_code)]
    fn process_pending_thread_opens(&mut self, cx: &mut Context<Self>) {
        if self.pending_thread_opens.is_empty() {
            return;
        }

        // Check if history is now loaded
        let history_loaded = self.history_store
            .as_ref()
            .map(|store| !store.read(cx).is_empty(cx))
            .unwrap_or(false);

        if history_loaded {
            // Clear pending opens - next time user clicks, we'll find the thread
            let pending_count = self.pending_thread_opens.len();
            self.pending_thread_opens.clear();
            log::info!("History loaded, cleared {} pending thread opens - will resume on next click", pending_count);
            cx.notify();
        }
    }

    /// Stores a session ID for an agent and persists it to storage
    pub fn store_agent_session(&mut self, agent_name: &str, session_id: String, cx: &mut Context<Self>) {
        self._agent_sessions.insert(agent_name.to_string(), session_id);
        self.save_agent_sessions(cx);
    }

    /// Mark an agent as currently processing (working in background)
    pub fn set_agent_processing(&mut self, agent_name: &str, is_processing: bool, cx: &mut Context<Self>) {
        let name: SharedString = agent_name.to_string().into();
        if is_processing {
            self.processing_agents.insert(name);
        } else {
            self.processing_agents.remove(&name);
        }
        cx.notify();
    }

    /// Check if an agent is currently processing
    pub fn is_agent_processing(&self, agent_name: &str) -> bool {
        let name: SharedString = agent_name.to_string().into();
        self.processing_agents.contains(&name)
    }

    fn save_agent_sessions(&self, cx: &mut Context<Self>) {
        // Serialize directly instead of cloning HashMap
        let data = SerializedAgentSessions {
            sessions: self._agent_sessions.clone(),
        };
        if let Ok(serialized) = serde_json::to_string(&data) {
            let key = AGENT_SESSIONS_KEY.to_string();
            cx.background_executor()
                .spawn(async move {
                    KEY_VALUE_STORE
                        .write_kvp(key, serialized)
                        .await
                        .log_err();
                })
                .detach();
        }
    }

    fn dismiss_onboarding(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.show_onboarding = false;
        cx.notify();
        cx.defer_in(window, |this, _, cx| {
            this.serialize(cx);
        });
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
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.dismiss_onboarding(window, cx);
                            }))
                    )
            )
    }

    fn filtered_agents(&self) -> Vec<&ConvergioAgent> {
        self.agents
            .iter()
            .filter(|agent| {
                // First check pack filter
                let matches_pack = match self.active_pack {
                    AgentPack::Enterprise => true, // All agents
                    AgentPack::Custom => true, // Uses show_active_only filter below
                    _ => {
                        // Check if agent is in the pack
                        if let Some(pack_agents) = self.active_pack.included_agents() {
                            pack_agents.contains(&agent.name.as_str())
                        } else {
                            true
                        }
                    }
                };

                let matches_query = agent.matches_query(&self.filter_query);
                let matches_active = if self.show_active_only {
                    self.active_agents.contains(&agent.name)
                } else {
                    true
                };
                matches_pack && matches_query && matches_active
            })
            .collect()
    }

    fn agents_in_category(&self, category: AgentCategory) -> Vec<&ConvergioAgent> {
        // Optimized: filter once with all criteria including category
        self.agents
            .iter()
            .filter(|agent| {
                // Category filter first (cheapest check)
                if agent.category != category {
                    return false;
                }

                // Then check pack filter
                let matches_pack = match self.active_pack {
                    AgentPack::Enterprise => true,
                    AgentPack::Custom => true,
                    _ => {
                        if let Some(pack_agents) = self.active_pack.included_agents() {
                            pack_agents.contains(&agent.name.as_str())
                        } else {
                            true
                        }
                    }
                };

                let matches_query = agent.matches_query(&self.filter_query);
                let matches_active = if self.show_active_only {
                    self.active_agents.contains(&agent.name)
                } else {
                    true
                };
                matches_pack && matches_query && matches_active
            })
            .collect()
    }

    fn active_count(&self) -> usize {
        self.active_agents.len()
    }

    fn render_category_header(&self, category: AgentCategory, agent_count: usize, active_in_cat: usize, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement + use<> {
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
            .on_click(cx.listener(move |this, _, window, cx| {
                this.toggle_category(cat, window, cx);
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

    fn render_agent(&self, agent: &ConvergioAgent, global_ix: usize, is_panel_focused: bool, cx: &Context<Self>) -> impl IntoElement {
        let is_selected = self.selected_index == Some(global_ix);
        let is_active = self.active_agents.contains(&agent.name);
        let is_processing = self.processing_agents.contains(&agent.name);
        let agent_name = agent.name.clone();
        let server_name = agent.server_name.clone();
        // Show focus ring when selected via keyboard navigation
        let show_focus_ring = is_selected && is_panel_focused;

        ListItem::new(global_ix)
            .inset(true)
            .spacing(ui::ListItemSpacing::Sparse)
            .toggle_state(is_selected)
            // Accessibility: outlined focus ring for keyboard navigation
            .when(show_focus_ring, |this| this.outlined())
            .start_slot(
                div()
                    .pl_4()
                    .flex()
                    .items_center()
                    .gap_1()
                    // Show processing indicator when agent is working in background
                    .when(is_processing, |this| {
                        this.child(
                            Icon::new(IconName::ArrowCircle)
                                .size(IconSize::XSmall)
                                .color(Color::Warning)
                        )
                    })
                    // Show green dot when active but not processing
                    .when(is_active && !is_processing, |this| {
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
                            // Show spinning indicator next to name when processing
                            .when(is_processing, |this| {
                                this.child(
                                    Label::new("⟳")
                                        .size(LabelSize::XSmall)
                                        .color(Color::Warning)
                                )
                            })
                            // Show green dot next to name when active but not processing
                            .when(is_active && !is_processing, |this| {
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

    fn render_pack_selector(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let current_pack = self.active_pack;

        div()
            .flex()
            .items_center()
            .justify_between()
            .mb_2()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .child(
                        Icon::new(current_pack.icon())
                            .size(IconSize::Small)
                            .color(Color::Accent)
                    )
                    .child(
                        Label::new(current_pack.display_name())
                            .size(LabelSize::Small)
                            .color(Color::Default)
                    )
            )
            .child(
                div()
                    .flex()
                    .gap_1()
                    .child(self.render_pack_button(AgentPack::Enterprise, current_pack, cx))
                    .child(self.render_pack_button(AgentPack::Startup, current_pack, cx))
                    .child(self.render_pack_button(AgentPack::Developer, current_pack, cx))
                    .child(self.render_pack_button(AgentPack::Minimal, current_pack, cx))
                    .child(self.render_pack_button(AgentPack::Custom, current_pack, cx))
            )
    }

    fn render_pack_button(&self, pack: AgentPack, current_pack: AgentPack, cx: &mut Context<Self>) -> impl IntoElement {
        let is_selected = pack == current_pack;
        let id = SharedString::from(format!("pack-{:?}", pack));
        div()
            .id(id)
            .px_2()
            .py_0p5()
            .rounded_sm()
            .cursor_pointer()
            .when(is_selected, |this| {
                this.bg(cx.theme().colors().element_selected)
            })
            .hover(|this| {
                this.bg(cx.theme().colors().element_hover)
            })
            .child(
                Icon::new(pack.icon())
                    .size(IconSize::XSmall)
                    .color(if is_selected { Color::Accent } else { Color::Muted })
            )
            .on_click(cx.listener(move |this, _, _, cx| {
                this.set_active_pack(pack, cx);
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
            .child(self.render_pack_selector(cx))
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
        ConvergioSettings::get_global(cx).dock.into()
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(&mut self, _position: DockPosition, _window: &mut Window, _cx: &mut Context<Self>) {
        // Settings update will be added later
    }

    fn size(&self, _window: &Window, cx: &App) -> Pixels {
        self.width
            .unwrap_or_else(|| ConvergioSettings::get_global(cx).default_width)
    }

    fn set_size(&mut self, size: Option<Pixels>, window: &mut Window, cx: &mut Context<Self>) {
        self.width = size;
        cx.notify();
        cx.defer_in(window, |this, _, cx| {
            this.serialize(cx);
        });
    }

    fn icon(&self, _window: &Window, cx: &App) -> Option<IconName> {
        ConvergioSettings::get_global(cx)
            .button
            .then_some(IconName::Convergio)
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
        // Accessibility: track if panel has keyboard focus for focus ring display
        let is_panel_focused = self.focus_handle.is_focused(window);

        div()
            .id("convergio-panel")
            .size_full()
            .flex()
            .flex_col()
            .bg(cx.theme().colors().panel_background)
            // Accessibility: keyboard navigation support
            .track_focus(&self.focus_handle(cx))
            .key_context(self.dispatch_context(window, cx))
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::select_first))
            .on_action(cx.listener(Self::select_last))
            .on_action(cx.listener(Self::confirm))
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
                                let header = self.render_category_header(category, agents_in_cat.len(), active_in_cat, window, cx);

                                let agent_elements: Vec<_> = if is_collapsed {
                                    vec![]
                                } else {
                                    agents_in_cat
                                        .into_iter()
                                        .map(|agent| {
                                            let element = self.render_agent(agent, global_ix, is_panel_focused, cx);
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
