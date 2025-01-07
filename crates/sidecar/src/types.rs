#![allow(dead_code)]
use serde::de::{self, Deserializer, Visitor};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UIEventWithID {
    pub request_id: String,
    pub exchange_id: String,
    pub event: UIEvent,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum UIEvent {
    SymbolEvent(SymbolEventRequest),
    SymbolLoctationUpdate(SymbolLocation),
    SymbolEventSubStep(SymbolEventSubStepRequest),
    RequestEvent(RequestEvents),
    EditRequestFinished(String),
    FrameworkEvent(FrameworkEvent),
    ChatEvent(ChatMessageEvent),
    ExchangeEvent(ExchangeMessageEvent),
    PlanEvent(PlanMessageEvent),
}

impl From<SymbolEventRequest> for UIEvent {
    fn from(req: SymbolEventRequest) -> Self {
        UIEvent::SymbolEvent(req)
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum SymbolEventProbeRequest {
    SubSymbolSelection,
    ProbeDeeperSymbol,
    /// The final answer for the probe is sent via this event
    ProbeAnswer(String),
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SymbolEventGoToDefinitionRequest {
    fs_file_path: String,
    range: Range,
    thinking: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RangeSelectionForEditRequest {
    range: Range,
    fs_file_path: String,
    // user_id: LSPQuickFixInvocationRequest,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct InsertCodeForEditRequest {
    range: Range,
    fs_file_path: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct EditedCodeForEditRequest {
    range: Range,
    fs_file_path: String,
    new_code: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CodeCorrectionToolSelection {
    range: Range,
    fs_file_path: String,
    tool_use_thinking: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum EditedCodeStreamingEvent {
    Start,
    Delta(String),
    End,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct EditedCodeStreamingRequest {
    pub edit_request_id: String,
    // This is the id of the session the edit is part of
    pub session_id: String,
    pub range: Range,
    pub fs_file_path: String,
    pub updated_code: Option<String>,
    pub event: EditedCodeStreamingEvent,
    pub apply_directly: bool,
    // The exchange id this edit is part of
    pub exchange_id: String,
    pub plan_step_id: Option<String>,
}

/// We have range selection and then the edited code, we should also show the
/// events which the AI is using for the tool correction and whats it is planning
/// on doing for that
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum SymbolEventEditRequest {
    RangeSelectionForEdit(RangeSelectionForEditRequest),
    /// We might be inserting code at a line which is a new symbol by itself
    InsertCode(InsertCodeForEditRequest),
    EditCode(EditedCodeForEditRequest),
    CodeCorrectionTool(CodeCorrectionToolSelection),
    EditCodeStreaming(EditedCodeStreamingRequest),
    ThinkingForEdit(ThinkingForEditRequest),
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ThinkingForEditRequest {
    edit_request_id: String,
    thinking: String,
    delta: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum SymbolEventSubStep {
    Probe(SymbolEventProbeRequest),
    GoToDefinition(SymbolEventGoToDefinitionRequest),
    Edit(SymbolEventEditRequest),
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SymbolEventSubStepRequest {
    symbol_identifier: SymbolIdentifier,
    event: SymbolEventSubStep,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RequestEventProbeFinished {
    reply: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum RequestEvents {
    ProbingStart,
    ProbeFinished(RequestEventProbeFinished),
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct InitialSearchSymbolInformation {
    symbol_name: String,
    fs_file_path: Option<String>,
    is_new: bool,
    thinking: String,
    // send over the range of this symbol
    range: Option<Range>,
}

pub type GroupedReferences = HashMap<String, Vec<Location>>;

pub type FoundReference = HashMap<String, usize>; // <file_path, count>

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct RelevantReference {
    fs_file_path: String,
    symbol_name: String,
    reason: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct InitialSearchSymbolEvent {
    request_id: String,
    symbols: Vec<InitialSearchSymbolInformation>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct OpenFileRequest {
    fs_file_path: String,
    request_id: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct FrameworkReferencesUsed {
    exchange_id: String,
    variables: Vec<VariableInformation>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum FrameworkEvent {
    RepoMapGenerationStart(String),
    RepoMapGenerationFinished(String),
    LongContextSearchStart(String),
    LongContextSearchFinished(String),
    InitialSearchSymbols(InitialSearchSymbolEvent),
    OpenFile(OpenFileRequest),
    CodeIterationFinished(String),
    ReferenceFound(FoundReference),
    RelevantReference(RelevantReference), // this naming sucks ass
    GroupedReferences(GroupedReferences),
    SearchIteration(IterativeSearchEvent),
    AgenticTopLevelThinking(String),
    AgenticSymbolLevelThinking(StepListItem),
    ReferencesUsed(FrameworkReferencesUsed),
    TerminalCommand(TerminalCommandEvent),
    ToolUseDetected(ToolUseDetectedEvent),
    ToolThinking(ToolThinkingEvent),
    ToolNotFound(ToolNotFoundEvent),
    ToolTypeFound(ToolTypeFoundEvent),
    ToolParameterFound(ToolParameterFoundEvent),
    ToolOutput(ToolOutputEvent),
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum ToolOutputEvent {
    ToolTypeForOutput(ToolTypeForOutputEvent),
    ToolOutputResponse(ToolOutputResponseEvent),
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ToolTypeForOutputEvent {
    tool_type: ToolType,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ToolOutputResponseEvent {
    delta: String,
    answer_up_until_now: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ToolParameterFoundEvent {
    tool_parameter_input: ToolParameters,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ToolTypeFoundEvent {
    tool_type: ToolType,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ToolNotFoundEvent {
    full_output: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ToolThinkingEvent {
    thinking: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ToolUseDetectedEvent {
    tool_use_partial_input: ToolInputPartial,
    thinking: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TerminalCommandEvent {
    session_id: String,
    exchange_id: String,
    command: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ChatMessageEvent {
    pub answer_up_until_now: String,
    pub delta: Option<String>,
    pub exchange_id: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum ExchangeMessageEvent {
    RegeneratePlan(RegeneratePlanExchangeEvent),
    FinishedExchange(FinishedExchangeEvent),
    EditsExchangeState(EditsExchangeStateEvent),
    PlansExchangeState(EditsExchangeStateEvent),
    ExecutionState(ExecutionExchangeStateEvent),
    TerminalCommand(TerminalCommandEvent),
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum ExecutionExchangeStateEvent {
    Inference,
    InReview,
    Cancelled,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum EditsStateEvent {
    Loading,
    MarkedComplete,
    Cancelled,
    Accepted,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct EditsExchangeStateEvent {
    edits_state: EditsStateEvent,
    files: Vec<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RegeneratePlanExchangeEvent {
    exchange_id: String,
    session_id: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct FinishedExchangeEvent {
    exchange_id: String,
    session_id: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum PlanMessageEvent {
    PlanStepCompleteAdded(PlanStepAddEvent),
    PlanStepTitleAdded(PlanStepTitleEvent),
    PlanStepDescriptionUpdate(PlanStepDescriptionUpdateEvent),
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PlanStepDescriptionUpdateEvent {
    session_id: String,
    exchange_id: String,
    files_to_edit: Vec<String>,
    delta: Option<String>,
    description_up_until_now: String,
    index: usize,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PlanStepAddEvent {
    session_id: String,
    exchange_id: String,
    files_to_edit: Vec<String>,
    title: String,
    description: String,
    index: usize,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PlanStepTitleEvent {
    session_id: String,
    exchange_id: String,
    files_to_edit: Vec<String>,
    title: String,
    index: usize,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SymbolSubStepUpdate {
    sybmol: SymbolIdentifier,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SymbolLocation {
    snippet: Snippet,
    symbol_identifier: SymbolIdentifier,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SymbolEventRequest {
    symbol: SymbolIdentifier,
    event: SymbolEvent,
    tool_properties: ToolProperties,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum SymbolEvent {
    InitialRequest(InitialRequestData),
    AskQuestion(AskQuestionRequest), // todo(zi) remove this shit everywhere...
    UserFeedback,
    Delete,
    Edit(SymbolToEditRequest),
    Outline,
    // Probe
    Probe(SymbolToProbeRequest),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SymbolEditedItem {
    symbol: String,
    fs_file_path: String,
    is_new: bool,
    thinking: String,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SymbolRequestHistoryItem {
    symbol: String,
    fs_file_path: String,
    request: String,
    // This is not perfect, because we are leaving behind the new nodes which are
    // getting created and what their type is
    outline_node_type: Option<OutlineNodeType>,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InitialRequestData {
    original_question: String,
    plan: String,
    history: Vec<SymbolRequestHistoryItem>,
    /// We operate on the full symbol instead of the
    full_symbol_request: bool,
    // This is an option for now since we for code-correctness we also send
    // this request, but this is more tied to the original plan
    // in the future this will be a reference to some plan object which will
    // dynamically update the symbol edited items inside
    symbols_edited_list: Option<Vec<SymbolEditedItem>>,
    // if this is a big search request
    is_big_search_request: bool,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, serde::Deserialize, serde::Serialize)]
pub struct SymbolIdentifier {
    symbol_name: String,
    fs_file_path: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LLMProperties {
    llm: LLMType,
    provider: LLMProvider,
    api_key: LLMProviderAPIKeys,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum LLMProviderAPIKeys {
    OpenAI(OpenAIProvider),
    TogetherAI(TogetherAIProvider),
    Ollama(OllamaProvider),
    OpenAIAzureConfig(AzureConfig),
    LMStudio(LMStudioConfig),
    OpenAICompatible(OpenAICompatibleConfig),
    CodeStory(CodestoryAccessToken),
    Anthropic(AnthropicAPIKey),
    FireworksAI(FireworksAPIKey),
    GeminiPro(GeminiProAPIKey),
    GoogleAIStudio(GoogleAIStudioKey),
    OpenRouter(OpenRouterAPIKey),
    GroqProvider(GroqProviderAPIKey),
}

#[derive(Default, Debug, Clone, serde::Deserialize, serde::Serialize, Hash, PartialEq, Eq)]
pub struct CodeStoryLLMTypes {
    // shoehorning the llm type here so we can provide the correct api keys
    pub llm_type: Option<LLMType>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Hash, PartialEq, Eq)]
pub enum LLMProvider {
    OpenAI,
    TogetherAI,
    Ollama,
    LMStudio,
    CodeStory(CodeStoryLLMTypes),
    // Azure(AzureOpenAIDeploymentId),
    OpenAICompatible,
    Anthropic,
    FireworksAI,
    GeminiPro,
    GoogleAIStudio,
    OpenRouter,
    Groq,
}

/// Represents different types of Language Learning Models (LLMs)
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum LLMType {
    /// Mixtral model
    Mixtral,
    /// Mistral Instruct model
    MistralInstruct,
    /// GPT-4 model
    Gpt4,
    /// GPT-3.5 with 16k context window
    GPT3_5_16k,
    /// GPT-4 with 32k context window
    Gpt4_32k,
    /// GPT-4 Optimized model
    Gpt4O,
    /// GPT-4 Optimized Mini model
    Gpt4OMini,
    /// GPT-4 Turbo model
    Gpt4Turbo,
    /// o1 model
    O1Preview,
    /// o1 mini model
    O1Mini,
    /// DeepSeek Coder 1.3B Instruct model
    DeepSeekCoder1_3BInstruct,
    /// DeepSeek Coder 33B Instruct model
    DeepSeekCoder33BInstruct,
    /// DeepSeek Coder 6B Instruct model
    DeepSeekCoder6BInstruct,
    /// DeepSeek Coder V2 model
    DeepSeekCoderV2,
    /// CodeLLama 70B Instruct model
    CodeLLama70BInstruct,
    /// CodeLlama 13B Instruct model
    CodeLlama13BInstruct,
    /// CodeLlama 7B Instruct model
    CodeLlama7BInstruct,
    /// Llama 3 8B Instruct model
    Llama3_8bInstruct,
    /// Llama 3.1 8B Instruct model
    Llama3_1_8bInstruct,
    /// Llama 3.1 70B Instruct model
    Llama3_1_70bInstruct,
    /// Claude Opus model
    ClaudeOpus,
    /// Claude Sonnet model
    ClaudeSonnet,
    /// Claude Haiku model
    ClaudeHaiku,
    /// PPLX Sonnet Small model
    PPLXSonnetSmall,
    /// Cohere Rerank V3 model
    CohereRerankV3,
    /// Gemini Pro model
    GeminiPro,
    /// Gemini Pro Flash model
    GeminiProFlash,
    /// Custom model type with a specified name
    Custom(String),
}

impl Serialize for LLMType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            LLMType::Custom(s) => serializer.serialize_str(s),
            _ => serializer.serialize_str(&format!("{:?}", self)),
        }
    }
}

impl<'de> Deserialize<'de> for LLMType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMTypeVisitor;

        impl<'de> Visitor<'de> for LLMTypeVisitor {
            type Value = LLMType;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing an LLMType")
            }

            fn visit_str<E>(self, value: &str) -> Result<LLMType, E>
            where
                E: de::Error,
            {
                match value {
                    "Mixtral" => Ok(LLMType::Mixtral),
                    "MistralInstruct" => Ok(LLMType::MistralInstruct),
                    "Gpt4" => Ok(LLMType::Gpt4),
                    "Gpt4OMini" => Ok(LLMType::Gpt4OMini),
                    "GPT3_5_16k" => Ok(LLMType::GPT3_5_16k),
                    "Gpt4_32k" => Ok(LLMType::Gpt4_32k),
                    "Gpt4Turbo" => Ok(LLMType::Gpt4Turbo),
                    "DeepSeekCoder1.3BInstruct" => Ok(LLMType::DeepSeekCoder1_3BInstruct),
                    "DeepSeekCoder6BInstruct" => Ok(LLMType::DeepSeekCoder6BInstruct),
                    "CodeLLama70BInstruct" => Ok(LLMType::CodeLLama70BInstruct),
                    "CodeLlama13BInstruct" => Ok(LLMType::CodeLlama13BInstruct),
                    "CodeLlama7BInstruct" => Ok(LLMType::CodeLlama7BInstruct),
                    "DeepSeekCoder33BInstruct" => Ok(LLMType::DeepSeekCoder33BInstruct),
                    "ClaudeOpus" => Ok(LLMType::ClaudeOpus),
                    "ClaudeSonnet" => Ok(LLMType::ClaudeSonnet),
                    "ClaudeHaiku" => Ok(LLMType::ClaudeHaiku),
                    "PPLXSonnetSmall" => Ok(LLMType::PPLXSonnetSmall),
                    "CohereRerankV3" => Ok(LLMType::CohereRerankV3),
                    "GeminiPro1.5" => Ok(LLMType::GeminiPro),
                    "Llama3_8bInstruct" => Ok(LLMType::Llama3_8bInstruct),
                    "Llama3_1_8bInstruct" => Ok(LLMType::Llama3_1_8bInstruct),
                    "Llama3_1_70bInstruct" => Ok(LLMType::Llama3_1_70bInstruct),
                    "Gpt4O" => Ok(LLMType::Gpt4O),
                    "GeminiProFlash" => Ok(LLMType::GeminiProFlash),
                    "DeepSeekCoderV2" => Ok(LLMType::DeepSeekCoderV2),
                    "o1-preview" => Ok(LLMType::O1Preview),
                    "o1-mini" => Ok(LLMType::O1Mini),
                    _ => Ok(LLMType::Custom(value.to_string())),
                }
            }
        }

        deserializer.deserialize_string(LLMTypeVisitor)
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct OpenAIProvider {
    pub api_key: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct TogetherAIProvider {
    pub api_key: String,
}

/// Groq API key which is used to use an account on Groq
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GroqProviderAPIKey {
    pub api_key: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OpenRouterAPIKey {
    pub api_key: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct GoogleAIStudioKey {
    pub api_key: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct GeminiProAPIKey {
    pub api_key: String,
    pub api_base: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FireworksAPIKey {
    pub api_key: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct AnthropicAPIKey {
    pub api_key: String,
}

// Named AccessToken for consistency with workOS / ide language
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct CodestoryAccessToken {
    pub access_token: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct OpenAICompatibleConfig {
    pub api_key: String,
    pub api_base: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct OllamaProvider {}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct AzureConfig {
    pub deployment_id: String,
    pub api_base: String,
    pub api_key: String,
    pub api_version: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct LMStudioConfig {
    pub api_base: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct CodeStoryConfig {
    pub llm_type: LLMType,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SymbolToEdit {
    outline: bool, // todo(zi): remove this mfer, test case
    range: Range,
    fs_file_path: String,
    symbol_name: String,
    instructions: Vec<String>,
    previous_messages: Vec<SessionChatMessage>,
    is_new: bool,
    // If this is a full symbol edit instead of being sub-symbol level
    is_full_edit: bool, // todo(zi): remove this mfer, test case 2
    original_user_query: String,
    symbol_edited_list: Option<Vec<SymbolEditedItem>>,
    // If we should gather definitions for editing
    gather_definitions_for_editing: bool,
    // user provided context as a string for the LLM to use
    user_provided_context: Option<String>,
    // Whether to disable followups and correctness checks
    disable_followups_and_correctness: bool,
    // if we should apply the edits directly
    apply_edits_directly: bool,
    // the recent changes which have happened in the editor ordered with priority
    diff_recent_changes: Option<DiffRecentChanges>,
    // any previous user queries which the user has done
    previous_user_queries: Vec<String>,
    // the plan-step-id if present for this edit
    plan_step_id: Option<String>,
    should_stream: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SymbolToEditRequest {
    symbols: Vec<SymbolToEdit>,
    symbol_identifier: SymbolIdentifier,
    history: Vec<SymbolRequestHistoryItem>,
}

/// Contains the diff recent changes, with the caveat that the l1_changes are
/// the variable one and the l2_changes are the static one
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DiffRecentChanges {
    l1_changes: String,
    l2_changes: String,
    file_contents: Vec<DiffFileContent>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DiffFileContent {
    fs_file_path: String,
    file_content_latest: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SessionChatMessage {
    message: String,
    images: Vec<SessionChatMessageImage>,
    tool_use: Vec<SessionChatToolUse>,
    tool_return: Vec<SessionChatToolReturn>,
    role: SessionChatRole,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum SessionChatRole {
    User,
    Assistant,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SessionChatMessageImage {
    r#type: String,
    media_type: String,
    data: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SessionChatToolUse {
    name: String,
    id: String,
    schema: serde_json::Value,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SessionChatToolReturn {
    tool_use_id: String,
    tool_name: String,
    content: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum OutlineNodeType {
    // the trait of the class if present, this represents the trait implementation which
    // might be part of the symbol, its not necessarily always present in every language
    // but it is a part of rust
    ClassTrait,
    // the defintion of the class if the language supports it (like rust, golang) struct A {...}
    // otherwise its inside the class struct (in languages like js, ts) class A {something: string; something_else: string}
    ClassDefinition,
    // The identifier for the complete class body
    Class,
    // the name of the class
    ClassName,
    // the identifier for the complete function body
    Function,
    // the name of the funciton
    FunctionName,
    // the body of the function
    FunctionBody,
    // function class name
    FunctionClassName,
    // The function parameter identifier
    FunctionParameterIdentifier,
    // The decorators which are present on top of functions/classes
    Decorator,
    // Assignment definition for all the constants etc which are present globally
    // but are relevant to the symbol
    DefinitionAssignment,
    // The identifier for the definition or the constant which we are interested in
    DefinitionIdentifier,
    // Represents a file in the outline
    File,
}

#[derive(
    Debug,
    Clone,
    Copy,
    serde::Deserialize,
    serde::Serialize,
    PartialEq,
    Eq,
    std::hash::Hash,
    Default,
)]
#[serde(rename_all = "camelCase")]
pub struct Range {
    pub start_position: Position,
    pub end_position: Position,
}

impl From<Range> for std::ops::Range<text::Point> {
    fn from(value: Range) -> Self {
        value.start_position.into()..value.end_position.into()
    }
}

// These are always 0 indexed
#[derive(
    Debug,
    Clone,
    Copy,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    std::hash::Hash,
    Default,
)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub line: usize,
    pub character: usize,
    pub byte_offset: usize,
}

impl From<Position> for text::Point {
    fn from(value: Position) -> Self {
        text::Point {
            row: value.line as u32,
            column: value.character as u32,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ToolProperties {
    swe_bench_test_endpoint: Option<String>,
    swe_bench_code_editing_llm: Option<LLMProperties>,
    swe_bench_reranking_llm: Option<LLMProperties>,
    swe_bench_long_context_editing_llm: Option<LLMProperties>,
    full_symbol_request: bool,
    fast_code_symbol_search: Option<LLMProperties>,
    // plan for the task instance this contains the overall plan we are going to
    // be following while making the edits
    plan_for_input: Option<String>,
    apply_edits_directly: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AskQuestionRequest {
    question: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SymbolToProbeRequest {
    symbol_identifier: SymbolIdentifier,
    probe_request: String,
    original_request: String,
    original_request_id: String,
    history: Vec<SymbolToProbeHistory>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SymbolToProbeHistory {
    symbol: String,
    fs_file_path: String,
    content: String,
    question: String,
}

#[derive(Debug, Clone, Eq, PartialEq, std::hash::Hash, serde::Serialize, serde::Deserialize)]
pub struct Snippet {
    range: Range,
    symbol_name: String,
    fs_file_path: String,
    content: String,
    language: Option<String>,
    // this represents completely a snippet of code which is a logical symbol
    // so a class here will have the complete node (along with all the function inside it),
    // and if its a function then this will be the funciton by itself
    outline_node_content: OutlineNodeContent,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, std::hash::Hash)]
pub struct OutlineNodeContent {
    range: Range,
    name: String,
    r#type: OutlineNodeType,
    // The content here gives the outline of the node which we are interested in
    content: String,
    fs_file_path: String,
    identifier_range: Range,
    body_range: Range,
    language: String,
    trait_implementation: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ToolInputPartial {
    CodeEditing(CodeEditingPartialRequest),
    ListFiles(ListFilesInput),
    SearchFileContentWithRegex(SearchFileContentInputPartial),
    OpenFile(OpenFileRequestPartial),
    LSPDiagnostics(WorkspaceDiagnosticsPartial),
    TerminalCommand(TerminalInputPartial),
    AskFollowupQuestions(AskFollowupQuestionsRequest),
    AttemptCompletion(AttemptCompletionClientRequest),
    RepoMapGeneration(RepoMapGeneratorRequestPartial),
    TestRunner(TestRunnerRequestPartial),
    CodeEditorParameters(CodeEditorParameters),
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CodeEditingPartialRequest {
    fs_file_path: String,
    instruction: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ListFilesInput {
    directory_path: String,
    recursive: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchFileContentInputPartial {
    directory_path: String,
    regex_pattern: String,
    file_pattern: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OpenFileRequestPartial {
    pub fs_file_path: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OpenFileResponse {
    pub fs_file_path: String,
    pub file_contents: String,
    pub exists: bool,
    // TODO(skcd): This might break
    pub language: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TestRunnerRequestPartial {
    fs_file_paths: Vec<String>,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RepoMapGeneratorRequestPartial {
    directory_path: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WorkspaceDiagnosticsPartial {}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TerminalInputPartial {
    command: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActionToolInputPartial {
    tool_use_id: String,
    tool_input_partial: ToolInputPartial,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct CodeEditorParameters {
    pub command: EditorCommand,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_line: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_str: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_str: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_range: Option<Vec<i32>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EditorCommand {
    View,
    Create,
    StrReplace,
    Insert,
    UndoEdit,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AskFollowupQuestionsRequest {
    question: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AttemptCompletionClientRequest {
    result: String,
    command: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ToolType {
    // AskDocumentation,
    // AskUser,
    PlanningBeforeCodeEdit,
    CodeEditing,
    OpenFile,
    // Search,
    GoToDefinitions,
    GoToReferences,
    // FileSystem,
    // FolderOutline,
    // Terminal,
    LSPDiagnostics,
    ReRank,
    // WebScrape,
    // searches of different kind are over here
    FindCodeSnippets,
    RequestImportantSymbols,
    FindCodeSymbolsCodeBaseWide,
    UtilityCodeSymbolSearch,
    GrepInFile,
    GoToImplementations,
    // filtering queries go here
    FilterCodeSnippetsForEditing,
    FilterCodeSnippetsSingleSymbolForEditing,
    // editor requests
    EditorApplyEdits,
    // quick fix options
    GetQuickFix,
    // apply quick fix
    ApplyQuickFix,
    // Error correction tool selection
    CodeCorrectnessActionSelection,
    CodeEditingForError,
    // Followup decision
    ClassSymbolFollowup,
    // COT chains
    CodeEditingCOT,
    // Probe operation
    ProbeCreateQuestionForSymbol,
    ProbeEnoughOrDeeper,
    ProbeSubSymbolFiltering,
    ProbePossible,
    ProbeQuestion,
    ProbeSubSymbol,
    ProbeFollowAlongSymbol,
    ProbeSummarizeAnswer,
    ProbeTryHardAnswer,
    // Repo map Search
    RepoMapSearch,
    // Get important files by inferring from repo tree
    ImportantFilesFinder,
    // SWE Bench tool endpoint
    SWEBenchToolEndpoint,
    // Test correction
    TestCorrection,
    // Code symbols which we want to follow
    CodeSymbolsToFollowInitialRequest,
    // Tool to use to generate the final probe answer
    ProbeFinalAnswerSummary,
    // New sub symbol in class for code editing
    NewSubSymbolRequired,
    // Find symbol in the codebase using the vscode api
    GrepSymbolInCodebase,
    // Find new symbol file location
    FindFileForNewSymbol,
    // Find symbol to edit in user context
    FindSymbolsToEditInContext,
    // ReRanking code snippets for code editing context
    ReRankingCodeSnippetsForCodeEditingContext,
    // Apply the outline of the changes to the range we are interested in
    ApplyOutlineEditToRange,
    // Big search
    BigSearch,
    // Filter edit operation
    FilterEditOperation,
    // Keyword search
    KeywordSearch,
    // inlay hints for the code
    InLayHints,
    // code location for the new symbol
    CodeSymbolNewLocation,
    // should edit the code or is it just a check
    ShouldEditCode,
    // use search and replace blocks for edits
    SearchAndReplaceEditing,
    // Grabs the git-diff
    GitDiff,
    // code editing warmup tool
    CodeEditingWarmupTool,
    // grab outline nodes using the editor
    OutlineNodesUsingEditor,
    // filters references
    ReferencesFilter,
    // scratch pad agent
    ScratchPadAgent,
    // edited files
    EditedFiles,
    // Reasoning (This is just plain reasoning with no settings right now)
    Reasoning,
    // Plan updater
    PlanUpdater,
    // Step generator
    StepGenerator,
    // Create a new file
    CreateFile,
    // File diagnostics
    FileDiagnostics,
    // Add steps to the plan
    PlanStepAdd,
    // Go to previous word at a position
    GoToPreviousWordRange,
    // Go to type definition
    GoToTypeDefinition,
    // Context driven chat reply
    ContextDrivenChatReply,
    // Create a new exchange during a session
    NewExchangeDuringSession,
    // Undo changes made via exchange
    UndoChangesMadeDuringSession,
    // context driven hot streak reply which looks at LSP errors
    ContextDriveHotStreakReply,
    // Terminal command
    TerminalCommand,
    // Run tests
    TestRunner,
    // Searches the files given a regex pattern
    SearchFileContentWithRegex,
    // List files
    ListFiles,
    // Ask for followup questions
    AskFollowupQuestions,
    // Attempt completion
    AttemptCompletion,
    // Repo map for a sub-directory
    RepoMapGeneration,
    // Sub-process spawned pending output
    SubProcessSpawnedPendingOutput,
    // Reward generation
    RewardGeneration,
    // Feedback generation
    FeedbackGeneration,
    // Code editor tool (this is special for anthropic)
    CodeEditorTool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ToolParameters {
    pub(crate) field_name: String,
    pub(crate) field_content_up_until_now: String,
    pub(crate) field_content_delta: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct VariableInformation {
    pub start_position: Position,
    pub end_position: Position,
    pub fs_file_path: String,
    pub name: String,
    #[serde(rename = "type")]
    pub variable_type: VariableType,
    pub content: String,
    pub language: String,
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum VariableType {
    File,
    CodeSymbol,
    Selection,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum IterativeSearchEvent {
    SearchStarted,
    SeedApplied(Duration),
    SearchQueriesGenerated(Vec<SearchQuery>, Duration),
    SearchExecuted(Vec<SearchResult>, Duration),
    IdentificationCompleted(IdentifyResponse, Duration),
    FileOutlineGenerated(Duration),
    DecisionMade(DecideResponse, Duration),
    LoopCompleted(usize, Duration),
    SearchCompleted(Duration),
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename = "response", default)]
pub struct DecideResponse {
    #[serde(default)]
    suggestions: String,
    complete: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    path: PathBuf,
    thinking: String,
    snippet: SearchResultSnippet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchResultSnippet {
    FileContent(Vec<u8>),
    Tag(String),
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchQuery {
    #[serde(default)]
    pub thinking: String,
    #[serde(default)]
    pub tool: SearchToolType,
    pub query: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub enum SearchToolType {
    #[default] // arbitrarily default to File
    File,
    Keyword,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename = "response", default)]
pub struct IdentifyResponse {
    #[serde(rename = "item", default)]
    pub items: Vec<IdentifiedFile>,
    pub scratch_pad: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentifiedFile {
    path: PathBuf,
    thinking: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Location {
    pub fs_file_path: String,
    pub symbol_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Locations {
    #[serde(rename = "location")]
    pub locations: Vec<Location>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename = "step_list")]
pub struct StepListItem {
    name: String,
    step: Vec<String>,
    #[serde(default)]
    new: bool,
    file_path: String,
}

/// We keep track of the thread-id over here
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AgentSessionChatRequest {
    pub session_id: String,
    pub exchange_id: String,
    pub editor_url: String,
    pub query: String,
    pub user_context: UserContext,
    // The mode in which we want to reply to the exchanges
    // agent_mode: AideAgentMode,
    pub repo_ref: RepoRef,
    pub root_directory: String,
    pub project_labels: Vec<String>,
    pub codebase_search: bool,
    pub access_token: String,
    pub model_configuration: LLMClientConfig,
    pub all_files: Vec<String>,
    pub open_files: Vec<String>,
    pub shell: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ImageInformation {
    r#type: String,
    media_type: String,
    data: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct FileContentValue {
    pub file_path: String,
    pub file_content: String,
    pub language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserContext {
    pub variables: Vec<VariableInformation>,
    pub file_content_map: Vec<FileContentValue>,
    pub terminal_selection: Option<String>,
    pub folder_paths: Vec<String>,
    pub is_plan_generation: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMClientConfig {
    pub slow_model: LLMType,
    pub fast_model: LLMType,
    pub models: HashMap<LLMType, Model>,
    pub providers: Vec<LLMProviderAPIKeys>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    pub context_length: u32,
    pub temperature: f32,
    pub provider: LLMProvider,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RepoRef(String);
impl RepoRef {
    pub fn new(path: &str) -> Self {
        Self(format!("local/{path}", path = path.to_string()))
    }
}
