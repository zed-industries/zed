Task ID: TA001 - Integrate Anthropic Models via Google Vertex AI**

**Objective:**
To develop a new language model provider, `anthropic_vertex_ai`, that seamlessly integrates Anthropic's models (e.g., Claude) into the Zed editor via the Google Cloud Vertex AI platform.

**Background:**
While Zed has a direct integration with Anthropic's API, many users operate within the Google Cloud ecosystem. Vertex AI provides access to third-party models like Anthropic's through its own endpoint. This task involves creating a new provider that bridges the existing `anthropic` API logic with the authentication and endpoint requirements of Google Cloud.

This integration will not use explicit API keys. Instead, it will leverage Google's Application Default Credentials (ADC), a standard mechanism for authenticating GCP services, ensuring a secure and streamlined user experience. Configuration will be provided through `settings.json` to specify the required `project_id` and `location` for the Vertex AI endpoint.

**Key Requirements:**
- **Authentication:** Must use Google Cloud's Application Default Credentials (ADC) for all API requests. The implementation should not handle manual tokens.
- **Configuration:** The provider must be configurable via `settings.json`, allowing the user to specify their Google Cloud `project_id` and `location`.
- **Endpoint Construction:** Must dynamically construct the correct Vertex AI endpoint URL for each request, in the format: `https://$LOCATION-aiplatform.googleapis.com/v1/projects/$PROJECT_ID/locations/$LOCATION/publishers/anthropic/models/$MODEL:streamRawPredict`.
- **Payload Adaptation:** The JSON payload sent to the endpoint must be modified to:
    - Include the mandatory field: `"anthropic_version": "vertex-2023-10-16"`.
    - Exclude the `model` field, as it is specified in the URL.
- **Integration:** The new provider must be a first-class citizen within Zed, appearing in the model selection list and functioning identically to other integrated providers.

**Implementation Plan:**

**Step 1: Foundational Analysis & Crate Setup**

*   **Action 1.1: Analyze `google_vertex` Crate:** Thoroughly examine `crates/google_vertex/src/google_vertex.rs` to understand its implementation of ADC-based authentication and how it reads settings like `project_id` and `location`. This will serve as the template for our authentication logic.
*   **Action 1.2: Define Configuration Struct:** In a new file, `crates/anthropic_vertex_ai/src/lib.rs`, define the `AnthropicVertexAISettings` struct. This struct will deserialize the `project_id` and `location` from the user's `settings.json`.
*   **Action 1.3: Update `Cargo.toml`:** Create/update the `Cargo.toml` file for the `anthropic_vertex_ai` crate. It should include dependencies from both `anthropic` (for serde structs) and `google_vertex` (for GCP-related dependencies like `gcp_auth`).
*   **Action 1.4: Create `lib.rs`:** Ensure `crates/anthropic_vertex_ai/src/lib.rs` exists to house the `LanguageModelProvider` implementation and serve as the crate's entry point.

**Step 2: Adapt Core Anthropic Logic**

*   **Action 2.1: Modify `Request` Struct:** In `crates/anthropic_vertex_ai/src/anthropic_vertex_ai.rs`, modify the main `Request` struct:
    -   Add a new field: `pub anthropic_version: &'static str`.
    -   Remove the existing `pub model: String` field.
*   **Action 2.2: Refactor Completion Functions:** Refactor the `stream_completion_with_rate_limit_info` function to be more generic.
    -   It will now accept the fully-constructed Vertex AI endpoint URL as a parameter.
    -   It will accept an ADC-aware `HttpClient` instance instead of a simple API key.
    -   The logic for setting the `Authorization` header will be updated to use a `Bearer` token provided by the `HttpClient`.

**Step 3: Implement the `LanguageModelProvider`**

*   **Action 3.1: Define Provider Struct:** In `crates/anthropic_vertex_ai/src/lib.rs`, define the main `AnthropicVertexAIProvider` struct. It will store the settings defined in Action 1.2.
*   **Action 3.2: Implement `LanguageModelProvider` Trait:** Implement the `language_model::LanguageModelProvider` trait for `AnthropicVertexAIProvider`.
*   **Action 3.3: Implement Core Logic:** The trait methods will contain the central logic:
    1.  On initialization, the provider will create an `HttpClient` configured to use Google's ADC, following the pattern in the `google_vertex` crate.
    2.  For each completion request, it will dynamically construct the full, model-specific Vertex AI URL using the configured `project_id`, `location`, and the requested model name.
    3.  It will create an instance of the modified `Request` struct from `anthropic_vertex_ai.rs`, setting the `anthropic_version` field correctly.
    4.  Finally, it will call the refactored `stream_completion_with_rate_limit_info` function, passing the authenticated client and the constructed request.

**Step 4: Final Integration**

*   **Action 4.1: Workspace Integration:** Add `anthropic_vertex_ai` to the main workspace `Cargo.toml` to link the new crate.
*   **Action 4.2: Module Declaration:** Add `pub mod anthropic_vertex_ai;` to `crates/language_models/src/provider.rs` to make the module visible.
*   **Action 4.3: Provider Registration:** In `crates/language_models/src/lib.rs`, update the central list of language model providers to include an instance of `AnthropicVertexAIProvider`.

**Verification Plan:**

*   **Compile-Time Verification:** At each major step, ask the human to review the code for compilation errors and adherence to project standards.
*   **Configuration Verification:** The implementation will be tested against a `settings.json` file configured as follows:
    ```json
    "language_servers": {
        "anthropic-vertex": {
            "enabled": true,
            "project_id": "your-gcp-project-id",
            "location": "europe-west1"
        }
    },
    "assistant": {
        "default_model": {
            "provider": "anthropic-vertex",
            "name": "claude-sonnet-4@20250514"
        }
    }
    ```
*   **Runtime Verification:**
    1.  Launch Zed with the above configuration.
    2.  Ensure the local environment is authenticated with GCP (e.g., via `gcloud auth application-default login`).
    3.  Open the assistant panel and confirm that `"anthropic-vertex/claude-sonnet-4@20250514"` is the selected model.
    4.  Send a test prompt to the assistant.
    5.  **Success Condition:** A valid, streamed response is received from the assistant, confirming that the entire chain—from configuration and authentication to request execution and response parsing—is working correctly.
