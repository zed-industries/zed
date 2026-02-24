## User

Have a look at the mcp-branch and compare it to the contents of this branch. The mcp-auth branch was written by a domain expert, but it doesn’t implement the whole feature. The contents on this branch are LLM generated, with a human driving (see [@OAUTH_PLAN.md](file:///Users/tom/gh/zed-industries/zed/crates/context_server/OAUTH_PLAN.md) and [@OAUTH_HANDOVER.md](file:///Users/tom/gh/zed-industries/zed/crates/context_server/OAUTH_HANDOVER.md) ). Please review this branch against the mcp-auth branch, see where they differ, what could be better or worse, etc. Please make sure you have the right checkouts and diffs first.

## Assistant

<thinking>
The user wants me to compare the current branch with the `mcp-auth` branch. Let me first understand what branch we're on, then look at the `mcp-auth` branch to understand the differences.

Let me start by checking the current branch and then comparing with `mcp-auth`.
</thinking>

Let me start by understanding what branches we're dealing with and getting the diffs.

**Tool Call: git branch --show-current && echo "---" && git log --oneline -5**
Status: Completed

Terminal:
```
---
ecb196a9aa (HEAD) phase4
4eb0bd1d2d phase3
d4bfa78374 phase2
74a083dbe9 Phase 1 part 2
fa1f67a9fe Phase1 part 1

```


**Tool Call: git branch -a | grep -i mcp-auth**
Status: Completed

Terminal:
```
  remotes/origin/mcp-auth




```


**Tool Call: git log --oneline -10 origin/mcp-auth**
Status: Completed

Terminal:
```
8a65bd6055 (origin/mcp-auth) Fix _supported enum decoding
55aa53f8d0 Remove incorrect check
5ba17fe907 Use POST for exchange token
c9183f010b Use URL_SAFE_NO_PAD
d0ea20df01 Timeout required
997dfb2aea Merge branch 'main' into mcp-auth
f1fd0ab529 remote: Fix not being able to cancel in connecting state (#46789)
55a7a939c1 collab: Bump minimum required version to collaborate from 0.204.1 to 0.220.0 to accomodate for project search RPC changes (#46715)
6c5da3df22 outline: Enable scrollbar in outline view picker (#46774)
db2f2ad45a agent: One Thread History (#46785)

```


**Tool Call: git merge-base HEAD origin/mcp-auth**
Status: Completed

Terminal:
```
f1fd0ab529fd9ec02cfe005bcc58ee0681419ec5




```


## Assistant

Good, they share a merge base. Let me get a clearer picture of both branches' changes.

**Tool Call: git log --oneline f1fd0ab529..origin/mcp-auth**
Status: Completed

Terminal:
```
8a65bd6055 (origin/mcp-auth) Fix _supported enum decoding
55aa53f8d0 Remove incorrect check
5ba17fe907 Use POST for exchange token
c9183f010b Use URL_SAFE_NO_PAD
d0ea20df01 Timeout required
997dfb2aea Merge branch 'main' into mcp-auth
063048a64d Merge origin/main into current branch
925b716cf3 handle more statuses in ui
d681eab746 Store www_auth_header internally
43047444e5 Store to keychain after ever auth update
cff8cb0056 Discard client on `invalid_token`
eaf7d5269b Check _supported fields first
12784ae109 Save/load oauth client state from keychain
3ebfe656ea Test init and authorize_url
a0ae2256cc Use structured errors
db42d5213d Include token in request and refresh
fee3d8089d Include access token in request when we have one
dff0a0f80b Handle callback
2c9883958c Begin authorize
808d262c3a Add explicit Accept: application/json
944634e98c Checkpoint: authorize and exchange token
28be655495 Scope selection strategy
935b16db1b Checkpoint: Sketching flow
f5f2f10e13 Test server metadata fallback
338dd62e0e Fetch authorization server metadata
f2cf745c3d Fetch ProtectedResourceMetadata
2493e42564 Add resource_metadata field
53d14b32e2 Parse WWW-Authenticate

```


**Tool Call: git log --oneline f1fd0ab529..HEAD**
Status: Completed

Terminal:
```
ecb196a9aa (HEAD) phase4
4eb0bd1d2d phase3
d4bfa78374 phase2
74a083dbe9 Phase 1 part 2
fa1f67a9fe Phase1 part 1
a3c62de696 git_graph: Add some design adjustments (#49899)
b775e5d63f Add a cargo wrapper to report build times (#49632)
77e68b4e41 agent: Remove warning for spawn_agent (#49903)
911a8d9ffa docs: Remove broken Linear MCP extension from docs (#48484)
8294fbb75b agent: Subagent low context warnings (#49902)
61757aa02b git: Remove unused actions (#49901)
162f4b4770 Use title case for UI labels (#49864)
8a51acec33 agent: Staff ship subagents (#49885)
304951a18f agent: Remove timeout for subagent (#49898)
993b0c8e71 Improve subagent permission UX (#49874)
a19bd2893b extension_ci: Allow some more time for tests job (#49900)
4cdf291a95 Avoid some false positives for "closed bugs w/comments" board (#49896)
1b91989c84 Fix OpenAI-compatible setting description (#49846)
7d87145004 livekit_client: Spawn audio input with high priority (#49887)
ea0c55b303 text: Swap `anchor_range_between` and `anchor_range_around` (#49888)
6741a1df28 Add duplicate bot effectiveness tracking (w/github project) (#49879)
15885647e1 git_graph: Improve commit detail panel UI (#49876)
1240fd1f36 git_graph: Allow cancelling active selection via keyboard (#49836)
852d6a7a6c Fix regression for windows-gnu build in audio (#49844)
aa9340547c gpui: Read thermal state from window handle (#49847)
84b1de6672 editor: Store folds by file path for persistence across tab close (#47698)
4325520064 recent_projects: Fix remote reconnect when server is not running (#49834)
450c66ce6e ep: Add a parameter to sample at most N near-duplicates (#49870)
67d9e7d011 ep: Fix teacher parser and other fixes (#49863)
d54a262436 languages: Add syntax highlighting for HTML character references (#48629)
af20a870cc docs: Auto-apply preview release suggestions (#49650)
a2f397be9d Revert "Optimize `workspace/didChangeWatchedFiles` handler" (#49826)
bb368ce1fe git_graph: Improve loading time (#49736)
da93e7dbc6 git_ui: Ignore extraneous links in git output (#49817)
b30ceb83df repl: Bump nbformat to v1.1.0 (#49787)
5901d580a9 editor: Remove redundant clone in `highlight_text` (#49810)
c8656ac96d gpui: Take advantage of unified memory on macOS (#49236)
5e3efc640e editor: Improve `colorize_bracket` highlight performance (#49803)
76c878ddc4 agent: More subagent wordsmithing (#49804)
c22eaa7a26 editor: Fix bracket highlight flickering when editing (#49797)
5205e54d7e project: Remove unused BufferOpened event (#49775)
b49698b3e5 visual_tests: Fix MultiWorkspace panics (#49784)
d08d98f6c1 Update Mistral AI provider icon (#49793)
1e9987dcca Update C# docs (#49789)
1ae087b6d7 Improve ETW profiler action (#49788)
d1cd9fb37a Fix certain title bar elements not being interactive on Windows (#49781)
9e0c5f42a9 diagnostics: Fix typo in toolbar controls (#49779)
21ac361663 Optimize `workspace/didChangeWatchedFiles` handler (#49778)
57725ca982 title_bar: Show organization plans (#49769)
1dc574f83d title_bar: Factor out `PlanChip` component (#49766)
327ad43ebd title_bar: Add organization selector (#49763)
930d9321dc git: Fix panic when unstaged diff is recalculated before its primary diff (#49753)
765444d100 zeta2: Include type definitions in related files (#49748)
8f5ae2d236 ep_cli: Only compute reversals on last user edit (#49747)
f676dfea4b docs: Update docs for external agents (Claude Agent) (#49739)
556271bc13 zeta2: Try not to jump to collaborators as much (#49742)
334584cea9 docs: Revise migration guides (#49743)
7bd4437375 Fix broken accounts.html links in migrate docs (#49744)
66bdb116d0 agent: Change tool name to spawn_agent (#49741)
9304e02b9a agent: Allow the agent to reprompt an existing subagent (#49737)
36b6e83c53 agent: More generous timeouts for subagents (#49734)
e42dae588a agent: Tweak subagent input params and docs (#49732)
56fd9da8a7 git_ui: Add diff stat numbers in Branch Diff view (#49716)
9ad8c7a2a3 editor: Distribute lines across cursors when pasting from external sources (#48676)
9fdfc42689 Fix cargo test -p editor for Linux/Wayland (#49730)
9b0cb4ec20 text: Store fragment's deletions on stack (#49726)
40c159c8f8 git: Prevent panic when updating excerpts in split diff (#49725)
d05ea29095 ep_cli: Delete sync-deployments command (#49728)
0823cacb7a multi_buffer: Fix outdated anchors resolving incorrectly in diff hunks in `resolve_summary_for_anchor` (#49719)
fd32e31d56 editor: Add copy button for project diagnostics messages (#49671)
46d11d210e agent: Allow tools to have structured output (#49722)
366d5fa808 collab_ui: Add a loading state when signing in (#49713)
989887ca0b Add ETW profile recorder action (#49712)
f22f4db6c0 agent: Make agent::addSelectionToThread use current line when no selection (#48045)
b1366c38b3 git_ui: Refine display of the "Review Diff" button (#49710)
9cefb04fb7 gpui_linux: Fix cargo test on wayland (#49686)
be6f27cb8e remote: Fix wsl interop detection failing on some setups (#49708)
10a7987837 agent: Use default tool behavior for subagents (#49706)
35f9c640ee agent: Change max subagent depth to 1 for now (#49704)
d9ece42cf5 text:  Inline `text::Anchor`'s timestamp field, shrinking its size (#49703)
0899b82145 gpui: Fix visual_test.rs feature-gating screen-capture import (#49701)
11bccbcdbf Fix stale folded buffers in split diff view (#49699)
325e941289 anthropic: Support alternative provider SSE formatting (#47847)
fb82b11f5d ep: Fix panic when switching to a non-store provider (#49698)
ae9bb6a628 repl: Add WSL and SSH remote kernel support (#47891)
7e4eb4f72b agent_ui: Fix panic in load/copy thread to/from clipboard (#49694)
d54dbe9d5f editor: Fix panic in minimap selection rendering (#49692)
aa91fd4a96 Reduce amount of closure monomorphizations part 2 (#49688)
a7df04c375 repl: Add clear output(s) command (#49631)
19404e43df repl: Support HTML outputs through `html_to_markdown` (#49646)
3a00810142 editor: Prevent comment extension on REPL separators (#48174)
5342a38384 extension: Stream wasi-sdk download (#49679)
8caebe6840 git: Replace some unwraps with `expect` (#49683)
85ca795977 ep_store: Don't handle project events for non-`EditPredictionStore` providers (#49678)
d896bf7c9d ep_cli: Don't filter rated predictions by Zed version (#49609)
42202edee9 Sign out upon receiving an Unauthorized response when acquiring an LLM token (#49673)
ee636bc71b git: Mitigate panic in split diff spacer calculation (#49674)
02ba2be932 collab: Remove unneeded `Arc::get_mut` (#49672)
1bfd77b398 Fix docs validation to detect unknown keys (#49660)
c06aab84c0 Fix Windows build (#49665)
bba415b2b8 ci: Make filter script less error-prone (#49670)
94b9628d42 Revert "Sign out upon receiving an Unauthorized response when acquiring an LLM token (#49661) (#49669)
2f9350bb6b Sign out upon receiving an Unauthorized response when acquiring an LLM token (#49661)
41c007f9ee Fix subagent card buttons getting clipped by long titles (#49648)
90d8c26796 Fix visual test screenshot functionality  (#49649)
be26e1a27f editor: Fix relative line numbering with deleted blocks present (#49656)
6a9d259fec gpui_linux: Fix headless build (#49652)
62af5b8105 Allow using Zeta through an arbitrary OpenAI-compatible self-hosted API (#49554)
e07d0ba2e4 Add telemetry for stack trace view (#49642)
04bdd17de2 Fix JS syntax error in docs_suggestions cherry-pick job (#49643)
1e847c1eb4 Fix panic involving growing and shrinking selections while buffer size changes (#48898)
32f05acf9f Add Gemini 3.1 Pro to model documentation (#49640)
17abde72b0 Add gemini-3.1-pro-preview model (#49622)
85c23d0d0b agent: Improve the subagent task structure (#49629)
bc31ad4a8c gpui: Extract gpui_platform out of gpui (#49277)
da7b8f2939 Fix font weight in multiple elements (#49539)
037add06e4 editor: Fix bracket colorization discoloring pure deletion diff hunks (#49618)
3bd1b0f759 Temporarily disabling codeowners to resolve noisy routing (#49623)
e8d453721c Revert "Enable type on search by default for the project search (#49374)" (#49619)
8815573101 buffer_search: Don’t hide UI icons if search query is too large (#49493)
712914a1f7 go: split definition and call highlights for functions, methods, and types (#49120)
3a1c3757dc ep: Fix pull examples query (#49610)
d8b3f476a5 Fix the panic when trying to interact with editor-like not-editors (#49608)
4efd07bb03 zlog: Broaden filter for usvg parser warnings (#49607)
2a0a304249 Seperate throttles for jump and edit based predictions (#49499)
aaa87a230a auto_update_ui: Add announcement toast component (#49543)
24a81d4e85 git_ui: Remove dependency on agent crates (#49599)
3129d7e6b5 ep: Parse "user accepted prediction" markers in evals (#49598)
b6cd147b9f Add retry logic to docs_suggestions workflow for transient Factory API failures (#49594)
bad3df6e53 vim: Apply linked edits for delete/change/substitute (#48458)
9d779879f2 editor: Remove duplicate method (#49592)
6652c5f4b2 agent: Remove `MAX_PARALLEL_SUBAGENTS` (#49593)
d6c0d3c8bd project: Block less in `Respository::paths_changed` (#49584)
9a26eabf0b buffer_diff: Reduce number of entity clones in base_text_buffer (#49573)
1ad5ec6db2 Support profiling remote server in the miniprofiler (#49582)
fc79a6fbe7 gpui: Remove some unnecessay heap allocations in entity handling (#49579)
2786d36786 editor: Yield less frequently in `WrapSnapshot::update` (#49497)
dace3ab152 project: Log errors when notifying JSON language server (#49480)
37d54d8482 multi_buffer: Bring `update_path_excerpts` ordering workaround back (#49563)
03416097a8 bedrock: Add Claude Sonnet 4.6 (#49439)
2132c54abc Fix gh auth conflict when GH_TOKEN is already set (#49556)
09cd340b6a Update CODEOWNERS (includes suggested entries) (#49552)
157a02e7d9 Fix `OpenFiles` action not respecting `use_system_path_prompts` setting (#47027)
d5d69369a3 git: Mitigate panic in split diff (#49546)
9b076586f1 Fix crash: vim paste panics on editor-copied entire-line selections (#49134)
4491c2cac8 collab: Remove unused `AccessToken` model (#49542)
f07cec59de collab: Validate access tokens through Cloud (#49535)
af050fc565 Fix git auth conflict in background_agent_mvp workflow (#49537)
d3fe7d9cfc Revert the gutter width limitation for multi buffers (#49536)
045b740090 Fix shell quoting and gh auth in docs suggestions workflow (#49518)
5b0a3de01c Add agent worktree directory setting + worktree info persistence (#49139)
423a8c4b29 git_ui: Add "Review Branch" with agent feature (#49513)
f03691e2f7 Revert "Added support for dismissing the toolchain path selector via … (#49504)
592219c2d9 sidebar: Improve toggle button placement when fullscreen on macOS (#49524)
1d66bbe06f Enable type on search by default for the project search (#49374)
0fdf175c32 anthropic: Remove deprecated models (#49522)
1702a05920 cloud_llm_client: Delete unused variants of CompletionRequestStatus (#49516)
ff0477b88a agent_ui: Route agent thread feedback comments through cloud (#49481)
52fc14f36d agent_ui: Add the ability to undo "reject all" (#48462)
b085d847d5 language_models: Activate StreamEnded support in completion streams for Cloud provider (#49337)
71a6d0005a Add sidebar status icons for agent thread states (#49505)
4e21e753ec Fix multi-workspace restoration and serialization bugs (#49380)
f63e4fbdef Revert "editor: Add `FlexClipped` block style and use it for spacer blocks" (#49508)
bf1bb52b60 Skip /dev/null redirects from terminal auto-allow command extraction (#49503)
27dffc12df Fix cancellation issues with subagents (#49350)
84de30d8a3 ep_cli: Filter fetched examples by `can_collect_data` (#49506)
16aeee6361 Post comments on duplicate bug reports (#49482)
652aff52ba Bump Zed to v0.226 (#49498)
71ffaeb817 Update AI docs for retired hosted models (#49486)
66f7aea166 agent: Disables multi workspace refresh queue for non-staff (#49496)
e72e9d6ce7 editor: Some more semantic highlighting perf work (#49491)
a1af11c8c3 editor: Add `FlexClipped` block style and use it for spacer blocks (#49478)
d4af86001c ep: Update teacher prompt (#49489)
4eb1ffa992 Remove older onboarding modals (#49484)
b34e1bdec1 Improve agent pull request hygiene (#49469)
6f578d8195 feature_flags: Add note about `acp-beta` feature flag (#49479)
41019b031b gpui: Use `ArenaBox` for `Effect::Emit` payload (#49467)
6ccf40c26f acp: Remove unnecessary logs for config options (#49476)
1957a43ef1 editor: More perf work (#49465)
dc41f71f57 Add documentation suggestion automation (#49194)
d5d49c1717 acp: Update terminal tool call updates if we recieve a new one (#49456)
7d80412cca Reduce amount of monomorphizations from FnMut closures (#49453)
5305c22223 ep: Encode cursor position in the predicted patch (#49450)
b4baea8f3b acp: Enable history for external agents for all users (#49449)
0b3a23c357 sum_tree: Implement `Iterator::last` and lower bound of `Iterator::size_hint` (#49448)
8244a5c6ed multi_buffer: Remove unnecessary reference in `as_singleton` return type (#49447)
073de5c24b editor: Improve `apply_fetched_hints` for many excerpt multibuffers (#49444)
418b51d6bc editor: Optimize TabMap chunks generation (#48378)
6daa541e77 docs: Apply documentation standards across all docs (#49177)
9743fe7dfd Put back logic for passing data collection choice to cloud (#49426)
d60b2911d9 Make terminal permission pattern suggestions subcommand-specific (#49148)
717ea9e998 Filter collection of snowflake requests to those with latest schemas (#49424)
2868c50da9 Update CODEOWNERS (includes suggested entries) (#47273)
74179fac7c Show agent icon instead of folder icon in sidebar thread list (#49421)
7eca6a9527  workspace: Group recent projects by date  (#49414)
ee9191ecd2 workspace: Add `Toggle` actions to all the side panels (#49395)
d2aa2e0c5a (origin/agent-the) ui: Remove unused components (#49410)
d858ac5013 extension_ci: Make parsing of current version more robust (#49411)
e6352278ce Remove project cache in edit prediction CLI (#49409)
e8816f27c7 git: More performance improvements when toggling between diff views (#49400)
90e1abd5bf Preallocate vector for `MultiBuffer::excerpt_ids` (#49381)
3b438222ef extension_ci: Improve job orchestration (#49407)
d900b2b656 ui: Put back left-border in tab bar end actions (#49402)
d0f5edeef2 extension_rollout: Prevent dispatch from non-main branches (#49396)
96c72c252d git_ui: Fix tree view next selection out of bounds (#49283)
9aa6b1481e extension_ci: Increase fetch depth for tests job (#49389)
5670e66fa7 Add support for Claude Sonnet 4.6 (#49386)
671f818209 edit_prediction: Route edit prediction rating through Cloud (#49385)
7620c684eb ci: Harden background-agent MVP git authentication loop (#49373)
03ed4e9036 ep: Fix Unicode chars boundary (#49384)
a5dee51fd9 sidebar: Serialize after adding/removing workspaces  (#49372)
91b319139e Add git worktree remove/rename API (#49135)
1a4d5fe4a9 Fix error message wrapping in Callout component (#49370)
9e02da90cf agent_ui: Route agent thread feedback through Cloud (#49324)
aaea63cf7c Strip comment and list prefixes in `vim: join lines` (#49295)
014bf4c332 editor: Fix newline below when selection is at the end of an multibuffer excerpt (#49132)
82ec1aa029 ruby: Add Kanayago as an available language server (#49262)
f64484a9e3 acp: Update npm package for claude agent (#49353)
c26784f094 editor: Improve `buffer_into_editor_highlights` performance (#49349)
f8f489271e vim: Apply strict paragraph motion only in vim mode (#48024)
97a21c13b9 ep: Fix Unicode chars boundary (#49351)
c70f300c13 ep: Make `extract_last_codeblock` more robust (#49347)
96035d137a ep: Repair predictions with too many edits (#49342)
92c0de00fc acp: Claude Code -> Claude Agent (#49345)
af4ecae176 acp_thread: Log token usage when receiving `StopReason::MaxTokens` (#49343)
6f8023530c bedrock: Model streamlining and cleanup (#49287)
891f432f66 ep: Rework `ep repair` to use original teacher prompt (#49335)
4344681188 ep: Rely on editable_region tags when parsing teacher output (#49334)
fb66035a6c ep: Fix edit predictions not showing in new buffer (#49102)
ae293de057 ep: Add metrics for inserted/deleted tokens (#49330)
806e944e25 Fix EP CLI issues found when generating new teacher predictions (#49327)
deb26f8c5f Update our notify crate to fix free after use panic (#49311)
2df11f7bae background-agent: Scaffold week-one crash MVP pipeline (#49299)
452696dcd3 client: Add organizations to `UserStore` (#49320)
fefb81c067 git: Fix panic in split diff when edits cause excerpts to merge (#49122)
a1b181f30f agent: Fix duplicate word in agent system prompt template (#49309)
ed47f1017d gpui: Fix content size scaling regression on X11 (#49174)
8c45f0c2e8 docs: Remove Supermaven (#49317)
6cbeb84880 Tune edit prediction teacher to leave fewer blank spots in predictions (#49315)
6b553e4d27 collab: Remove leftover impersonation code (#49314)
e269569241 collab: Remove `GET /rpc_server_snapshot` endpoint (#49312)
2839506516 collab: Remove `POST /users/:id/access_tokens` endpoint (#49297)
d168301c5d Reuse existing remote workspaces when opening files from the CLI (#49307)
da2d4ca5d9 ci: Install Linux dependencies for Collab deployment workflow (#49308)
958798d20e ci: Generate the Collab deployment workflow (#49306)
0594076d49 xtask: Fix naming of `WorkflowType::ExtensionCi` variant (#49302)
613f73b898 ci: Move all Collab deployment steps to Namespace runners (#49300)
671c3e723e git_ui: Add button for the `branch diff` action (#49298)
f21d51cf00 editor: Fix sticky headers not accounting for deleted diff hunks (#49296)
e9a8ef1c9e workspace: Fix read-only button tooltip for non-toggleable buffers (#48998)
248a3c6c95 multi_buffer: Fix `update_path_excerpts` inserting excerpts out of order (#49290)
9c889c163d editor: Batch calls to fold_buffer in ProjectDiff::refresh (#49278)
3335a9b993 languages: Fix C++ indentation for multi-line declarations and assignments (#47447)
dcc69c053f docs: Fix theme publishing link in themes guide (#49276)
6e33d838c9 copilot: Display cost multiplier for Github Copilot models (#44800)
0e7d63348b agent_ui: Ship thinking effort selection for Zed provider (#49274)
c5bfa93bfb acp: Allow permissions on existing terminal tool calls (#49270)
323b579578 bedrock: Add placeholder tool when summarising threads with tool history (#48863)
cf09993ab5 Enable 'Add to Agent Thread' in worktree-less windows (#49084)
cbf03f5b38 acp_thread: Await Tree-sitter parsing before building agent panel diffs (#49101)
93ead966c2 cloud_llm_client: Add StreamEnded and Unknown variants to CompletionRequestStatus (#49121)
52a9b6c864 ollama: Remove backwards compatibility for Ollama tool call IDs (#49246)
b725beb327 acp: Refresh history less often (#49263)
2c5a3c0693 Tone down color for invisible characters in One themes (#46247)
d0c0c33abf Fix missing right border on pinned tabs in two-row layout (#46952)
27ab898e65 project_panel: Fix mixed sort with incorrect ordering when same file and dir name (#47863)
909c147b4b git_ui: Add git init button to empty repository state (#49203)
506570a73a git_graph: Add resizable commit view support (#49171)
c9dafd2674 Streamline Markdown notification rendering (#48162)
2c097ef09e Add automatic syntax highlighting for GeoJSON files (#49261)
01ac5a8c05 workspace: Use FolderOpen icon for active workspace project (#49180)
01b1d7dc55 editor: Sticky diff hunk controls (#49260)
b6c48b60ab search: Fix collapse/expand all button sync (#48773)
24dbd216d7 agent: Prompt for permission for symlinks outside the project (#49255)
2f439e5f1d Stop notifying first responders for closed bugs (#49253)
d83d9d3e8c Fix API errors where tool_result blocks are sent without their corresponding tool_use blocks in the assistant message (#48002)
c4f98689a1 Revert "gpui: Defer thermal/keyboard state updates when app is borrowed" (#49251)
7f982cae8f Simplify calculation of Windows client area size (#49249)
9d59aa038e svg_preview: Fix SVG preview opening duplicate tabs (#49250)
2c59293b69 gpui: Fix thermal state notifications on intel macOS (#49086)
f3c6dcfa19 extensions_ui: Apply category filter to dev extensions (#48954)
e2f3d5ed8e collab: Update test database schema (#49242)
982e71624d docs: Simplify Deno test task example (#49235)
667b43083c Revert "multi_buffer: Fix "cannot seek backward" crash in summaries_for_anchors (#49047)" (#49238)
ae5abf0b26 Remove dependency on bm25. Use exact prefix match in settings UI search instead (#49231)
2ccbed0a5e extension: Make `provides` a method on `ExtensionManifest` (#49234)
28c82ecad6 docs: Add content about the theme builder (#49229)
9dd3f3a6c5 docs: Add some adjustments to the AI pages (#49227)
81f4cdbc64 docs: Adjust some styles (#49221)
87ff3eb625 editor: Fix to remove non-functional folding creases from the gutter (#49200)
99a558f368 gpui: Defer thermal/keyboard state updates when app is borrowed (#49189)
dd836fc8b2 docs: Add documentation conventions, brand voice guidelines, and gold standards (#49176)
9eb6fc098a gpui: Fix RefCell panic in thermal/keyboard state callbacks (#49187)
d89ee32a7b extensions_ui: Respect category filter for installed extensions when searching (#49183)
e4f2492099 debugger: Fix panic at non-UTF-8 character in paths on Linux (#49103)
180d32023d auto_update_ui: Set a title when viewing release notes locally (#49153)
ae12c80dca Revert proejct search on type (#49163)
0f84a366d9 multi_buffer: Fix "cannot seek backward" crash in summaries_for_anchors (#49047)
3023cf3e28 chore: remove no-longer needed search_on_input=false toggle before tests (#49161)
c932dd1935 recent_projects: Check for WSL paths in OpenFolderInWsl (#49156)
144dd9302b python: Prevent shell command injection in conda environment activation (#49160)
b90a370c86 docs: Add Tailwind LSP configuration sections (#48096)
21bc4f13b8 Fix exotic fold range kinds (#49151)
a0eb63d1af Fix search on input behavior (#49150)
63ea24259a docs: Add rules hygiene guidance for AI agents (#49143)
6db4a25b37 Don’t foreground minimized windows when changing displays (#48902)
39063ab3b1 agent_ui:  Use circular progress component for displaying context window use (#49138)
ed9150f6c9 docs: SEO content improvements for AI section (#49128)
b77fb2941a Fix feature request guidelines link in discussion template (#49129)
16dfc60ad2 util: Always use posix_spawn on macOS even with pre_exec hooks (#49090)
4f54c3e07c settings: Get available audio devices lazily when first required (#49115)
630aadbc05 Fix thinking models when using non-zed providers (#49117)
692a137782 agent: Add project-level `disable_ai` setting (#47902)
88e3954857 lsp: Sanitize newlines in document and workspace symbol names (#49092)
fc6f8d5706 terminal: Fix hyperlinks not being detected correctly when preceded by box-drawing chars (#48447)
abaabce3b7 file_finder: Remove project's root name from the file finder history (#46957)
fee42e1d89 Add search_on_input setting to Project Search (#42889)
73853be484 git: Remove unnecessary block map recomputation when splitting `SplittableEditor` (#49075)
3cd051ac7c terminal: Normalize path-like targets with leading `..` (#47289)
b50de99870 linux: Fix desktop entry pointing to wrong binary in manual install docs (#49109)
46b986c5c6 Fix 'no thread found' error when restoring agent thread on workspace open (#49045)
b17ffdbec1 git: Fix misalignment in the split diff when inlays fall at the end of an excerpt (#49078)
c586bb37cf settings_ui: Fix project settings selecting wrong project (#47338)
50f7ba753e Turn `subagents` flag to false (#49104)
337311bdc8 linux: Name desktop file correctly during bundle (#45508)
ab3987eae3 ui: Add circular progress component (#49100)
81ec5b79ed workspace: Fix remote project buttons sharing same element ID on welcome page (#49096)
8afc67345f git_ui: Fix graph icon button design in the panel (#49099)
51d34a5b47 agent_ui: Fix max tokens error not being shown (#49098)
c093590f2e ep: Fix panic when merging events from different buffers (#49097)
771457f17b workspace: Enable adding a recent project to current workspace from modal (#49094)
7b569de35c mistral: Add x-affinity header (#48584)
7687e1f390 settings: Await config worktree before opening settings.json (#47199)
49dee40073 agent: Skip empty assistant messages for Mistral provider (#47579)
c981245a72 ep: Don't run predictions for excerpts with special tokens (#49040)
13a9386a29 language_models: Add image support for Bedrock (#47673)
c131713901 Fix Conda activation error appearing during task execution (#48736)
a6653f0d95 language: Respect combined injection sub-ranges for language queries (#48522)
0c29a0933f editor: Fix soft wrap premature wrapping with certain fonts (#45206)
856ba20261 markdown_preview: Add Mermaid Diagram Support (#49064)
bd2333d573 Update pull_request_template.md (#48805)
e4ea8ae3b9 docs: Add note about escaping `$` in snippets (#48710)
9081af8dd1 Tweak stalebot (words, number of days) (#48603)
af8ea0d6c2 gpui: Remove blade, reimplement linux renderer with wgpu (#46758)
8249ef5618 repl: Initial stdin support for kernels (#48851)
0b8424a14c Remove deprecated GPT-4o, GPT-4.1, GPT-4.1-mini, and o4-mini (#49082)
20ed14254d Revert "Default agent panel to left dock when multi-workspace is enabled" (#49080)
69a2ea3fff Fix document highlight data issues (#49079)
4fefc6f890 Fix discard not reloading buffer from disk (#48936)
1a491707e3 devcontainer: Fix OpenDevContainer action panic due to double workspace entity lease (#49058)
aa4e1b47dc Separate accepted edit predictions in edit history (#49056)
500f87da88 sidebar: Improve subheader component (#49074)
cabf404bf7 git_graph: Open graph from Git Panel and Commit Historic view from Git Graph rows (#48842)
d453d959f2 Suppress agent popup notification when status is already visible (#49044)
f39e3fbd23 Fix agent panel closing unexpectedly when zoomed (#49037)
9446eef9b9 Preserve panel zoom state across workspace switches (#49069)
65027dd4aa Handle newlines better in parse_edits (#48960)
44015e0379 Fix semantic highlights not cleared when disabled in settings (#49066)
009cc7ebb0 Remove Agents Panel and utility panes (#49038)
889d0db8e1 Add prompts and scripts for automatic crash repro and fix (#49063)
45cd96182f git_graph: Add basic keyboard navigation (#49051)
25b1377d1d Unify zeta endpoints (#48900)
bd72484994 Potentially fix hang when opening LSP menu (#49046)
2eb015d10b Allow changing the context window size for Ollama (#44506)
9c102a5675 languages: Add runnable support for bash  (#48807)
71de6edd93 Fix VSCode tasks.json parsing for tasks without explicit labels (#47754)
b229520f8e Increase SQLite busy_timeout from 1ms to 500ms (#49039)
99db990e01 agent_ui: Fix thread title being overridden even when manually edited (#49028)
85736ec892 Install sccache *after* we rm-rf target... (#49035)
deb52aa099 Default agent panel to left dock when multi-workspace is enabled (#49034)
13ad175c8d copilot_chat: Return true context window size (#47557)
c1907c94d2 json_schema_store: Include available LSP adapters in settings schema (#46766)
839dac198f agent_ui: Add some UI tweaks to the subagents thread (#49030)
1ea2f2f02c agent: Sanitize MCP server IDs in tool name disambiguation (#45789)
5026280131 bedrock: Enable 1M context window (#48542)
22c16b690a language_models: Handle empty tool call arguments consistently (#48958)
8742889301 Add user picked model to be used as a default for open router provider when generating comments and thread summary (#47475)
dc56998c0f agent_ui: Fix MCP tool results not displaying after app restart (#47654)
521ffc3a91 agent: Check is_error flag in MCP tool responses (#47095)
b02ad38ff2 docs: Add link back to Ruby LSP config documentation (#48613)
e7926480dd settings: Add ability to select audio input/output devices for collab (#49015)
ceec6ee5f5 ep: Add SeedCoder prompt format (#49020)
b66e6ea02f Fix grammar in comments (#48693)
1844b96c88 languages: Improve function parameter highlighting for Bash (#48067)
842825170f workspace: Adjust remote projects modal (#49019)
255ee7296d languages: Highlight case statements and special variables for Shell Scripts (#48200)
015913afea search: Support brace syntax in project search include/exclude patterns  (#47860)
81d01ef853 languages: Add shebang highlight support for Bash (#48064)
011113ea3c workspace: Improve recent projects picker for multi-project scenario (#48989)
7179a96f04 agent: Fix edit_file tool description (#48588)
c9fc5feb0e vim: Make `:bdelete` use new `workspace::CloseItemInAllPanes` command  (#48988)
e24538dc62 Fix assistant commands visibility when agent is disabled (#48951)
90ff8fe6d6 ep: Add option to use prompt prefill (#48964)
07a0ce91a1 Add support for ACP registry in remote projects (#48935)
76a61d626a ui: Refresh copy for the update button (#48963)
bc72f6ef53 agent_ui: Tone down thinking icon button (#48962)
7556609ff5 file_finder: Don’t use focused file’s directory for CreateNew unless it belongs to project worktrees (#42076)
b9c1970401 cli: Use Terminal Panel's window for `zed --add` (#45073)
cbb1eed3f4 Surface UI checklist in CONTRIBUTING.md (#49013)
e8cfd1941d terminal: Preserve terminal order and fix pinned count on workspace restore (#44464)
213de2ec9b editor: Do not include inlays in word diff highlights (#49007)
21ad340f01 Re-land #48959 (#48990)
8e04d034a0 gpui(windows): Create d3d11 device only once, not twice (#49001)
a6797892ec Resolve semantic merge conflict (#48999)
c6cd08e37a Use document symbols' ranges to derive their outline labels (#48978)
317a578f6a repl: List python environments first (#48763)
377407c2bb languages: Fix local path of JSON and YAML schemas (#44794)
b8df3374bb Remove unnused line from bundle-mac (#48972)
4f2a277a63 go: Add first-line pattern detection for executable Go scripts (#48913)
ee3f40fe25 Re-add MultiWorkspace (#48800)
83de8a25e0 Revert PRs for landing in main (#48969)
d7900120e8 Revert "ci: Use cargo-mtime to track timestamps" (#48987)
8976b33a8e Fix more sccache problems on windows runners for external PRs (#48980)
d07955208f Swap `agent.single_file_review`'s default value to false (#48619)
71a704ed19 ci: Use cargo-mtime to track timestamps (#48956)
b53ff3b893 git: Disable some more expensive things for the split diff's left-hand side (#48953)
11d812cd87 Fix sccache --show-stats not working sometimes on windows (#48974)
7ccd6976c4 Remove thinking tool (#48952)
718dc62dbc docs: Add AI SEO frontmatter (#48559)
56504fdd0f Support LSP document symbols in breadcrumbs and outline UI (#48780)
1e2874c988 Fix ACP agents not loading when not authenticated (#48959)
f233ae4c29 Add telemetry for user-facing notifications (#48558)
c7782553db Fix error when loading ACP registry (#48957)
964818733c copilot: Fix Copilot not respecting `disable_ai` setting (#48495)
f571fab30f Fix sccache on windows (#48943)
8238f3bfca zed: Remove windows nightly channel toast (#48950)
770ff7cbd9 repl: Write test for JSON output (#48897)
de5fc22335 repl: Test outputs and ExecutionView (#48892)
5c6aa4481c Bump Zed to v0.225 (#48941)
4ed2b3d041 rope: Fix `point_utf16_to_offset` returning relative offset instead of absolute (#48862)
40b468e69e git: Fix another `WrapPointCursor` panic in `spacer_blocks` (#48932)
3938403a01 Fix terminal pane showing welcome screen during loading (#48930)
975d5acc64 ztracing: Annotate more callsites that invoke TreeSitter (#47522)
bf01a928e5 agent_ui: Fix editor not getting focused when adding selections (#48926)
8f707653c2 docs: Update the formatting section for Python (#48904)
38800907cc gpui: Fix double-close of fd in read_fd on linux (#48850)
f7314976bd git: Enable the split diff for everyone (#48912)
05e99dad54 git: Standardize nomenclature for side-by-side diff (#48910)
2b7b8865ba agent_ui: Fix keybinding display inside tooltips in the footer actions (#48909)
6f09ead50e git: Add click with modifier to set default for side-by-side diff toolbar buttons (#48908)
47cb1cf4ab Use sccache in CI (#48895)
74419f6a0d git: Correct logic for updating companion excerpt IDs mapping (#48906)
8e3e6b8129 Fix path handling regressions on Windows (#48905)
6cd7586c16 Switch to a Zed user-agent header for Copilot traffic (#48591)
de213ad4e7 Replace `always_allow_tool_actions` with `tool_permissions.default` (#48553)
c8054cacbd git: Fix searching in the split diff (#48894)
d2c922b815 Reuse existing windows when opening files in remote workspaces (#48891)
aba74e6ca3 git: Fewer buffer diff snapshots (#48886)
aa37edaef1 Don't run nix checks on PRs (#48849)
094122af92 devcontainer: Support `.devcontainer.json` in project root (#48814)
191b4ccd4f workspace: Move the update Zed button to the title bar (#48467)
695fcde635 Canonicalize extension paths (#48632)
9e1fed301d ep_cli: Include `request_id` in rated events and tag rated examples with experiment name and prompt format (#48883)
82065217c9 ep_cli: Add sync deployments command (#48880)
4b4fb67c0e Un-feature-flag granular tool permissions (#48881)
77df6bfef2 Tool permissions settings UI improvements (#48796)
1176c39476 ci: Fix builds on Windows following mtime finnickery (#48878)
de7f2f0186 Improve performance when detecting JSX auto close (#48622)
53a0075e03 terminal: Fix Spawn Task and Agent terminal hangs on macOS (#48877)
4911855194 vim: Make `:bdelete` use new `workspace::CloseItemInAllPanes` command  (#48592)
95890a987e Revert "ci: Restore mtime of source files to the git log time" (#48874)
f7a4fb1809 linux: Forward env vars starting with ZED_ to flatpak-spawn (#48118)
7f4540804b acp: Only use the resumed cwd if it is in the workspace (#48873)
8b5328cad3 Bump tree-sitter-go to v0.25.0 (#48870)
13154e1b0a gpui: Impl `Cow<'static, str>` to `IntoElement` (#48585)
4084a5ac13 ci: Restore mtime of source files to the git log time (#48607)
61e575547c languages: Change syntax highlighting for C/C++ preprocessor directives (#48109)
7a6674d5dc agent: Move subagent spawning to `ThreadEnvironment` (#48381)
ad7b1f52b4 vim: Make end of line infallible (#48867)
64c3e8a848 languages: Exclude TestMain from Go runnables (#46575)
4177be4479 git: Optimize `patch_for_range` functions (#48852)
da8a611113 Expose the scheduler executor for ex (#48854)
4ffaf643d2 repl: Bump jupyter-protocol to v1.1.1 (#48848)
303c1aa15f repl: Be more resilient with message parsing (#48837)
f81dfca30a collab: Remove unused `Config` fields (#48836)
2efdd35808 ui: Improve icon preview visualization (#48834)
24d9c00e0e repl: Use split() on shell and control dealer sockets (#48823)
2028f85546 languages: Fix block comment indentation for C/C++ (#47810)
5ba3d6aa22 languages: Fix C/C++ outline for function definitions and declarations with mixed storage and type qualifiers  (#47500)
b2150a4f02 Fix incorrect first line detection for libstdc++ files (#48351)
28e743b4d6 languages: Add attribute highlighting in C++ (#47825)
5af15f7c9f Fix panic in zeta1 prompt parsing (#48822)
3f53a3a8ab Add a `Show All Channels` button to the collab panel (#48826)
b97a6c3886 extension_ci: Increase timeout for extension version bump job (#48828)
0821d6fb73 collab: Remove `GET /extensions` endpoint (#48827)
d7129634ee git: Fix missing excerpts panic with side-by-side diff (#48755)
3d327cf2aa extension_ci: Be more precise with clippy targets (#48824)
7a18c4126a git: Side-by-side diff UX improvements (#48821)
81f93541d7 docs: Mention support for alternate LSP for Kotlin (#45650)
29f2048f88 repl: Shutdown all kernels on app quit (#48760)
7f2ee83173 repl: Bump `runtimed` ecosystem packages to 1.x (#48817)
2d1e211133 Update community champions (#48818)
c873440ca1 docs: Suggest alternative to Vim tag navigation feature (#48735)
5e86a8d148 extension_rollout: Improve naming for PR titles (#48815)
82dedcbc6c extension_ci: Merge extension workflow files into one file (#48810)
fb0af50553 gpui: Allow OS caption/buttons for custom Windows titlebar (#48330)
aea984bda0 visual_tests: Init agent UI globals in runner (#48076)
196504fe0b repl: Pluck project off editor directly (#48808)
495d9665c5 repl: Pluck project off editor directly (#48808)
7390b449f9 git: Fix panic when folding ranges (#48809)
3b81feb7c3 devcontainer: Fix project search returning no results on single-CPU containers (#48798)
9120c96bfa git_ui: Dismiss modal when viewing stash (#48491)
f0287d69e5 (tag: collab-staging) ep: Compress EP requests (#48793)
8582ca68a5 git: Fix a panic in `spacer_blocks` (#48753)
165b404460 (tag: nightly) Revert "New multi workspace (#47795)" (#48776)
bf2171d3f0 project_panel: Improve file/folder creation behavior in folded paths (#46750)
a41bec82f9 agent_ui: Insert images at cursor position (#48779)
e0a2561e14 Add format-on-save support to streaming edit file tool (#48663)
415b558868 Extract shell_command_parser into shared crate (#48660)
a27304bf91 agent_ui: Refine thinking effort selector (#48790)
44364ebd03 extension_host: Add DAP methods dispatch for v0.8.0 (#48777)
916b13248f Fix cargo package metadata (#48778)
404cb3eeed agent: Remove duplicate `line_hint` assignment in `StreamingFuzzyMatcher::push` (#48317)
3d88c7063a title_bar: Fix `ToggleProjects` panic (#48775)
97835f9a5f agent_ui: Fix panic when resizing panel (#48772)
bdff8bf47c Ensure proper workspace is used for various actions (#48767)
1137b3c0f7 bedrock: Add Claude Opus 4.6 (#48525)
db6c006f73 repl: Pluck project off of editor directly (#48762)
5e0d3123e9 docs: Add theme ID suffix recommendation (#48679)
1c21718587 New multi workspace (#47795)
930f484fe4 git: Follow-up fixes for custom blocks in the side-by-side diff (#48747)
a96d777988 agent: Fix disabled MCP servers disappearing from UI after restart (#47758)
3b31224448 Update Rust crate git2 to v0.20.4 [SECURITY] (#48400)
1ebfc9158f Update Rust crate time to v0.3.47 [SECURITY] (#48514)
87c92f3db3 gpui: Reset `external_files_dragged` after successful drag-drop on macOS (#48727)
f1f8c5523b language: Return early if no grammars are added (#48685)
809d54524e Properly handle multi-char folds (#48721)
aa11edf99d collab: Proxy `GET /extensions` to Cloud (#48717)
35160555da git: Fix a potential misalignment in the side-by-side diff (#48690)
5ed13a0293 Move extension API DTOs into `cloud_api_types` (#48689)
869919160b git: Add a setting for the default view mode of `SplittableEditor` (#48440)
7fdafe33fa Use proper settings name for semantic tokens' settings UI (#48686)
79f38fea64 gpui: Fix restarting panicking due to double borrows on windows (#48667)
85294063fc Strip broken thinking blocks from Anthropic requests (#48548)
ec7e9dc1cc keymap_editor: Add `alt-l` keybinding for cycling favorite models (#48390)
496b12e9b8 Only raise Windows timer resolution while blocking with timeout (#48379)
10c3c088fe editor: Propagate `buffer_font_features` to signatureHelp popover (#48653)
db53a65ab6 Add configurable LSP timeout setting (#44745)
52cddaae37 editor: Use buffer_font for folds and change foreground color (#48652)
641c58d04c lsp: Update root_path for compatibility with language servers (#48587)
52099b45e7 Fix panic with LSP folds on disappearing excerpts (#48649)
7410e1050d Harden tool authorization: sensitive settings, deferred ops, copy/move patterns (#48641)
38815c1a45 Add tests for user-reported rm security bypass variants (#48647)
148b102762 Update the PR template to have a checklist (#48646)
73d10cb818 Shell parser: Handle I/O redirects, here-documents, and compound commands (#48635)
cab418a64b Fix MCP tool name parsing: use newline delimiter instead of colon (#48636)
7fa4cfcfed Strengthen hardcoded rm security rules and add path normalization (#48640)
4a89623977 Handle authorization send errors instead of silently dropping with .ok() (#48639)
acbc6a16ac Remove fine-grained tool streaming beta header (now GA) (#48631)
d8b2c03c2e Improve rate predictions modal (#48630)
263d8e58d8 Remove zeta example capturing (#48627)
30ec2ca370 Support custom fold text for LSP folds (#48624)
438a820822 Clean up some `Cargo.toml` files (#48623)
a8ae51cb97 build: Tear up crate graph (move terminal closer to editor) (#48602)
afafb66f76 agent: Highlight latest models available through the Zed provider (#48614)
fd1d5fcb9b settings_ui: Fix Ollama icon and other small UI details in EP subpage (#48612)
6c253a7d68 Add `textDocument/foldingRange` LSP support (#48611)
101a53d904 Respect workspace override in `git: diff` (#48535)
980479fb7c Refactor LSP-related logic (#48604)
4f8ff64452 Fix settings migrations for nested platform/channel/profile keys (#48550)
aa33a50ebe language_models: Enable setting reasoning effort for OpenAI models through Zed provider (#48605)
35459f0ad6 Detect and reject overlapping edit ranges in streaming_edit_file_tool (#48547)
3d57951b41 feature_flags: Rename `cloud-thinking-toggle` feature flag (#48598)
8e56667625 git: Implement `OpenExcerpts` for the left side of the side-by-side diff (#48438)
e2267aba12 Fix text_threads_dir() non-compliance with XDG spec (#45771)
7377cb6554 Canonicalize --user-data-dir path to match file watcher events (#48470)
4167f170de SplittableEditor: Sync custom blocks between RHS and LHS editors (#48575)
669f8ccda4 project_panel: Refactor selection state ownership (#48586)
7667fd32a3 Tidy up semantic tokens settings ui (#48581)
fe0f7f3536 project_panel: Fix previous selection not restoring when filename editor blurs (#48578)
37bd7b31f0 Autolabel pull requests by bots (#48579)
c94b28fecf Autolabel staff's pull requests (#48576)
c430681211 Do not pass zeta prompt format in production endpoint (#48541)
2cecdab47d Don't watch parent directory of global gitexcludes file (#48408)
7cb4d75139 Housekeeping: Improve error handling and small fixes (#48552)
ff4489f426 Revert "Remove VS Code references from Copilot integration" (#48555)
9860106b8e agent: Add support for setting thinking effort for Zed provider (#48545)
8d730846eb git: Side-by-side search matches panic (#48544)
eb9103e667 git: Side-by-side diff searching (#48539)
9743dbc1ec agent: Add `agent.default_model.enable_thinking` setting (#48536)
0d9bcf0ee5 Fix Codestral API key credentials URL mismatch (#48513)
e4d82099eb agent_ui: Add additional telemetry for thread errors (#47971)
a2ca07514c language_model: Add `supported_effort_levels` method to `LanguageModel` (#48523)
5259b24896 settings_ui: Fix tab navigation in edit predictions settings (#48530)
e8030aea4a Improve extension installation (#48518)
4144422c2b ep: Fix `parse-output` for repaired outputs (#48529)
72a12eb97c Remove VS Code references from Copilot integration (#48528)
a22092c3bc settings_content: Remove unused methods from `AgentSettingsContent` (#48526)
0e29766682 Update `cosmic-text` to v0.17 to fix font ligatures on Linux (#48504)
46017f9612 Add tab switcher mode similar to vim/helix buffer picker (#47079)
74746aa974 multi_buffer: Optimize a bunch of things (#48519)
7dcff21dc9 anthropic: Update types for adaptive thinking (#48517)
65da2c2c6a Use AgentTool::NAME const instead of name() method and hardcoded strings (#48506)
24b6cbf575 Add Claude Opus 4.6 and 1M context window model variants (#48508)
c47430debf helix: Fix line select with empty first line (#48026)
8bb69dcb14 Properly discard tokens on language server stop (#48490)
723f683dda text: Actually short circuit anchored_edits_since_in_range (#48486)
4389ebb6f4 Follow-up semantic token fixes (#48485)
de17358fb4 language: Use TreeMap for `language::BufferSnapshot::diagnostics` (#48482)
044dca12b7 ep: Use programmatic scores as a signal for repair (#48478)
8c047d9469 workspace: Remove layout shift from multibuffer toolbar (#48472)
389f66591c ep: Store cursor position in global coordinates (#48469)
a3b36ae9a4 ep: Fix trailing newlines parsing (#48473)
e5579cfc91 agent_settings: Fix Bedrock model catalog URL (#48382)
a8c3c1a6a5 extension_api: Add `digest` to `GithubReleaseAsset` (#48413)
ee41463038 ci: Move more jobs to Namespace runners (#48228)
dedf390b75 ci: Use package name for contents of changed_pkgs (#48459)
6e5e6cf8a5 languages: Remove duplicate keywords in JS syntax highlighting (#48460)
7694ea2687 gpui: Detect thermal state to help the system (#45638)
92f8ae8635 debugger:  Wrap TaskContext in Arc to reduce cloning overhead (#47087)
1fe80040b1 debugger: Fix breakpoint color only updating when mouse moves (#46427)
87dff2a1db JetBrains keymap: Add escape key binding to cancel in auto_height mode (#45306)
9cd2343432 editor: Put more syntax walking features on the background (#48450)
81562a30c7 Make middle click not mousedown close tabs (#44916)
053a8fed22 Handle soft wrap modes when computing wrap width (#46422)
1dca30e3a1 languages: Remove unnecessary `unresolvedReference` semantic token rule (#48448)
86f307ef9a project panel: Collapse All improvements (#47328)
12091a9f2c project_panel: Fix collapse_all_entries collapsing single worktree root completely (#48443)
23e8f13e9c agent_ui: Fix missing TerminalSelection match arm in mention_set (#48445)
592d21898d Parse and render markdown mention links from pasted text (#45426)
804caba60c git: Add toolbar buttons for `SplittableEditor` (#48323)
323680f6fe Fix shell quote bypass in terminal permission system (#48436)
0dd09b6097 git: Render hunk gutter highlights for the side-by-side diff (#48433)
49c777779a Add stacked and split diff icons (#48427)
2b5386b438 Fix vim increment/decrement on Markdown list markers (#47978)
7d4dd8735c docs: Add missing AI features (Dec. 2025) (#48418)
18a0103112 vim: Add $ and = as subword motion boundaries (#48276)
ed3e18de6f Reduce rewrapping when agent diffs are present (#48423)
cfb4cefb37 Fix multibuffer to not mark everything as edited when buffer diffs don't change (#48424)
f4b715b787 docs: Fix rules description in AI overview (#48420)
bae3eda75c git: Shader for checkerboard pattern for side-by-side diff (#48417)
307f537ab3 Remove cmpl- prefix validation from Mercury feedback (#48410)
4cb85917c5 Differentiate between explicit rejection and ignored in ep acceptance tracking (#48409)
6a71a60a12 lsp: Add schema support for LSP settings field (#48332)
dd6f327c99 Revert "git: Render hunk gutter highlights for side-by-side diff (#48321)" (#48416)
f0dfb443d6 ci: Trigger tests on Cargo.toml and Cargo.lock changes (#48412)
0056bbe386 editor: Use tabs bitmask when syncing (#48366)
4433d6d855 docs: Apply brand voice standards to AI documentation (#48401)
4563b5ac4f lsp: Add `container_name` to `lsp::Symbol` (#46822)
a71aa4ecf9 Add telemetry for EP Menu Actions (#48229)
c82d104e07 Add telemetry for toolbar menu open events (#48225)
e39c1906e1 edit_prediction: Add Mercury accept/reject tracking (#48306)
477bb89f10 Expand hardcoded agent terminal security rules (#48399)
54588a1e7d Allow always_allow for nushell, elvish, and rc (#48395)
825cf2db7e ci: Parse Cargo.toml and Cargo.lock with Python script to determine affected packages (#48389)
fdc6282544 git: Fix branch diff not updating when committing or changing branches (#48388)
bb2ee343b4 project_search: Guard replace_next match lookup (#48327)
13fbbaeb59 Refer to devcontainer.js rather than devcontainer.cmd.js (#48387)
356fb94cae ci: Run tests only for the changed packages and their dependants (#48380)
c390dda824 git: Render hunk gutter highlights for side-by-side diff (#48321)
ad339e31ca docs: Update edit prediction providers (#48055)
a5e6964186 agent_ui: Refactor `AcpThreadView` (#48339)
5d84932f4e Do not treat binary Doom WAD files as text (#48349)
5d2feaa144 editor: Implement semantic highlighting (#46356)
c43dba103b project: Make individual DAP settings optional (#43647)
52c963df1d git: Ensure the sides of the side-by-side diff use the same hunk controls (#48376)
e106117b87 Bump Zed to v0.224 (#48375)
280295cbc0 Fix scroll in EP provider sub page when setting migration banner is showing (#48372)
3251f69657 ep: Lower max_output_tokens default for Ollama (#48370)
86b9893282 gpui(windows): Reduce timer resolution to 1ms (#48364)
e54480bd2d agent_ui: Fix regression that made edited files unclickable (#48362)
327cc601d6 docs: Add BSD 2-Clause as accepted extension license (#48363)
d35ac6a6a3 acp: Bump agent-client-protocol sdk to 0.9.4 (#48356)
09fece9cb8 languages: Do not show Zed keybinding context in language selector (#48354)
57e1681809 acp: Cleanup unneeded cx from config state methods (#48353)
ab54a7e29b repl: Add quality of life changes in Jupyter view (#47533)
78c121d98f docs: Update `git_hosting_providers` setting documentation (#42129)
fa37d87ce8 ep: Meaure isolated whitespace change ratio (#48352)
8ff52904a2 acp: Remove feature flag for session config options (#48350)
3b8d003ccc ep: Heuristic for detecting wrong editable region (#48343)
e27924a2d3 Auto-detect Rust for CodeLLDB panic breakpoints (#48236)
99f80d7693 remote: Fix build_remote_server_from_source compression on unix (#48319)
a74ad50913 Disable `subagents` feature flag for staff (#48335)
21b3cfea6c docs(gpui): fix incomplete docs for function (#48333)
d5e58b9eb4 workspace: Close toasts on middle mouse click (#48208)
203d03d405 Remove sweep and mercury feature flags (#48329)
fb40850c74 settings ui: Add page for AI tool permissions (#48277)
d1aab6a511 Show provider configuration in EP menu when signed out (#48325)
389a20471a git: Add support for staging/unstaging/restoring to side-by-side diff LHS, and render hunk controls there (#48320)
7c11b03838 Eagerly connect to Ollama (#48318)
3ae0da3acd git: Use `WrapPointCursor` for spacer blocks calculation (#48315)
5b08f4ccac Remove Postgres and `stripe-mock` from Docker Compose (#48313)
8b6bd20a6a Try to improve nix caching (#48297)
8bf3b4fece git: Refactor buffer_diff point translation APIs for more efficient side-by-side diff syncing (#48237)
555c002499 Add initial support for edit predictions via Ollama (#48233)
eff4b9356b ci: Re-enable docs deployments (#48303)
1dffb8c198 Simplify error message and provide a route to Zed log (#48301)
9ef1afd6d5 Optimize resource upload in D3D11 (#48282)
8f4c493e90 (tag: collab-production) Update Rust crate jsonwebtoken to v10 [SECURITY] (#48294)
cb647fc482 Disable default tool permissions (#48278)
477069ef62 languages: Remove duplicate keywords in TSX syntax highlighting (#48196)
30b7762283 Update Rust crate bytes to v1.11.1 [SECURITY] (#48293)
1aee8b48cb agent_thread: Improve wording of confirmation label text in agent thread (#48202)
f07305ba71 Remove duplicated logic to unify code paths (#48291)
d95478295a editor: Ensure that spacer blocks are visible in light themes (#48287)
9fbf609e6a ci: Temporarily disable docs deployments (#48292)
1b86dbc78b git_ui: Hide "View on GitHub" button when viewing stashes in commit view (#48271)
0952ee7d3a project_panel: Fix test-support feature mismatch (#48280)
13a06e673b Add detection of devcontainers in subfolders (#47411)
18a3b0c53a Re-add deleted comment in `default.json` (#48279)
6600154f44 Fix incorrect memory display in the language servers menu on Linux (#48245)
3f8bc2d4b7 languages: Add support for detecting libstdc++ headers (#48250)
f565fb8750 gpui: Make entities no longer implement Element (they go through AnyElement now) (#48217)
92ad7c3000 project_panel: Add right-click download option for folders/files stored in remote development server (#47344)
f21a93391f settings: Improve performance for internal editorconfig resolution (#48243)
b951bd3d6f Fix subdirectory `.editorconfig` files being ignored in certain directory structures (#48203)
14621b66b8 repl: Fix cursor visibility on last line after re-running cells (#48218)
602d64f238 Add configurable REPL output size limits (#47114)
8b86ab9284 Revert "git: Fix stage/unstage failure with a large number of files (#47800)" (#48238)
2539ac0263 vim: Revert changes to search (#48127) (#48234)
81c65414ca Make mercury and sweep non experimental (#48227)
8fd3b85699 Migrate `features.edit_prediction_provider` to `edit_predictions.provider` (#48224)
64a0254204 ci: Generate `publish_extension_cli` with `gh_workflow` (#47890)
85d03d5122 Use remote user from devcontainer CLI and respect shell in passwd (#48230)
7b9beb3678 Show memory used by language servers (#48226)
478ca3a762 block_map: Consolidate all companion-related refs into a single CompanionView (#48223)
68b2cb3e72 Fix capitalization of libX11 in FreeBSD dependencies (#48159)
b96f1c4738 Add `sweep_ai` privacy mode setting (#48220)
b62d73e4bc Add tool security rules that can't be overridden by settings (#48209)
f0594e0990 settings_ui: Make it so settings links open sub pages (#48212)
1ac0b77d02 toolchain: Include rust-src in the rust-toolchain components (#48210)
eae101f3c9 Revert "rust: Highlight enum variants as variant" (#48211)
571ea7cf22 extension_api: Improve documentation for `make_file_executable` (#48198)
9867f16c77 edit prediction: Drop project state after a project goes away (#48191)
162f3efdd9 Fix issues with predicted cursor positions (#48205)
64829e992f Add `ep truncate-patch` command to prepare commits evalset (#48204)
a5e15da0fd Add completions to rate prediction modal for common failure modes (#48199)
c06847e48e language: Avoid cloning of the previous tree_sitter::Tree (#48197)
174fc51152 Added support for dismissing the toolchain path selector via `esc` (#48201)
324edc1883 git_ui: Fix indent guide rendering in tree view with collapsed folders (#48194)
fa0db76811 Fix SSH reconnection message (#48190)
70c47e96bd markdown: Fix markdown table selection hit testing (#47720)
a1558bc5cf ep_cli: Integrate `ep qa` and `ep repair` into core `ep` loop (#47987)
f0c165ac41 vim: Refactor vim search settings test (#48165)
45455dbd06 gpui: Reduce amount of monomorphizations taking place (#48014)
4c0e348588 keymaps: Fix accept edit predictions key binding collisions (#48184)
cfbf6ebe96 nix: Use flake-parts, partitions, and treefmt-nix (#45321)
a4fe57d607 Fix vim method/comment navigation with expanded diff hunks (#47976)
be8842b941 agent_ui: More refactors (#48186)
9c3ae09896 Improve icon selection for edit prediction providers (#47911)
e99c11dee6 build: Decouple git_ui from recent_projects (#48062)
e74ceba097 agent_ui: Disable "Copy Selection" when no text is selected (#47997)
33fbc7163a docs: Document visual mode bindings for multi-line insert/append (#48156)
431c623eb7 workspace: Add copy button to ErrorMessagePrompt popup (#48107)
9ee0f08944 agent_ui: Fix panic when inserting context prefix with multi-byte characters (#48179)
6d772579e1 terminal_view: Add support for opening the terminal in the current file directory (#47739)
782f91f320 Fix launching from WSL with fish as default shell (#48136)
af0e45e046 terminal: Prevent terminal cursor stretching by always using cell width (#45545)
9b1fca4967 docs: document right options for `lsp_document_colors` (#48170)
0e118ac9ae project: Simplify file search logic (#47982)
aafecf043a vim: Respect search settings in buffer search (#48127)
b874a40549 ep: Add V0131GitMergeMarkersPrefix prompt format (#48145)
fb428b5a50 gpui: Rewrite Windows `PlatformTextSystem` for better performance (#48087)
2570382f88 git: Disable vertical scrollbar for side-by-side diff LHS (#48129)
f7fe6181cd Implement `extract_pull_request` for GitLab provider (#47973)
839b4f1e60 git: Fix stage/unstage failure with a large number of files (#47800)
c2959a7f7e git: Disable folding in side-by-side diff (#48108)
48f712b49b Fix git panel not updating after commit (#47222)
d6eda52b32 Add Azure as a git provider (#47598)
a2dbf0439f git_ui: Open file on double-click in status list (#47989)
2867ec4bc2 git: Fix force-added files always showing as ignored (#47057)
9101f0ffc9 git_ui: Fix visual optimistic staging bug for tree view (#48078)
608cc9953f git: Remove panes from SplittableEditor (#48102)
36e35f7df1 git: Use CDN endpoint for GitHub avatars to avoid rate limiting (#47894)
8d18b7b927 git: Fix desynced scrolling between LHS and RHS of side-by-side diff (#47913)
b642565526 Allow zeta2 to predict next cursor position along with edits (#47916)
795eb34098 Fix open path prompt not showing hidden files (#46965)
03663b966d rust: Highlight enum variants as variant (#47918)
4dde88cfe5 languages: Support double quoted keys in YAML highlights (#48063)
a6ab1f1eb7 Fail windows bundle when cargo about fails (#48056)
fe653a77ae settings_ui: Fix editable project settings not being updated when a new workspace is created (#47992)
609e915b10 vim: Restore cursor position when dismissing buffer search (#47732)
72b151e3aa Revert "Allow always_allow patterns for Nushell, Elvish, and Rc shells" (#48050)
cfd6144af8 ep_cli: Only check cursor excerpt for reversals (#48044)
3a2a43ab84 ep_cli: Allow dynamically loading prompts (#48046)
b029238af2 settings: Extract project settings JSON schema (#47939)
325afbddff API-based version of duplicates bot (#48041)
a7596c468e Bump wax to 0.7 (#48042)
3d7fe6e1f8 agent_ui: Add debug command to copy and paste agent threads (#48039)
3723a2560a repl: Render `application/json` media types from Jupyter kernels (#47905)
7aa8c742d8 Add `ep split --no-stratify` option (#48036)
fa68aa7704 ep_cli: Further improve reversal ratio (#48028)
c3ce20b72b ep: Update teacher prompt to reduce edit reversals (#48022)
e80221ddc3 multi_buffer: Reduce region clones (#48025)
9b3777a14e multi_buffer: Speed up `Anchor::to_offset` resolution (#48021)
316a9702d4 workspace: Fix closed pane items not reopening due to stale preview state (#45286)
024345f999 editor: Deduplicate multibuffer blame error messages (#48013)
80b9c3a908 ep: Add --cache-only option to avoid sending requests (#48011)
5eeff4942c agent_ui: Subagent permissions (#48005)
bf829a6da5 ep: Handle input data errors (#48004)
3e5d1ee8b9 terminal: Update title changed info on the background (#48000)
185a80a21b cli: Teach `--diff` to recurse into directories and add a `MultiDiffView` (#45131)
7053561b63 docs: Document default for include_warnings (#47838)
b9c0f756c4 fs: Cache filesystem case sensitivity (#47995)
19765cd0b4 gpui(windows): Move jumplist creation to background thread (#47956)
28f0461e94 workspace: Do not attempt to unnecessarily update jump list on invisible worktree updates (#47990)
9aa71ddbce Python: Improved the `Add Virtual Environment` path browser to start at the workspace root (#47988)
0ede074c05 Fix missing directories in path prompt when show_hidden is enabled (#47983)
0c8a753a3c ci: Fix release failure notification (#47977)
fed8b93aa6 Revert "gpui: Take advantage of unified memory on Apple silicon" (#47974)
f1554f83ed Remove label field from Feature Request template (#47970)
b1fd95b4f9 copilot: Fix references to uninitialized CopilotAuth global (#47954)
22e2c7ac8a collab: Improve project and call notification UI (#47964)
ad69357968 agent: Support initial prompt via zed://agent URL schema (#47959)
b9e9b85cc8 Add security warnings to issue templates (#47957)
e713879a9c Add feature request discussion template (#47962)
ba41499bf4 gpui: Take advantage of unified memory on Apple silicon (#45577)
04d878eef8 typescript: Fix type import highlighting when alias is not present (#47190)
a09f0bba9c git: Use LHS/RHS terminology consistently for side-by-side diff (#47949)
cd8a7b4292 Add back "fs: Replace MacWatcher with notify::FsEventWatcher"" (#47952)
263e0004ce agent_ui: Add adjustments to terminal selection as context (#47950)
ad24b3f0a0 git: Fix missing hunk controls on side-by-side diff RHS (#47815)
949972bbbe project: Always allocate WorktreeIDs on the remote client (#47936)
52182e43cd agent_ui: Add ability to add terminal output to agent context (#47637)
d18d4b188c thread view: Improve thinking mode toggle UI (#47945)
7161bf5940 thread view: Improve load errors UI (#47943)
2c4a823e70 project: Increase search detect yield threshold to 20KB (#47824)
723a58e9bf docs: Update content in the External Agents page (#47935)
4c8b699f03 agent_ui: Refactor in preparation for subagent permission work (#47874)
b7e11b38f4 Allow always_allow patterns for Nushell, Elvish, and Rc shells (#47908)
8bdfbfe591 git: Fix a couple of bugs with buffer headers in the side-by-side diff UI (#47809)
1870425b2b ep_cli: `rated-after:` query (#47906)
05a021c4a3 docs: Correct action descriptions to reflect actual purpose (#46931)
483e64a287 image_viewer: Compute better initial zoom level (#47886)
9437e84465 Add student plan (#47866)
3e7f0ce7b7 languages: Add highlighting for Go directives (#47621)
97f0ab77ca ci: Define default shell for all workflows (#47883)
dbeb0af9ad Fix shell injection vulnerability in terminal tool permissions (#47807)
e18162216e ep_cli: Further improvements to reversal tracking (#47897)
7e065aedec docs: Update AI docs with new features that went out (#47898)
2cd8b4241c Improve EP teacher prompt, add some CLI features (#47814)
081bb237f4 Improve Zed agent thread history to update across all open windows (#47803)
1c1a6578c2 client: Fix ACP registry blog post URL (#47881)
446227cf6d repl: Streamline Markdown output usage (#47713)
a21e21c15c git: Hide toggle split diff button behind feature flag (#47878)
308537f4ad cli: Add GitHub token to `generate_message` step (#47876)
c3fddcd593 client: Store `ReleaseChannel` instead of `&'static str` (#47868)
5e0b8cc0f7 extension_ci: Update extension CLI SHA (#47875)
28a9da4b32 time_format: Determine year difference based on months passed (#47687)
c37a7ba2f6 Update community champions list (#47785)
c21b019eaa dap: Make `ProtoConversion::to_proto` consume `self` (#47871)
bd59ead0fe Bump Zed to v0.223 (#47869)
bc0e826723 editor: Move scroll position persistence to background and debounce it (#47834)
4dac6bd6d7 docs: Add content about the ACP registry (#47870)
03d8e9aee9 (tag: extension-cli) editor: Add select to {start,end} of larger syntax node commands (#47571)
eb14c9d345 Use `SharedString::new_static` for string literals (#47865)
7c6f7118e4 git_graph: Add feature graph (#47861)
ec4a9cbffa gpui: Remove some unnecessay heap allocations and memcpies (#47858)
bcd78afabf git_ui: Align repo and branch pickers (#47752)
7cfa0fb876 extension: Ensure that compiled extension provides at least one feature  (#47715)
e29522a1b7 acp: Add upsell banner for built-in agents in registry page (#47853)
9bfb900caa remote_server: Cleanup old server binaries on wsl (#47839)
9622179798 agent_servers: Inherit codex api key environment vars for remote (#47850)
1c7fa87d18 markdown: Improve double-click word selection using CharClassifier (#47844)
92495328d1 languages: Add first-line C++ language detection for extensionless libcpp headers (#47443)
2dd4897925 lsp: Exclude dynamic settings from LanguageServerSeed identity (#47376)
e2f54a1272 languages: Add highlighting for document comments in C/C++ (#47556)
3a1f4776d4 docs: Add missing syntax highlighting captures (#47791)
3339c2d982 Fix the duplicate bot (move timeout, perms) (#47840)
0e1802a596 acp: Add agent server extension deprecation banner (#47817)
ade8749537 Fix typing emoji (#47835)
80f1cc032e project_panel: Add `bold_folder_labels` setting (#47631)
e752ec1a92 Add item for opening Markdown/SVG files in preview tab in right-click menu (#47821)
17d34db366 Fix issues with Windows SSH support (#47822)
06b5ec4664 ep: Diagnostics as `retrieval_chunks` for sweep ep (#47441)
cedfc7e98c zeta2: Split out last edit in history (#47812)
9079f97066 ep_cli: Ignore common subsequences for more intuitive reversal metric values (#47818)
64df7f2675 ci: Enable namespace caching for clippy on Mac (#47819)
b9c56658ff Disable opening dev container from within a remote project (#47816)
344ee9fa83 zeta2: Improve format prompt budgeting (#47808)
4ea2c0923f Use suggested name for `"use_system_path_prompts": false` dialogue when creating new files (#47802)
87d41c1d97 Revert "fs: Replace MacWatcher with notify::FsEventWatcher" (#47799)
b1f9ca96ec fs: Replace MacWatcher with notify::FsEventWatcher (#47322)
4ffe748461 agent_ui: Fix inline assistant line height (#47772)
37b8f66df5 agent_ui: Add a menu for inserting context (#47768)
40468ed651 gpui: Add `min_size` and `max_size` style methods (#47775)
681fee2c1f agent_ui: Improve image preview on mention hover (#47781)
69d408b4b0 acp: Improve the ACP registry UI (#47788)
797ad8cf44 Fix language injections sticking after language comment removed (#46134)
a9fdd07e8d cloud_api_types: Rework `Plan` type (#47784)
07e675db7b zeta2: Remove `experimental_edit_prediction_context_retrieval` setting (#47783)
41ce23f896 vim: Add `gdefault` setting to set `/g` as a default substitution flag (#47664)
e77c528c1c Update notify dependency version to include the flag watch root fix (#47782)
cc1d3af65e Use faster brotli compression for sweep requests (#47776)
26381fe6ef More permissions to GitHub duplicates bot (#47780)
4723dbe696 cloud_llm_client: Move `Plan` type into `cloud_api_types` (#47778)
73bb119126 Use semibold (#47725)
8a89042289 Fix getting token in a GitHub bot (#47777)
39b34f8f33 cloud_llm_client: Remove unused code (#47774)
f324c3ef23 Add a test version of 'find duplicates' bot (#47773)
01409a2687 extensions_ui: Fix duplicated button element IDs preventing uninstall (#47745)
a07b08f63c Show QA metrics in the `ep eval` report (#47766)
be314db159 Tidy up signature help delays (#47762)
9ecafe1960 vim: Ensure paragraph motions use empty and not blank lines (#47734)
757ee0571e ep: Use rejected_output for DPO training + OpenAI support (#47697)
4866861f07 xtask: Remove transitive GPUI dependency (#47748)
8fb71083fe askpass: Fix ssh auth not updating entered password on windows (#47755)
aa6afca524 terminal_view: Fix new flaky terminal tests (#47753)
7e8ced2cd1 Remove cron schedule for unit evals (#47743)
e4dff26857 extension_ci: Use different target for nextest (#47738)
969fb8fe03 Fix native agent thread history notify refresh (#47733)
58a4a84e98 terminal_view: Add ability to rename terminal tabs (#45800)
b6ff556d6b go: Add support for running sub-tests in table tests without explicit variables for test cases (#46645)
e64985e516 Add optional relative line jumps to go-to-line action (#46932)
8e291ec404 encoding: Add "reopen with encoding" (#46553)
e868446ea2 Reapply "Try namespace mac runners (#47675)" (#47721) (#47727)
d0c3b33fa2 git: Don't rebuild the project diff when the active repository changes (#47718)
786b17dbeb git: Retain "since" diffs in the `GitStore` (#47619)
00ba24a7ca git: Avoid unwrapping active repository in panel rendering (#47700)
c93773cfe5 Make `workspace::SendKeystrokes` not use layout key equivalents (#47061)
c50120199f Revert "Try namespace mac runners (#47675)" (#47721)
d27fa3ba85 extension_ci: Ensure specified target is installed (#47717)
e7e8f0ba34 extension_ci: Set default toolchain for all jobs (#47714)
bdb84818ac Send some traffic to zeta2 for testing (#47710)
7a0a26dcf7 extension_ci: Set default target for all jobs (#47712)
f6018800d3 project: Add spinner while search is underway (#47620)
0db0cc65d6 Handle removed `IconName` variants in text thread deserialization (#47624)
40e7b10733 extension_ci: Bump timeout for extension bundling test (#47708)
b02192a9ef ep_cli: Add reversal metric (#47689)
21db564353 ci: Cache Rust toolchains in Linux namespace cache (#47699)
2b45efc1ea Try namespace mac runners (#47675)
18bc26866e Fix extraction of Windows remote server (#47690)
b9233dad73 Cap number of saved hang traces at 3 (#47674)
f73242abdb Return to Keep/Reject for agent edits (#47688)
d1f7c249b6 helix: Fix normal-mode selection expansion during `vim::Scroll` commands (#47024)
d9658563b5 extension_ci: Increase runners for tests job (#47686)
9d5f0f5aaf tasks: Show error for unknown `ZED_` variables (#45621)
b74b1977d4 collab: Extract tests into integration tests crate (#47668)
9326d61c44 remote_server: Avoid panic when writing to stderr (#47683)
ee721d393f acp: Dedupe session/list calls (#47677)
4007bf9ec4 ci: Notify on more release workflow events (#47565)
dd8e5a9417 StreamingEditFileTool: Disable for now (#47658)
51399d68a8 Fix remote server exit timeout (#47660)
e49dd8ba37 Fix Zed not entering disconnected state when remote proxy dies (#47659)
ee0e370e82 extension_cli: Improve snippet error logging (#47653)
987b140efa Improve TypeScript keyword syntax highlighting (#47435)
485e9b188a docs: Remove preview indicator for Tailwind CSS Mode (#47651)
86772d52e9 agent: Cancel subagents (#47647)
3a949102ed extension_cli: Validate that all snippets can be parsed (#47649)
1c113e7065 project_panel: Fix alignment of folded directory path components (#47648)
0a338c3731 Update nim entry with additional file suffixes  (#47616)
71c5e14665 Add `ep repair` command to address judge feedback (#47646)
c31ad941fe agent: Encourage more parallel tool use (#47644)
fb928c85e1 acp: Check for claude login message in terminal-auth as well (#47642)
3b5443ba96 syntax_tree_view: Fix highlights persisting after view was closed (#47638)
a1df9baa29 Allow EP cli to fetch rejections from snowflake (#47628)
9e183d94ee project: Do not send `UpdateProject` message on remote server spawn (#47633)
0af2e843ce Revert "Fix link for `Ownership and data flow` in GPUI's README.md" (#47630)
95cf4ab3f2 gpui: Add action_schema_by_name for O(1) action schema lookup (#47180)
7af276a984 terminal: Fix double quoting of commands on Windows cmd.exe (#47534)
b397bd049a remote: Add configurable remote server cache retention (default 5) (#47202)
9d7b6596dc git: Fix incorrect diff transforms being computed for side-by-side diff LHS (#47515)
9e2c415abb git: Fix incorrect spacers for added or deleted files at the end of the multibuffer (#47525)
c8139fb4e9 git: Fix missing spacers in side-by-side diff when editing inside an addition hunk (#47568)
6ebcd84182 git: Ensure folded buffers are synced when deploying side-by-side diff (#47498)
aceff52f8a project: Move tests to integration layer (#47596)
dadb5ea273 Fix link for `Ownership and data flow` in GPUI's README.md (#47599)
2f3c013cc7 copilot_chat: Fix Anthropic models not appearing in model picker (#47549)
e464905220 build: Re-enable incremental compilation (#47586)
b584192a4c terminal: Fix terminal freeze when child process is killed by signal (#47420)
6f090d3656 acp: Skip setting language on secondary diff buffer (#47526)
bf19f03f51 Apply common prefix/suffix stripping to zeta2 and mercury (#47530)
4e64180508 acp: Use the official ACP registry URL (#47564)
9931c6f944 Add SSH remote server for Windows (#47460)
e9d94748e8 gpui: Map diagonal resize cursor styles on Windows (#47477)
f21a357f76 Hide block cursor text when the cursor is within a redacted range (#45549)
3d4582d4dc git: Skip setting language on git index buffer (#47524)
19b4e432d9 agent: Add setting for controlling terminal tool stop button behavior (#47521)
2301c5f9f0 Send EP trigger as part of zeta2 prediction request (#47523)
84b40d507f ep: Add `qa` subcommand to check predictions quality (#47520)
ff34f22835 zeta2: Include context in captured examples (#47516)
e1076cd454 languages: Add structs, unions and enums to outline in C (#46125)
56c10d5f8a git_graph: Add resizable columns (#47485)
2bfe0bfd30 git_graph: Add loading icon when loading initial commit chunk (#47514)
9569157743 Clean up error handling for some edge cases to prevent panic (#47513)
38821a66a7 Reduce intensity of refreshing pull diagnostics (#47510)
25904f691e Add support for refreshing outdated LLM tokens (#47512)
fa534aeabe Don't try to spawn conda if it's not there (#47261)
097cfae77e Add helper method for checking if the LLM token needs to be refreshed (#47511)
5d8fd87953 editor: Fix bracket color change when system theme changes (#47505)
30f8841d04 editor: Fix inlay hint navigation for WSL/remote file paths (#46473)
c4747714c7 git_graph: Fix subject not wrapping (#47486)
9201e1f7b7 Don't always expose streaming edit tool (#47495)
6b0b95f539 Show red X icon for interrupted subagents (#47499)
01d72aa7d6 editor: Fix panics that could occur when content mask had negative bounds (#47327)
c9997592e4 build: Simplify build graph (#47253)
8419979cf9 acp: Add one more registry ID to filter out (#47496)
29cf14ed2f Fix rate limiter holding permits during tool execution (#47494)
9ef825d753 docs: Improve Git documentation accuracy and structure (#47419)
e6d87976bb Allow specifying ZED_VERSION with fallback to latest in `install.sh` (#45522)
7c98f1732e copilot: Decouple authentication from the lifetime of any single Copilot instance (#47473)
21f49eba7a agent: Ensure the activity bar shows up with the `StreamingEditFileTool` (#47417)
ff513cb14d copilot: Rename enabled_next_edit_suggestions setting to enable_next_edit_suggestions (#47484)
3a7c746043 Revert " Add vim/emacs modeline support " (#47479)
4efe93cfbf ui: Fix LoadingLabel animation panic on CJK/emoji text (#45632)
e1c80f4706 workspace: Support hot-exit for empty workspaces and single files (#46557)
9db716123d copilot: Add the option to disable Next Edit Suggestions (#47438)
30ca3ac44e languages: Add attribute highlighting in C (#47156)
9da3f2db47 util: Implement host independent Url to PathBuf conversion (#47474)
336bc3c26a outline: Fix uninitiated var declaration not showing in outline (#46499)
da3847af7b Support multiple snippets locations per extension (#45829)
515a840e76 build: Bump Rust version to 1.93 (#47358)
753302ef5c acp: Promote registry as default way of installing new agents (#47464)
36fef4446d agent_ui: Prioritize file completions that are closer to recently viewed files (#47466)
01e67d8321 acp: Support unstable session/resume request (#47387)
4333d6b472 agent_ui: Enable the message queue feature for external agents (#47379)
d4f2056ea4 remote: Fix when .ssh config is missing (#47310)
5eb2ff065b Relax the checks on `xdg_wm_base` to ensure compatibility with Weston 9 (#47185)
916da3d2a5 Increase operations/run for stalebot (#47459)
d3b1f7f042 git: Fix graph view slow initial loading times (#47453)
4186e5764f  Add vim/emacs modeline support  (#44210)
8273e6d868 agent: Multiline review comments (#47448)
7ba0bda2c1 git: Add graph support (#44434)
5418c91917 Make auto-signature help respect popover delay (#46745)
a5eb666951 git: Fix panic when committing from side-by-side view (#47425)
ad8f41a7ce git: Fix double-lease panic when syncing selections in side-by-side diff (#47430)
398da336f6 terminal: Fix `test_terminal_eof` test failing on fish shell (#47410)
13e130bc7e Update .mailmap (#47413)
ec981b8301 agent: Add thinking toggle for Zed provider (#47407)
76c885771a terminal: Give child processes time to exit on their own (#47408)
179112532a ep: Option to save evaluation report in JSON (#47409)
ff73838bab Add "Agent Panel Error Shown" telemetry with ACP error details (#46848)
a3fd8e6881 ep: Don't fail on lines with inference errors (#47406)
c63a0bc358 Parse output from older version of the devcontainer CLI by looking for a JSON object in plaintext (#47403)
d6bd3661e2 workspace: Persist dock state for empty workspaces (#45966)
5e9ca9cf77 Support Insider variants of Code and VSCodium for settings.json import (#47247)
66e8889c3b terminal: Clear hovered link when no target found (#47134)
4eff8975d0 Revert "Make apostrophes not brackets in markdown (#46414)" (#47396)
d2070975fe ep: Add line-level exact match metric (#47383)
8842cc696f Make apostrophes not brackets in markdown (#46414)
94e60a0669 zeta2: Cut oldest events to maintain prompt size budget (#47394)
4bc3b710ee Enable configurable dismissal of language server notifications that do not require user interaction (#46708)
e9aadaf0af repl: Add initial notebook execution + KernelSession abstraction (#43553)
a09aeb14e8 settings_ui: Fix font weight number field rendering (#47384)
192ea0da99 agent: Don't show inline assistant feedback buttons if setting is disabled (#47382)
da53a2b22e lsp: Skip serializing of params if unit type (#46027)
ccedb24757 agent: Clearer distinction that we are in Text Thread history (#47295)
a85142e336 ep: Don't log steps that took <500ms (#47371)
e603173a5f Bump alacritty-terminal (#47372)
73515d9ebd rust: Highlight enum variants as types (#45546)
f1e87022a7 Add missing #elifdef, #elifndef directives to C/C++ highlights (#45672)
e0212b8922 acp: Add beta label to ACP registry (#47369)
03a5f7b07f acp_thread: Rename resume to retry (#47370)
29b917449b Add `ep import-batch` to download finsihed Anthropic batches (#47364)
b082481a7a project_panel: Show Reveal in File Manager on wsl (#47288)
071b3d92e8 acp: Optimize registry refresh (#47367)
21fbceb8d4 workspace: Add close project menu item (#47365)
c35d1da245 acp: Support the session_info_update notification (#47363)
8db954a492 agent_ui: Defer spawning agents until the agent panel has been opened (#47355)
2be94ce893 fs: Use public-api-only tests (#47149)
1f4741cd5f worktree: Use public-api-only tests (#47152)
67e0cec0b1 agent_ui: Fix message editor stealing focus from other panels (#47321)
618f848c1f git: Side-by-side diff UI (#47349)
7adb97acb9 User-defined slash commands (#46815)
9ac94ce3e6 extension_rollout: Add support for renaming and deleting files (#47329)
d42fe0bba4 Fix Visual mode flip on project search  (#44284)
59738a7776 Support zooming and panning in the image viewer (#43944)
01160a75a3 extension_ci: Remove rollout exclusion list (#47315)
e24c81994a prompt_store: Fix deleted rules from the rules library being restored after restart (#47323)
54114d24f1 ep_cli: Chunk teacher prediction requests to Anthropic batch API (#47318)
63543349c0 language_models: Remove `open-ai-reponses-api` feature flag (#47317)
901fe23e25 Pop end row if the end is equal to the starting sticky row (#47314)
851ab2996e ep: Update teacher prompt to avoid undoing user edits (#47313)
049da46ed4 Bump Zed to v0.222 (#47305)
6b5c06e323 Add a streaming edit file tool (#47244)
e1e7676b5a ui: Improve performance in the `CopyButton` component (#47292)
099650fdfb ep: Check whether predictions worsen brace balance (#47301)
40fe79938b Use arithmetic expansion instead of `expr` in `./script/clear-target-dir-if-larger-than` (#47298)
8fb5587ed8 agent_ui: Alphabetically sort edited files in the panel & review view (#47300)
1b0771eb9c agent_ui: Improve UI for subagent calls (#47299)
1103f3b9d4 acp: Allow running NPM agents from registry (#47291)
69ea59d2ef agent_servers: Don't fill in unneeded fields in settings when auto-adding new agent servers (#47293)
db37fb0a7d themes: Fix One Light terminal cyan color (#45691)
5e1b7ae90c agent_ui: Fix keybinding conflict with editing queued messages (#47290)
5751aeb4a3 languages: Fix `poetry` environment discovery on Linux (#47100)
ed29b46783 project_panel: Fix "Add Folder to Project" menu hidden on remote projects (#47283)
a53017cc36 ep: Don't compute n-grams for regions that match exactly (#47281)
a608ee66d3 remote_server: Fix remote project search include/exclude filters for multiple worktrees (#47280)
b22d129fa8 acp: Filter out built-in agents from the registry (#47279)
6ab03c6c1f onboarding: Fix theme picker always updating theme based on system appearance (#47245)
7bd3075d53 open_ai: Support reasoning content (#43662)
5731066b6f ep: Fix in-place processing (#47274)
67d3804ac8 agent: Allow env overrides for extension and registry agents (#47275)
8c8639ffcc git: Fix buffer diff crash that could occur during the stage all operation (#47265)
a1914442d8 gpui: Preserve rem_size context for deferred draws (#47263)
21050e2d37 Fix nested request rate limiting deadlock for subagent edit_file (#47232)
ff35da22fb agent_ui: Fix keybinding conflict with action to clean up the queue (#47254)
aa4e94f81c docs: Replace ellipsis with proper character (#47250)
ece66be3df docs: Big reorganize  (#45276)
9c3dc216c6 Use `anyOf` instead of `oneOf` in keymap schema (#47248)
cd0b373501 ep_cli: Add filter languages subcommand (#47242)
d63067762a Fix config file watch task leak (#47246)
37185ea864 ep_cli: Fix "Too many open files" errors (#47243)
2ccca66dc1 agent_ui: Add support for editing queued messages (#47234)
4731000388 settings_ui: Remote project setting files support (#45292)
a0728db61b Add --offset flag to ep cli (#47175)
a8bf82c469 debugger: Fix crash when dragging pane items to split view (#46806)
9fce07599a ep: Make --provider optional, skip prediction when results exist (#47225)
8e48a16193 Add `ep parse-output` command (#47220)
8870bd94f6 acp: Allow installing ACP agents from the registry (#47218)
9a97c5c3db ep: Add a prompt with git-style merge markers (#47215)
d71fe4cc7f agent_ui: Add `@diagnostics` mention to the thread (#42270)
58051d6e84 languages: Fix symbol label for enum variants (#47210)
1ca52793ff rope: Fix an unintentional panic (#47019)
164e37e41d remote: Bring back docker exit status mapping (#47206)
8d555271ac remote: Fix connecting to remote with running server failing (#47203)
02a19f7a74 remote: Fix being unable to reconnect when the remote server dies (#47200)
89e9ab97aa project: Implement range formatting for remoting (#47194)
46b275b9dd rope: Add missing early return in `log_err_char_boundary` (#47191)
09ceec0a36 settings_ui: Prevent panic when trying to configure edit prediction providers for language (#47162)
4e066564ba docs: Update some AI-related content (#47178)
01028489e9 Remove the unused file (#47176)
c1da016013 agent: Patch image format bug (#45978)
3e309abe59 ep: Fix teacher prompt formatting (#47172)
6265305248 extension_host: Fix extension rebuild building in release mode (#47165)
6718320373 agent_ui: Use `.w_full()` instead of `.size_full()` when rendering markdown tables in lists (#46783)
384c8e6478 git_picker: Make the unified version of it available through the title bar (#47151)
f98acf4ca9 Make `ep split-commit` respect `--failed=skip` (#47150)
adc15943fe agent_ui: Don't fold terminal tool command lines into a disclosure (#47148)
dcfe81f8cc Support external `.editorconfig` (#46332)
ad7c30e539 ep: Missing newlines in teacher prompt (#47143)
099226e50c ai: Symlink an `AGENTS.md` file to `.rules` (#45939)
a4ebfa0e7a ui: Adjust BoxShadow offset for elevation styles (#47010)
5e2e917a8a workspace: Improve the welcome and launchpad pages UI (#47072)
36873662cf agent: Make sure ACP still gets classic tool permission UI (#47142)
6c712d88e4 terminal: Fix fast scrolling during mouse mode (#45600)
54929011b1 Add action listener to workspace for pane::CloseActiveItem (#46421)
1e2392ed94 ci: Move clippy off of run_platform_tests and into a separate job (#47139)
56b8be876f docs: Explain how git commit message prompt can be customized (#47136)
2321178f28 languages: Apply JSX fixes to JavaScript highlights (#47130)
e12dadd7da chore: Remove `contexts.scm` (#47127)
376e958569 Fix inverted char boundary check causing invalid offsets (#47112)
50a90d35b2 Add a 'rejected patch' field to example specs, for DPO examples (#47043)
e476af6417 Show language server path in language server menu (#47037)
7ce845210d ztracing: Enable memory profiling and callstack sampling (#47052)
4064af3458 Preserve and restore focus across window activation cycles (#47044)
b8dfc31fcd docs: Add documentation for `text_rendering_mode` (#46358)
1468ee2ae5 Fix more errors found when retrieving context for a huge example batch (#47039)
39e2f26671 Bump async-task (#47036)
10a536b328 editor: Fix relative line numbers breaking with nested folds (#47035)
afaccf9c67 Fix edit history clearing bug in ep (#47017)
6acca17c44 Devcontainer setup modal (#47021)
258d9223e4 Make always_allow_tool_actions override always_confirm and default_mode (#47012)
c0bfba85c5 ty: Add support for using venv-installed binary (#47029)
e05a577fd3 danger: Remove check for modifying prompts (#47028)
e5706f2349 Add BYOK GPT-5.2-codex support (#47025)
3389d84e03 Improve bracket colorization in Markdown files (#47026)
eedff99645 Format update_top_ranking_issues with latest ruff (#47023)
37715d5a50 Send trust messages only to the clients related to the current WorktreeStore (#47005)
1f97c0e10c sum_tree: Implement find functions iteratively (#47013)
d1e4ef09ee Fix not sending `file_chunks` parameter to Sweep in evals (#46999)
19be78c8d3 Fix opening schemas via path args (#47008)
43d5326075 acp: Support loading a session for external agents (#46992)
47a276328b Remove scheduler test failure status plan (#47000)
2aa8d5c747 project_panel: Split up `render_entry` into smaller methods (#46850)
73458a074c Fix context enabled race condition in edit prediction cli (#46997)
d092f3a41b text: Speed up `offset_for_anchor` and `fragment_id_for_anchor` conversions (#46989)
deb1b6e46a Fix json-language-server not considering `file_types` from project settings (#46991)
116c44426d livekit_client: Revert #46986 for macos (#46990)
59707157af agent: Move thread loading to the AgentConnection (#46985)
14628482a3 livekit_client: Do not encode video frames on the main thread (#46986)
c2f49c9e0a remote_server: Fix panic handler not logging useful info (#46975)
c6f70411ce Add slack notifications for bad bugs (#46982)
f1a8b05391 agent: End turn at message boundary when queue has pending messages (#46980)
77bf7b7942 Fix zoomed dock closing on window reactivation (#46972)
2618f2922f Don't show diff review button on folded buffers (#46968)
780a87dd98 Introduce new predict_edits/v3 endpoint (#46960)
267cf56943 agent_ui: Fix preview for images and creases in queue entries (#46964)
d183902dc7 agent_ui: Make it easier to interrupt with message in queue (#46954)
24b7e3edb5 Enable review comments in branch diff view (#46951)
4250871a1c Fewer race-conditions in Vercel redeploy (#46826)
70b0a95a22 agent_ui: Fix review buttons getting cut-off (#46948)
a92df1eee4 Remove Burn Mode code (#46950)
5c622e3e46 acp: Always pass env to builtin agents (#46946)
3ce386a118 ep: Add 180 token editable region experiment (#46945)
f319845f9a Autofix run fmt after clippy (#46935)
20163f1bc4 agent: Fix panic caused by polling ended stream (#46839)
189c9f4124 ep cli: Compute editable region during format-prompt (#46929)
4f4f69c05e Remove previous teammates from REVIEWERS.conl (#46934)
fcd80dd972 Restore run_with_clock_advancement (#46930)
edae88c223 editor: Fix add selection when skipping over soft-wrapped lines (#46911)
a2728e39a8 Split settings content into its own crate (#46845)
6fcc5e9461 Remove legacy billing code (#46927)
1ded60a660 Implement Vim's tag stack (#46002)
a10fdfd2b8 ep: Combine PredictionProvider and ZetaVersion (#46896)
3ed6c68f3b gpui: Fix utf8 slicing panic in `truncate_line` (#46914)
6e0dc03c1e Cache dynamic JSON schemas with invalidation (#46824)
3d0222a5af remote: Support opening builtin host files in remote workspaces on wsl (#46910)
adf2fec2e0 Revert "gpui: Add dynamic padding to prevent glyph clipping in text rendering on macOS (#45957)" (#46906)
e1f73a61fe languages: Support terminal auto-activation for `poetry` managed Python environment (#46900)
d139c5e6d2 ui: Dismiss context menus when window loses focus (#46866)
39569ef27b agent_ui: Improve icon for discarding interrupted edit (#46903)
78d98773d6 agent_ui: Clean UI code a bit for the tool permission controls (#46902)
cc62b53ec1 git_panel: Disable AI commit message button instead of hiding it (#46525)
b06f4088a3 Revert "collab: Bump minimum required version to collaborate from 0.204.1 to 0.220.0 to accomodate for project search RPC changes" (#46892)
9b9f25ff39 multi_buffer: Use stable storage for buffer IDs (#46884)
4a2d1e3747 Fix delay in Copilot completion (#46885)
1321aa932c workspace: Do not attempt to unnecessarily update jump list on worktree updates (#46882)
b4d1ba7a0d terminal: Fix vi mode cursor not updating on k/j navigation (#46762)
b8c5f672cd workspace: Fix tab reordering regression (#46872)
83ca31055c gpui: Add dynamic padding to prevent glyph clipping in text rendering on macOS (#45957)
95c698d31d Fix emoji on Linux when using High DPI (#46857)
ceecf82287 Allow EP synthesize command to take multiple repos (#46853)
fd877a9a3d Use LazyLock for static JSON schemas (#46823)
445c95aa3c Fix issues processing captured edit prediction examples (#46773)
9c5fc6ecbd Split token display for OpenAI (#46829)
b5b13d1f4d agent_ui: Refine the subagent UI design (#46838)
138c335d7c markdown_preview: Fix wide tables not being fully visible (#46834)
99829c7ce7 workspace: Add recent projects in the multi-project dropdown (#46828)
14e05ccb78 Allow passing custom access token to zeta2 models via env var (#46830)
cd41a21ca7 Allow running MCP servers on the remote server when using remote development (#46756)
4791e1ddc4 Granular Tool Permission Buttons (#46284)
581e559002 Render subagents in thread (#46188)
5e5951a43e workspace: Move panel telemetry to workspace level (#46809)
415f384ff3 Diff review comments: store locally and batch submit to agent (#46669)
0350e0ca87 agent_ui: Add right-click menu item for copying a given agent response's content (#46529)
5e9634c718 Batch completion events to reduce notify() calls during streaming (#46802)
993bbf596d html: Exclude all brackets from bracket colorization (#46808)
c67328ab2e languages: Separate control flow keywords for Shell Scripts (#46722)
8150d59b27 agent: Fix terminal command truncation in confirmation dialogs (#40570)
8e8e88c1ec Add sponsor button to main Zed repo (#46807)
9621fac835 agent_ui: Queue prompts by default when sending them while generating (#46797)
9e44d92a01 Bump Zed to v0.221 (#46804)
ca23fa7c7c copilot: Un-globalify copilot + handle it more directly with EditPredictionStore (#46618)
cd12d45e4a agent: Initial support for ACP session listing (#46796)

```


## Assistant

The current branch is much newer (based on more recent main). Let me now look at the actual file-level diffs for the relevant files.

**Tool Call: git --no-pager diff origin/mcp-auth...HEAD --stat -- 'crates/context_server/' 'crates/project/src/context_server_store.rs' 'crates/agent_ui/' 'crates/agent/' 'crates/assistant_text_thread/'**
Status: Completed

Terminal:
```
 crates/agent/Cargo.toml                              |     4 +
 crates/agent/src/agent.rs                            |   883 +-
 crates/agent/src/db.rs                               |   250 +-
 crates/agent/src/edit_agent.rs                       |     4 +-
 crates/agent/src/edit_agent/evals.rs                 |   130 +-
 .../agent/src/edit_agent/streaming_fuzzy_matcher.rs  |    20 +-
 crates/agent/src/legacy_thread.rs                    |     8 +-
 crates/agent/src/native_agent_server.rs              |     2 +
 crates/agent/src/outline.rs                          |     2 +-
 crates/agent/src/pattern_extraction.rs               |   354 +
 crates/agent/src/templates/system_prompt.hbs         |    27 +-
 crates/agent/src/tests/edit_file_thread_test.rs      |   204 +
 crates/agent/src/tests/mod.rs                        |  2963 +++--
 crates/agent/src/tests/test_tools.rs                 |    62 +-
 crates/agent/src/thread.rs                           |  1429 ++-
 crates/agent/src/thread_store.rs                     |    25 +-
 crates/agent/src/tool_permissions.rs                 |  2387 +++-
 crates/agent/src/tools.rs                            |    72 +-
 crates/agent/src/tools/context_server_registry.rs    |   100 +-
 crates/agent/src/tools/copy_path_tool.rs             |   391 +-
 crates/agent/src/tools/create_directory_tool.rs      |   382 +-
 crates/agent/src/tools/delete_path_tool.rs           |   453 +-
 crates/agent/src/tools/diagnostics_tool.rs           |    17 +-
 crates/agent/src/tools/edit_file_tool.rs             |  1035 +-
 crates/agent/src/tools/fetch_tool.rs                 |    29 +-
 crates/agent/src/tools/find_path_tool.rs             |    69 +-
 crates/agent/src/tools/grep_tool.rs                  |    33 +-
 crates/agent/src/tools/list_directory_tool.rs        |   573 +-
 crates/agent/src/tools/move_path_tool.rs             |   393 +-
 crates/agent/src/tools/now_tool.rs                   |     7 +-
 crates/agent/src/tools/open_tool.rs                  |    78 +-
 crates/agent/src/tools/read_file_tool.rs             |   527 +-
 .../agent/src/tools/restore_file_from_disk_tool.rs   |   348 +-
 crates/agent/src/tools/save_file_tool.rs             |   478 +-
 crates/agent/src/tools/spawn_agent_tool.rs           |   188 +
 crates/agent/src/tools/streaming_edit_file_tool.rs   |  1295 ++
 crates/agent/src/tools/subagent_tool.rs              |   551 -
 crates/agent/src/tools/terminal_tool.rs              |   182 +-
 crates/agent/src/tools/thinking_tool.rs              |    50 -
 crates/agent/src/tools/tool_permissions.rs           |   837 ++
 crates/agent/src/tools/web_search_tool.rs            |    61 +-
 crates/agent_ui/Cargo.toml                           |    12 +-
 crates/agent_ui/src/acp.rs                           |     4 +-
 crates/agent_ui/src/acp/entry_view_state.rs          |    12 +-
 crates/agent_ui/src/acp/message_editor.rs            |   723 +-
 crates/agent_ui/src/acp/mode_selector.rs             |    26 +-
 crates/agent_ui/src/acp/model_selector.rs            |    45 +-
 crates/agent_ui/src/acp/model_selector_popover.rs    |    16 +-
 crates/agent_ui/src/acp/thread_history.rs            |   748 +-
 crates/agent_ui/src/acp/thread_view.rs               | 12286 +++++++------------
 crates/agent_ui/src/acp/thread_view/active_thread.rs |  7376 +++++++++++
 crates/agent_ui/src/agent_configuration.rs           |   272 +-
 .../agent_configuration/add_llm_provider_modal.rs    |    32 +-
 .../configure_context_server_modal.rs                |    55 +-
 .../src/agent_configuration/manage_profiles_modal.rs |    30 +-
 crates/agent_ui/src/agent_diff.rs                    |   153 +-
 crates/agent_ui/src/agent_model_selector.rs          |    10 +-
 crates/agent_ui/src/agent_panel.rs                   |  1367 ++-
 crates/agent_ui/src/agent_registry_ui.rs             |   760 ++
 crates/agent_ui/src/agent_ui.rs                      |   142 +-
 crates/agent_ui/src/buffer_codegen.rs                |     8 +-
 crates/agent_ui/src/completion_provider.rs           |   503 +-
 crates/agent_ui/src/favorite_models.rs               |     2 +
 crates/agent_ui/src/inline_assistant.rs              |    27 +-
 crates/agent_ui/src/inline_prompt_editor.rs          |   155 +-
 crates/agent_ui/src/language_model_selector.rs       |    55 +-
 crates/agent_ui/src/mention_set.rs                   |   284 +-
 crates/agent_ui/src/profile_selector.rs              |    17 +-
 crates/agent_ui/src/terminal_inline_assistant.rs     |     2 +-
 crates/agent_ui/src/text_thread_editor.rs            |   281 +-
 crates/agent_ui/src/text_thread_history.rs           |     2 +-
 crates/agent_ui/src/ui.rs                            |    12 +-
 crates/agent_ui/src/ui/agent_notification.rs         |    18 +-
 crates/agent_ui/src/ui/burn_mode_tooltip.rs          |    69 -
 ...ing_modal.rs => claude_agent_onboarding_modal.rs} |    20 +-
 crates/agent_ui/src/ui/end_trial_upsell.rs           |    14 +-
 crates/agent_ui/src/ui/model_selector_components.rs  |    46 +-
 crates/agent_ui/src/ui/onboarding_modal.rs           |   174 -
 crates/agent_ui/src/ui/undo_reject_toast.rs          |    25 +
 crates/agent_ui/src/ui/usage_callout.rs              |   200 -
 crates/assistant_text_thread/Cargo.toml              |     2 +-
 .../src/assistant_text_thread.rs                     |     1 +
 .../src/assistant_text_thread_tests.rs               |     9 -
 .../src/context_server_command.rs                    |   251 +
 crates/assistant_text_thread/src/text_thread.rs      |    75 +-
 .../assistant_text_thread/src/text_thread_store.rs   |    51 +-
 crates/context_server/Cargo.toml                     |     1 +
 crates/context_server/OAUTH_HANDOVER.md              |   263 +
 crates/context_server/OAUTH_PLAN.md                  |   522 +
 crates/context_server/src/context_server.rs          |     1 +
 crates/context_server/src/oauth.rs                   |  2313 ++++
 crates/context_server/src/transport/http.rs          |   412 +-
 crates/project/src/context_server_store.rs           |  1944 ++-
 93 files changed, 34540 insertions(+), 14191 deletions(-)

```


**Tool Call: git --no-pager diff f1fd0ab529..origin/mcp-auth --stat**
Status: Completed

Terminal:
```
 Cargo.lock                                           |    9 +
 crates/agent/src/tools/context_server_registry.rs    |    7 +-
 crates/agent_ui/src/agent_configuration.rs           |  150 +-
 .../configure_context_server_modal.rs                |    2 +-
 .../assistant_text_thread/src/text_thread_store.rs   |    2 +-
 crates/context_server/Cargo.toml                     |   11 +-
 crates/context_server/src/client.rs                  |   14 +-
 crates/context_server/src/context_server.rs          |   73 +-
 crates/context_server/src/protocol.rs                |    2 +-
 crates/context_server/src/transport/http.rs          |  148 +-
 crates/context_server/src/transport/http/auth.rs     | 2124 ++++++++++++++++++++
 .../src/transport/http/www_authenticate.rs           |  325 +++
 crates/project/Cargo.toml                            |    1 +
 crates/project/src/context_server_store.rs           |  315 ++-
 crates/zed/Cargo.toml                                |    1 +
 crates/zed/src/main.rs                               |   12 +
 crates/zed/src/zed/open_listener.rs                  |   10 +
 17 files changed, 3094 insertions(+), 112 deletions(-)

```


## Assistant

Now let me look at the actual OAuth-related files in detail on both branches. Let me get the mcp-auth branch's key files.

**Tool Call: git --no-pager show origin/mcp-auth:crates/context_server/src/transport/http/auth.rs | head -200**
Status: Completed

Terminal:
```
use std::{
    error::Error,
    fmt::{self, Display},
    sync::Arc,
    time::{Duration, SystemTime},
};

use smallvec::{SmallVec, smallvec};

use anyhow::{Context as _, Result};
use base64::Engine as _;
use http_client::{AsyncBody, HttpClient, Method, Request, Response, Uri};
use rand::distr::Distribution;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::json;
use sha2::{Digest, Sha256};
use smol::io::AsyncReadExt;
use thiserror::Error;
use url::Url;

use crate::{ContextServerId, transport::http::www_authenticate::WwwAuthenticate};
use abs_uri::AbsUri;

pub struct OAuthClient {
    registration: ClientRegistration,
    server: AuthorizationServer,
    scope: Option<String>,
    state: State,
    http_client: Arc<dyn HttpClient>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
enum State {
    #[default]
    Unauthenticated,
    WaitingForCode {
        code_verifier: String,
    },
    Authenticated {
        access_token: String,
        token_type: String,
        expires_at: Option<SystemTime>,
        refresh_token: Option<String>,
    },
}

impl OAuthClient {
    pub async fn init(
        endpoint_url: &str,
        www_authenticate: Option<&WwwAuthenticate<'_>>,
        http_client: &Arc<dyn HttpClient>,
    ) -> Result<Self> {
        // https://modelcontextprotocol.io/specification/draft/basic/authorization#authorization-server-discovery
        // https://modelcontextprotocol.io/specification/draft/basic/authorization#protected-resource-metadata-discovery-requirements
        let resource =
            match www_authenticate.and_then(|challenge| challenge.resource_metadata.as_ref()) {
                Some(url) => ProtectedResource::fetch(url, http_client).await?,
                None => ProtectedResource::fetch_well_known(endpoint_url, http_client).await?,
            };

        if !resource
            .bearer_methods_supported
            .supports(BearerMethod::Header)
        {
            anyhow::bail!(InitError::UnsupportedBearerMethod);
        }

        // https://modelcontextprotocol.io/specification/draft/basic/authorization#authorization-server-metadata-discovery
        let auth_server_url = resource
            .authorization_servers
            // todo! try others?
            .first()
            .ok_or(InitError::NoAuthorizationServers)?;

        let server = AuthorizationServer::fetch(auth_server_url, http_client).await?;

        if !server.response_types_supported.supports(ResponseType::Code) {
            anyhow::bail!(InitError::UnsupportedResponseType);
        }

        if !server
            .grant_types_supported
            .supports(GrantType::AuthorizationCode)
        {
            anyhow::bail!(InitError::UnsupportedGrantType);
        }

        if !server
            .code_challenge_methods_supported
            .supports(CodeChallengeMethod::S256)
        {
            anyhow::bail!(InitError::UnsupportedCodeChallengeMethod);
        }

        // https://modelcontextprotocol.io/specification/draft/basic/authorization#client-registration-approaches
        let registration = if server.client_id_metadata_document_supported {
            ClientRegistration {
                // todo! actually host this
                client_id: "https://zed.dev/mcp/oauth-client.json".into(),
                client_secret: None,
                client_id_issued_at: None,
                client_secret_expires_at: None,
            }
        } else if let Some(registration_endpoint) = server.registration_endpoint.as_ref() {
            Self::register(registration_endpoint, http_client).await?
        } else {
            // TODO: Support custom registration
            anyhow::bail!(InitError::UnsupportedRegistration);
        };

        // https://modelcontextprotocol.io/specification/draft/basic/authorization#scope-selection-strategy
        let scope = www_authenticate
            .and_then(|challenge| challenge.scope.as_ref().map(|s| s.to_string()))
            .or_else(|| {
                if resource.scopes_supported.is_empty() {
                    None
                } else {
                    Some(resource.scopes_supported.join(" "))
                }
            });

        Ok(Self {
            registration,
            server,
            scope,
            state: State::Unauthenticated,
            http_client: http_client.clone(),
        })
    }

    pub fn authorize_url(&mut self) -> Result<AuthorizeUrl> {
        let auth_endpoint = self
            .server
            .authorization_endpoint
            .as_ref()
            .ok_or(AuthorizeUrlError::MissingAuthorizationEndpoint)?;

        let code_verifier = generate_code_verifier();
        let code_challenge =
            base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(Sha256::digest(&code_verifier));

        let mut url = Url::parse(&auth_endpoint.to_string())?;

        url.query_pairs_mut()
            .append_pair("response_type", "code")
            .append_pair("client_id", &self.registration.client_id)
            .append_pair("redirect_uri", OAuthCallback::URI)
            .append_pair("code_challenge", &code_challenge)
            .append_pair("code_challenge_method", "S256")
            .extend_pairs(self.scope.iter().map(|value| ("scope", value)));

        self.state = State::WaitingForCode { code_verifier };

        anyhow::Ok(AuthorizeUrl { url })
    }

    pub async fn exchange_token(&mut self, code: &str) -> Result<()> {
        let State::WaitingForCode { code_verifier } = &self.state else {
            return Err(ExchangeTokenError::NotWaitingForAuthorizationCode.into());
        };

        let token_endpoint = self
            .server
            .token_endpoint
            .as_ref()
            // todo! implicit?
            .ok_or(ExchangeTokenError::MissingTokenEndpoint)?;

        let form = url::form_urlencoded::Serializer::new(String::new())
            .append_pair("grant_type", "authorization_code")
            .append_pair("code", code)
            .append_pair("redirect_uri", OAuthCallback::URI)
            .append_pair("client_id", &self.registration.client_id)
            .append_pair("code_verifier", code_verifier)
            .finish();

        let request = Request::builder()
            .uri(token_endpoint.clone())
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Accept", "application/json")
            .method(Method::POST)
            .body(AsyncBody::from(form))
            .context(ExchangeTokenError::BuildTokenExchangeRequest)?;

        let requested_at = SystemTime::now();

        let mut response = self.http_client.send(request).await?;
        let token_response: TokenResponse = decode_response_json(&mut response).await?;

        self.state = State::Authenticated {
            access_token: token_response.access_token,
            token_type: token_response.token_type,
            expires_at: token_response
                .expires_in
                .map(|expires_in| requested_at + Duration::from_secs(expires_in)),
            refresh_token: token_response.refresh_token,
        };

        anyhow::Ok(())
    }

```


**Tool Call: git --no-pager show origin/mcp-auth:crates/context_server/src/transport/http/www_authenticate.rs | wc -l && echo "---" && git --no-pager show origin/mcp-auth:crates/context_server/src/transport/http/auth.rs | wc -l**
Status: Completed

Terminal:
```
325
---
2124



```


## User

Wait, first tell me how many commits (and which ones) you see on each branch please.

## Assistant



Sure. Here's what I found:

**Current branch (HEAD, detached)** — 5 commits on top of the merge base (`f1fd0ab529`):

```/dev/null/log.txt#L1-5
ecb196a9aa phase4
4eb0bd1d2d phase3
d4bfa78374 phase2
74a083dbe9 Phase 1 part 2
fa1f67a9fe Phase1 part 1
```

**`origin/mcp-auth`** — 22 commits on top of the same merge base:

```/dev/null/log.txt#L1-22
8a65bd6055 Fix _supported enum decoding
55aa53f8d0 Remove incorrect check
5ba17fe907 Use POST for exchange token
c9183f010b Use URL_SAFE_NO_PAD
d0ea20df01 Timeout required
997dfb2aea Merge branch 'main' into mcp-auth
063048a64d Merge origin/main into current branch
925b716cf3 handle more statuses in ui
d681eab746 Store www_auth_header internally
43047444e5 Store to keychain after ever auth update
cff8cb0056 Discard client on `invalid_token`
eaf7d5269b Check _supported fields first
12784ae109 Save/load oauth client state from keychain
3ebfe656ea Test init and authorize_url
a0ae2256cc Use structured errors
db42d5213d Include token in request and refresh
fee3d8089d Include access token in request when we have one
dff0a0f80b Handle callback
2c9883958c Begin authorize
808d262c3a Add explicit Accept: application/json
944634e98c Checkpoint: authorize and exchange token
28be655495 Scope selection strategy
935b16db1b Checkpoint: Sketching flow
f5f2f10e13 Test server metadata fallback
338dd62e0e Fetch authorization server metadata
f2cf745c3d Fetch ProtectedResourceMetadata
2493e42564 Add resource_metadata field
53d14b32e2 Parse WWW-Authenticate
```

Note that the `--stat` for `origin/mcp-auth` showed only 17 files changed (3,094 insertions), while the current branch's diff is much larger (93 files, 34,540 insertions) — but most of that bulk is unrelated upstream changes between the two different merge points with `main`. The actual OAuth-relevant files overlap in both branches.

## User

Good, that’s correct. Let’s continue.

## Assistant

<thinking>
Let me continue the comparison. I need to look at the key files on both branches in detail. Let me get the mcp-auth branch's full OAuth implementation and compare it with the current branch's implementation.
</thinking>

Let me now dig into the actual OAuth implementations side by side, starting with the core files.

**Tool Call: git --no-pager show origin/mcp-auth:crates/context_server/src/transport/http/auth.rs | wc -l && echo "=== www_authenticate ===" && git --no-pager show origin/mcp-auth:crates/context_server/src/transport/http/www_authenticate.rs | wc -l && echo "=== http.rs ===" && git --no-pager show origin/mcp-auth:crates/context_server/src/transport/http.rs | wc -l && echo "=== context_server.rs ===" && git --no-pager show origin/mcp-auth:crates/context_server/src/context_server.rs | wc -l && echo "=== context_server_store.rs ===" && git --no-pager show origin/mcp-auth:crates/project/src/context_server_store.rs | wc -l**
Status: Completed

Terminal:
```
2124
=== www_authenticate ===
325
=== http.rs ===
403
=== context_server.rs ===
218
=== context_server_store.rs ===
1828

```


**Tool Call: wc -l crates/context_server/src/oauth.rs crates/context_server/src/transport/http.rs crates/context_server/src/context_server.rs crates/project/src/context_server_store.rs**
Status: Completed

Terminal:
```
  2313 crates/context_server/src/oauth.rs
   649 crates/context_server/src/transport/http.rs
   162 crates/context_server/src/context_server.rs
  1465 crates/project/src/context_server_store.rs
  4589 total

```


**Tool Call: git --no-pager show origin/mcp-auth:crates/context_server/src/transport/http/auth.rs**
Status: Completed

Terminal:
```
use std::{
    error::Error,
    fmt::{self, Display},
    sync::Arc,
    time::{Duration, SystemTime},
};

use smallvec::{SmallVec, smallvec};

use anyhow::{Context as _, Result};
use base64::Engine as _;
use http_client::{AsyncBody, HttpClient, Method, Request, Response, Uri};
use rand::distr::Distribution;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::json;
use sha2::{Digest, Sha256};
use smol::io::AsyncReadExt;
use thiserror::Error;
use url::Url;

use crate::{ContextServerId, transport::http::www_authenticate::WwwAuthenticate};
use abs_uri::AbsUri;

pub struct OAuthClient {
    registration: ClientRegistration,
    server: AuthorizationServer,
    scope: Option<String>,
    state: State,
    http_client: Arc<dyn HttpClient>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
enum State {
    #[default]
    Unauthenticated,
    WaitingForCode {
        code_verifier: String,
    },
    Authenticated {
        access_token: String,
        token_type: String,
        expires_at: Option<SystemTime>,
        refresh_token: Option<String>,
    },
}

impl OAuthClient {
    pub async fn init(
        endpoint_url: &str,
        www_authenticate: Option<&WwwAuthenticate<'_>>,
        http_client: &Arc<dyn HttpClient>,
    ) -> Result<Self> {
        // https://modelcontextprotocol.io/specification/draft/basic/authorization#authorization-server-discovery
        // https://modelcontextprotocol.io/specification/draft/basic/authorization#protected-resource-metadata-discovery-requirements
        let resource =
            match www_authenticate.and_then(|challenge| challenge.resource_metadata.as_ref()) {
                Some(url) => ProtectedResource::fetch(url, http_client).await?,
                None => ProtectedResource::fetch_well_known(endpoint_url, http_client).await?,
            };

        if !resource
            .bearer_methods_supported
            .supports(BearerMethod::Header)
        {
            anyhow::bail!(InitError::UnsupportedBearerMethod);
        }

        // https://modelcontextprotocol.io/specification/draft/basic/authorization#authorization-server-metadata-discovery
        let auth_server_url = resource
            .authorization_servers
            // todo! try others?
            .first()
            .ok_or(InitError::NoAuthorizationServers)?;

        let server = AuthorizationServer::fetch(auth_server_url, http_client).await?;

        if !server.response_types_supported.supports(ResponseType::Code) {
            anyhow::bail!(InitError::UnsupportedResponseType);
        }

        if !server
            .grant_types_supported
            .supports(GrantType::AuthorizationCode)
        {
            anyhow::bail!(InitError::UnsupportedGrantType);
        }

        if !server
            .code_challenge_methods_supported
            .supports(CodeChallengeMethod::S256)
        {
            anyhow::bail!(InitError::UnsupportedCodeChallengeMethod);
        }

        // https://modelcontextprotocol.io/specification/draft/basic/authorization#client-registration-approaches
        let registration = if server.client_id_metadata_document_supported {
            ClientRegistration {
                // todo! actually host this
                client_id: "https://zed.dev/mcp/oauth-client.json".into(),
                client_secret: None,
                client_id_issued_at: None,
                client_secret_expires_at: None,
            }
        } else if let Some(registration_endpoint) = server.registration_endpoint.as_ref() {
            Self::register(registration_endpoint, http_client).await?
        } else {
            // TODO: Support custom registration
            anyhow::bail!(InitError::UnsupportedRegistration);
        };

        // https://modelcontextprotocol.io/specification/draft/basic/authorization#scope-selection-strategy
        let scope = www_authenticate
            .and_then(|challenge| challenge.scope.as_ref().map(|s| s.to_string()))
            .or_else(|| {
                if resource.scopes_supported.is_empty() {
                    None
                } else {
                    Some(resource.scopes_supported.join(" "))
                }
            });

        Ok(Self {
            registration,
            server,
            scope,
            state: State::Unauthenticated,
            http_client: http_client.clone(),
        })
    }

    pub fn authorize_url(&mut self) -> Result<AuthorizeUrl> {
        let auth_endpoint = self
            .server
            .authorization_endpoint
            .as_ref()
            .ok_or(AuthorizeUrlError::MissingAuthorizationEndpoint)?;

        let code_verifier = generate_code_verifier();
        let code_challenge =
            base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(Sha256::digest(&code_verifier));

        let mut url = Url::parse(&auth_endpoint.to_string())?;

        url.query_pairs_mut()
            .append_pair("response_type", "code")
            .append_pair("client_id", &self.registration.client_id)
            .append_pair("redirect_uri", OAuthCallback::URI)
            .append_pair("code_challenge", &code_challenge)
            .append_pair("code_challenge_method", "S256")
            .extend_pairs(self.scope.iter().map(|value| ("scope", value)));

        self.state = State::WaitingForCode { code_verifier };

        anyhow::Ok(AuthorizeUrl { url })
    }

    pub async fn exchange_token(&mut self, code: &str) -> Result<()> {
        let State::WaitingForCode { code_verifier } = &self.state else {
            return Err(ExchangeTokenError::NotWaitingForAuthorizationCode.into());
        };

        let token_endpoint = self
            .server
            .token_endpoint
            .as_ref()
            // todo! implicit?
            .ok_or(ExchangeTokenError::MissingTokenEndpoint)?;

        let form = url::form_urlencoded::Serializer::new(String::new())
            .append_pair("grant_type", "authorization_code")
            .append_pair("code", code)
            .append_pair("redirect_uri", OAuthCallback::URI)
            .append_pair("client_id", &self.registration.client_id)
            .append_pair("code_verifier", code_verifier)
            .finish();

        let request = Request::builder()
            .uri(token_endpoint.clone())
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Accept", "application/json")
            .method(Method::POST)
            .body(AsyncBody::from(form))
            .context(ExchangeTokenError::BuildTokenExchangeRequest)?;

        let requested_at = SystemTime::now();

        let mut response = self.http_client.send(request).await?;
        let token_response: TokenResponse = decode_response_json(&mut response).await?;

        self.state = State::Authenticated {
            access_token: token_response.access_token,
            token_type: token_response.token_type,
            expires_at: token_response
                .expires_in
                .map(|expires_in| requested_at + Duration::from_secs(expires_in)),
            refresh_token: token_response.refresh_token,
        };

        anyhow::Ok(())
    }

    async fn register(
        registration_endpoint: &AbsUri,
        http_client: &Arc<dyn HttpClient>,
    ) -> Result<ClientRegistration> {
        let metadata = json!({
            "redirect_uris": [OAuthCallback::URI],
            "token_endpoint_auth_method": "none",
            "grant_types": ["authorization_code", "refresh_token"],
            "response_types": ["code"],
            "client_name": "Zed",
            "client_uri": "https://zed.dev",
            "logo_uri": "https://zed.dev/_next/static/media/stable-app-logo.9b5f959f.png"
        });

        post_json(&registration_endpoint.to_string(), metadata, http_client).await
    }

    pub fn is_authenticated(&self) -> bool {
        matches!(self.state, State::Authenticated { .. })
    }

    pub async fn access_token(&mut self) -> Result<AccessToken<'_>> {
        let State::Authenticated {
            expires_at,
            refresh_token,
            ..
        } = &self.state
        else {
            return Ok(AccessToken {
                token: None,
                refreshed: false,
            });
        };

        if expires_at.is_some_and(|expires_at| expires_at <= SystemTime::now()) {
            if refresh_token.is_none() {
                return Err(AccessTokenError::AccessTokenExpiredNoRefreshToken.into());
            }

            self.refresh_access_token().await?;

            let State::Authenticated { access_token, .. } = &self.state else {
                return Ok(AccessToken {
                    token: None,
                    refreshed: false,
                });
            };

            return Ok(AccessToken {
                token: Some(access_token.as_str()),
                refreshed: true,
            });
        }

        let State::Authenticated { access_token, .. } = &self.state else {
            return Ok(AccessToken {
                token: None,
                refreshed: false,
            });
        };

        Ok(AccessToken {
            token: Some(access_token.as_str()),
            refreshed: false,
        })
    }

    async fn refresh_access_token(&mut self) -> Result<()> {
        if matches!(self.state, State::WaitingForCode { .. }) {
            return Err(RefreshTokenError::WaitingForAuthorizationCode.into());
        }

        if !self
            .server
            .grant_types_supported
            .supports(GrantType::RefreshToken)
        {
            return Err(RefreshTokenError::UnsupportedGrantType.into());
        }

        let State::Authenticated {
            refresh_token: previous_refresh_token,
            token_type: previous_token_type,
            ..
        } = std::mem::take(&mut self.state)
        else {
            return Err(RefreshTokenError::NotAuthenticated.into());
        };

        let refresh_token = previous_refresh_token
            .clone()
            .ok_or(RefreshTokenError::MissingRefreshToken)?;

        let token_endpoint = self
            .server
            .token_endpoint
            .as_ref()
            .ok_or(RefreshTokenError::MissingTokenEndpoint)?;

        let form = {
            let mut serializer = url::form_urlencoded::Serializer::new(String::new());
            serializer
                .append_pair("grant_type", "refresh_token")
                .append_pair("refresh_token", &refresh_token)
                .append_pair("client_id", &self.registration.client_id);

            if let Some(scope) = self.scope.as_ref() {
                serializer.append_pair("scope", scope);
            }

            serializer.finish()
        };

        let request = Request::builder()
            .uri(token_endpoint.clone())
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Accept", "application/json")
            .body(AsyncBody::from(form))
            .context(RefreshTokenError::BuildTokenRefreshRequest)?;

        let requested_at = SystemTime::now();

        let mut response = self.http_client.send(request).await?;
        let token_response: TokenResponse = decode_response_json(&mut response).await?;

        self.state = State::Authenticated {
            access_token: token_response.access_token,
            token_type: if token_response.token_type.is_empty() {
                previous_token_type
            } else {
                token_response.token_type
            },
            expires_at: token_response
                .expires_in
                .map(|expires_in| requested_at + Duration::from_secs(expires_in)),
            refresh_token: token_response.refresh_token.or(previous_refresh_token),
        };

        Ok(())
    }

    pub fn from_credentials(
        credentials: ContextServerCredentials,
        http_client: &Arc<dyn HttpClient>,
    ) -> Self {
        OAuthClient {
            registration: credentials.registration,
            server: credentials.server,
            scope: credentials.scope,
            state: credentials.state,
            http_client: http_client.clone(),
        }
    }

    pub fn to_credentials(&self) -> ContextServerCredentials {
        ContextServerCredentials {
            registration: self.registration.clone(),
            server: self.server.clone(),
            scope: self.scope.clone(),
            state: self.state.clone(),
        }
    }

    pub fn logout(&mut self) {
        self.state = State::Unauthenticated;
    }
}

#[derive(Debug, Error)]
pub enum InitError {
    #[error("protected resource does not support 'header' bearer method")]
    UnsupportedBearerMethod,

    #[error("resource metadata specified 0 authorization servers")]
    NoAuthorizationServers,

    #[error(
        "authorization server does not support client ID metadata or dynamic client registration"
    )]
    UnsupportedRegistration,

    #[error("authorization server does not support 'code' response type")]
    UnsupportedResponseType,

    #[error("authorization server does not support 'authorization_code' grant type")]
    UnsupportedGrantType,

    #[error("authorization server does not support 'S256' code challenge method")]
    UnsupportedCodeChallengeMethod,

    #[error("authorization server does not support 'none' token endpoint auth method")]
    UnsupportedTokenEndpointAuthMethod,
}

#[derive(Debug, Error)]
pub enum AuthorizeUrlError {
    #[error("authorization server metadata does not specify an authorization_endpoint")]
    MissingAuthorizationEndpoint,
}

#[derive(Debug, Error)]
pub enum ExchangeTokenError {
    #[error("cannot exchange token: oauth client is not waiting for an authorization code")]
    NotWaitingForAuthorizationCode,

    #[error("authorization server metadata does not specify a token_endpoint")]
    MissingTokenEndpoint,

    #[error("failed to build token exchange request")]
    BuildTokenExchangeRequest,
}

#[derive(Debug, Error)]
pub enum AccessTokenError {
    #[error("OAuth access token is expired and no refresh token is available")]
    AccessTokenExpiredNoRefreshToken,
}

#[derive(Debug, Error)]
pub enum RefreshTokenError {
    #[error("cannot refresh: OAuth client is waiting for an authorization code")]
    WaitingForAuthorizationCode,

    #[error("cannot refresh: OAuth client is not authenticated")]
    NotAuthenticated,

    #[error("cannot refresh: missing refresh token")]
    MissingRefreshToken,

    #[error("cannot refresh: authorization server metadata does not specify a token_endpoint")]
    MissingTokenEndpoint,

    #[error("failed to build token refresh request")]
    BuildTokenRefreshRequest,

    #[error("authorization server does not support 'refresh_token' grant type")]
    UnsupportedGrantType,
}

#[derive(Debug, Error)]
pub enum CallbackParseError {
    #[error("invalid oauth callback query: missing code")]
    MissingCode,

    #[error("invalid oauth callback query: missing state")]
    MissingState,

    #[error("invalid oauth callback state: missing server id")]
    MissingServerId,
}

#[derive(Debug)]
pub struct AuthorizeUrl {
    url: Url,
}

impl AuthorizeUrl {
    pub fn url(mut self, server_id: ContextServerId) -> Url {
        self.url
            .query_pairs_mut()
            .append_pair("state", &server_id.0);
        self.url
    }
}

#[derive(Debug)]
pub struct OAuthCallback {
    pub server_id: ContextServerId,
    pub code: String,
}

impl OAuthCallback {
    pub const URI: &str = "zed://mcp/oauth/callback";

    pub fn parse_query(query: &str) -> Result<Self> {
        let mut code: Option<String> = None;
        let mut state: Option<String> = None;

        for (key, value) in url::form_urlencoded::parse(query.as_bytes()) {
            match key.as_ref() {
                "code" => {
                    if !value.is_empty() {
                        code = Some(value.into_owned());
                    }
                }
                "state" => {
                    if !value.is_empty() {
                        state = Some(value.into_owned());
                    }
                }
                _ => {}
            }

            if code.is_some() && state.is_some() {
                break;
            }
        }

        let code = code.ok_or(CallbackParseError::MissingCode)?;
        let state = state.ok_or(CallbackParseError::MissingState)?;

        let state = state.trim();
        if state.is_empty() {
            return Err(CallbackParseError::MissingServerId.into());
        }

        let server_id = ContextServerId(Arc::<str>::from(state.to_string()));

        Ok(Self { server_id, code })
    }
}

fn generate_code_verifier() -> String {
    const LENGTH: usize = 64;
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-._~";

    let dist = rand::distr::slice::Choose::new(ALPHABET).unwrap();

    let bytes: Vec<u8> = dist
        .sample_iter(rand::rng())
        .take(LENGTH)
        .copied()
        .collect();

    // SAFETY: All bytes come from ALPHABET which is ASCII
    unsafe { String::from_utf8_unchecked(bytes) }
}

#[cfg_attr(test, derive(Default))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClientRegistration {
    client_id: String,
    // todo! where does this go?
    #[serde(default)]
    client_secret: Option<String>,
    #[serde(default)]
    client_id_issued_at: Option<u64>,
    #[serde(default)]
    client_secret_expires_at: Option<u64>,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: Option<u64>,
    refresh_token: Option<String>,
}

// Resource Metadata

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BearerMethod {
    Header,
    Body,
    Query,
    #[serde(other)]
    Other,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct BearerMethodsSupported(Vec<BearerMethod>);

impl BearerMethodsSupported {
    pub fn supports(&self, method: BearerMethod) -> bool {
        if self.0.is_empty() {
            return true;
        }
        self.0.contains(&method)
    }
}

#[cfg_attr(test, derive(Default, Serialize))]
#[derive(Deserialize)]
pub struct ProtectedResource {
    #[allow(dead_code)]
    resource: String,

    #[serde(default)]
    authorization_servers: Vec<AbsUri>,

    #[serde(default)]
    scopes_supported: Vec<String>,

    #[serde(default)]
    bearer_methods_supported: BearerMethodsSupported,

    #[serde(default)]
    #[allow(dead_code)]
    resource_name: Option<String>,
}

impl ProtectedResource {
    pub async fn fetch(url: &str, http_client: &Arc<dyn HttpClient>) -> Result<Self> {
        get_json(url, http_client)
            .await
            .context("Fetching resource metadata")
    }

    pub async fn fetch_well_known(
        server_endpoint: &str,
        http_client: &Arc<dyn HttpClient>,
    ) -> Result<Self> {
        let endpoint_uri = server_endpoint.parse::<Uri>()?.try_into()?;
        let well_known_uri = well_known_pre(&endpoint_uri, "oauth-protected-resource");

        return Self::fetch(&well_known_uri, http_client)
            .await
            .context("From well-known URL");
    }
}

// Server Metadata

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseType {
    Code,
    Token,
    #[serde(other)]
    Other,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResponseTypesSupported(SmallVec<[ResponseType; 4]>);

impl ResponseTypesSupported {
    pub fn supports(&self, response_type: ResponseType) -> bool {
        self.0.contains(&response_type)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GrantType {
    AuthorizationCode,
    Implicit,
    RefreshToken,
    ClientCredentials,
    #[serde(other)]
    Other,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GrantTypesSupported(SmallVec<[GrantType; 4]>);

impl Default for GrantTypesSupported {
    fn default() -> Self {
        Self(smallvec![GrantType::AuthorizationCode, GrantType::Implicit])
    }
}

impl GrantTypesSupported {
    pub fn supports(&self, grant_type: GrantType) -> bool {
        self.0.contains(&grant_type)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenEndpointAuthMethod {
    None,
    ClientSecretBasic,
    ClientSecretPost,
    #[serde(other)]
    Other,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenEndpointAuthMethodsSupported(SmallVec<[TokenEndpointAuthMethod; 4]>);

impl Default for TokenEndpointAuthMethodsSupported {
    fn default() -> Self {
        Self(smallvec![TokenEndpointAuthMethod::ClientSecretBasic])
    }
}

impl TokenEndpointAuthMethodsSupported {
    pub fn supports(&self, method: TokenEndpointAuthMethod) -> bool {
        self.0.contains(&method)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CodeChallengeMethod {
    #[serde(rename = "plain")]
    Plain,
    #[serde(rename = "S256")]
    S256,
    #[serde(other)]
    Other,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeChallengeMethodsSupported(SmallVec<[CodeChallengeMethod; 4]>);

impl CodeChallengeMethodsSupported {
    pub fn supports(&self, method: CodeChallengeMethod) -> bool {
        self.0.contains(&method)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthorizationServer {
    issuer: String,

    #[serde(default)]
    authorization_endpoint: Option<AbsUri>,

    #[serde(default)]
    token_endpoint: Option<AbsUri>,

    #[serde(default)]
    jwks_uri: Option<AbsUri>,

    #[serde(default)]
    registration_endpoint: Option<AbsUri>,

    #[serde(default)]
    scopes_supported: Vec<String>,

    #[serde(default)]
    response_types_supported: ResponseTypesSupported,

    #[serde(default)]
    grant_types_supported: GrantTypesSupported,

    #[serde(default)]
    token_endpoint_auth_methods_supported: TokenEndpointAuthMethodsSupported,

    #[serde(default)]
    code_challenge_methods_supported: CodeChallengeMethodsSupported,

    #[serde(default)]
    client_id_metadata_document_supported: bool,
}

#[cfg(test)]
impl Default for AuthorizationServer {
    fn default() -> Self {
        Self {
            issuer: String::new(),
            authorization_endpoint: None,
            token_endpoint: None,
            jwks_uri: None,
            registration_endpoint: None,
            scopes_supported: Vec::new(),
            response_types_supported: ResponseTypesSupported(smallvec![ResponseType::Code]),
            grant_types_supported: GrantTypesSupported::default(),
            token_endpoint_auth_methods_supported: TokenEndpointAuthMethodsSupported::default(),
            code_challenge_methods_supported: CodeChallengeMethodsSupported(smallvec![
                CodeChallengeMethod::S256
            ]),
            client_id_metadata_document_supported: false,
        }
    }
}

impl AuthorizationServer {
    pub async fn fetch(
        issuer_uri: &AbsUri,
        http_client: &Arc<dyn HttpClient>,
    ) -> Result<Self, AuthorizationServerMetadataDiscoveryError> {
        // We must attempt multiple well-known endpoints based on the issuer url
        //
        // https://modelcontextprotocol.io/specification/2025-11-25/basic/authorization#authorization-server-metadata-discovery
        let candidates: [fn(&AbsUri) -> Option<String>; _] = [
            // 1. OAuth 2.0 Authorization Server Metadata
            |base| well_known_pre(base, "oauth-authorization-server").into(),
            // 2. OpenID Connect Discovery 1.0 with path insertion
            |base| well_known_pre(base, "openid-configuration").into(),
            // 3. OpenID Connect Discovery 1.0 with path appening
            |base| {
                if base.path() != "/" {
                    Some(well_known_post(base, "openid-configuration"))
                } else {
                    // We already tried the root in the previous step
                    None
                }
            },
        ];

        let mut attempted_urls = Vec::new();

        for build_url in candidates {
            let Some(url) = build_url(&issuer_uri) else {
                continue;
            };

            match get_json(&url, &http_client).await {
                Ok(meta) => return Ok(meta),
                Err(err) => {
                    attempted_urls.push((url, err));
                }
            }
        }

        Err(AuthorizationServerMetadataDiscoveryError { attempted_urls })
    }
}

fn well_known_pre(base_uri: &AbsUri, well_known_segment: &str) -> String {
    format!(
        "{}://{}/.well-known/{well_known_segment}{}",
        base_uri.scheme_str(),
        base_uri.authority(),
        base_uri.path().trim_end_matches('/')
    )
}

fn well_known_post(base_uri: &AbsUri, well_known_segment: &str) -> String {
    let path = base_uri.path();
    let separator = if path.ends_with('/') { "" } else { "/" };
    format!(
        "{}://{}{}{separator}.well-known/{well_known_segment}",
        base_uri.scheme_str(),
        base_uri.authority(),
        path,
    )
}

#[derive(Debug)]
pub struct AuthorizationServerMetadataDiscoveryError {
    attempted_urls: Vec<(String, anyhow::Error)>,
}

impl Error for AuthorizationServerMetadataDiscoveryError {}

impl Display for AuthorizationServerMetadataDiscoveryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Failed to discover authorization server metadata. Attempted URLs:"
        )?;

        for (url, err) in &self.attempted_urls {
            writeln!(f, "- {url}: {err}")?;
        }

        fmt::Result::Ok(())
    }
}

async fn get_json<Out: DeserializeOwned>(
    url: &str,
    http_client: &Arc<dyn HttpClient>,
) -> Result<Out> {
    {
        let mut response = http_client.get(url, AsyncBody::empty(), true).await?;
        decode_response_json(&mut response).await
    }
    .with_context(|| format!("GET {url}"))
}

async fn post_json<In: Serialize, Out: DeserializeOwned>(
    url: &str,
    payload: In,
    http_client: &Arc<dyn HttpClient>,
) -> Result<Out> {
    {
        let mut response = http_client
            .post_json(url, serde_json::to_string(&payload)?.into())
            .await?;
        decode_response_json(&mut response).await
    }
    .with_context(|| format!("POST {url}"))
}

async fn decode_response_json<T: DeserializeOwned>(
    response: &mut Response<AsyncBody>,
) -> Result<T> {
    let mut content = Vec::new();
    response.body_mut().read_to_end(&mut content).await?;
    if response.status().is_success() {
        Ok(serde_json::from_slice(&content)?)
    } else {
        anyhow::bail!(
            "Status: {}.\nBody: {}",
            response.status(),
            String::from_utf8_lossy(&content)
        );
    }
}

// Types used externally for UI and persistance

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ContextServerAuthStatus {
    None,
    Authenticated,
    AwaitingAuthorization,
    Required,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContextServerCredentials {
    registration: ClientRegistration,
    server: AuthorizationServer,
    scope: Option<String>,
    state: State,
}

#[derive(Clone, Debug)]
pub struct ContextServerAuth {
    pub status: ContextServerAuthStatus,
    pub credentials: Option<ContextServerCredentials>,
}

impl ContextServerAuth {
    pub fn from_credentials(credentials: Option<ContextServerCredentials>) -> Self {
        let status = match &credentials {
            None => ContextServerAuthStatus::None,
            Some(creds) => match &creds.state {
                State::Unauthenticated => ContextServerAuthStatus::None,
                State::WaitingForCode { .. } => ContextServerAuthStatus::AwaitingAuthorization,
                State::Authenticated { .. } => ContextServerAuthStatus::Authenticated,
            },
        };
        Self {
            status,
            credentials,
        }
    }

    pub fn required(credentials: Option<ContextServerCredentials>) -> Self {
        Self {
            status: ContextServerAuthStatus::Required,
            credentials,
        }
    }
}

pub struct AccessToken<'a> {
    pub token: Option<&'a str>,
    pub refreshed: bool,
}

mod abs_uri {
    use std::{
        error::Error,
        fmt::{self, Display},
        ops::Deref,
    };

    use http_client::{Uri, http::uri::Authority};
    use serde::Deserialize;

    #[derive(Debug, Clone)]
    pub struct AbsUri(Uri);

    impl AbsUri {
        pub fn authority(&self) -> &Authority {
            self.0.authority().unwrap()
        }

        pub fn scheme_str(&self) -> &str {
            self.0.scheme_str().unwrap()
        }
    }

    impl Into<Uri> for AbsUri {
        fn into(self) -> Uri {
            self.0
        }
    }

    impl TryFrom<Uri> for AbsUri {
        type Error = AbsUriError;

        fn try_from(uri: Uri) -> Result<Self, Self::Error> {
            if uri.scheme().is_none() {
                return Err(AbsUriError::MissingScheme);
            }
            if uri.authority().is_none() {
                return Err(AbsUriError::MissingAuthority);
            }
            Ok(Self(uri))
        }
    }

    impl Deref for AbsUri {
        type Target = Uri;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<'de> Deserialize<'de> for AbsUri {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse::<Uri>()
                .map_err(serde::de::Error::custom)?
                .try_into()
                .map_err(|e| serde::de::Error::custom(format!("{e:?}")))
        }
    }

    impl serde::Serialize for AbsUri {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_str(&self.0.to_string())
        }
    }

    #[derive(Debug)]
    pub enum AbsUriError {
        MissingScheme,
        MissingAuthority,
    }

    impl Error for AbsUriError {}

    impl Display for AbsUriError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                AbsUriError::MissingScheme => write!(f, "URI is not absolute: Missing scheme"),
                AbsUriError::MissingAuthority => {
                    write!(f, "URI is not absolute: Missing authority")
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    use futures::StreamExt;
    use futures::channel::{mpsc, oneshot};
    use gpui::{TestAppContext, prelude::*};
    use http_client::{FakeHttpClient, Request, Response};
    use pretty_assertions::{assert_eq, assert_matches};

    #[gpui::test]
    async fn init_resource_metadata_url_and_url_client_id(cx: &mut TestAppContext) {
        let (http_client, mut requests) = fake_client();

        let www_authenticate = WwwAuthenticate {
            realm: None,
            scope: None,
            error: None,
            error_description: None,
            error_uri: None,
            resource_metadata: Some("https://resource.example.com/meta.json".into()),
        };

        let init_task = cx.background_spawn(async move {
            OAuthClient::init(
                "https://mcp.example.com",
                Some(&www_authenticate),
                &http_client,
            )
            .await
        });

        let request = requests.next().await.expect("Expected request");
        assert_eq!(request.uri, "https://resource.example.com/meta.json");
        respond_json(
            request,
            200,
            &ProtectedResource {
                resource: "https://mcp.example.com".to_string(),
                authorization_servers: vec![
                    "https://auth.example.com"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ],
                scopes_supported: vec!["mcp:read".to_string(), "mcp:write".to_string()],
                ..Default::default()
            },
        );

        let request = requests.next().await.expect("Expected request");
        assert_eq!(
            request.uri,
            "https://auth.example.com/.well-known/oauth-authorization-server"
        );
        respond_json(
            request,
            200,
            &AuthorizationServer {
                issuer: "https://auth.example.com".to_string(),
                authorization_endpoint: Some(
                    "https://auth.example.com/authorize"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                token_endpoint: Some(
                    "https://auth.example.com/token"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                registration_endpoint: Some(
                    "https://auth.example.com/register"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                client_id_metadata_document_supported: true,
                ..Default::default()
            },
        );

        let client = init_task.await.expect("init should succeed");
        assert_eq!(
            client.registration.client_id,
            "https://zed.dev/mcp/oauth-client.json"
        );
        assert_eq!(client.scope.as_deref(), Some("mcp:read mcp:write"));

        drop(client);

        let unexpected_request = requests.next().await;
        assert!(
            unexpected_request.is_none(),
            "did not expect dynamic registration request when client_id_metadata_document_supported is true"
        );
    }

    #[gpui::test]
    async fn init_well_known_resource_metadata(cx: &mut TestAppContext) {
        let (client, mut requests) = fake_client();
        let http_client = client.clone();

        let init_task = cx.background_spawn(async move {
            OAuthClient::init("https://mcp.example.com/rpc", None, &http_client).await
        });

        let request = requests.next().await.expect("Expected request");
        assert_eq!(
            request.uri,
            "https://mcp.example.com/.well-known/oauth-protected-resource/rpc"
        );
        respond_json(
            request,
            200,
            &ProtectedResource {
                resource: "https://mcp.example.com/rpc".to_string(),
                authorization_servers: vec![
                    "https://auth.example.com"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ],
                ..Default::default()
            },
        );

        let request = requests.next().await.expect("Expected request");
        assert_eq!(
            request.uri,
            "https://auth.example.com/.well-known/oauth-authorization-server"
        );
        respond_json(
            request,
            200,
            &AuthorizationServer {
                issuer: "https://auth.example.com".to_string(),
                authorization_endpoint: Some(
                    "https://auth.example.com/authorize"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                token_endpoint: Some(
                    "https://auth.example.com/token"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                client_id_metadata_document_supported: true,
                ..Default::default()
            },
        );

        let client = init_task.await.expect("init should succeed");
        assert_eq!(
            client.scope, None,
            "no scopes_supported and no challenge scope"
        );
    }

    #[gpui::test]
    async fn init_errors_when_unsupported_bearer_method(cx: &mut TestAppContext) {
        let (client, mut requests) = fake_client();
        let http_client = client.clone();

        let init_task = cx.background_spawn(async move {
            OAuthClient::init("https://mcp.example.com", None, &http_client).await
        });

        let request = requests.next().await.expect("Expected request");
        respond_json(
            request,
            200,
            &ProtectedResource {
                resource: "https://mcp.example.com".to_string(),
                authorization_servers: vec![
                    "https://auth.example.com"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ],
                bearer_methods_supported: BearerMethodsSupported(vec![BearerMethod::Body]),
                ..Default::default()
            },
        );

        assert_matches!(
            init_task.await.err().unwrap().downcast::<InitError>(),
            Ok(InitError::UnsupportedBearerMethod)
        );
    }

    #[gpui::test]
    async fn init_errors_when_no_authorization_servers(cx: &mut TestAppContext) {
        let (client, mut requests) = fake_client();
        let http_client = client.clone();

        let init_task = cx.background_spawn(async move {
            OAuthClient::init("https://mcp.example.com", None, &http_client).await
        });

        let request = requests.next().await.expect("Expected request");
        assert_eq!(
            request.uri,
            "https://mcp.example.com/.well-known/oauth-protected-resource"
        );
        respond_json(
            request,
            200,
            &ProtectedResource {
                resource: "https://mcp.example.com".to_string(),
                ..Default::default()
            },
        );

        assert_matches!(
            init_task.await.err().unwrap().downcast::<InitError>(),
            Ok(InitError::NoAuthorizationServers)
        );
    }

    #[gpui::test]
    async fn init_uses_dynamic_registration_when_supported(cx: &mut TestAppContext) {
        let (client, mut requests) = fake_client();
        let http_client = client.clone();

        let init_task = cx.background_spawn(async move {
            OAuthClient::init("https://mcp.example.com", None, &http_client).await
        });

        let request = requests.next().await.expect("Expected request");
        assert_eq!(
            request.uri,
            "https://mcp.example.com/.well-known/oauth-protected-resource"
        );
        respond_json(
            request,
            200,
            &ProtectedResource {
                resource: "https://mcp.example.com".to_string(),
                authorization_servers: vec![
                    "https://auth.example.com"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ],
                ..Default::default()
            },
        );

        let request = requests.next().await.expect("Expected request");
        assert_eq!(
            request.uri,
            "https://auth.example.com/.well-known/oauth-authorization-server"
        );
        respond_json(
            request,
            200,
            &AuthorizationServer {
                issuer: "https://auth.example.com".to_string(),
                authorization_endpoint: Some(
                    "https://auth.example.com/authorize"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                token_endpoint: Some(
                    "https://auth.example.com/token"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                registration_endpoint: Some(
                    "https://auth.example.com/register"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                token_endpoint_auth_methods_supported: TokenEndpointAuthMethodsSupported(
                    smallvec![TokenEndpointAuthMethod::None],
                ),
                client_id_metadata_document_supported: false,
                ..Default::default()
            },
        );

        let request = requests.next().await.expect("Expected request");
        assert_eq!(request.uri, "https://auth.example.com/register");
        respond_json(
            request,
            200,
            &ClientRegistration {
                client_id: "client-id-123".to_string(),
                ..Default::default()
            },
        );

        let client = init_task.await.expect("init should succeed");
        assert_eq!(client.registration.client_id, "client-id-123");
    }

    #[gpui::test]
    async fn init_errors_when_no_registration_supported(cx: &mut TestAppContext) {
        let (client, mut requests) = fake_client();
        let http_client = client.clone();

        let init_task = cx.background_spawn(async move {
            OAuthClient::init("https://mcp.example.com", None, &http_client).await
        });

        // Resource metadata via well-known
        let request = requests.next().await.expect("Expected request");
        assert_eq!(
            request.uri,
            "https://mcp.example.com/.well-known/oauth-protected-resource"
        );
        respond_json(
            request,
            200,
            &ProtectedResource {
                resource: "https://mcp.example.com".to_string(),
                authorization_servers: vec![
                    "https://auth.example.com"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ],
                ..Default::default()
            },
        );

        // Server metadata: neither client_id_metadata_document_supported nor registration_endpoint
        let request = requests.next().await.expect("Expected request");
        assert_eq!(
            request.uri,
            "https://auth.example.com/.well-known/oauth-authorization-server"
        );
        respond_json(
            request,
            200,
            &AuthorizationServer {
                issuer: "https://auth.example.com".to_string(),
                authorization_endpoint: Some(
                    "https://auth.example.com/authorize"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                token_endpoint: Some(
                    "https://auth.example.com/token"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                client_id_metadata_document_supported: false,
                ..Default::default()
            },
        );

        assert_matches!(
            init_task.await.err().unwrap().downcast::<InitError>(),
            Ok(InitError::UnsupportedRegistration)
        );
    }

    #[gpui::test]
    async fn init_errors_when_unsupported_response_type(cx: &mut TestAppContext) {
        let (client, mut requests) = fake_client();
        let http_client = client.clone();

        let init_task = cx.background_spawn(async move {
            OAuthClient::init("https://mcp.example.com", None, &http_client).await
        });

        let request = requests.next().await.expect("Expected request");
        respond_json(
            request,
            200,
            &ProtectedResource {
                resource: "https://mcp.example.com".to_string(),
                authorization_servers: vec![
                    "https://auth.example.com"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ],
                ..Default::default()
            },
        );

        let request = requests.next().await.expect("Expected request");
        respond_json(
            request,
            200,
            &AuthorizationServer {
                issuer: "https://auth.example.com".to_string(),
                response_types_supported: ResponseTypesSupported(smallvec![ResponseType::Token]),
                client_id_metadata_document_supported: true,
                ..Default::default()
            },
        );

        assert_matches!(
            init_task.await.err().unwrap().downcast::<InitError>(),
            Ok(InitError::UnsupportedResponseType)
        );
    }

    #[gpui::test]
    async fn init_errors_when_unsupported_grant_type(cx: &mut TestAppContext) {
        let (client, mut requests) = fake_client();
        let http_client = client.clone();

        let init_task = cx.background_spawn(async move {
            OAuthClient::init("https://mcp.example.com", None, &http_client).await
        });

        let request = requests.next().await.expect("Expected request");
        respond_json(
            request,
            200,
            &ProtectedResource {
                resource: "https://mcp.example.com".to_string(),
                authorization_servers: vec![
                    "https://auth.example.com"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ],
                ..Default::default()
            },
        );

        let request = requests.next().await.expect("Expected request");
        respond_json(
            request,
            200,
            &AuthorizationServer {
                issuer: "https://auth.example.com".to_string(),
                response_types_supported: ResponseTypesSupported(smallvec![ResponseType::Code]),
                grant_types_supported: GrantTypesSupported(smallvec![GrantType::Implicit]),
                client_id_metadata_document_supported: true,
                ..Default::default()
            },
        );

        assert_matches!(
            init_task.await.err().unwrap().downcast::<InitError>(),
            Ok(InitError::UnsupportedGrantType)
        );
    }

    #[gpui::test]
    async fn init_errors_when_unsupported_code_challenge_method(cx: &mut TestAppContext) {
        let (client, mut requests) = fake_client();
        let http_client = client.clone();

        let init_task = cx.background_spawn(async move {
            OAuthClient::init("https://mcp.example.com", None, &http_client).await
        });

        let request = requests.next().await.expect("Expected request");
        respond_json(
            request,
            200,
            &ProtectedResource {
                resource: "https://mcp.example.com".to_string(),
                authorization_servers: vec![
                    "https://auth.example.com"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ],
                ..Default::default()
            },
        );

        let request = requests.next().await.expect("Expected request");
        respond_json(
            request,
            200,
            &AuthorizationServer {
                issuer: "https://auth.example.com".to_string(),
                response_types_supported: ResponseTypesSupported(smallvec![ResponseType::Code]),
                code_challenge_methods_supported: CodeChallengeMethodsSupported(smallvec![
                    CodeChallengeMethod::Plain
                ]),
                client_id_metadata_document_supported: true,
                ..Default::default()
            },
        );

        assert_matches!(
            init_task.await.err().unwrap().downcast::<InitError>(),
            Ok(InitError::UnsupportedCodeChallengeMethod)
        );
    }

    #[gpui::test]
    async fn init_errors_when_unsupported_token_endpoint_auth_method(cx: &mut TestAppContext) {
        let (client, mut requests) = fake_client();
        let http_client = client.clone();

        let init_task = cx.background_spawn(async move {
            OAuthClient::init("https://mcp.example.com", None, &http_client).await
        });

        let request = requests.next().await.expect("Expected request");
        respond_json(
            request,
            200,
            &ProtectedResource {
                resource: "https://mcp.example.com".to_string(),
                authorization_servers: vec![
                    "https://auth.example.com"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ],
                ..Default::default()
            },
        );

        let request = requests.next().await.expect("Expected request");
        respond_json(
            request,
            200,
            &AuthorizationServer {
                issuer: "https://auth.example.com".to_string(),
                response_types_supported: ResponseTypesSupported(smallvec![ResponseType::Code]),
                token_endpoint_auth_methods_supported: TokenEndpointAuthMethodsSupported(
                    smallvec![TokenEndpointAuthMethod::ClientSecretBasic],
                ),
                registration_endpoint: Some(
                    "https://auth.example.com/register"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                client_id_metadata_document_supported: false,
                ..Default::default()
            },
        );

        assert_matches!(
            init_task.await.err().unwrap().downcast::<InitError>(),
            Ok(InitError::UnsupportedTokenEndpointAuthMethod)
        );
    }

    #[gpui::test]
    async fn init_prefers_challenge_scope_over_resource_scopes(cx: &mut TestAppContext) {
        let (client, mut requests) = fake_client();
        let http_client = client.clone();

        let www_authenticate = WwwAuthenticate {
            realm: None,
            scope: Some("from-challenge".into()),
            error: None,
            error_description: None,
            error_uri: None,
            resource_metadata: None,
        };

        let init_task = cx.background_spawn(async move {
            OAuthClient::init(
                "https://mcp.example.com",
                Some(&www_authenticate),
                &http_client,
            )
            .await
        });

        let request = requests.next().await.expect("Expected request");
        assert_eq!(
            request.uri,
            "https://mcp.example.com/.well-known/oauth-protected-resource"
        );
        respond_json(
            request,
            200,
            &ProtectedResource {
                resource: "https://mcp.example.com".to_string(),
                authorization_servers: vec![
                    "https://auth.example.com"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ],
                scopes_supported: vec!["from-resource".to_string()],
                ..Default::default()
            },
        );

        let request = requests.next().await.expect("Expected request");
        assert_eq!(
            request.uri,
            "https://auth.example.com/.well-known/oauth-authorization-server"
        );
        respond_json(
            request,
            200,
            &AuthorizationServer {
                issuer: "https://auth.example.com".to_string(),
                authorization_endpoint: Some(
                    "https://auth.example.com/authorize"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                token_endpoint: Some(
                    "https://auth.example.com/token"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                client_id_metadata_document_supported: true,
                ..Default::default()
            },
        );

        let client = init_task.await.expect("init should succeed");
        assert_eq!(client.scope.as_deref(), Some("from-challenge"));
    }

    #[gpui::test]
    async fn init_uses_resource_scopes_when_challenge_scope_missing(cx: &mut TestAppContext) {
        let (client, mut requests) = fake_client();
        let http_client = client.clone();

        let www_authenticate = WwwAuthenticate {
            realm: None,
            scope: None,
            error: None,
            error_description: None,
            error_uri: None,
            resource_metadata: None,
        };

        let init_task = cx.background_spawn(async move {
            OAuthClient::init(
                "https://mcp.example.com",
                Some(&www_authenticate),
                &http_client,
            )
            .await
        });

        let request = requests.next().await.expect("Expected request");
        assert_eq!(
            request.uri,
            "https://mcp.example.com/.well-known/oauth-protected-resource"
        );
        respond_json(
            request,
            200,
            &ProtectedResource {
                resource: "https://mcp.example.com".to_string(),
                authorization_servers: vec![
                    "https://auth.example.com"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ],
                scopes_supported: vec![
                    "from-resource".to_string(),
                    "also-from-resource".to_string(),
                ],
                ..Default::default()
            },
        );

        let request = requests.next().await.expect("Expected request");
        assert_eq!(
            request.uri,
            "https://auth.example.com/.well-known/oauth-authorization-server"
        );
        respond_json(
            request,
            200,
            &AuthorizationServer {
                issuer: "https://auth.example.com".to_string(),
                authorization_endpoint: Some(
                    "https://auth.example.com/authorize"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                token_endpoint: Some(
                    "https://auth.example.com/token"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                client_id_metadata_document_supported: true,
                ..Default::default()
            },
        );

        let client = init_task.await.expect("init should succeed");
        assert_eq!(
            client.scope.as_deref(),
            Some("from-resource also-from-resource")
        );
    }

    #[gpui::test]
    async fn fetch_server_metadata_chain(cx: &mut TestAppContext) {
        expect_fallback_chain(
            "https://auth.example.com/tenant/123",
            &[
                "https://auth.example.com/.well-known/oauth-authorization-server/tenant/123",
                "https://auth.example.com/.well-known/openid-configuration/tenant/123",
                "https://auth.example.com/tenant/123/.well-known/openid-configuration",
            ],
            cx,
        )
        .await;

        expect_fallback_chain(
            "https://auth.example.com/tenant/123/",
            &[
                "https://auth.example.com/.well-known/oauth-authorization-server/tenant/123",
                "https://auth.example.com/.well-known/openid-configuration/tenant/123",
                "https://auth.example.com/tenant/123/.well-known/openid-configuration",
            ],
            cx,
        )
        .await;

        expect_fallback_chain(
            "https://auth.example.com",
            &[
                "https://auth.example.com/.well-known/oauth-authorization-server",
                "https://auth.example.com/.well-known/openid-configuration",
            ],
            cx,
        )
        .await;
    }

    async fn expect_fallback_chain(issuer_uri: &str, urls: &[&str], cx: &mut TestAppContext) {
        let issuer_uri: AbsUri = issuer_uri.parse::<Uri>().unwrap().try_into().unwrap();
        let (client, mut request_rx) = fake_client();

        for i in 0..urls.len() {
            let issuer_uri = issuer_uri.clone();
            let client = client.clone();
            let fetch_task = cx.background_spawn(async move {
                AuthorizationServer::fetch(&issuer_uri, &client).await
            });

            for request_url in &urls[..i] {
                let request = request_rx.next().await.unwrap();
                assert_eq!(request.uri, *request_url);
                respond(request, not_found());
            }

            let request = request_rx.next().await.unwrap();
            assert_eq!(request.uri, *urls[i]);
            respond(
                request,
                Response::builder()
                    .status(200)
                    .header("Content-Type", "application/json")
                    .body(AsyncBody::from(valid_metadata_json(
                        "https://auth.example.com",
                    )))
                    .unwrap(),
            );

            let metadata = fetch_task.await.expect("fetch should succeed");
            assert_eq!(metadata.issuer, "https://auth.example.com");
        }
    }

    #[gpui::test]
    async fn fetch_server_metadata_openid_root_stops_on_fail(cx: &mut TestAppContext) {
        let (client, mut requests) = fake_client();
        let http_client = client.clone();

        let fetch_task = cx.background_spawn(async move {
            let issuer_uri: AbsUri = "https://auth.example.com"
                .parse::<Uri>()
                .unwrap()
                .try_into()
                .unwrap();

            AuthorizationServer::fetch(&issuer_uri, &http_client).await
        });

        let request = requests.next().await.expect("Expected first request");
        assert_eq!(
            request.uri,
            "https://auth.example.com/.well-known/oauth-authorization-server"
        );
        respond(request, not_found());

        let request = requests.next().await.expect("Expected second request");
        assert_eq!(
            request.uri,
            "https://auth.example.com/.well-known/openid-configuration"
        );
        respond(request, not_found());

        // should not attempt well_known_post since it'd be the same as well_known_pre
        let error = fetch_task.await.expect_err("fetch should fail");
        assert_eq!(error.attempted_urls.len(), 2);
    }

    #[gpui::test]
    async fn fetch_server_metadata_all_fail(cx: &mut TestAppContext) {
        let (client, mut requests) = fake_client();
        let http_client = client.clone();

        let fetch_task = cx.background_spawn(async move {
            let issuer_uri: AbsUri = "https://auth.example.com/tenant/123"
                .parse::<Uri>()
                .unwrap()
                .try_into()
                .unwrap();

            AuthorizationServer::fetch(&issuer_uri, &http_client).await
        });

        for _ in 0..3 {
            let request = requests.next().await.expect("Expected request");
            respond(request, not_found());
        }

        let error = fetch_task.await.expect_err("fetch should fail");
        assert_eq!(error.attempted_urls.len(), 3);
    }

    #[gpui::test]
    async fn authorize_url_includes_required_oauth_params(cx: &mut TestAppContext) {
        let mut client = init_oauth_client_for_authorize_url(None, cx).await;

        let url = client
            .authorize_url()
            .expect("authorize_url should succeed");
        let url = url.url;

        assert_eq!(
            url.as_str().split('?').next().unwrap(),
            "https://auth.example.com/authorize"
        );

        let query_pairs: std::collections::HashMap<String, String> = url
            .query_pairs()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect();

        assert_eq!(
            query_pairs.get("response_type").map(String::as_str),
            Some("code")
        );
        assert_eq!(
            query_pairs.get("client_id").map(String::as_str),
            Some("https://zed.dev/mcp/oauth-client.json")
        );
        assert_eq!(
            query_pairs.get("redirect_uri").map(String::as_str),
            Some(OAuthCallback::URI)
        );
        assert_eq!(
            query_pairs.get("code_challenge_method").map(String::as_str),
            Some("S256")
        );

        let code_challenge = query_pairs
            .get("code_challenge")
            .map(String::as_str)
            .expect("code_challenge should be present");
        assert!(
            !code_challenge.is_empty(),
            "code_challenge should be non-empty"
        );
        assert!(
            code_challenge
                .chars()
                .all(|ch| { ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' || ch == '=' }),
            "code_challenge should be base64url-ish"
        );

        assert!(
            !query_pairs.contains_key("scope"),
            "scope should be absent when no scope is configured"
        );
    }

    #[gpui::test]
    async fn authorize_url_includes_scope_when_present(cx: &mut TestAppContext) {
        let mut client = init_oauth_client_for_authorize_url(Some("mcp:read mcp:write"), cx).await;

        let url = client
            .authorize_url()
            .expect("authorize_url should succeed");
        let url = url.url;

        let scopes: Vec<String> = url
            .query_pairs()
            .filter_map(|(key, value)| (key == "scope").then_some(value.to_string()))
            .collect();

        assert_eq!(scopes.as_slice(), &["mcp:read mcp:write".to_string()]);
    }

    #[gpui::test]
    async fn authorize_url_errors_when_missing_authorization_endpoint(cx: &mut TestAppContext) {
        let mut client = init_oauth_client_for_authorize_url(None, cx).await;
        client.server.authorization_endpoint = None;

        assert_matches!(
            client
                .authorize_url()
                .err()
                .unwrap()
                .downcast::<AuthorizeUrlError>(),
            Ok(AuthorizeUrlError::MissingAuthorizationEndpoint)
        );
    }

    struct FakeRequest {
        uri: String,
        respond: oneshot::Sender<Response<AsyncBody>>,
    }

    fn fake_client() -> (
        Arc<http_client::HttpClientWithUrl>,
        mpsc::UnboundedReceiver<FakeRequest>,
    ) {
        let (request_sender, request_receiver) = mpsc::unbounded::<FakeRequest>();

        let client = FakeHttpClient::create(move |req: Request<AsyncBody>| {
            let request_sender = request_sender.clone();
            async move {
                let (respond, response_receiver) = oneshot::channel();
                request_sender
                    .unbounded_send(FakeRequest {
                        uri: req.uri().to_string(),
                        respond,
                    })
                    .expect("Test receiver dropped");

                response_receiver
                    .await
                    .map_err(|_| anyhow::anyhow!("Test dropped response sender"))
            }
        });

        (client, request_receiver)
    }

    async fn init_oauth_client_for_authorize_url(
        scope: Option<&'static str>,
        cx: &mut TestAppContext,
    ) -> OAuthClient {
        let (http_client, mut requests) = fake_client();

        let www_authenticate = WwwAuthenticate {
            realm: None,
            scope: scope.map(Into::into),
            error: None,
            error_description: None,
            error_uri: None,
            resource_metadata: None,
        };

        let init_task = cx.background_spawn(async move {
            OAuthClient::init(
                "https://mcp.example.com",
                Some(&www_authenticate),
                &http_client,
            )
            .await
        });

        let request = requests.next().await.expect("Expected request");
        assert_eq!(
            request.uri,
            "https://mcp.example.com/.well-known/oauth-protected-resource"
        );
        respond_json(
            request,
            200,
            &ProtectedResource {
                resource: "https://mcp.example.com".to_string(),
                authorization_servers: vec![
                    "https://auth.example.com"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ],
                ..Default::default()
            },
        );

        let request = requests.next().await.expect("Expected request");
        assert_eq!(
            request.uri,
            "https://auth.example.com/.well-known/oauth-authorization-server"
        );
        respond_json(
            request,
            200,
            &AuthorizationServer {
                issuer: "https://auth.example.com".to_string(),
                authorization_endpoint: Some(
                    "https://auth.example.com/authorize"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                token_endpoint: Some(
                    "https://auth.example.com/token"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                client_id_metadata_document_supported: true,
                ..Default::default()
            },
        );

        init_task.await.expect("init should succeed")
    }

    fn not_found() -> Response<AsyncBody> {
        Response::builder()
            .status(404)
            .body(AsyncBody::from("Not found".to_string()))
            .unwrap()
    }

    fn valid_metadata_json(issuer: &str) -> String {
        serde_json::json!({
            "issuer": issuer,
            "authorization_endpoint": format!("{}/authorize", issuer),
            "token_endpoint": format!("{}/token", issuer),
        })
        .to_string()
    }

    fn respond(request: FakeRequest, response: Response<AsyncBody>) {
        request.respond.send(response).ok();
    }

    fn respond_json<T: Serialize>(request: FakeRequest, status: u16, value: &T) {
        let body = serde_json::to_string(value).expect("serialize test json");
        respond(
            request,
            Response::builder()
                .status(status)
                .header("Content-Type", "application/json")
                .body(AsyncBody::from(body))
                .unwrap(),
        );
    }
}

```


**Tool Call: git --no-pager show origin/mcp-auth:crates/context_server/src/transport/http/auth.rs | tail -n +450**
Status: Completed

Terminal:
```
    MissingServerId,
}

#[derive(Debug)]
pub struct AuthorizeUrl {
    url: Url,
}

impl AuthorizeUrl {
    pub fn url(mut self, server_id: ContextServerId) -> Url {
        self.url
            .query_pairs_mut()
            .append_pair("state", &server_id.0);
        self.url
    }
}

#[derive(Debug)]
pub struct OAuthCallback {
    pub server_id: ContextServerId,
    pub code: String,
}

impl OAuthCallback {
    pub const URI: &str = "zed://mcp/oauth/callback";

    pub fn parse_query(query: &str) -> Result<Self> {
        let mut code: Option<String> = None;
        let mut state: Option<String> = None;

        for (key, value) in url::form_urlencoded::parse(query.as_bytes()) {
            match key.as_ref() {
                "code" => {
                    if !value.is_empty() {
                        code = Some(value.into_owned());
                    }
                }
                "state" => {
                    if !value.is_empty() {
                        state = Some(value.into_owned());
                    }
                }
                _ => {}
            }

            if code.is_some() && state.is_some() {
                break;
            }
        }

        let code = code.ok_or(CallbackParseError::MissingCode)?;
        let state = state.ok_or(CallbackParseError::MissingState)?;

        let state = state.trim();
        if state.is_empty() {
            return Err(CallbackParseError::MissingServerId.into());
        }

        let server_id = ContextServerId(Arc::<str>::from(state.to_string()));

        Ok(Self { server_id, code })
    }
}

fn generate_code_verifier() -> String {
    const LENGTH: usize = 64;
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-._~";

    let dist = rand::distr::slice::Choose::new(ALPHABET).unwrap();

    let bytes: Vec<u8> = dist
        .sample_iter(rand::rng())
        .take(LENGTH)
        .copied()
        .collect();

    // SAFETY: All bytes come from ALPHABET which is ASCII
    unsafe { String::from_utf8_unchecked(bytes) }
}

#[cfg_attr(test, derive(Default))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClientRegistration {
    client_id: String,
    // todo! where does this go?
    #[serde(default)]
    client_secret: Option<String>,
    #[serde(default)]
    client_id_issued_at: Option<u64>,
    #[serde(default)]
    client_secret_expires_at: Option<u64>,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: Option<u64>,
    refresh_token: Option<String>,
}

// Resource Metadata

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BearerMethod {
    Header,
    Body,
    Query,
    #[serde(other)]
    Other,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct BearerMethodsSupported(Vec<BearerMethod>);

impl BearerMethodsSupported {
    pub fn supports(&self, method: BearerMethod) -> bool {
        if self.0.is_empty() {
            return true;
        }
        self.0.contains(&method)
    }
}

#[cfg_attr(test, derive(Default, Serialize))]
#[derive(Deserialize)]
pub struct ProtectedResource {
    #[allow(dead_code)]
    resource: String,

    #[serde(default)]
    authorization_servers: Vec<AbsUri>,

    #[serde(default)]
    scopes_supported: Vec<String>,

    #[serde(default)]
    bearer_methods_supported: BearerMethodsSupported,

    #[serde(default)]
    #[allow(dead_code)]
    resource_name: Option<String>,
}

impl ProtectedResource {
    pub async fn fetch(url: &str, http_client: &Arc<dyn HttpClient>) -> Result<Self> {
        get_json(url, http_client)
            .await
            .context("Fetching resource metadata")
    }

    pub async fn fetch_well_known(
        server_endpoint: &str,
        http_client: &Arc<dyn HttpClient>,
    ) -> Result<Self> {
        let endpoint_uri = server_endpoint.parse::<Uri>()?.try_into()?;
        let well_known_uri = well_known_pre(&endpoint_uri, "oauth-protected-resource");

        return Self::fetch(&well_known_uri, http_client)
            .await
            .context("From well-known URL");
    }
}

// Server Metadata

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseType {
    Code,
    Token,
    #[serde(other)]
    Other,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResponseTypesSupported(SmallVec<[ResponseType; 4]>);

impl ResponseTypesSupported {
    pub fn supports(&self, response_type: ResponseType) -> bool {
        self.0.contains(&response_type)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GrantType {
    AuthorizationCode,
    Implicit,
    RefreshToken,
    ClientCredentials,
    #[serde(other)]
    Other,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GrantTypesSupported(SmallVec<[GrantType; 4]>);

impl Default for GrantTypesSupported {
    fn default() -> Self {
        Self(smallvec![GrantType::AuthorizationCode, GrantType::Implicit])
    }
}

impl GrantTypesSupported {
    pub fn supports(&self, grant_type: GrantType) -> bool {
        self.0.contains(&grant_type)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenEndpointAuthMethod {
    None,
    ClientSecretBasic,
    ClientSecretPost,
    #[serde(other)]
    Other,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenEndpointAuthMethodsSupported(SmallVec<[TokenEndpointAuthMethod; 4]>);

impl Default for TokenEndpointAuthMethodsSupported {
    fn default() -> Self {
        Self(smallvec![TokenEndpointAuthMethod::ClientSecretBasic])
    }
}

impl TokenEndpointAuthMethodsSupported {
    pub fn supports(&self, method: TokenEndpointAuthMethod) -> bool {
        self.0.contains(&method)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CodeChallengeMethod {
    #[serde(rename = "plain")]
    Plain,
    #[serde(rename = "S256")]
    S256,
    #[serde(other)]
    Other,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeChallengeMethodsSupported(SmallVec<[CodeChallengeMethod; 4]>);

impl CodeChallengeMethodsSupported {
    pub fn supports(&self, method: CodeChallengeMethod) -> bool {
        self.0.contains(&method)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthorizationServer {
    issuer: String,

    #[serde(default)]
    authorization_endpoint: Option<AbsUri>,

    #[serde(default)]
    token_endpoint: Option<AbsUri>,

    #[serde(default)]
    jwks_uri: Option<AbsUri>,

    #[serde(default)]
    registration_endpoint: Option<AbsUri>,

    #[serde(default)]
    scopes_supported: Vec<String>,

    #[serde(default)]
    response_types_supported: ResponseTypesSupported,

    #[serde(default)]
    grant_types_supported: GrantTypesSupported,

    #[serde(default)]
    token_endpoint_auth_methods_supported: TokenEndpointAuthMethodsSupported,

    #[serde(default)]
    code_challenge_methods_supported: CodeChallengeMethodsSupported,

    #[serde(default)]
    client_id_metadata_document_supported: bool,
}

#[cfg(test)]
impl Default for AuthorizationServer {
    fn default() -> Self {
        Self {
            issuer: String::new(),
            authorization_endpoint: None,
            token_endpoint: None,
            jwks_uri: None,
            registration_endpoint: None,
            scopes_supported: Vec::new(),
            response_types_supported: ResponseTypesSupported(smallvec![ResponseType::Code]),
            grant_types_supported: GrantTypesSupported::default(),
            token_endpoint_auth_methods_supported: TokenEndpointAuthMethodsSupported::default(),
            code_challenge_methods_supported: CodeChallengeMethodsSupported(smallvec![
                CodeChallengeMethod::S256
            ]),
            client_id_metadata_document_supported: false,
        }
    }
}

impl AuthorizationServer {
    pub async fn fetch(
        issuer_uri: &AbsUri,
        http_client: &Arc<dyn HttpClient>,
    ) -> Result<Self, AuthorizationServerMetadataDiscoveryError> {
        // We must attempt multiple well-known endpoints based on the issuer url
        //
        // https://modelcontextprotocol.io/specification/2025-11-25/basic/authorization#authorization-server-metadata-discovery
        let candidates: [fn(&AbsUri) -> Option<String>; _] = [
            // 1. OAuth 2.0 Authorization Server Metadata
            |base| well_known_pre(base, "oauth-authorization-server").into(),
            // 2. OpenID Connect Discovery 1.0 with path insertion
            |base| well_known_pre(base, "openid-configuration").into(),
            // 3. OpenID Connect Discovery 1.0 with path appening
            |base| {
                if base.path() != "/" {
                    Some(well_known_post(base, "openid-configuration"))
                } else {
                    // We already tried the root in the previous step
                    None
                }
            },
        ];

        let mut attempted_urls = Vec::new();

        for build_url in candidates {
            let Some(url) = build_url(&issuer_uri) else {
                continue;
            };

            match get_json(&url, &http_client).await {
                Ok(meta) => return Ok(meta),
                Err(err) => {
                    attempted_urls.push((url, err));
                }
            }
        }

        Err(AuthorizationServerMetadataDiscoveryError { attempted_urls })
    }
}

fn well_known_pre(base_uri: &AbsUri, well_known_segment: &str) -> String {
    format!(
        "{}://{}/.well-known/{well_known_segment}{}",
        base_uri.scheme_str(),
        base_uri.authority(),
        base_uri.path().trim_end_matches('/')
    )
}

fn well_known_post(base_uri: &AbsUri, well_known_segment: &str) -> String {
    let path = base_uri.path();
    let separator = if path.ends_with('/') { "" } else { "/" };
    format!(
        "{}://{}{}{separator}.well-known/{well_known_segment}",
        base_uri.scheme_str(),
        base_uri.authority(),
        path,
    )
}

#[derive(Debug)]
pub struct AuthorizationServerMetadataDiscoveryError {
    attempted_urls: Vec<(String, anyhow::Error)>,
}

impl Error for AuthorizationServerMetadataDiscoveryError {}

impl Display for AuthorizationServerMetadataDiscoveryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Failed to discover authorization server metadata. Attempted URLs:"
        )?;

        for (url, err) in &self.attempted_urls {
            writeln!(f, "- {url}: {err}")?;
        }

        fmt::Result::Ok(())
    }
}

async fn get_json<Out: DeserializeOwned>(
    url: &str,
    http_client: &Arc<dyn HttpClient>,
) -> Result<Out> {
    {
        let mut response = http_client.get(url, AsyncBody::empty(), true).await?;
        decode_response_json(&mut response).await
    }
    .with_context(|| format!("GET {url}"))
}

async fn post_json<In: Serialize, Out: DeserializeOwned>(
    url: &str,
    payload: In,
    http_client: &Arc<dyn HttpClient>,
) -> Result<Out> {
    {
        let mut response = http_client
            .post_json(url, serde_json::to_string(&payload)?.into())
            .await?;
        decode_response_json(&mut response).await
    }
    .with_context(|| format!("POST {url}"))
}

async fn decode_response_json<T: DeserializeOwned>(
    response: &mut Response<AsyncBody>,
) -> Result<T> {
    let mut content = Vec::new();
    response.body_mut().read_to_end(&mut content).await?;
    if response.status().is_success() {
        Ok(serde_json::from_slice(&content)?)
    } else {
        anyhow::bail!(
            "Status: {}.\nBody: {}",
            response.status(),
            String::from_utf8_lossy(&content)
        );
    }
}

// Types used externally for UI and persistance

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ContextServerAuthStatus {
    None,
    Authenticated,
    AwaitingAuthorization,
    Required,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContextServerCredentials {
    registration: ClientRegistration,
    server: AuthorizationServer,
    scope: Option<String>,
    state: State,
}

#[derive(Clone, Debug)]
pub struct ContextServerAuth {
    pub status: ContextServerAuthStatus,
    pub credentials: Option<ContextServerCredentials>,
}

impl ContextServerAuth {
    pub fn from_credentials(credentials: Option<ContextServerCredentials>) -> Self {
        let status = match &credentials {
            None => ContextServerAuthStatus::None,
            Some(creds) => match &creds.state {
                State::Unauthenticated => ContextServerAuthStatus::None,
                State::WaitingForCode { .. } => ContextServerAuthStatus::AwaitingAuthorization,
                State::Authenticated { .. } => ContextServerAuthStatus::Authenticated,
            },
        };
        Self {
            status,
            credentials,
        }
    }

    pub fn required(credentials: Option<ContextServerCredentials>) -> Self {
        Self {
            status: ContextServerAuthStatus::Required,
            credentials,
        }
    }
}

pub struct AccessToken<'a> {
    pub token: Option<&'a str>,
    pub refreshed: bool,
}

mod abs_uri {
    use std::{
        error::Error,
        fmt::{self, Display},
        ops::Deref,
    };

    use http_client::{Uri, http::uri::Authority};
    use serde::Deserialize;

    #[derive(Debug, Clone)]
    pub struct AbsUri(Uri);

    impl AbsUri {
        pub fn authority(&self) -> &Authority {
            self.0.authority().unwrap()
        }

        pub fn scheme_str(&self) -> &str {
            self.0.scheme_str().unwrap()
        }
    }

    impl Into<Uri> for AbsUri {
        fn into(self) -> Uri {
            self.0
        }
    }

    impl TryFrom<Uri> for AbsUri {
        type Error = AbsUriError;

        fn try_from(uri: Uri) -> Result<Self, Self::Error> {
            if uri.scheme().is_none() {
                return Err(AbsUriError::MissingScheme);
            }
            if uri.authority().is_none() {
                return Err(AbsUriError::MissingAuthority);
            }
            Ok(Self(uri))
        }
    }

    impl Deref for AbsUri {
        type Target = Uri;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<'de> Deserialize<'de> for AbsUri {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse::<Uri>()
                .map_err(serde::de::Error::custom)?
                .try_into()
                .map_err(|e| serde::de::Error::custom(format!("{e:?}")))
        }
    }

    impl serde::Serialize for AbsUri {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_str(&self.0.to_string())
        }
    }

    #[derive(Debug)]
    pub enum AbsUriError {
        MissingScheme,
        MissingAuthority,
    }

    impl Error for AbsUriError {}

    impl Display for AbsUriError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                AbsUriError::MissingScheme => write!(f, "URI is not absolute: Missing scheme"),
                AbsUriError::MissingAuthority => {
                    write!(f, "URI is not absolute: Missing authority")
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    use futures::StreamExt;
    use futures::channel::{mpsc, oneshot};
    use gpui::{TestAppContext, prelude::*};
    use http_client::{FakeHttpClient, Request, Response};
    use pretty_assertions::{assert_eq, assert_matches};

    #[gpui::test]
    async fn init_resource_metadata_url_and_url_client_id(cx: &mut TestAppContext) {
        let (http_client, mut requests) = fake_client();

        let www_authenticate = WwwAuthenticate {
            realm: None,
            scope: None,
            error: None,
            error_description: None,
            error_uri: None,
            resource_metadata: Some("https://resource.example.com/meta.json".into()),
        };

        let init_task = cx.background_spawn(async move {
            OAuthClient::init(
                "https://mcp.example.com",
                Some(&www_authenticate),
                &http_client,
            )
            .await
        });

        let request = requests.next().await.expect("Expected request");
        assert_eq!(request.uri, "https://resource.example.com/meta.json");
        respond_json(
            request,
            200,
            &ProtectedResource {
                resource: "https://mcp.example.com".to_string(),
                authorization_servers: vec![
                    "https://auth.example.com"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ],
                scopes_supported: vec!["mcp:read".to_string(), "mcp:write".to_string()],
                ..Default::default()
            },
        );

        let request = requests.next().await.expect("Expected request");
        assert_eq!(
            request.uri,
            "https://auth.example.com/.well-known/oauth-authorization-server"
        );
        respond_json(
            request,
            200,
            &AuthorizationServer {
                issuer: "https://auth.example.com".to_string(),
                authorization_endpoint: Some(
                    "https://auth.example.com/authorize"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                token_endpoint: Some(
                    "https://auth.example.com/token"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                registration_endpoint: Some(
                    "https://auth.example.com/register"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                client_id_metadata_document_supported: true,
                ..Default::default()
            },
        );

        let client = init_task.await.expect("init should succeed");
        assert_eq!(
            client.registration.client_id,
            "https://zed.dev/mcp/oauth-client.json"
        );
        assert_eq!(client.scope.as_deref(), Some("mcp:read mcp:write"));

        drop(client);

        let unexpected_request = requests.next().await;
        assert!(
            unexpected_request.is_none(),
            "did not expect dynamic registration request when client_id_metadata_document_supported is true"
        );
    }

    #[gpui::test]
    async fn init_well_known_resource_metadata(cx: &mut TestAppContext) {
        let (client, mut requests) = fake_client();
        let http_client = client.clone();

        let init_task = cx.background_spawn(async move {
            OAuthClient::init("https://mcp.example.com/rpc", None, &http_client).await
        });

        let request = requests.next().await.expect("Expected request");
        assert_eq!(
            request.uri,
            "https://mcp.example.com/.well-known/oauth-protected-resource/rpc"
        );
        respond_json(
            request,
            200,
            &ProtectedResource {
                resource: "https://mcp.example.com/rpc".to_string(),
                authorization_servers: vec![
                    "https://auth.example.com"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ],
                ..Default::default()
            },
        );

        let request = requests.next().await.expect("Expected request");
        assert_eq!(
            request.uri,
            "https://auth.example.com/.well-known/oauth-authorization-server"
        );
        respond_json(
            request,
            200,
            &AuthorizationServer {
                issuer: "https://auth.example.com".to_string(),
                authorization_endpoint: Some(
                    "https://auth.example.com/authorize"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                token_endpoint: Some(
                    "https://auth.example.com/token"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                client_id_metadata_document_supported: true,
                ..Default::default()
            },
        );

        let client = init_task.await.expect("init should succeed");
        assert_eq!(
            client.scope, None,
            "no scopes_supported and no challenge scope"
        );
    }

    #[gpui::test]
    async fn init_errors_when_unsupported_bearer_method(cx: &mut TestAppContext) {
        let (client, mut requests) = fake_client();
        let http_client = client.clone();

        let init_task = cx.background_spawn(async move {
            OAuthClient::init("https://mcp.example.com", None, &http_client).await
        });

        let request = requests.next().await.expect("Expected request");
        respond_json(
            request,
            200,
            &ProtectedResource {
                resource: "https://mcp.example.com".to_string(),
                authorization_servers: vec![
                    "https://auth.example.com"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ],
                bearer_methods_supported: BearerMethodsSupported(vec![BearerMethod::Body]),
                ..Default::default()
            },
        );

        assert_matches!(
            init_task.await.err().unwrap().downcast::<InitError>(),
            Ok(InitError::UnsupportedBearerMethod)
        );
    }

    #[gpui::test]
    async fn init_errors_when_no_authorization_servers(cx: &mut TestAppContext) {
        let (client, mut requests) = fake_client();
        let http_client = client.clone();

        let init_task = cx.background_spawn(async move {
            OAuthClient::init("https://mcp.example.com", None, &http_client).await
        });

        let request = requests.next().await.expect("Expected request");
        assert_eq!(
            request.uri,
            "https://mcp.example.com/.well-known/oauth-protected-resource"
        );
        respond_json(
            request,
            200,
            &ProtectedResource {
                resource: "https://mcp.example.com".to_string(),
                ..Default::default()
            },
        );

        assert_matches!(
            init_task.await.err().unwrap().downcast::<InitError>(),
            Ok(InitError::NoAuthorizationServers)
        );
    }

    #[gpui::test]
    async fn init_uses_dynamic_registration_when_supported(cx: &mut TestAppContext) {
        let (client, mut requests) = fake_client();
        let http_client = client.clone();

        let init_task = cx.background_spawn(async move {
            OAuthClient::init("https://mcp.example.com", None, &http_client).await
        });

        let request = requests.next().await.expect("Expected request");
        assert_eq!(
            request.uri,
            "https://mcp.example.com/.well-known/oauth-protected-resource"
        );
        respond_json(
            request,
            200,
            &ProtectedResource {
                resource: "https://mcp.example.com".to_string(),
                authorization_servers: vec![
                    "https://auth.example.com"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ],
                ..Default::default()
            },
        );

        let request = requests.next().await.expect("Expected request");
        assert_eq!(
            request.uri,
            "https://auth.example.com/.well-known/oauth-authorization-server"
        );
        respond_json(
            request,
            200,
            &AuthorizationServer {
                issuer: "https://auth.example.com".to_string(),
                authorization_endpoint: Some(
                    "https://auth.example.com/authorize"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                token_endpoint: Some(
                    "https://auth.example.com/token"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                registration_endpoint: Some(
                    "https://auth.example.com/register"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                token_endpoint_auth_methods_supported: TokenEndpointAuthMethodsSupported(
                    smallvec![TokenEndpointAuthMethod::None],
                ),
                client_id_metadata_document_supported: false,
                ..Default::default()
            },
        );

        let request = requests.next().await.expect("Expected request");
        assert_eq!(request.uri, "https://auth.example.com/register");
        respond_json(
            request,
            200,
            &ClientRegistration {
                client_id: "client-id-123".to_string(),
                ..Default::default()
            },
        );

        let client = init_task.await.expect("init should succeed");
        assert_eq!(client.registration.client_id, "client-id-123");
    }

    #[gpui::test]
    async fn init_errors_when_no_registration_supported(cx: &mut TestAppContext) {
        let (client, mut requests) = fake_client();
        let http_client = client.clone();

        let init_task = cx.background_spawn(async move {
            OAuthClient::init("https://mcp.example.com", None, &http_client).await
        });

        // Resource metadata via well-known
        let request = requests.next().await.expect("Expected request");
        assert_eq!(
            request.uri,
            "https://mcp.example.com/.well-known/oauth-protected-resource"
        );
        respond_json(
            request,
            200,
            &ProtectedResource {
                resource: "https://mcp.example.com".to_string(),
                authorization_servers: vec![
                    "https://auth.example.com"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ],
                ..Default::default()
            },
        );

        // Server metadata: neither client_id_metadata_document_supported nor registration_endpoint
        let request = requests.next().await.expect("Expected request");
        assert_eq!(
            request.uri,
            "https://auth.example.com/.well-known/oauth-authorization-server"
        );
        respond_json(
            request,
            200,
            &AuthorizationServer {
                issuer: "https://auth.example.com".to_string(),
                authorization_endpoint: Some(
                    "https://auth.example.com/authorize"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                token_endpoint: Some(
                    "https://auth.example.com/token"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                client_id_metadata_document_supported: false,
                ..Default::default()
            },
        );

        assert_matches!(
            init_task.await.err().unwrap().downcast::<InitError>(),
            Ok(InitError::UnsupportedRegistration)
        );
    }

    #[gpui::test]
    async fn init_errors_when_unsupported_response_type(cx: &mut TestAppContext) {
        let (client, mut requests) = fake_client();
        let http_client = client.clone();

        let init_task = cx.background_spawn(async move {
            OAuthClient::init("https://mcp.example.com", None, &http_client).await
        });

        let request = requests.next().await.expect("Expected request");
        respond_json(
            request,
            200,
            &ProtectedResource {
                resource: "https://mcp.example.com".to_string(),
                authorization_servers: vec![
                    "https://auth.example.com"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ],
                ..Default::default()
            },
        );

        let request = requests.next().await.expect("Expected request");
        respond_json(
            request,
            200,
            &AuthorizationServer {
                issuer: "https://auth.example.com".to_string(),
                response_types_supported: ResponseTypesSupported(smallvec![ResponseType::Token]),
                client_id_metadata_document_supported: true,
                ..Default::default()
            },
        );

        assert_matches!(
            init_task.await.err().unwrap().downcast::<InitError>(),
            Ok(InitError::UnsupportedResponseType)
        );
    }

    #[gpui::test]
    async fn init_errors_when_unsupported_grant_type(cx: &mut TestAppContext) {
        let (client, mut requests) = fake_client();
        let http_client = client.clone();

        let init_task = cx.background_spawn(async move {
            OAuthClient::init("https://mcp.example.com", None, &http_client).await
        });

        let request = requests.next().await.expect("Expected request");
        respond_json(
            request,
            200,
            &ProtectedResource {
                resource: "https://mcp.example.com".to_string(),
                authorization_servers: vec![
                    "https://auth.example.com"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ],
                ..Default::default()
            },
        );

        let request = requests.next().await.expect("Expected request");
        respond_json(
            request,
            200,
            &AuthorizationServer {
                issuer: "https://auth.example.com".to_string(),
                response_types_supported: ResponseTypesSupported(smallvec![ResponseType::Code]),
                grant_types_supported: GrantTypesSupported(smallvec![GrantType::Implicit]),
                client_id_metadata_document_supported: true,
                ..Default::default()
            },
        );

        assert_matches!(
            init_task.await.err().unwrap().downcast::<InitError>(),
            Ok(InitError::UnsupportedGrantType)
        );
    }

    #[gpui::test]
    async fn init_errors_when_unsupported_code_challenge_method(cx: &mut TestAppContext) {
        let (client, mut requests) = fake_client();
        let http_client = client.clone();

        let init_task = cx.background_spawn(async move {
            OAuthClient::init("https://mcp.example.com", None, &http_client).await
        });

        let request = requests.next().await.expect("Expected request");
        respond_json(
            request,
            200,
            &ProtectedResource {
                resource: "https://mcp.example.com".to_string(),
                authorization_servers: vec![
                    "https://auth.example.com"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ],
                ..Default::default()
            },
        );

        let request = requests.next().await.expect("Expected request");
        respond_json(
            request,
            200,
            &AuthorizationServer {
                issuer: "https://auth.example.com".to_string(),
                response_types_supported: ResponseTypesSupported(smallvec![ResponseType::Code]),
                code_challenge_methods_supported: CodeChallengeMethodsSupported(smallvec![
                    CodeChallengeMethod::Plain
                ]),
                client_id_metadata_document_supported: true,
                ..Default::default()
            },
        );

        assert_matches!(
            init_task.await.err().unwrap().downcast::<InitError>(),
            Ok(InitError::UnsupportedCodeChallengeMethod)
        );
    }

    #[gpui::test]
    async fn init_errors_when_unsupported_token_endpoint_auth_method(cx: &mut TestAppContext) {
        let (client, mut requests) = fake_client();
        let http_client = client.clone();

        let init_task = cx.background_spawn(async move {
            OAuthClient::init("https://mcp.example.com", None, &http_client).await
        });

        let request = requests.next().await.expect("Expected request");
        respond_json(
            request,
            200,
            &ProtectedResource {
                resource: "https://mcp.example.com".to_string(),
                authorization_servers: vec![
                    "https://auth.example.com"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ],
                ..Default::default()
            },
        );

        let request = requests.next().await.expect("Expected request");
        respond_json(
            request,
            200,
            &AuthorizationServer {
                issuer: "https://auth.example.com".to_string(),
                response_types_supported: ResponseTypesSupported(smallvec![ResponseType::Code]),
                token_endpoint_auth_methods_supported: TokenEndpointAuthMethodsSupported(
                    smallvec![TokenEndpointAuthMethod::ClientSecretBasic],
                ),
                registration_endpoint: Some(
                    "https://auth.example.com/register"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                client_id_metadata_document_supported: false,
                ..Default::default()
            },
        );

        assert_matches!(
            init_task.await.err().unwrap().downcast::<InitError>(),
            Ok(InitError::UnsupportedTokenEndpointAuthMethod)
        );
    }

    #[gpui::test]
    async fn init_prefers_challenge_scope_over_resource_scopes(cx: &mut TestAppContext) {
        let (client, mut requests) = fake_client();
        let http_client = client.clone();

        let www_authenticate = WwwAuthenticate {
            realm: None,
            scope: Some("from-challenge".into()),
            error: None,
            error_description: None,
            error_uri: None,
            resource_metadata: None,
        };

        let init_task = cx.background_spawn(async move {
            OAuthClient::init(
                "https://mcp.example.com",
                Some(&www_authenticate),
                &http_client,
            )
            .await
        });

        let request = requests.next().await.expect("Expected request");
        assert_eq!(
            request.uri,
            "https://mcp.example.com/.well-known/oauth-protected-resource"
        );
        respond_json(
            request,
            200,
            &ProtectedResource {
                resource: "https://mcp.example.com".to_string(),
                authorization_servers: vec![
                    "https://auth.example.com"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ],
                scopes_supported: vec!["from-resource".to_string()],
                ..Default::default()
            },
        );

        let request = requests.next().await.expect("Expected request");
        assert_eq!(
            request.uri,
            "https://auth.example.com/.well-known/oauth-authorization-server"
        );
        respond_json(
            request,
            200,
            &AuthorizationServer {
                issuer: "https://auth.example.com".to_string(),
                authorization_endpoint: Some(
                    "https://auth.example.com/authorize"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                token_endpoint: Some(
                    "https://auth.example.com/token"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                client_id_metadata_document_supported: true,
                ..Default::default()
            },
        );

        let client = init_task.await.expect("init should succeed");
        assert_eq!(client.scope.as_deref(), Some("from-challenge"));
    }

    #[gpui::test]
    async fn init_uses_resource_scopes_when_challenge_scope_missing(cx: &mut TestAppContext) {
        let (client, mut requests) = fake_client();
        let http_client = client.clone();

        let www_authenticate = WwwAuthenticate {
            realm: None,
            scope: None,
            error: None,
            error_description: None,
            error_uri: None,
            resource_metadata: None,
        };

        let init_task = cx.background_spawn(async move {
            OAuthClient::init(
                "https://mcp.example.com",
                Some(&www_authenticate),
                &http_client,
            )
            .await
        });

        let request = requests.next().await.expect("Expected request");
        assert_eq!(
            request.uri,
            "https://mcp.example.com/.well-known/oauth-protected-resource"
        );
        respond_json(
            request,
            200,
            &ProtectedResource {
                resource: "https://mcp.example.com".to_string(),
                authorization_servers: vec![
                    "https://auth.example.com"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ],
                scopes_supported: vec![
                    "from-resource".to_string(),
                    "also-from-resource".to_string(),
                ],
                ..Default::default()
            },
        );

        let request = requests.next().await.expect("Expected request");
        assert_eq!(
            request.uri,
            "https://auth.example.com/.well-known/oauth-authorization-server"
        );
        respond_json(
            request,
            200,
            &AuthorizationServer {
                issuer: "https://auth.example.com".to_string(),
                authorization_endpoint: Some(
                    "https://auth.example.com/authorize"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                token_endpoint: Some(
                    "https://auth.example.com/token"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                client_id_metadata_document_supported: true,
                ..Default::default()
            },
        );

        let client = init_task.await.expect("init should succeed");
        assert_eq!(
            client.scope.as_deref(),
            Some("from-resource also-from-resource")
        );
    }

    #[gpui::test]
    async fn fetch_server_metadata_chain(cx: &mut TestAppContext) {
        expect_fallback_chain(
            "https://auth.example.com/tenant/123",
            &[
                "https://auth.example.com/.well-known/oauth-authorization-server/tenant/123",
                "https://auth.example.com/.well-known/openid-configuration/tenant/123",
                "https://auth.example.com/tenant/123/.well-known/openid-configuration",
            ],
            cx,
        )
        .await;

        expect_fallback_chain(
            "https://auth.example.com/tenant/123/",
            &[
                "https://auth.example.com/.well-known/oauth-authorization-server/tenant/123",
                "https://auth.example.com/.well-known/openid-configuration/tenant/123",
                "https://auth.example.com/tenant/123/.well-known/openid-configuration",
            ],
            cx,
        )
        .await;

        expect_fallback_chain(
            "https://auth.example.com",
            &[
                "https://auth.example.com/.well-known/oauth-authorization-server",
                "https://auth.example.com/.well-known/openid-configuration",
            ],
            cx,
        )
        .await;
    }

    async fn expect_fallback_chain(issuer_uri: &str, urls: &[&str], cx: &mut TestAppContext) {
        let issuer_uri: AbsUri = issuer_uri.parse::<Uri>().unwrap().try_into().unwrap();
        let (client, mut request_rx) = fake_client();

        for i in 0..urls.len() {
            let issuer_uri = issuer_uri.clone();
            let client = client.clone();
            let fetch_task = cx.background_spawn(async move {
                AuthorizationServer::fetch(&issuer_uri, &client).await
            });

            for request_url in &urls[..i] {
                let request = request_rx.next().await.unwrap();
                assert_eq!(request.uri, *request_url);
                respond(request, not_found());
            }

            let request = request_rx.next().await.unwrap();
            assert_eq!(request.uri, *urls[i]);
            respond(
                request,
                Response::builder()
                    .status(200)
                    .header("Content-Type", "application/json")
                    .body(AsyncBody::from(valid_metadata_json(
                        "https://auth.example.com",
                    )))
                    .unwrap(),
            );

            let metadata = fetch_task.await.expect("fetch should succeed");
            assert_eq!(metadata.issuer, "https://auth.example.com");
        }
    }

    #[gpui::test]
    async fn fetch_server_metadata_openid_root_stops_on_fail(cx: &mut TestAppContext) {
        let (client, mut requests) = fake_client();
        let http_client = client.clone();

        let fetch_task = cx.background_spawn(async move {
            let issuer_uri: AbsUri = "https://auth.example.com"
                .parse::<Uri>()
                .unwrap()
                .try_into()
                .unwrap();

            AuthorizationServer::fetch(&issuer_uri, &http_client).await
        });

        let request = requests.next().await.expect("Expected first request");
        assert_eq!(
            request.uri,
            "https://auth.example.com/.well-known/oauth-authorization-server"
        );
        respond(request, not_found());

        let request = requests.next().await.expect("Expected second request");
        assert_eq!(
            request.uri,
            "https://auth.example.com/.well-known/openid-configuration"
        );
        respond(request, not_found());

        // should not attempt well_known_post since it'd be the same as well_known_pre
        let error = fetch_task.await.expect_err("fetch should fail");
        assert_eq!(error.attempted_urls.len(), 2);
    }

    #[gpui::test]
    async fn fetch_server_metadata_all_fail(cx: &mut TestAppContext) {
        let (client, mut requests) = fake_client();
        let http_client = client.clone();

        let fetch_task = cx.background_spawn(async move {
            let issuer_uri: AbsUri = "https://auth.example.com/tenant/123"
                .parse::<Uri>()
                .unwrap()
                .try_into()
                .unwrap();

            AuthorizationServer::fetch(&issuer_uri, &http_client).await
        });

        for _ in 0..3 {
            let request = requests.next().await.expect("Expected request");
            respond(request, not_found());
        }

        let error = fetch_task.await.expect_err("fetch should fail");
        assert_eq!(error.attempted_urls.len(), 3);
    }

    #[gpui::test]
    async fn authorize_url_includes_required_oauth_params(cx: &mut TestAppContext) {
        let mut client = init_oauth_client_for_authorize_url(None, cx).await;

        let url = client
            .authorize_url()
            .expect("authorize_url should succeed");
        let url = url.url;

        assert_eq!(
            url.as_str().split('?').next().unwrap(),
            "https://auth.example.com/authorize"
        );

        let query_pairs: std::collections::HashMap<String, String> = url
            .query_pairs()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect();

        assert_eq!(
            query_pairs.get("response_type").map(String::as_str),
            Some("code")
        );
        assert_eq!(
            query_pairs.get("client_id").map(String::as_str),
            Some("https://zed.dev/mcp/oauth-client.json")
        );
        assert_eq!(
            query_pairs.get("redirect_uri").map(String::as_str),
            Some(OAuthCallback::URI)
        );
        assert_eq!(
            query_pairs.get("code_challenge_method").map(String::as_str),
            Some("S256")
        );

        let code_challenge = query_pairs
            .get("code_challenge")
            .map(String::as_str)
            .expect("code_challenge should be present");
        assert!(
            !code_challenge.is_empty(),
            "code_challenge should be non-empty"
        );
        assert!(
            code_challenge
                .chars()
                .all(|ch| { ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' || ch == '=' }),
            "code_challenge should be base64url-ish"
        );

        assert!(
            !query_pairs.contains_key("scope"),
            "scope should be absent when no scope is configured"
        );
    }

    #[gpui::test]
    async fn authorize_url_includes_scope_when_present(cx: &mut TestAppContext) {
        let mut client = init_oauth_client_for_authorize_url(Some("mcp:read mcp:write"), cx).await;

        let url = client
            .authorize_url()
            .expect("authorize_url should succeed");
        let url = url.url;

        let scopes: Vec<String> = url
            .query_pairs()
            .filter_map(|(key, value)| (key == "scope").then_some(value.to_string()))
            .collect();

        assert_eq!(scopes.as_slice(), &["mcp:read mcp:write".to_string()]);
    }

    #[gpui::test]
    async fn authorize_url_errors_when_missing_authorization_endpoint(cx: &mut TestAppContext) {
        let mut client = init_oauth_client_for_authorize_url(None, cx).await;
        client.server.authorization_endpoint = None;

        assert_matches!(
            client
                .authorize_url()
                .err()
                .unwrap()
                .downcast::<AuthorizeUrlError>(),
            Ok(AuthorizeUrlError::MissingAuthorizationEndpoint)
        );
    }

    struct FakeRequest {
        uri: String,
        respond: oneshot::Sender<Response<AsyncBody>>,
    }

    fn fake_client() -> (
        Arc<http_client::HttpClientWithUrl>,
        mpsc::UnboundedReceiver<FakeRequest>,
    ) {
        let (request_sender, request_receiver) = mpsc::unbounded::<FakeRequest>();

        let client = FakeHttpClient::create(move |req: Request<AsyncBody>| {
            let request_sender = request_sender.clone();
            async move {
                let (respond, response_receiver) = oneshot::channel();
                request_sender
                    .unbounded_send(FakeRequest {
                        uri: req.uri().to_string(),
                        respond,
                    })
                    .expect("Test receiver dropped");

                response_receiver
                    .await
                    .map_err(|_| anyhow::anyhow!("Test dropped response sender"))
            }
        });

        (client, request_receiver)
    }

    async fn init_oauth_client_for_authorize_url(
        scope: Option<&'static str>,
        cx: &mut TestAppContext,
    ) -> OAuthClient {
        let (http_client, mut requests) = fake_client();

        let www_authenticate = WwwAuthenticate {
            realm: None,
            scope: scope.map(Into::into),
            error: None,
            error_description: None,
            error_uri: None,
            resource_metadata: None,
        };

        let init_task = cx.background_spawn(async move {
            OAuthClient::init(
                "https://mcp.example.com",
                Some(&www_authenticate),
                &http_client,
            )
            .await
        });

        let request = requests.next().await.expect("Expected request");
        assert_eq!(
            request.uri,
            "https://mcp.example.com/.well-known/oauth-protected-resource"
        );
        respond_json(
            request,
            200,
            &ProtectedResource {
                resource: "https://mcp.example.com".to_string(),
                authorization_servers: vec![
                    "https://auth.example.com"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ],
                ..Default::default()
            },
        );

        let request = requests.next().await.expect("Expected request");
        assert_eq!(
            request.uri,
            "https://auth.example.com/.well-known/oauth-authorization-server"
        );
        respond_json(
            request,
            200,
            &AuthorizationServer {
                issuer: "https://auth.example.com".to_string(),
                authorization_endpoint: Some(
                    "https://auth.example.com/authorize"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                token_endpoint: Some(
                    "https://auth.example.com/token"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                client_id_metadata_document_supported: true,
                ..Default::default()
            },
        );

        init_task.await.expect("init should succeed")
    }

    fn not_found() -> Response<AsyncBody> {
        Response::builder()
            .status(404)
            .body(AsyncBody::from("Not found".to_string()))
            .unwrap()
    }

    fn valid_metadata_json(issuer: &str) -> String {
        serde_json::json!({
            "issuer": issuer,
            "authorization_endpoint": format!("{}/authorize", issuer),
            "token_endpoint": format!("{}/token", issuer),
        })
        .to_string()
    }

    fn respond(request: FakeRequest, response: Response<AsyncBody>) {
        request.respond.send(response).ok();
    }

    fn respond_json<T: Serialize>(request: FakeRequest, status: u16, value: &T) {
        let body = serde_json::to_string(value).expect("serialize test json");
        respond(
            request,
            Response::builder()
                .status(status)
                .header("Content-Type", "application/json")
                .body(AsyncBody::from(body))
                .unwrap(),
        );
    }
}

```


**Tool Call: git --no-pager show origin/mcp-auth:crates/context_server/src/transport/http/auth.rs | sed -n '1100,1500p'**
Status: Completed

Terminal:
```
                        .unwrap(),
                ),
                token_endpoint: Some(
                    "https://auth.example.com/token"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                registration_endpoint: Some(
                    "https://auth.example.com/register"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                client_id_metadata_document_supported: true,
                ..Default::default()
            },
        );

        let client = init_task.await.expect("init should succeed");
        assert_eq!(
            client.registration.client_id,
            "https://zed.dev/mcp/oauth-client.json"
        );
        assert_eq!(client.scope.as_deref(), Some("mcp:read mcp:write"));

        drop(client);

        let unexpected_request = requests.next().await;
        assert!(
            unexpected_request.is_none(),
            "did not expect dynamic registration request when client_id_metadata_document_supported is true"
        );
    }

    #[gpui::test]
    async fn init_well_known_resource_metadata(cx: &mut TestAppContext) {
        let (client, mut requests) = fake_client();
        let http_client = client.clone();

        let init_task = cx.background_spawn(async move {
            OAuthClient::init("https://mcp.example.com/rpc", None, &http_client).await
        });

        let request = requests.next().await.expect("Expected request");
        assert_eq!(
            request.uri,
            "https://mcp.example.com/.well-known/oauth-protected-resource/rpc"
        );
        respond_json(
            request,
            200,
            &ProtectedResource {
                resource: "https://mcp.example.com/rpc".to_string(),
                authorization_servers: vec![
                    "https://auth.example.com"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ],
                ..Default::default()
            },
        );

        let request = requests.next().await.expect("Expected request");
        assert_eq!(
            request.uri,
            "https://auth.example.com/.well-known/oauth-authorization-server"
        );
        respond_json(
            request,
            200,
            &AuthorizationServer {
                issuer: "https://auth.example.com".to_string(),
                authorization_endpoint: Some(
                    "https://auth.example.com/authorize"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                token_endpoint: Some(
                    "https://auth.example.com/token"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                client_id_metadata_document_supported: true,
                ..Default::default()
            },
        );

        let client = init_task.await.expect("init should succeed");
        assert_eq!(
            client.scope, None,
            "no scopes_supported and no challenge scope"
        );
    }

    #[gpui::test]
    async fn init_errors_when_unsupported_bearer_method(cx: &mut TestAppContext) {
        let (client, mut requests) = fake_client();
        let http_client = client.clone();

        let init_task = cx.background_spawn(async move {
            OAuthClient::init("https://mcp.example.com", None, &http_client).await
        });

        let request = requests.next().await.expect("Expected request");
        respond_json(
            request,
            200,
            &ProtectedResource {
                resource: "https://mcp.example.com".to_string(),
                authorization_servers: vec![
                    "https://auth.example.com"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ],
                bearer_methods_supported: BearerMethodsSupported(vec![BearerMethod::Body]),
                ..Default::default()
            },
        );

        assert_matches!(
            init_task.await.err().unwrap().downcast::<InitError>(),
            Ok(InitError::UnsupportedBearerMethod)
        );
    }

    #[gpui::test]
    async fn init_errors_when_no_authorization_servers(cx: &mut TestAppContext) {
        let (client, mut requests) = fake_client();
        let http_client = client.clone();

        let init_task = cx.background_spawn(async move {
            OAuthClient::init("https://mcp.example.com", None, &http_client).await
        });

        let request = requests.next().await.expect("Expected request");
        assert_eq!(
            request.uri,
            "https://mcp.example.com/.well-known/oauth-protected-resource"
        );
        respond_json(
            request,
            200,
            &ProtectedResource {
                resource: "https://mcp.example.com".to_string(),
                ..Default::default()
            },
        );

        assert_matches!(
            init_task.await.err().unwrap().downcast::<InitError>(),
            Ok(InitError::NoAuthorizationServers)
        );
    }

    #[gpui::test]
    async fn init_uses_dynamic_registration_when_supported(cx: &mut TestAppContext) {
        let (client, mut requests) = fake_client();
        let http_client = client.clone();

        let init_task = cx.background_spawn(async move {
            OAuthClient::init("https://mcp.example.com", None, &http_client).await
        });

        let request = requests.next().await.expect("Expected request");
        assert_eq!(
            request.uri,
            "https://mcp.example.com/.well-known/oauth-protected-resource"
        );
        respond_json(
            request,
            200,
            &ProtectedResource {
                resource: "https://mcp.example.com".to_string(),
                authorization_servers: vec![
                    "https://auth.example.com"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ],
                ..Default::default()
            },
        );

        let request = requests.next().await.expect("Expected request");
        assert_eq!(
            request.uri,
            "https://auth.example.com/.well-known/oauth-authorization-server"
        );
        respond_json(
            request,
            200,
            &AuthorizationServer {
                issuer: "https://auth.example.com".to_string(),
                authorization_endpoint: Some(
                    "https://auth.example.com/authorize"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                token_endpoint: Some(
                    "https://auth.example.com/token"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                registration_endpoint: Some(
                    "https://auth.example.com/register"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                token_endpoint_auth_methods_supported: TokenEndpointAuthMethodsSupported(
                    smallvec![TokenEndpointAuthMethod::None],
                ),
                client_id_metadata_document_supported: false,
                ..Default::default()
            },
        );

        let request = requests.next().await.expect("Expected request");
        assert_eq!(request.uri, "https://auth.example.com/register");
        respond_json(
            request,
            200,
            &ClientRegistration {
                client_id: "client-id-123".to_string(),
                ..Default::default()
            },
        );

        let client = init_task.await.expect("init should succeed");
        assert_eq!(client.registration.client_id, "client-id-123");
    }

    #[gpui::test]
    async fn init_errors_when_no_registration_supported(cx: &mut TestAppContext) {
        let (client, mut requests) = fake_client();
        let http_client = client.clone();

        let init_task = cx.background_spawn(async move {
            OAuthClient::init("https://mcp.example.com", None, &http_client).await
        });

        // Resource metadata via well-known
        let request = requests.next().await.expect("Expected request");
        assert_eq!(
            request.uri,
            "https://mcp.example.com/.well-known/oauth-protected-resource"
        );
        respond_json(
            request,
            200,
            &ProtectedResource {
                resource: "https://mcp.example.com".to_string(),
                authorization_servers: vec![
                    "https://auth.example.com"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ],
                ..Default::default()
            },
        );

        // Server metadata: neither client_id_metadata_document_supported nor registration_endpoint
        let request = requests.next().await.expect("Expected request");
        assert_eq!(
            request.uri,
            "https://auth.example.com/.well-known/oauth-authorization-server"
        );
        respond_json(
            request,
            200,
            &AuthorizationServer {
                issuer: "https://auth.example.com".to_string(),
                authorization_endpoint: Some(
                    "https://auth.example.com/authorize"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                token_endpoint: Some(
                    "https://auth.example.com/token"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                client_id_metadata_document_supported: false,
                ..Default::default()
            },
        );

        assert_matches!(
            init_task.await.err().unwrap().downcast::<InitError>(),
            Ok(InitError::UnsupportedRegistration)
        );
    }

    #[gpui::test]
    async fn init_errors_when_unsupported_response_type(cx: &mut TestAppContext) {
        let (client, mut requests) = fake_client();
        let http_client = client.clone();

        let init_task = cx.background_spawn(async move {
            OAuthClient::init("https://mcp.example.com", None, &http_client).await
        });

        let request = requests.next().await.expect("Expected request");
        respond_json(
            request,
            200,
            &ProtectedResource {
                resource: "https://mcp.example.com".to_string(),
                authorization_servers: vec![
                    "https://auth.example.com"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ],
                ..Default::default()
            },
        );

        let request = requests.next().await.expect("Expected request");
        respond_json(
            request,
            200,
            &AuthorizationServer {
                issuer: "https://auth.example.com".to_string(),
                response_types_supported: ResponseTypesSupported(smallvec![ResponseType::Token]),
                client_id_metadata_document_supported: true,
                ..Default::default()
            },
        );

        assert_matches!(
            init_task.await.err().unwrap().downcast::<InitError>(),
            Ok(InitError::UnsupportedResponseType)
        );
    }

    #[gpui::test]
    async fn init_errors_when_unsupported_grant_type(cx: &mut TestAppContext) {
        let (client, mut requests) = fake_client();
        let http_client = client.clone();

        let init_task = cx.background_spawn(async move {
            OAuthClient::init("https://mcp.example.com", None, &http_client).await
        });

        let request = requests.next().await.expect("Expected request");
        respond_json(
            request,
            200,
            &ProtectedResource {
                resource: "https://mcp.example.com".to_string(),
                authorization_servers: vec![
                    "https://auth.example.com"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ],
                ..Default::default()
            },
        );

        let request = requests.next().await.expect("Expected request");
        respond_json(
            request,
            200,
            &AuthorizationServer {
                issuer: "https://auth.example.com".to_string(),
                response_types_supported: ResponseTypesSupported(smallvec![ResponseType::Code]),
                grant_types_supported: GrantTypesSupported(smallvec![GrantType::Implicit]),
                client_id_metadata_document_supported: true,
                ..Default::default()
            },
        );

        assert_matches!(
            init_task.await.err().unwrap().downcast::<InitError>(),

```


**Tool Call: git --no-pager show origin/mcp-auth:crates/context_server/src/transport/http/auth.rs | sed -n '1500,1700p'**
Status: Completed

Terminal:
```
            init_task.await.err().unwrap().downcast::<InitError>(),
            Ok(InitError::UnsupportedGrantType)
        );
    }

    #[gpui::test]
    async fn init_errors_when_unsupported_code_challenge_method(cx: &mut TestAppContext) {
        let (client, mut requests) = fake_client();
        let http_client = client.clone();

        let init_task = cx.background_spawn(async move {
            OAuthClient::init("https://mcp.example.com", None, &http_client).await
        });

        let request = requests.next().await.expect("Expected request");
        respond_json(
            request,
            200,
            &ProtectedResource {
                resource: "https://mcp.example.com".to_string(),
                authorization_servers: vec![
                    "https://auth.example.com"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ],
                ..Default::default()
            },
        );

        let request = requests.next().await.expect("Expected request");
        respond_json(
            request,
            200,
            &AuthorizationServer {
                issuer: "https://auth.example.com".to_string(),
                response_types_supported: ResponseTypesSupported(smallvec![ResponseType::Code]),
                code_challenge_methods_supported: CodeChallengeMethodsSupported(smallvec![
                    CodeChallengeMethod::Plain
                ]),
                client_id_metadata_document_supported: true,
                ..Default::default()
            },
        );

        assert_matches!(
            init_task.await.err().unwrap().downcast::<InitError>(),
            Ok(InitError::UnsupportedCodeChallengeMethod)
        );
    }

    #[gpui::test]
    async fn init_errors_when_unsupported_token_endpoint_auth_method(cx: &mut TestAppContext) {
        let (client, mut requests) = fake_client();
        let http_client = client.clone();

        let init_task = cx.background_spawn(async move {
            OAuthClient::init("https://mcp.example.com", None, &http_client).await
        });

        let request = requests.next().await.expect("Expected request");
        respond_json(
            request,
            200,
            &ProtectedResource {
                resource: "https://mcp.example.com".to_string(),
                authorization_servers: vec![
                    "https://auth.example.com"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ],
                ..Default::default()
            },
        );

        let request = requests.next().await.expect("Expected request");
        respond_json(
            request,
            200,
            &AuthorizationServer {
                issuer: "https://auth.example.com".to_string(),
                response_types_supported: ResponseTypesSupported(smallvec![ResponseType::Code]),
                token_endpoint_auth_methods_supported: TokenEndpointAuthMethodsSupported(
                    smallvec![TokenEndpointAuthMethod::ClientSecretBasic],
                ),
                registration_endpoint: Some(
                    "https://auth.example.com/register"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                client_id_metadata_document_supported: false,
                ..Default::default()
            },
        );

        assert_matches!(
            init_task.await.err().unwrap().downcast::<InitError>(),
            Ok(InitError::UnsupportedTokenEndpointAuthMethod)
        );
    }

    #[gpui::test]
    async fn init_prefers_challenge_scope_over_resource_scopes(cx: &mut TestAppContext) {
        let (client, mut requests) = fake_client();
        let http_client = client.clone();

        let www_authenticate = WwwAuthenticate {
            realm: None,
            scope: Some("from-challenge".into()),
            error: None,
            error_description: None,
            error_uri: None,
            resource_metadata: None,
        };

        let init_task = cx.background_spawn(async move {
            OAuthClient::init(
                "https://mcp.example.com",
                Some(&www_authenticate),
                &http_client,
            )
            .await
        });

        let request = requests.next().await.expect("Expected request");
        assert_eq!(
            request.uri,
            "https://mcp.example.com/.well-known/oauth-protected-resource"
        );
        respond_json(
            request,
            200,
            &ProtectedResource {
                resource: "https://mcp.example.com".to_string(),
                authorization_servers: vec![
                    "https://auth.example.com"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ],
                scopes_supported: vec!["from-resource".to_string()],
                ..Default::default()
            },
        );

        let request = requests.next().await.expect("Expected request");
        assert_eq!(
            request.uri,
            "https://auth.example.com/.well-known/oauth-authorization-server"
        );
        respond_json(
            request,
            200,
            &AuthorizationServer {
                issuer: "https://auth.example.com".to_string(),
                authorization_endpoint: Some(
                    "https://auth.example.com/authorize"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                token_endpoint: Some(
                    "https://auth.example.com/token"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                client_id_metadata_document_supported: true,
                ..Default::default()
            },
        );

        let client = init_task.await.expect("init should succeed");
        assert_eq!(client.scope.as_deref(), Some("from-challenge"));
    }

    #[gpui::test]
    async fn init_uses_resource_scopes_when_challenge_scope_missing(cx: &mut TestAppContext) {
        let (client, mut requests) = fake_client();
        let http_client = client.clone();

        let www_authenticate = WwwAuthenticate {
            realm: None,
            scope: None,
            error: None,
            error_description: None,
            error_uri: None,
            resource_metadata: None,
        };

        let init_task = cx.background_spawn(async move {
            OAuthClient::init(
                "https://mcp.example.com",

```


**Tool Call: git --no-pager show origin/mcp-auth:crates/context_server/src/transport/http/auth.rs | sed -n '1700,2124p'**
Status: Completed

Terminal:
```
                "https://mcp.example.com",
                Some(&www_authenticate),
                &http_client,
            )
            .await
        });

        let request = requests.next().await.expect("Expected request");
        assert_eq!(
            request.uri,
            "https://mcp.example.com/.well-known/oauth-protected-resource"
        );
        respond_json(
            request,
            200,
            &ProtectedResource {
                resource: "https://mcp.example.com".to_string(),
                authorization_servers: vec![
                    "https://auth.example.com"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ],
                scopes_supported: vec![
                    "from-resource".to_string(),
                    "also-from-resource".to_string(),
                ],
                ..Default::default()
            },
        );

        let request = requests.next().await.expect("Expected request");
        assert_eq!(
            request.uri,
            "https://auth.example.com/.well-known/oauth-authorization-server"
        );
        respond_json(
            request,
            200,
            &AuthorizationServer {
                issuer: "https://auth.example.com".to_string(),
                authorization_endpoint: Some(
                    "https://auth.example.com/authorize"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                token_endpoint: Some(
                    "https://auth.example.com/token"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                client_id_metadata_document_supported: true,
                ..Default::default()
            },
        );

        let client = init_task.await.expect("init should succeed");
        assert_eq!(
            client.scope.as_deref(),
            Some("from-resource also-from-resource")
        );
    }

    #[gpui::test]
    async fn fetch_server_metadata_chain(cx: &mut TestAppContext) {
        expect_fallback_chain(
            "https://auth.example.com/tenant/123",
            &[
                "https://auth.example.com/.well-known/oauth-authorization-server/tenant/123",
                "https://auth.example.com/.well-known/openid-configuration/tenant/123",
                "https://auth.example.com/tenant/123/.well-known/openid-configuration",
            ],
            cx,
        )
        .await;

        expect_fallback_chain(
            "https://auth.example.com/tenant/123/",
            &[
                "https://auth.example.com/.well-known/oauth-authorization-server/tenant/123",
                "https://auth.example.com/.well-known/openid-configuration/tenant/123",
                "https://auth.example.com/tenant/123/.well-known/openid-configuration",
            ],
            cx,
        )
        .await;

        expect_fallback_chain(
            "https://auth.example.com",
            &[
                "https://auth.example.com/.well-known/oauth-authorization-server",
                "https://auth.example.com/.well-known/openid-configuration",
            ],
            cx,
        )
        .await;
    }

    async fn expect_fallback_chain(issuer_uri: &str, urls: &[&str], cx: &mut TestAppContext) {
        let issuer_uri: AbsUri = issuer_uri.parse::<Uri>().unwrap().try_into().unwrap();
        let (client, mut request_rx) = fake_client();

        for i in 0..urls.len() {
            let issuer_uri = issuer_uri.clone();
            let client = client.clone();
            let fetch_task = cx.background_spawn(async move {
                AuthorizationServer::fetch(&issuer_uri, &client).await
            });

            for request_url in &urls[..i] {
                let request = request_rx.next().await.unwrap();
                assert_eq!(request.uri, *request_url);
                respond(request, not_found());
            }

            let request = request_rx.next().await.unwrap();
            assert_eq!(request.uri, *urls[i]);
            respond(
                request,
                Response::builder()
                    .status(200)
                    .header("Content-Type", "application/json")
                    .body(AsyncBody::from(valid_metadata_json(
                        "https://auth.example.com",
                    )))
                    .unwrap(),
            );

            let metadata = fetch_task.await.expect("fetch should succeed");
            assert_eq!(metadata.issuer, "https://auth.example.com");
        }
    }

    #[gpui::test]
    async fn fetch_server_metadata_openid_root_stops_on_fail(cx: &mut TestAppContext) {
        let (client, mut requests) = fake_client();
        let http_client = client.clone();

        let fetch_task = cx.background_spawn(async move {
            let issuer_uri: AbsUri = "https://auth.example.com"
                .parse::<Uri>()
                .unwrap()
                .try_into()
                .unwrap();

            AuthorizationServer::fetch(&issuer_uri, &http_client).await
        });

        let request = requests.next().await.expect("Expected first request");
        assert_eq!(
            request.uri,
            "https://auth.example.com/.well-known/oauth-authorization-server"
        );
        respond(request, not_found());

        let request = requests.next().await.expect("Expected second request");
        assert_eq!(
            request.uri,
            "https://auth.example.com/.well-known/openid-configuration"
        );
        respond(request, not_found());

        // should not attempt well_known_post since it'd be the same as well_known_pre
        let error = fetch_task.await.expect_err("fetch should fail");
        assert_eq!(error.attempted_urls.len(), 2);
    }

    #[gpui::test]
    async fn fetch_server_metadata_all_fail(cx: &mut TestAppContext) {
        let (client, mut requests) = fake_client();
        let http_client = client.clone();

        let fetch_task = cx.background_spawn(async move {
            let issuer_uri: AbsUri = "https://auth.example.com/tenant/123"
                .parse::<Uri>()
                .unwrap()
                .try_into()
                .unwrap();

            AuthorizationServer::fetch(&issuer_uri, &http_client).await
        });

        for _ in 0..3 {
            let request = requests.next().await.expect("Expected request");
            respond(request, not_found());
        }

        let error = fetch_task.await.expect_err("fetch should fail");
        assert_eq!(error.attempted_urls.len(), 3);
    }

    #[gpui::test]
    async fn authorize_url_includes_required_oauth_params(cx: &mut TestAppContext) {
        let mut client = init_oauth_client_for_authorize_url(None, cx).await;

        let url = client
            .authorize_url()
            .expect("authorize_url should succeed");
        let url = url.url;

        assert_eq!(
            url.as_str().split('?').next().unwrap(),
            "https://auth.example.com/authorize"
        );

        let query_pairs: std::collections::HashMap<String, String> = url
            .query_pairs()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect();

        assert_eq!(
            query_pairs.get("response_type").map(String::as_str),
            Some("code")
        );
        assert_eq!(
            query_pairs.get("client_id").map(String::as_str),
            Some("https://zed.dev/mcp/oauth-client.json")
        );
        assert_eq!(
            query_pairs.get("redirect_uri").map(String::as_str),
            Some(OAuthCallback::URI)
        );
        assert_eq!(
            query_pairs.get("code_challenge_method").map(String::as_str),
            Some("S256")
        );

        let code_challenge = query_pairs
            .get("code_challenge")
            .map(String::as_str)
            .expect("code_challenge should be present");
        assert!(
            !code_challenge.is_empty(),
            "code_challenge should be non-empty"
        );
        assert!(
            code_challenge
                .chars()
                .all(|ch| { ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' || ch == '=' }),
            "code_challenge should be base64url-ish"
        );

        assert!(
            !query_pairs.contains_key("scope"),
            "scope should be absent when no scope is configured"
        );
    }

    #[gpui::test]
    async fn authorize_url_includes_scope_when_present(cx: &mut TestAppContext) {
        let mut client = init_oauth_client_for_authorize_url(Some("mcp:read mcp:write"), cx).await;

        let url = client
            .authorize_url()
            .expect("authorize_url should succeed");
        let url = url.url;

        let scopes: Vec<String> = url
            .query_pairs()
            .filter_map(|(key, value)| (key == "scope").then_some(value.to_string()))
            .collect();

        assert_eq!(scopes.as_slice(), &["mcp:read mcp:write".to_string()]);
    }

    #[gpui::test]
    async fn authorize_url_errors_when_missing_authorization_endpoint(cx: &mut TestAppContext) {
        let mut client = init_oauth_client_for_authorize_url(None, cx).await;
        client.server.authorization_endpoint = None;

        assert_matches!(
            client
                .authorize_url()
                .err()
                .unwrap()
                .downcast::<AuthorizeUrlError>(),
            Ok(AuthorizeUrlError::MissingAuthorizationEndpoint)
        );
    }

    struct FakeRequest {
        uri: String,
        respond: oneshot::Sender<Response<AsyncBody>>,
    }

    fn fake_client() -> (
        Arc<http_client::HttpClientWithUrl>,
        mpsc::UnboundedReceiver<FakeRequest>,
    ) {
        let (request_sender, request_receiver) = mpsc::unbounded::<FakeRequest>();

        let client = FakeHttpClient::create(move |req: Request<AsyncBody>| {
            let request_sender = request_sender.clone();
            async move {
                let (respond, response_receiver) = oneshot::channel();
                request_sender
                    .unbounded_send(FakeRequest {
                        uri: req.uri().to_string(),
                        respond,
                    })
                    .expect("Test receiver dropped");

                response_receiver
                    .await
                    .map_err(|_| anyhow::anyhow!("Test dropped response sender"))
            }
        });

        (client, request_receiver)
    }

    async fn init_oauth_client_for_authorize_url(
        scope: Option<&'static str>,
        cx: &mut TestAppContext,
    ) -> OAuthClient {
        let (http_client, mut requests) = fake_client();

        let www_authenticate = WwwAuthenticate {
            realm: None,
            scope: scope.map(Into::into),
            error: None,
            error_description: None,
            error_uri: None,
            resource_metadata: None,
        };

        let init_task = cx.background_spawn(async move {
            OAuthClient::init(
                "https://mcp.example.com",
                Some(&www_authenticate),
                &http_client,
            )
            .await
        });

        let request = requests.next().await.expect("Expected request");
        assert_eq!(
            request.uri,
            "https://mcp.example.com/.well-known/oauth-protected-resource"
        );
        respond_json(
            request,
            200,
            &ProtectedResource {
                resource: "https://mcp.example.com".to_string(),
                authorization_servers: vec![
                    "https://auth.example.com"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ],
                ..Default::default()
            },
        );

        let request = requests.next().await.expect("Expected request");
        assert_eq!(
            request.uri,
            "https://auth.example.com/.well-known/oauth-authorization-server"
        );
        respond_json(
            request,
            200,
            &AuthorizationServer {
                issuer: "https://auth.example.com".to_string(),
                authorization_endpoint: Some(
                    "https://auth.example.com/authorize"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                token_endpoint: Some(
                    "https://auth.example.com/token"
                        .parse::<Uri>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ),
                client_id_metadata_document_supported: true,
                ..Default::default()
            },
        );

        init_task.await.expect("init should succeed")
    }

    fn not_found() -> Response<AsyncBody> {
        Response::builder()
            .status(404)
            .body(AsyncBody::from("Not found".to_string()))
            .unwrap()
    }

    fn valid_metadata_json(issuer: &str) -> String {
        serde_json::json!({
            "issuer": issuer,
            "authorization_endpoint": format!("{}/authorize", issuer),
            "token_endpoint": format!("{}/token", issuer),
        })
        .to_string()
    }

    fn respond(request: FakeRequest, response: Response<AsyncBody>) {
        request.respond.send(response).ok();
    }

    fn respond_json<T: Serialize>(request: FakeRequest, status: u16, value: &T) {
        let body = serde_json::to_string(value).expect("serialize test json");
        respond(
            request,
            Response::builder()
                .status(status)
                .header("Content-Type", "application/json")
                .body(AsyncBody::from(body))
                .unwrap(),
        );
    }
}

```


## Assistant

Now let me look at the mcp-auth branch's transport, context_server_store, www_authenticate, and UI changes:

**Tool Call: git --no-pager show origin/mcp-auth:crates/context_server/src/transport/http.rs**
Status: Completed

Terminal:
```
mod auth;
mod www_authenticate;

use anyhow::{Result, anyhow};
use async_trait::async_trait;
use collections::HashMap;
use futures::{Stream, StreamExt, lock::Mutex};
use gpui::BackgroundExecutor;
use http_client::{AsyncBody, HttpClient, Request, Response, http::Method};
use parking_lot::Mutex as SyncMutex;
use smol::channel;
use std::{pin::Pin, sync::Arc};

use crate::transport::Transport;
use auth::OAuthClient;
use www_authenticate::WwwAuthenticate;

pub use auth::{
    AuthorizeUrl, ContextServerAuth, ContextServerAuthStatus, ContextServerCredentials,
    OAuthCallback,
};

pub type OnAuthUpdated = Arc<dyn Fn(ContextServerAuth) + Send + Sync>;

// Constants from MCP spec
const HEADER_SESSION_ID: &str = "Mcp-Session-Id";
const EVENT_STREAM_MIME_TYPE: &str = "text/event-stream";
const JSON_MIME_TYPE: &str = "application/json";

/// HTTP Transport with session management and SSE support
pub struct HttpTransport {
    http_client: Arc<dyn HttpClient>,
    endpoint: String,
    session_id: Arc<SyncMutex<Option<String>>>,
    executor: BackgroundExecutor,
    response_tx: channel::Sender<String>,
    response_rx: channel::Receiver<String>,
    error_tx: channel::Sender<String>,
    error_rx: channel::Receiver<String>,
    headers: HashMap<String, String>,
    on_auth_updated: OnAuthUpdated,
    oauth_client: Arc<Mutex<Option<OAuthClient>>>,
    www_auth_header: Arc<SyncMutex<Option<String>>>,
}

impl HttpTransport {
    pub fn new(
        http_client: Arc<dyn HttpClient>,
        endpoint: String,
        headers: HashMap<String, String>,
        executor: BackgroundExecutor,
        on_auth_updated: OnAuthUpdated,
    ) -> Self {
        let (response_tx, response_rx) = channel::unbounded();
        let (error_tx, error_rx) = channel::unbounded();

        Self {
            http_client,
            executor,
            endpoint,
            session_id: Arc::new(SyncMutex::new(None)),
            response_tx,
            response_rx,
            error_tx,
            error_rx,
            headers,
            on_auth_updated,
            oauth_client: Arc::new(Mutex::new(None)),
            www_auth_header: Arc::new(SyncMutex::new(None)),
        }
    }

    /// Send a message and handle the response based on content type
    async fn send_message(&self, message: String) -> Result<()> {
        let is_notification =
            !message.contains("\"id\":") || message.contains("notifications/initialized");

        let mut request_builder = Request::builder()
            .method(Method::POST)
            .uri(&self.endpoint)
            .header("Content-Type", JSON_MIME_TYPE)
            .header(
                "Accept",
                format!("{}, {}", JSON_MIME_TYPE, EVENT_STREAM_MIME_TYPE),
            );

        for (key, value) in &self.headers {
            request_builder = request_builder.header(key.as_str(), value.as_str());
        }

        let access_token: Option<String> = {
            let mut oauth_client_guard = self.oauth_client.lock().await;
            if let Some(oauth_client) = oauth_client_guard.as_mut() {
                let result = oauth_client.access_token().await;
                match result {
                    Ok(access_token) => {
                        let token = access_token.token.map(|t| t.to_owned());
                        if access_token.refreshed {
                            (self.on_auth_updated)(ContextServerAuth::from_credentials(Some(
                                oauth_client.to_credentials(),
                            )));
                        }
                        token
                    }
                    Err(error) => {
                        (self.on_auth_updated)(ContextServerAuth::from_credentials(Some(
                            oauth_client.to_credentials(),
                        )));
                        return Err(error);
                    }
                }
            } else {
                None
            }
        };

        if let Some(access_token) = access_token {
            request_builder =
                request_builder.header("Authorization", format!("Bearer {}", access_token));
        }

        // Add session ID if we have one (except for initialize)
        if let Some(ref session_id) = *self.session_id.lock() {
            request_builder = request_builder.header(HEADER_SESSION_ID, session_id.as_str());
        }

        let request = request_builder.body(AsyncBody::from(message.into_bytes()))?;
        let mut response = self.http_client.send(request).await?;

        // Handle different response types based on status and content-type
        match response.status() {
            status if status.is_success() => {
                // Check content type
                let content_type = response
                    .headers()
                    .get("content-type")
                    .and_then(|v| v.to_str().ok());

                // Extract session ID from response headers if present
                if let Some(session_id) = response
                    .headers()
                    .get(HEADER_SESSION_ID)
                    .and_then(|v| v.to_str().ok())
                {
                    *self.session_id.lock() = Some(session_id.to_string());
                    log::debug!("Session ID set: {}", session_id);
                }

                match content_type {
                    Some(ct) if ct.starts_with(JSON_MIME_TYPE) => {
                        // JSON response - read and forward immediately
                        let mut body = String::new();
                        futures::AsyncReadExt::read_to_string(response.body_mut(), &mut body)
                            .await?;

                        // Only send non-empty responses
                        if !body.is_empty() {
                            self.response_tx
                                .send(body)
                                .await
                                .map_err(|_| anyhow!("Failed to send JSON response"))?;
                        }
                    }
                    Some(ct) if ct.starts_with(EVENT_STREAM_MIME_TYPE) => {
                        // SSE stream - set up streaming
                        self.setup_sse_stream(response).await?;
                    }
                    _ => {
                        // For notifications, 202 Accepted with no content type is ok
                        if is_notification && status.as_u16() == 202 {
                            log::debug!("Notification accepted");
                        } else {
                            return Err(anyhow!("Unexpected content type: {:?}", content_type));
                        }
                    }
                }
            }
            status if status.as_u16() == 202 => {
                // Accepted - notification acknowledged, no response needed
                log::debug!("Notification accepted");
            }
            status if status.as_u16() == 401 => {
                let www_auth_header = response
                    .headers()
                    .get("WWW-Authenticate")
                    .and_then(|value| Some(value.to_str().ok()?.to_string()));

                let invalid_client = www_auth_header
                    .as_deref()
                    .and_then(WwwAuthenticate::parse)
                    .and_then(|www_auth| www_auth.error)
                    .is_some_and(|error| error.indicates_invalid_client());

                *self.www_auth_header.lock() = www_auth_header;

                let credentials = if invalid_client {
                    self.oauth_client
                        .lock()
                        .await
                        .take_if(|client| client.is_authenticated());
                    None
                } else {
                    self.oauth_client
                        .lock()
                        .await
                        .as_ref()
                        .map(|client| client.to_credentials())
                };

                (self.on_auth_updated)(ContextServerAuth::required(credentials));

                anyhow::bail!("Unauthorized");
            }
            _ => {
                let mut error_body = String::new();
                futures::AsyncReadExt::read_to_string(response.body_mut(), &mut error_body).await?;

                self.error_tx
                    .send(format!("HTTP {}: {}", response.status(), error_body))
                    .await
                    .map_err(|_| anyhow!("Failed to send error"))?;
            }
        }

        Ok(())
    }

    /// Set up SSE streaming from the response
    async fn setup_sse_stream(&self, mut response: Response<AsyncBody>) -> Result<()> {
        let response_tx = self.response_tx.clone();
        let error_tx = self.error_tx.clone();

        // Spawn a task to handle the SSE stream
        smol::spawn(async move {
            let reader = futures::io::BufReader::new(response.body_mut());
            let mut lines = futures::AsyncBufReadExt::lines(reader);

            let mut data_buffer = Vec::new();
            let mut in_message = false;

            while let Some(line_result) = lines.next().await {
                match line_result {
                    Ok(line) => {
                        if line.is_empty() {
                            // Empty line signals end of event
                            if !data_buffer.is_empty() {
                                let message = data_buffer.join("\n");

                                // Filter out ping messages and empty data
                                if !message.trim().is_empty() && message != "ping" {
                                    if let Err(e) = response_tx.send(message).await {
                                        log::error!("Failed to send SSE message: {}", e);
                                        break;
                                    }
                                }
                                data_buffer.clear();
                            }
                            in_message = false;
                        } else if let Some(data) = line.strip_prefix("data: ") {
                            // Handle data lines
                            let data = data.trim();
                            if !data.is_empty() {
                                // Check if this is a ping message
                                if data == "ping" {
                                    log::trace!("Received SSE ping");
                                    continue;
                                }
                                data_buffer.push(data.to_string());
                                in_message = true;
                            }
                        } else if line.starts_with("event:")
                            || line.starts_with("id:")
                            || line.starts_with("retry:")
                        {
                            // Ignore other SSE fields
                            continue;
                        } else if in_message {
                            // Continuation of data
                            data_buffer.push(line);
                        }
                    }
                    Err(e) => {
                        let _ = error_tx.send(format!("SSE stream error: {}", e)).await;
                        break;
                    }
                }
            }
        })
        .detach();

        Ok(())
    }

    pub async fn restore_credentials(&self, credentials: ContextServerCredentials) {
        let mut client_guard = self.oauth_client.lock().await;

        if client_guard.is_some() {
            return;
        }

        let client = OAuthClient::from_credentials(credentials, &self.http_client);
        client_guard.replace(client);
    }

    pub async fn start_auth(&self) -> Result<AuthorizeUrl> {
        let mut client_guard = self.oauth_client.lock().await;

        let www_auth_header = self.www_auth_header.lock().take();
        let www_authenticate = www_auth_header.as_deref().and_then(WwwAuthenticate::parse);

        let client = match client_guard.as_mut() {
            Some(client) => client,
            None => {
                let new_client =
                    OAuthClient::init(&self.endpoint, www_authenticate.as_ref(), &self.http_client)
                        .await?;
                client_guard.replace(new_client);
                client_guard.as_mut().unwrap()
            }
        };

        let url = client.authorize_url()?;

        (self.on_auth_updated)(ContextServerAuth::from_credentials(Some(
            client.to_credentials(),
        )));

        Ok(url)
    }

    pub async fn handle_oauth_callback(&self, callback: &OAuthCallback) -> Result<()> {
        let mut client_guard = self.oauth_client.lock().await;
        let client = match client_guard.as_mut() {
            Some(client) => client,
            None => return Err(anyhow!("oauth client is not initialized; start auth first")),
        };

        client.exchange_token(&callback.code).await?;

        (self.on_auth_updated)(ContextServerAuth::from_credentials(Some(
            client.to_credentials(),
        )));

        Ok(())
    }

    pub async fn logout(&self) {
        let mut client_guard = self.oauth_client.lock().await;
        if let Some(client) = client_guard.as_mut() {
            client.logout();
            (self.on_auth_updated)(ContextServerAuth::from_credentials(Some(
                client.to_credentials(),
            )));
        }
    }
}

#[async_trait]
impl Transport for HttpTransport {
    async fn send(&self, message: String) -> Result<()> {
        self.send_message(message).await
    }

    fn receive(&self) -> Pin<Box<dyn Stream<Item = String> + Send>> {
        Box::pin(self.response_rx.clone())
    }

    fn receive_err(&self) -> Pin<Box<dyn Stream<Item = String> + Send>> {
        Box::pin(self.error_rx.clone())
    }
}

impl Drop for HttpTransport {
    fn drop(&mut self) {
        // Try to cleanup session on drop
        let http_client = self.http_client.clone();
        let endpoint = self.endpoint.clone();
        let session_id = self.session_id.lock().clone();
        let headers = self.headers.clone();

        if let Some(session_id) = session_id {
            self.executor
                .spawn(async move {
                    let mut request_builder = Request::builder()
                        .method(Method::DELETE)
                        .uri(&endpoint)
                        .header(HEADER_SESSION_ID, &session_id);

                    // Add authentication headers if present
                    for (key, value) in headers {
                        request_builder = request_builder.header(key.as_str(), value.as_str());
                    }

                    let request = request_builder.body(AsyncBody::empty());

                    if let Ok(request) = request {
                        let _ = http_client.send(request).await;
                    }
                })
                .detach();
        }
    }
}

```


**Tool Call: git --no-pager show origin/mcp-auth:crates/context_server/src/transport/http/www_authenticate.rs**
Status: Completed

Terminal:
```
use std::borrow::Cow;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct WwwAuthenticate<'a> {
    pub realm: Option<Cow<'a, str>>,
    pub scope: Option<Cow<'a, str>>,
    pub error: Option<BearerError>,
    pub error_description: Option<Cow<'a, str>>,
    pub error_uri: Option<Cow<'a, str>>,
    pub resource_metadata: Option<Cow<'a, str>>,
}

const BEARER_SCHEME: &str = "Bearer";

impl<'a> WwwAuthenticate<'a> {
    pub fn parse(input: &'a str) -> Option<Self> {
        // Header format (simplified):
        //   Bearer realm="example", error="invalid_token", error_description="...", error_uri="..."
        let input = input.trim_ascii_start();

        let (scheme, mut input) = input
            .trim_start()
            .split_once(|c: char| c.is_ascii_whitespace())
            .unwrap_or((input, ""));

        // We only parse Bearer challenges as defined by RFC 6750 section 3.
        if !scheme.eq_ignore_ascii_case(BEARER_SCHEME) {
            return None;
        }

        let mut challenge = Self::default();

        loop {
            input = input.trim_ascii_start();

            if input.is_empty() {
                break;
            }

            // Stop at a subsequent Bearer challenge in a combined header.
            if let Some(sub) = input.strip_prefix(BEARER_SCHEME)
                && sub
                    .chars()
                    .next()
                    .is_some_and(|character| character.is_ascii_whitespace())
            {
                break;
            }

            let (name, rest) = parse_token(input)?;
            let mut rest = rest.trim_ascii_start();

            rest = rest.strip_prefix('=')?.trim_ascii_start();

            let (value, rest) = parse_value(rest)?;
            input = rest;

            match name {
                "realm" => challenge.realm = Some(value),
                "scope" => challenge.scope = Some(value),
                "error" => challenge.error = Some(BearerError::parse(&value)),
                "error_description" => challenge.error_description = Some(value),
                "error_uri" => {
                    challenge.error_uri = Some(value);
                }
                "resource_metadata" => {
                    challenge.resource_metadata = Some(value);
                }
                _ => {
                    // Ignore extension auth-params.
                }
            }

            input = input.trim_start();
            if let Some(after_comma) = input.strip_prefix(',') {
                input = after_comma;
            } else {
                // If there's no comma, we either reached the end or encountered something invalid.
                if !input.is_empty() {
                    return None;
                }
            }
        }

        Some(challenge)
    }
}

/// Error codes defined by RFC 6750 Section 3.1 for Bearer token authentication.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BearerError {
    /// The request is missing a required parameter, includes an unsupported parameter
    /// or parameter value, repeats the same parameter, uses more than one method for
    /// including an access token, or is otherwise malformed.
    InvalidRequest,
    /// The access token provided is expired, revoked, malformed, or invalid for other reasons.
    InvalidToken,
    /// The request requires higher privileges than provided by the access token.
    InsufficientScope,
    /// An unrecognized error code (extension or future spec addition).
    Other,
}

impl BearerError {
    fn parse(value: &str) -> Self {
        match value {
            "invalid_request" => BearerError::InvalidRequest,
            "invalid_token" => BearerError::InvalidToken,
            "insufficient_scope" => BearerError::InsufficientScope,
            _ => BearerError::Other,
        }
    }

    /// Returns true if the error indicates the OAuth client registration may be invalid
    /// and should be discarded to force re-registration.
    pub fn indicates_invalid_client(&self) -> bool {
        match self {
            BearerError::InvalidRequest => false,
            BearerError::InsufficientScope => false,
            BearerError::InvalidToken => true,
            BearerError::Other => true,
        }
    }
}

fn parse_token(input: &str) -> Option<(&str, &str)> {
    let bytes = input.as_bytes();
    let mut end = 0;

    while end < bytes.len() && is_tchar(bytes[end]) {
        end += 1;
    }

    if end == 0 {
        return None;
    }

    Some((&input[..end], &input[end..]))
}

fn is_tchar(byte: u8) -> bool {
    matches!(
        byte,
        b'!' | b'#' | b'$' | b'%' | b'&' | b'\'' | b'*' | b'+' | b'-' | b'.' | b'^' | b'_' | b'`' | b'|' | b'~'
            | b'0'..=b'9'
            | b'A'..=b'Z'
            | b'a'..=b'z'
    )
}

fn parse_value<'a>(input: &'a str) -> Option<(Cow<'a, str>, &'a str)> {
    if let Some(rest) = input.strip_prefix('"') {
        parse_quoted_value(rest)
    } else {
        let (token, rest) = parse_token(input)?;
        Some((Cow::Borrowed(token), rest))
    }
}

fn parse_quoted_value<'a>(input: &'a str) -> Option<(Cow<'a, str>, &'a str)> {
    let mut output: Option<String> = None;
    let mut segment_start: usize = 0;

    let mut iter = input.as_bytes().iter().enumerate();

    while let Some((index, byte)) = iter.next() {
        match byte {
            b'"' => {
                let remainder = &input[index + 1..];

                if let Some(mut output) = output {
                    output.push_str(&input[segment_start..index]);
                    return Some((Cow::Owned(output), remainder));
                }

                return Some((Cow::Borrowed(&input[..index]), remainder));
            }
            b'\\' => {
                let (escaped_index, escaped_byte) = iter.next()?;

                let output = output.get_or_insert_with(String::new);
                output.push_str(&input[segment_start..index]);
                output.push(*escaped_byte as char);

                segment_start = escaped_index + 1;
            }
            _ => {}
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_empty_bearer_challenge() {
        let challenge = WwwAuthenticate::parse("Bearer").expect("should parse Bearer scheme");
        assert_eq!(challenge, WwwAuthenticate::default());
    }

    #[test]
    fn rejects_non_bearer_scheme() {
        assert!(WwwAuthenticate::parse("Basic realm=\"example\"").is_none());
        assert!(WwwAuthenticate::parse("Digest realm=\"example\"").is_none());
    }

    #[test]
    fn parses_known_parameters_with_quoted_strings_and_tokens() {
        let challenge = WwwAuthenticate::parse(
            "Bearer realm=\"example\", scope=\"read write\", error=invalid_token, error_description=\"The access token expired\"",
        )
        .expect("should parse");

        assert_eq!(
            challenge,
            WwwAuthenticate {
                realm: Some(Cow::Borrowed("example")),
                scope: Some(Cow::Borrowed("read write")),
                error: Some(BearerError::InvalidToken),
                error_description: Some(Cow::Borrowed("The access token expired")),
                ..Default::default()
            }
        );
    }

    #[test]
    fn quoted_string_allows_commas_and_backslash_escapes() {
        let challenge = WwwAuthenticate::parse(
            "Bearer error_description=\"contains, comma and a quote: \\\" and a backslash: \\\\\"",
        )
        .expect("should parse");

        assert_eq!(
            challenge,
            WwwAuthenticate {
                error_description: Some(Cow::Owned(
                    "contains, comma and a quote: \" and a backslash: \\".to_string()
                )),
                ..Default::default()
            }
        );
    }

    #[test]
    fn ignores_unknown_extension_parameters() {
        let challenge =
            WwwAuthenticate::parse("Bearer realm=\"example\", foo=\"bar\"").expect("should parse");

        assert_eq!(
            challenge,
            WwwAuthenticate {
                realm: Some(Cow::Borrowed("example")),
                ..Default::default()
            }
        );
    }

    #[test]
    fn stops_at_subsequent_bearer_challenge_in_combined_header_value() {
        let challenge = WwwAuthenticate::parse(
            "Bearer realm=\"first\", error=\"invalid_token\", Bearer realm=\"second\"",
        )
        .expect("should parse");

        assert_eq!(
            challenge,
            WwwAuthenticate {
                realm: Some(Cow::Borrowed("first")),
                error: Some(BearerError::InvalidToken),
                ..Default::default()
            }
        );
    }

    #[test]
    fn parses_all_standard_error_codes() {
        let invalid_request =
            WwwAuthenticate::parse("Bearer error=invalid_request").expect("should parse");
        assert_eq!(invalid_request.error, Some(BearerError::InvalidRequest));

        let invalid_token =
            WwwAuthenticate::parse("Bearer error=invalid_token").expect("should parse");
        assert_eq!(invalid_token.error, Some(BearerError::InvalidToken));

        let insufficient_scope =
            WwwAuthenticate::parse("Bearer error=insufficient_scope").expect("should parse");
        assert_eq!(
            insufficient_scope.error,
            Some(BearerError::InsufficientScope)
        );
    }

    #[test]
    fn parses_unknown_error_as_other() {
        let challenge =
            WwwAuthenticate::parse("Bearer error=some_future_error").expect("should parse");
        assert_eq!(challenge.error, Some(BearerError::Other));
    }

    #[test]
    fn indicates_invalid_client_for_appropriate_errors() {
        assert!(!BearerError::InvalidRequest.indicates_invalid_client());
        assert!(!BearerError::InsufficientScope.indicates_invalid_client());
        assert!(BearerError::InvalidToken.indicates_invalid_client());
        assert!(BearerError::Other.indicates_invalid_client());
    }

    #[test]
    fn returns_none_on_invalid_trailing_garbage() {
        assert!(WwwAuthenticate::parse("Bearer realm=\"example\" garbage").is_none());
    }

    #[test]
    fn returns_none_on_missing_equals() {
        assert!(WwwAuthenticate::parse("Bearer realm \"example\"").is_none());
    }

    #[test]
    fn returns_none_on_unterminated_quoted_string() {
        assert!(WwwAuthenticate::parse("Bearer realm=\"example").is_none());
    }
}

```


## Assistant

Now let me look at the mcp-auth branch's store and UI changes:

**Tool Call: git --no-pager diff f1fd0ab529..origin/mcp-auth -- crates/project/src/context_server_store.rs**
Status: Completed

Terminal:
```
diff --git a/crates/project/src/context_server_store.rs b/crates/project/src/context_server_store.rs
index 05a8ee243f..4e3ede1176 100644
--- a/crates/project/src/context_server_store.rs
+++ b/crates/project/src/context_server_store.rs
@@ -6,11 +6,19 @@ use std::time::Duration;

 use anyhow::{Context as _, Result};
 use collections::{HashMap, HashSet};
-use context_server::{ContextServer, ContextServerCommand, ContextServerId};
-use futures::{FutureExt as _, future::join_all};
-use gpui::{App, AsyncApp, Context, Entity, EventEmitter, Subscription, Task, WeakEntity, actions};
+use context_server::{
+    ContextServer, ContextServerAuth, ContextServerAuthStatus, ContextServerCommand,
+    ContextServerId, transport::ContextServerCredentials,
+};
+use credentials_provider::CredentialsProvider;
+use futures::{FutureExt as _, channel::mpsc, future::join_all};
+use gpui::{
+    App, AppContext as _, AsyncApp, Context, Entity, EventEmitter, Subscription, Task, WeakEntity,
+    actions,
+};
 use registry::ContextServerDescriptorRegistry;
 use settings::{Settings as _, SettingsStore};
+use smol::stream::StreamExt;
 use util::{ResultExt as _, rel_path::RelPath};

 use crate::{
@@ -38,7 +46,7 @@ actions!(
 #[derive(Debug, Clone, PartialEq, Eq, Hash)]
 pub enum ContextServerStatus {
     Starting,
-    Running,
+    Running(ContextServerAuthStatus),
     Stopped,
     Error(Arc<str>),
 }
@@ -47,7 +55,9 @@ impl ContextServerStatus {
     fn from_state(state: &ContextServerState) -> Self {
         match state {
             ContextServerState::Starting { .. } => ContextServerStatus::Starting,
-            ContextServerState::Running { .. } => ContextServerStatus::Running,
+            ContextServerState::Running { auth_status, .. } => {
+                ContextServerStatus::Running(auth_status.clone())
+            }
             ContextServerState::Stopped { .. } => ContextServerStatus::Stopped,
             ContextServerState::Error { error, .. } => ContextServerStatus::Error(error.clone()),
         }
@@ -63,6 +73,7 @@ enum ContextServerState {
     Running {
         server: Arc<ContextServer>,
         configuration: Arc<ContextServerConfiguration>,
+        auth_status: ContextServerAuthStatus,
     },
     Stopped {
         server: Arc<ContextServer>,
@@ -180,6 +191,8 @@ pub struct ContextServerStore {
     update_servers_task: Option<Task<Result<()>>>,
     context_server_factory: Option<ContextServerFactory>,
     needs_server_update: bool,
+    auth_updated_tx: mpsc::UnboundedSender<AuthUpdatedEvent>,
+    _auth_updated_task: Task<()>,
     _subscriptions: Vec<Subscription>,
 }

@@ -190,6 +203,14 @@ pub enum Event {
     },
 }

+#[derive(Debug, Clone)]
+pub struct AuthUpdatedEvent {
+    pub server_id: ContextServerId,
+    pub auth: ContextServerAuth,
+}
+
+const KEYCHAIN_USER: &str = "mcp";
+
 impl EventEmitter<Event> for ContextServerStore {}

 impl ContextServerStore {
@@ -295,6 +316,13 @@ impl ContextServerStore {
             Vec::new()
         };

+        let (auth_updated_tx, mut auth_updated_rx) = mpsc::unbounded::<AuthUpdatedEvent>();
+        let auth_updated_task = cx.spawn(async move |this, cx| {
+            while let Some(event) = auth_updated_rx.next().await {
+                Self::handle_auth_updated(this.clone(), event, cx).await;
+            }
+        });
+
         let mut this = Self {
             _subscriptions: subscriptions,
             context_server_settings: Self::resolve_project_settings(&worktree_store, cx)
@@ -307,6 +335,8 @@ impl ContextServerStore {
             servers: HashMap::default(),
             update_servers_task: None,
             context_server_factory,
+            auth_updated_tx,
+            _auth_updated_task: auth_updated_task,
         };
         if maintain_server_loop {
             this.available_context_servers_changed(cx);
@@ -444,13 +474,25 @@ impl ContextServerStore {
         ) {
             self.stop_server(&id, cx).log_err();
         }
+
+        let credentials_provider = <dyn CredentialsProvider>::global(cx);
+
         let task = cx.spawn({
             let id = server.id();
             let server = server.clone();
             let configuration = configuration.clone();

             async move |this, cx| {
-                match server.clone().start(cx).await {
+                let credentials = match configuration.as_ref() {
+                    ContextServerConfiguration::Http { url, .. } => {
+                        Self::load_credentials_from_keychain(url.as_str(), credentials_provider, cx)
+                            .await
+                    }
+                    ContextServerConfiguration::Custom { .. }
+                    | ContextServerConfiguration::Extension { .. } => None,
+                };
+
+                match server.clone().start(credentials, cx).await {
                     Ok(_) => {
                         debug_assert!(server.client().is_some());

@@ -460,6 +502,7 @@ impl ContextServerStore {
                                 ContextServerState::Running {
                                     server,
                                     configuration,
+                                    auth_status: ContextServerAuthStatus::None,
                                 },
                                 cx,
                             )
@@ -527,16 +570,32 @@ impl ContextServerStore {
                 url,
                 headers,
                 timeout,
-            } => Ok(Arc::new(ContextServer::http(
-                id,
-                url,
-                headers.clone(),
-                cx.http_client(),
-                cx.background_executor().clone(),
-                Some(Duration::from_secs(
-                    timeout.unwrap_or(global_timeout).min(MAX_TIMEOUT_SECS),
-                )),
-            )?)),
+            } => {
+                let on_auth_updated = {
+                    let auth_updated_tx = self.auth_updated_tx.clone();
+                    let id = id.clone();
+
+                    Arc::new(move |auth: ContextServerAuth| {
+                        auth_updated_tx
+                            .unbounded_send(AuthUpdatedEvent {
+                                server_id: id.clone(),
+                                auth,
+                            })
+                            .log_err();
+                    })
+                };
+                Ok(Arc::new(ContextServer::http(
+                    id,
+                    url,
+                    headers.clone(),
+                    cx.http_client(),
+                    cx.background_executor().clone(),
+                    on_auth_updated,
+                    Some(Duration::from_secs(
+                        timeout.unwrap_or(global_timeout).min(MAX_TIMEOUT_SECS),
+                    )),
+                )?))
+            }
             _ => {
                 let root_path = self
                     .project
@@ -705,6 +764,168 @@ impl ContextServerStore {
             anyhow::Ok(())
         })?
     }
+
+    async fn handle_auth_updated(
+        this: WeakEntity<Self>,
+        event: AuthUpdatedEvent,
+        cx: &mut AsyncApp,
+    ) {
+        let Some((endpoint_url, credentials_provider)) = this
+            .update(cx, |this, cx| {
+                let Some(state) = this.servers.get(&event.server_id) else {
+                    return None;
+                };
+
+                let ContextServerState::Running {
+                    server,
+                    configuration,
+                    ..
+                } = state
+                else {
+                    return None;
+                };
+
+                let ContextServerConfiguration::Http { url, .. } = configuration.as_ref() else {
+                    return None;
+                };
+
+                let endpoint_url = url.as_str().to_owned();
+
+                this.update_server_state(
+                    event.server_id.clone(),
+                    ContextServerState::Running {
+                        server: server.clone(),
+                        configuration: configuration.clone(),
+                        auth_status: event.auth.status.clone(),
+                    },
+                    cx,
+                );
+
+                Some((endpoint_url, <dyn CredentialsProvider>::global(cx)))
+            })
+            .ok()
+            .flatten()
+        else {
+            return;
+        };
+
+        match event.auth.credentials {
+            Some(persisted) => {
+                if let Some(json) = cx
+                    .background_spawn(async move { serde_json::to_vec(&persisted) })
+                    .await
+                    .log_err()
+                {
+                    credentials_provider
+                        .write_credentials(&endpoint_url, KEYCHAIN_USER, &json, cx)
+                        .await
+                        .log_err();
+                }
+            }
+            None => {
+                credentials_provider
+                    .delete_credentials(&endpoint_url, cx)
+                    .await
+                    .log_err();
+            }
+        }
+    }
+
+    async fn load_credentials_from_keychain(
+        endpoint_url: &str,
+        credentials_provider: Arc<dyn CredentialsProvider>,
+        cx: &AsyncApp,
+    ) -> Option<ContextServerCredentials> {
+        let Some((username, data)) = credentials_provider
+            .read_credentials(endpoint_url, cx)
+            .await
+            .log_err()?
+        else {
+            return None;
+        };
+
+        if username != KEYCHAIN_USER {
+            return None;
+        }
+
+        serde_json::from_slice(&data).log_err()
+    }
+
+    pub fn start_auth(&self, server_id: ContextServerId, cx: &mut Context<ContextServerStore>) {
+        let Some(state) = self.servers.get(&server_id) else {
+            return;
+        };
+
+        let ContextServerState::Running {
+            server,
+            auth_status: ContextServerAuthStatus::Required,
+            ..
+        } = state
+        else {
+            return;
+        };
+
+        let server = server.clone();
+
+        cx.spawn(async move |_, cx| match server.start_auth().await {
+            Ok(auth_url) => {
+                let url = auth_url.url(server_id);
+                cx.update(|cx| {
+                    cx.open_url(url.as_str());
+                });
+            }
+            Err(err) => {
+                log::error!(
+                    "{} context server failed to authenticate: {}",
+                    server_id,
+                    err
+                );
+            }
+        })
+        .detach();
+    }
+
+    pub fn logout(&self, server_id: ContextServerId, cx: &mut Context<Self>) {
+        let Some(server) = self.get_server(&server_id) else {
+            return;
+        };
+
+        cx.spawn(async move |_, _cx| {
+            server.logout().await;
+        })
+        .detach();
+    }
+
+    pub fn handle_oauth_callback(
+        &mut self,
+        callback: context_server::OAuthCallback,
+        cx: &mut Context<Self>,
+    ) {
+        let server = match self.get_server(&callback.server_id) {
+            Some(server) => server,
+            None => {
+                log::error!(
+                    "got MCP OAuth callback for unknown context server {}",
+                    callback.server_id
+                );
+                return;
+            }
+        };
+
+        cx.spawn(async move |_, _cx| {
+            server
+                .handle_oauth_callback(&callback)
+                .await
+                .with_context(|| {
+                    format!(
+                        "failed to handle MCP OAuth callback for {}",
+                        callback.server_id
+                    )
+                })
+                .log_err();
+        })
+        .detach();
+    }
 }

 #[cfg(test)]
@@ -757,7 +978,7 @@ mod tests {
         cx.update(|cx| {
             assert_eq!(
                 store.read(cx).status_for_server(&server_1_id),
-                Some(ContextServerStatus::Running)
+                Some(ContextServerStatus::Running(ContextServerAuthStatus::None))
             );
             assert_eq!(store.read(cx).status_for_server(&server_2_id), None);
         });
@@ -771,11 +992,11 @@ mod tests {
         cx.update(|cx| {
             assert_eq!(
                 store.read(cx).status_for_server(&server_1_id),
-                Some(ContextServerStatus::Running)
+                Some(ContextServerStatus::Running(ContextServerAuthStatus::None))
             );
             assert_eq!(
                 store.read(cx).status_for_server(&server_2_id),
-                Some(ContextServerStatus::Running)
+                Some(ContextServerStatus::Running(ContextServerAuthStatus::None))
             );
         });

@@ -786,7 +1007,7 @@ mod tests {
         cx.update(|cx| {
             assert_eq!(
                 store.read(cx).status_for_server(&server_1_id),
-                Some(ContextServerStatus::Running)
+                Some(ContextServerStatus::Running(ContextServerAuthStatus::None))
             );
             assert_eq!(
                 store.read(cx).status_for_server(&server_2_id),
@@ -828,9 +1049,15 @@ mod tests {
             &store,
             vec![
                 (server_1_id.clone(), ContextServerStatus::Starting),
-                (server_1_id, ContextServerStatus::Running),
+                (
+                    server_1_id,
+                    ContextServerStatus::Running(ContextServerAuthStatus::None),
+                ),
                 (server_2_id.clone(), ContextServerStatus::Starting),
-                (server_2_id.clone(), ContextServerStatus::Running),
+                (
+                    server_2_id.clone(),
+                    ContextServerStatus::Running(ContextServerAuthStatus::None),
+                ),
                 (server_2_id.clone(), ContextServerStatus::Stopped),
             ],
             cx,
@@ -885,7 +1112,10 @@ mod tests {
                 (server_id.clone(), ContextServerStatus::Starting),
                 (server_id.clone(), ContextServerStatus::Stopped),
                 (server_id.clone(), ContextServerStatus::Starting),
-                (server_id.clone(), ContextServerStatus::Running),
+                (
+                    server_id.clone(),
+                    ContextServerStatus::Running(ContextServerAuthStatus::None),
+                ),
             ],
             cx,
         );
@@ -902,7 +1132,7 @@ mod tests {
         cx.update(|cx| {
             assert_eq!(
                 store.read(cx).status_for_server(&server_id),
-                Some(ContextServerStatus::Running)
+                Some(ContextServerStatus::Running(ContextServerAuthStatus::None))
             );
         });
     }
@@ -956,7 +1186,10 @@ mod tests {
                 &store,
                 vec![
                     (server_1_id.clone(), ContextServerStatus::Starting),
-                    (server_1_id.clone(), ContextServerStatus::Running),
+                    (
+                        server_1_id.clone(),
+                        ContextServerStatus::Running(ContextServerAuthStatus::None),
+                    ),
                 ],
                 cx,
             );
@@ -970,7 +1203,10 @@ mod tests {
                 vec![
                     (server_1_id.clone(), ContextServerStatus::Stopped),
                     (server_1_id.clone(), ContextServerStatus::Starting),
-                    (server_1_id.clone(), ContextServerStatus::Running),
+                    (
+                        server_1_id.clone(),
+                        ContextServerStatus::Running(ContextServerAuthStatus::None),
+                    ),
                 ],
                 cx,
             );
@@ -1015,7 +1251,10 @@ mod tests {
                 &store,
                 vec![
                     (server_2_id.clone(), ContextServerStatus::Starting),
-                    (server_2_id.clone(), ContextServerStatus::Running),
+                    (
+                        server_2_id.clone(),
+                        ContextServerStatus::Running(ContextServerAuthStatus::None),
+                    ),
                 ],
                 cx,
             );
@@ -1056,7 +1295,10 @@ mod tests {
                 vec![
                     (server_2_id.clone(), ContextServerStatus::Stopped),
                     (server_2_id.clone(), ContextServerStatus::Starting),
-                    (server_2_id.clone(), ContextServerStatus::Running),
+                    (
+                        server_2_id.clone(),
+                        ContextServerStatus::Running(ContextServerAuthStatus::None),
+                    ),
                 ],
                 cx,
             );
@@ -1138,7 +1380,7 @@ mod tests {
             cx.update(|cx| {
                 assert_eq!(
                     store.read(cx).status_for_server(&server_1_id),
-                    Some(ContextServerStatus::Running)
+                    Some(ContextServerStatus::Running(ContextServerAuthStatus::None))
                 );
                 assert_eq!(store.read(cx).status_for_server(&server_2_id), None);
             });
@@ -1186,7 +1428,10 @@ mod tests {
                 &store,
                 vec![
                     (server_1_id.clone(), ContextServerStatus::Starting),
-                    (server_1_id.clone(), ContextServerStatus::Running),
+                    (
+                        server_1_id.clone(),
+                        ContextServerStatus::Running(ContextServerAuthStatus::None),
+                    ),
                 ],
                 cx,
             );
@@ -1225,7 +1470,10 @@ mod tests {
                 &store,
                 vec![
                     (server_1_id.clone(), ContextServerStatus::Starting),
-                    (server_1_id.clone(), ContextServerStatus::Running),
+                    (
+                        server_1_id.clone(),
+                        ContextServerStatus::Running(ContextServerAuthStatus::None),
+                    ),
                 ],
                 cx,
             );
@@ -1318,7 +1566,10 @@ mod tests {
             &store,
             vec![
                 (server_id.clone(), ContextServerStatus::Starting),
-                (server_id.clone(), ContextServerStatus::Running),
+                (
+                    server_id.clone(),
+                    ContextServerStatus::Running(ContextServerAuthStatus::None),
+                ),
             ],
             cx,
         );

```


**Tool Call: git --no-pager diff f1fd0ab529..origin/mcp-auth -- crates/agent_ui/src/agent_configuration.rs | head -300**
Status: Completed

Terminal:
```
diff --git a/crates/agent_ui/src/agent_configuration.rs b/crates/agent_ui/src/agent_configuration.rs
index 4e10dc0478..cee561b1ae 100644
--- a/crates/agent_ui/src/agent_configuration.rs
+++ b/crates/agent_ui/src/agent_configuration.rs
@@ -11,7 +11,7 @@ use anyhow::Result;
 use client::zed_urls;
 use cloud_llm_client::{Plan, PlanV1, PlanV2};
 use collections::HashMap;
-use context_server::ContextServerId;
+use context_server::{ContextServerAuthStatus, ContextServerId};
 use editor::{Editor, MultiBufferOffset, SelectionEffects, scroll::Autoscroll};
 use extension::ExtensionManifest;
 use extension_host::ExtensionStore;
@@ -653,7 +653,11 @@ impl AgentConfiguration {
             .read(cx)
             .configuration_for_server(&context_server_id);

-        let is_running = matches!(server_status, ContextServerStatus::Running);
+        let is_running = matches!(server_status, ContextServerStatus::Running(_));
+        let is_authenticated = matches!(
+            server_status,
+            ContextServerStatus::Running(ContextServerAuthStatus::Authenticated)
+        );
         let item_id = SharedString::from(context_server_id.0.clone());
         // Servers without a configuration can only be provided by extensions.
         let provided_by_extension = server_configuration.as_ref().is_none_or(|config| {
@@ -699,10 +703,27 @@ impl AgentConfiguration {
                     .into_any_element(),
                 "Server is starting.",
             ),
-            ContextServerStatus::Running => (
-                Indicator::dot().color(Color::Success).into_any_element(),
-                "Server is active.",
-            ),
+            ContextServerStatus::Running(ref auth_status) => match auth_status {
+                ContextServerAuthStatus::Required => (
+                    Indicator::dot().color(Color::Warning).into_any_element(),
+                    "Authentication required",
+                ),
+                ContextServerAuthStatus::AwaitingAuthorization => (
+                    Icon::new(IconName::LoadCircle)
+                        .size(IconSize::XSmall)
+                        .color(Color::Accent)
+                        .with_keyed_rotate_animation(
+                            SharedString::from(format!("{}-awaiting-auth", context_server_id.0)),
+                            3,
+                        )
+                        .into_any_element(),
+                    "Waiting for auth...",
+                ),
+                _ => (
+                    Indicator::dot().color(Color::Success).into_any_element(),
+                    "Server is active.",
+                ),
+            },
             ContextServerStatus::Error(_) => (
                 Indicator::dot().color(Color::Error).into_any_element(),
                 "Server has an error.",
@@ -730,6 +751,7 @@ impl AgentConfiguration {
                 let language_registry = self.language_registry.clone();
                 let workspace = self.workspace.clone();
                 let context_server_registry = self.context_server_registry.clone();
+                let context_server_store = self.context_server_store.clone();

                 move |window, cx| {
                     Some(ContextMenu::build(window, cx, |menu, _window, _cx| {
@@ -776,6 +798,17 @@ impl AgentConfiguration {
                                 .ok();
                             }
                         }))
+                        .when(is_remote && is_authenticated, |menu| {
+                            menu.entry("Log Out", None, {
+                                let context_server_id = context_server_id.clone();
+                                let context_server_store = context_server_store.clone();
+                                move |_window, cx| {
+                                    context_server_store.update(cx, |store, cx| {
+                                        store.logout(context_server_id.clone(), cx);
+                                    });
+                                }
+                            })
+                        })
                         .separator()
                         .entry("Uninstall", None, {
                             let fs = fs.clone();
@@ -832,6 +865,63 @@ impl AgentConfiguration {
                 }
             });

+        let action = if matches!(
+            server_status,
+            ContextServerStatus::Running(ContextServerAuthStatus::Required { .. })
+        ) {
+            Button::new("context-server-authenticate", "Authenticate")
+                .style(ButtonStyle::Filled)
+                .on_click(cx.listener(move |this, _event, _window, cx| {
+                    this.context_server_store.update(cx, |store, cx| {
+                        store.start_auth(context_server_id.clone(), cx);
+                    });
+                }))
+                .into_any_element()
+        } else {
+            Switch::new("context-server-switch", is_running.into())
+                .on_click({
+                    let context_server_manager = self.context_server_store.clone();
+                    let fs = self.fs.clone();
+
+                    move |state, _window, cx| {
+                        let is_enabled = match state {
+                            ToggleState::Unselected | ToggleState::Indeterminate => {
+                                context_server_manager.update(cx, |this, cx| {
+                                    this.stop_server(&context_server_id, cx).log_err();
+                                });
+                                false
+                            }
+                            ToggleState::Selected => {
+                                context_server_manager.update(cx, |this, cx| {
+                                    if let Some(server) = this.get_server(&context_server_id) {
+                                        this.start_server(server, cx);
+                                    }
+                                });
+                                true
+                            }
+                        };
+                        update_settings_file(fs.clone(), cx, {
+                            let context_server_id = context_server_id.clone();
+
+                            move |settings, _| {
+                                settings
+                                    .project
+                                    .context_servers
+                                    .entry(context_server_id.0)
+                                    .or_insert_with(|| {
+                                        settings::ContextServerSettingsContent::Extension {
+                                            enabled: is_enabled,
+                                            settings: serde_json::json!({}),
+                                        }
+                                    })
+                                    .set_enabled(is_enabled);
+                            }
+                        });
+                    }
+                })
+                .into_any_element()
+        };
+
         v_flex()
             .id(item_id.clone())
             .child(
@@ -882,53 +972,7 @@ impl AgentConfiguration {
                             .gap_0p5()
                             .flex_none()
                             .child(context_server_configuration_menu)
-                            .child(
-                            Switch::new("context-server-switch", is_running.into())
-                                .on_click({
-                                    let context_server_manager = self.context_server_store.clone();
-                                    let fs = self.fs.clone();
-
-                                    move |state, _window, cx| {
-                                        let is_enabled = match state {
-                                            ToggleState::Unselected
-                                            | ToggleState::Indeterminate => {
-                                                context_server_manager.update(cx, |this, cx| {
-                                                    this.stop_server(&context_server_id, cx)
-                                                        .log_err();
-                                                });
-                                                false
-                                            }
-                                            ToggleState::Selected => {
-                                                context_server_manager.update(cx, |this, cx| {
-                                                    if let Some(server) =
-                                                        this.get_server(&context_server_id)
-                                                    {
-                                                        this.start_server(server, cx);
-                                                    }
-                                                });
-                                                true
-                                            }
-                                        };
-                                        update_settings_file(fs.clone(), cx, {
-                                            let context_server_id = context_server_id.clone();
-
-                                            move |settings, _| {
-                                                settings
-                                                    .project
-                                                    .context_servers
-                                                    .entry(context_server_id.0)
-                                                    .or_insert_with(|| {
-                                                        settings::ContextServerSettingsContent::Extension {
-                                                            enabled: is_enabled,
-                                                            settings: serde_json::json!({}),
-                                                        }
-                                                    })
-                                                    .set_enabled(is_enabled);
-                                            }
-                                        });
-                                    }
-                                }),
-                        ),
+                            .child(action),
                     ),
             )
             .map(|parent| {

```


**Tool Call: git --no-pager diff f1fd0ab529..origin/mcp-auth -- crates/context_server/src/context_server.rs**
Status: Completed

Terminal:
```
diff --git a/crates/context_server/src/context_server.rs b/crates/context_server/src/context_server.rs
index 40d372db68..1273dced38 100644
--- a/crates/context_server/src/context_server.rs
+++ b/crates/context_server/src/context_server.rs
@@ -15,12 +15,16 @@ use std::{fmt::Display, path::PathBuf};

 use anyhow::Result;
 use client::Client;
-use gpui::AsyncApp;
+use gpui::{AsyncApp, BackgroundExecutor};
 use parking_lot::RwLock;
 pub use settings::ContextServerCommand;
 use url::Url;

-use crate::transport::HttpTransport;
+use crate::transport::{ContextServerCredentials, HttpTransport, OnAuthUpdated};
+
+pub use crate::transport::http::{
+    AuthorizeUrl, ContextServerAuth, ContextServerAuthStatus, OAuthCallback,
+};

 #[derive(Debug, Clone, PartialEq, Eq, Hash)]
 pub struct ContextServerId(pub Arc<str>);
@@ -33,6 +37,7 @@ impl Display for ContextServerId {

 enum ContextServerTransport {
     Stdio(ContextServerCommand, Option<PathBuf>),
+    Http(Arc<HttpTransport>),
     Custom(Arc<dyn crate::transport::Transport>),
 }

@@ -65,19 +70,30 @@ impl ContextServer {
         endpoint: &Url,
         headers: HashMap<String, String>,
         http_client: Arc<dyn HttpClient>,
-        executor: gpui::BackgroundExecutor,
+        executor: BackgroundExecutor,
+        on_auth_updated: OnAuthUpdated,
         request_timeout: Option<Duration>,
     ) -> Result<Self> {
         let transport = match endpoint.scheme() {
             "http" | "https" => {
                 log::info!("Using HTTP transport for {}", endpoint);
-                let transport =
-                    HttpTransport::new(http_client, endpoint.to_string(), headers, executor);
-                Arc::new(transport) as _
+                Arc::new(HttpTransport::new(
+                    http_client,
+                    endpoint.to_string(),
+                    headers,
+                    executor,
+                    on_auth_updated,
+                ))
             }
             _ => anyhow::bail!("unsupported MCP url scheme {}", endpoint.scheme()),
         };
-        Ok(Self::new_with_timeout(id, transport, request_timeout))
+
+        Ok(Self {
+            id,
+            client: RwLock::new(None),
+            configuration: ContextServerTransport::Http(transport),
+            request_timeout,
+        })
     }

     pub fn new(id: ContextServerId, transport: Arc<dyn crate::transport::Transport>) -> Self {
@@ -105,7 +121,17 @@ impl ContextServer {
         self.client.read().clone()
     }

-    pub async fn start(&self, cx: &AsyncApp) -> Result<()> {
+    pub async fn start(
+        &self,
+        persisted: Option<ContextServerCredentials>,
+        cx: &AsyncApp,
+    ) -> Result<()> {
+        if let ContextServerTransport::Http(http) = &self.configuration {
+            if let Some(persisted) = persisted {
+                http.restore_credentials(persisted).await;
+            }
+        }
+
         self.initialize(self.new_client(cx)?).await
     }

@@ -122,6 +148,13 @@ impl ContextServer {
                 working_directory,
                 cx.clone(),
             )?,
+            ContextServerTransport::Http(transport) => Client::new(
+                client::ContextServerId(self.id.0.clone()),
+                self.id().0,
+                transport.clone(),
+                None,
+                cx.clone(),
+            )?,
             ContextServerTransport::Custom(transport) => Client::new(
                 client::ContextServerId(self.id.0.clone()),
                 self.id().0,
@@ -158,4 +191,28 @@ impl ContextServer {
         }
         Ok(())
     }
+
+    pub async fn start_auth(&self) -> Result<AuthorizeUrl> {
+        let ContextServerTransport::Http(http) = &self.configuration else {
+            anyhow::bail!("authorization is only supported for HTTP context servers");
+        };
+
+        http.start_auth().await
+    }
+
+    pub async fn handle_oauth_callback(&self, callback: &OAuthCallback) -> Result<()> {
+        let ContextServerTransport::Http(http) = &self.configuration else {
+            anyhow::bail!("authorization is only supported for HTTP context servers");
+        };
+
+        http.handle_oauth_callback(&callback).await
+    }
+
+    pub async fn logout(&self) {
+        let ContextServerTransport::Http(http) = &self.configuration else {
+            return;
+        };
+
+        http.logout().await
+    }
 }

```


## Assistant

Now let me also look at the zed://mcp/oauth/callback open_listener and main.rs changes on mcp-auth:

**Tool Call: git --no-pager diff f1fd0ab529..origin/mcp-auth -- crates/zed/src/main.rs crates/zed/src/zed/open_listener.rs**
Status: Completed

Terminal:
```
diff --git a/crates/zed/src/main.rs b/crates/zed/src/main.rs
index 1f72a894a2..649325d1ca 100644
--- a/crates/zed/src/main.rs
+++ b/crates/zed/src/main.rs
@@ -809,6 +809,18 @@ fn main() {
 fn handle_open_request(request: OpenRequest, app_state: Arc<AppState>, cx: &mut App) {
     if let Some(kind) = request.kind {
         match kind {
+            OpenRequestKind::McpOAuthCallback { callback } => {
+                cx.defer(move |cx| {
+                    workspace::with_active_or_new_workspace(cx, move |workspace, _window, cx| {
+                        let context_server_store =
+                            workspace.project().read(cx).context_server_store();
+
+                        context_server_store.update(cx, |store, cx| {
+                            store.handle_oauth_callback(callback, cx);
+                        });
+                    });
+                });
+            }
             OpenRequestKind::CliConnection(connection) => {
                 cx.spawn(async move |cx| handle_cli_connection(connection, app_state, cx).await)
                     .detach();
diff --git a/crates/zed/src/zed/open_listener.rs b/crates/zed/src/zed/open_listener.rs
index 6e92e50423..dcf1d7df33 100644
--- a/crates/zed/src/zed/open_listener.rs
+++ b/crates/zed/src/zed/open_listener.rs
@@ -5,6 +5,7 @@ use cli::{CliRequest, CliResponse, ipc::IpcSender};
 use cli::{IpcHandshake, ipc};
 use client::{ZedLink, parse_zed_link};
 use collections::HashMap;
+use context_server::transport::http::OAuthCallback as McpOAuthCallback;
 use db::kvp::KEY_VALUE_STORE;
 use editor::Editor;
 use fs::Fs;
@@ -68,6 +69,9 @@ pub enum OpenRequestKind {
     GitCommit {
         sha: String,
     },
+    McpOAuthCallback {
+        callback: McpOAuthCallback,
+    },
 }

 impl OpenRequest {
@@ -108,6 +112,12 @@ impl OpenRequest {
                 this.kind = Some(OpenRequestKind::Extension {
                     extension_id: extension_id.to_string(),
                 });
+            } else if let Some(callback) = url.strip_prefix(McpOAuthCallback::URI) {
+                let query = callback
+                    .strip_prefix('?')
+                    .context("invalid oauth callback url: missing query")?;
+                let callback = McpOAuthCallback::parse_query(query)?;
+                this.kind = Some(OpenRequestKind::McpOAuthCallback { callback });
             } else if url == "zed://agent" {
                 this.kind = Some(OpenRequestKind::AgentPanel);
             } else if let Some(session_id_str) = url.strip_prefix("zed://agent/shared/") {

```


## Assistant

Now let me look at the current branch's OAuth implementation for comparison:

**Tool Call: Read file `crates/context_server/src/oauth.rs`**
Status: Completed

```zed/crates/context_server/src/oauth.rs
use anyhow::{Context as _, Result, anyhow, bail};
use async_trait::async_trait;
use futures::AsyncReadExt as _;
use futures::channel::mpsc;
use http_client::{AsyncBody, HttpClient, Request};
use parking_lot::Mutex as SyncMutex;
use serde::{Deserialize, Serialize};
use smol::net::TcpListener;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use url::Url;

/// The CIMD URL where Zed's OAuth client metadata document is hosted.
pub const CIMD_URL: &str = "https://zed.dev/oauth/client-metadata.json";

/// Parsed from the MCP server's WWW-Authenticate header or well-known endpoint
/// per RFC 9728 (OAuth 2.0 Protected Resource Metadata).
#[derive(Debug, Clone)]
pub struct ProtectedResourceMetadata {
    pub resource: Url,
    pub authorization_servers: Vec<Url>,
    pub scopes_supported: Option<Vec<String>>,
}

/// Parsed from the authorization server's .well-known endpoint
/// per RFC 8414 (OAuth 2.0 Authorization Server Metadata).
#[derive(Debug, Clone)]
pub struct AuthServerMetadata {
    pub issuer: Url,
    pub authorization_endpoint: Url,
    pub token_endpoint: Url,
    pub registration_endpoint: Option<Url>,
    pub scopes_supported: Option<Vec<String>>,
    pub code_challenge_methods_supported: Option<Vec<String>>,
    pub client_id_metadata_document_supported: bool,
}

/// The result of client registration — either CIMD or DCR.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthClientRegistration {
    pub client_id: String,
    /// Only present for DCR-minted registrations.
    pub client_secret: Option<String>,
}

/// Access and refresh tokens obtained from the token endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthTokens {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: Option<SystemTime>,
}

/// Everything needed to kick off the browser flow, obtained during discovery.
/// Cached on the AuthRequired state so we don't re-discover on every attempt.
#[derive(Debug, Clone)]
pub struct OAuthDiscovery {
    pub resource_metadata: ProtectedResourceMetadata,
    pub auth_server_metadata: AuthServerMetadata,
    pub client_registration: OAuthClientRegistration,
    pub scopes: Vec<String>,
}

/// Fields extracted from a `WWW-Authenticate: Bearer` header.
///
/// Per RFC 9728 Section 5.1, MCP servers include `resource_metadata` to point
/// at the Protected Resource Metadata document. The optional `scope` parameter
/// (RFC 6750 Section 3) indicates scopes required for the request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WwwAuthenticate {
    pub resource_metadata: Option<Url>,
    pub scope: Option<Vec<String>>,
    /// The `error` parameter, if present (e.g. "insufficient_scope").
    pub error: Option<String>,
    pub error_description: Option<String>,
}

/// Parse a `WWW-Authenticate` header value.
///
/// Expects the `Bearer` scheme followed by comma-separated `key="value"` pairs.
/// Per RFC 6750 and RFC 9728, the relevant parameters are:
/// - `resource_metadata` — URL of the Protected Resource Metadata document
/// - `scope` — space-separated list of required scopes
/// - `error` — error code (e.g. "insufficient_scope")
/// - `error_description` — human-readable error description
pub fn parse_www_authenticate(header: &str) -> Result<WwwAuthenticate> {
    let header = header.trim();

    // Strip the "Bearer" scheme prefix (case-insensitive).
    let params_str = header
        .strip_prefix("Bearer")
        .or_else(|| header.strip_prefix("bearer"))
        .or_else(|| header.strip_prefix("BEARER"))
        .ok_or_else(|| anyhow!("WWW-Authenticate header does not use Bearer scheme"))?
        .trim();

    if params_str.is_empty() {
        return Ok(WwwAuthenticate {
            resource_metadata: None,
            scope: None,

```


**Tool Call: Read file `crates/context_server/src/transport/http.rs`**
Status: Completed

```zed/crates/context_server/src/transport/http.rs
SUCCESS: File outline retrieved. This file is too large to read all at once, so the outline below shows the file's structure with line numbers.

IMPORTANT: Do NOT retry this call without line numbers - you will get the same outline.
Instead, use the line numbers below to read specific sections by calling this tool again with start_line and end_line parameters.

# File outline for /Users/tom/gh/zed-industries/zed/crates/context_server/src/transport/http.rs

pub enum TransportError [L17-21]
 AuthRequired [L20]
  www_authenticate [L20]
impl std::fmt::Display for TransportError [L23-31]
 fn fmt [L24-30]
impl std::error::Error for TransportError [L33]
const HEADER_SESSION_ID [L36]
const EVENT_STREAM_MIME_TYPE [L37]
const JSON_MIME_TYPE [L38]
pub struct HttpTransport [L41-55]
 http_client [L42]
 endpoint [L43]
 session_id [L44]
 executor [L45]
 response_tx [L46]
 response_rx [L47]
 error_tx [L48]
 error_rx [L49]
 headers [L51]
 token_provider [L54]
impl HttpTransport [L57-294]
 pub fn new [L58-65]
 pub fn new_with_token_provider [L67-89]
 fn build_request [L94-119]
 async fn send_message [L122-227]
 async fn setup_sse_stream [L230-293]
impl Transport for HttpTransport [L297-309]
 async fn send [L298-300]
 fn receive [L302-304]
 fn receive_err [L306-308]
impl Drop for HttpTransport [L311-348]
 fn drop [L312-347]
mod tests [L351-649]
 struct FakeTokenProvider [L360-364]
  token [L361]
  refresh_succeeds [L362]
  refresh_count [L363]
 impl FakeTokenProvider [L366-382]
  fn new [L367-373]
  fn set_token [L375-377]
  fn refresh_count [L379-381]
 impl OAuthTokenProvider for FakeTokenProvider [L385-394]
  fn access_token [L386-388]
  async fn try_refresh [L390-393]
 fn make_fake_http_client [L396-406]
 fn json_response [L408-414]
 async fn test_bearer_token_attached_to_requests [L417-449]
 async fn test_no_bearer_token_without_provider [L452-478]
 async fn test_401_triggers_refresh_and_retry [L481-526]
 async fn test_401_returns_auth_required_when_refresh_fails [L529-577]
 async fn test_401_returns_auth_required_without_provider [L580-613]
 async fn test_401_after_successful_refresh_still_returns_auth_required [L616-648]

Showing symbols 1-52 (total symbols: 52)


NEXT STEPS: To read a specific symbol's implementation, call read_file with the same path plus start_line and end_line from the outline above.
For example, to read a function shown as [L100-150], use start_line: 100 and end_line: 150.
```


**Tool Call: Read file `crates/context_server/src/oauth.rs` (lines 100-350)**
Status: Completed

```zed/crates/context_server/src/oauth.rs
            scope: None,
            error: None,
            error_description: None,
        });
    }

    let params = parse_auth_params(params_str);

    let resource_metadata = params
        .get("resource_metadata")
        .map(|v| Url::parse(v))
        .transpose()
        .map_err(|e| anyhow!("invalid resource_metadata URL: {}", e))?;

    let scope = params
        .get("scope")
        .map(|v| v.split_whitespace().map(String::from).collect());

    let error = params.get("error").cloned();
    let error_description = params.get("error_description").cloned();

    Ok(WwwAuthenticate {
        resource_metadata,
        scope,
        error,
        error_description,
    })
}

/// Parse comma-separated `key="value"` or `key=token` parameters from an
/// auth-param list (RFC 7235 Section 2.1).
fn parse_auth_params(input: &str) -> collections::HashMap<String, String> {
    let mut params = collections::HashMap::default();
    let mut remaining = input.trim();

    while !remaining.is_empty() {
        // Skip leading whitespace and commas.
        remaining = remaining.trim_start_matches(|c: char| c == ',' || c.is_whitespace());
        if remaining.is_empty() {
            break;
        }

        // Find the key (everything before '=').
        let eq_pos = match remaining.find('=') {
            Some(pos) => pos,
            None => break,
        };

        let key = remaining[..eq_pos].trim().to_lowercase();
        remaining = &remaining[eq_pos + 1..];
        remaining = remaining.trim_start();

        // Parse the value: either quoted or unquoted (token).
        let value;
        if remaining.starts_with('"') {
            // Quoted string: find the closing quote, handling escaped chars.
            remaining = &remaining[1..]; // skip opening quote
            let mut val = String::new();
            let mut chars = remaining.char_indices();
            loop {
                match chars.next() {
                    Some((_, '\\')) => {
                        // Escaped character — take the next char literally.
                        if let Some((_, c)) = chars.next() {
                            val.push(c);
                        }
                    }
                    Some((i, '"')) => {
                        remaining = &remaining[i + 1..];
                        break;
                    }
                    Some((_, c)) => val.push(c),
                    None => {
                        remaining = "";
                        break;
                    }
                }
            }
            value = val;
        } else {
            // Unquoted token: read until comma or whitespace.
            let end = remaining
                .find(|c: char| c == ',' || c.is_whitespace())
                .unwrap_or(remaining.len());
            value = remaining[..end].to_string();
            remaining = &remaining[end..];
        }

        if !key.is_empty() {
            params.insert(key, value);
        }
    }

    params
}

/// Construct the well-known Protected Resource Metadata URIs for a given MCP
/// server URL, per RFC 9728 Section 3.
///
/// Returns URIs in priority order:
/// 1. Path-specific: `https://<host>/.well-known/oauth-protected-resource/<path>`
/// 2. Root: `https://<host>/.well-known/oauth-protected-resource`
pub fn protected_resource_metadata_urls(server_url: &Url) -> Vec<Url> {
    let mut urls = Vec::new();
    let base = format!("{}://{}", server_url.scheme(), server_url.authority());

    let path = server_url.path().trim_start_matches('/');
    if !path.is_empty() {
        if let Ok(url) = Url::parse(&format!(
            "{}/.well-known/oauth-protected-resource/{}",
            base, path
        )) {
            urls.push(url);
        }
    }

    if let Ok(url) = Url::parse(&format!("{}/.well-known/oauth-protected-resource", base)) {
        urls.push(url);
    }

    urls
}

/// Construct the well-known Authorization Server Metadata URIs for a given
/// issuer URL, per RFC 8414 Section 3.1 and Section 5 (OIDC compat).
///
/// Returns URIs in priority order, which differs depending on whether the
/// issuer URL has a path component.
pub fn auth_server_metadata_urls(issuer: &Url) -> Vec<Url> {
    let mut urls = Vec::new();
    let base = format!("{}://{}", issuer.scheme(), issuer.authority());
    let path = issuer.path().trim_matches('/');

    if !path.is_empty() {
        // Issuer with path: try path-inserted variants first.
        if let Ok(url) = Url::parse(&format!(
            "{}/.well-known/oauth-authorization-server/{}",
            base, path
        )) {
            urls.push(url);
        }
        if let Ok(url) = Url::parse(&format!(
            "{}/.well-known/openid-configuration/{}",
            base, path
        )) {
            urls.push(url);
        }
        if let Ok(url) = Url::parse(&format!(
            "{}/{}/.well-known/openid-configuration",
            base, path
        )) {
            urls.push(url);
        }
    } else {
        // No path: standard well-known locations.
        if let Ok(url) = Url::parse(&format!("{}/.well-known/oauth-authorization-server", base)) {
            urls.push(url);
        }
        if let Ok(url) = Url::parse(&format!("{}/.well-known/openid-configuration", base)) {
            urls.push(url);
        }
    }

    urls
}

// -- Canonical server URI (RFC 8707) -----------------------------------------

/// Derive the canonical resource URI for an MCP server URL, suitable for the
/// `resource` parameter in authorization and token requests per RFC 8707.
///
/// Lowercases the scheme and host, preserves the path (without trailing slash),
/// strips fragments and query strings.
pub fn canonical_server_uri(server_url: &Url) -> String {
    let mut uri = format!(
        "{}://{}",
        server_url.scheme().to_ascii_lowercase(),
        server_url.host_str().unwrap_or("").to_ascii_lowercase(),
    );
    if let Some(port) = server_url.port() {
        uri.push_str(&format!(":{}", port));
    }
    let path = server_url.path();
    if path != "/" {
        uri.push_str(path.trim_end_matches('/'));
    }
    uri
}

// -- Scope selection ---------------------------------------------------------

/// Select scopes following the MCP spec's Scope Selection Strategy:
/// 1. Use `scope` from the `WWW-Authenticate` challenge if present.
/// 2. Fall back to `scopes_supported` from Protected Resource Metadata.
/// 3. Return empty if neither is available.
pub fn select_scopes(
    www_authenticate: &WwwAuthenticate,
    resource_metadata: &ProtectedResourceMetadata,
) -> Vec<String> {
    if let Some(ref scopes) = www_authenticate.scope {
        if !scopes.is_empty() {
            return scopes.clone();
        }
    }
    resource_metadata
        .scopes_supported
        .clone()
        .unwrap_or_default()
}

// -- Client registration strategy --------------------------------------------

/// The registration approach to use, determined from auth server metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClientRegistrationStrategy {
    /// The auth server supports CIMD. Use the CIMD URL as client_id directly.
    Cimd { client_id: String },
    /// The auth server has a registration endpoint. Caller must POST to it.
    Dcr { registration_endpoint: Url },
    /// No supported registration mechanism.
    Unavailable,
}

/// Determine how to register with the authorization server, following the
/// spec's recommended priority: CIMD first, DCR fallback.
pub fn determine_registration_strategy(
    auth_server_metadata: &AuthServerMetadata,
) -> ClientRegistrationStrategy {
    if auth_server_metadata.client_id_metadata_document_supported {
        ClientRegistrationStrategy::Cimd {
            client_id: CIMD_URL.to_string(),
        }
    } else if let Some(ref endpoint) = auth_server_metadata.registration_endpoint {
        ClientRegistrationStrategy::Dcr {
            registration_endpoint: endpoint.clone(),
        }
    } else {
        ClientRegistrationStrategy::Unavailable
    }
}

// -- PKCE (RFC 7636) ---------------------------------------------------------

/// A PKCE code verifier and its S256 challenge.
#[derive(Debug, Clone)]
pub struct PkceChallenge {
    pub verifier: String,
    pub challenge: String,
}

/// Generate a PKCE code verifier and S256 challenge per RFC 7636.

```


**Tool Call: Read file `crates/context_server/src/transport/http.rs` (lines 1-230)**
Status: Completed

```zed/crates/context_server/src/transport/http.rs
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use collections::HashMap;
use futures::{Stream, StreamExt};
use gpui::BackgroundExecutor;
use http_client::{AsyncBody, HttpClient, Request, Response, http::Method};
use parking_lot::Mutex as SyncMutex;
use smol::channel;
use std::{pin::Pin, sync::Arc};

use crate::oauth::{self, OAuthTokenProvider, WwwAuthenticate};
use crate::transport::Transport;

/// Typed errors returned by the HTTP transport that callers can downcast from
/// `anyhow::Error` to handle specific failure modes.
#[derive(Debug)]
pub enum TransportError {
    /// The server returned 401 and token refresh either wasn't possible or
    /// failed. The caller should initiate the OAuth authorization flow.
    AuthRequired { www_authenticate: WwwAuthenticate },
}

impl std::fmt::Display for TransportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransportError::AuthRequired { .. } => {
                write!(f, "OAuth authorization required")
            }
        }
    }
}

impl std::error::Error for TransportError {}

// Constants from MCP spec
const HEADER_SESSION_ID: &str = "Mcp-Session-Id";
const EVENT_STREAM_MIME_TYPE: &str = "text/event-stream";
const JSON_MIME_TYPE: &str = "application/json";

/// HTTP Transport with session management and SSE support.
pub struct HttpTransport {
    http_client: Arc<dyn HttpClient>,
    endpoint: String,
    session_id: Arc<SyncMutex<Option<String>>>,
    executor: BackgroundExecutor,
    response_tx: channel::Sender<String>,
    response_rx: channel::Receiver<String>,
    error_tx: channel::Sender<String>,
    error_rx: channel::Receiver<String>,
    /// Static headers to include in every request (e.g. from server config).
    headers: HashMap<String, String>,
    /// When set, the transport attaches `Authorization: Bearer` headers and
    /// handles 401 responses with token refresh + retry.
    token_provider: Option<Arc<dyn OAuthTokenProvider>>,
}

impl HttpTransport {
    pub fn new(
        http_client: Arc<dyn HttpClient>,
        endpoint: String,
        headers: HashMap<String, String>,
        executor: BackgroundExecutor,
    ) -> Self {
        Self::new_with_token_provider(http_client, endpoint, headers, executor, None)
    }

    pub fn new_with_token_provider(
        http_client: Arc<dyn HttpClient>,
        endpoint: String,
        headers: HashMap<String, String>,
        executor: BackgroundExecutor,
        token_provider: Option<Arc<dyn OAuthTokenProvider>>,
    ) -> Self {
        let (response_tx, response_rx) = channel::unbounded();
        let (error_tx, error_rx) = channel::unbounded();

        Self {
            http_client,
            executor,
            endpoint,
            session_id: Arc::new(SyncMutex::new(None)),
            response_tx,
            response_rx,
            error_tx,
            error_rx,
            headers,
            token_provider,
        }
    }

    /// Build a POST request for the given message body, attaching all standard
    /// headers (content-type, accept, session ID, static headers, and bearer
    /// token if available).
    fn build_request(&self, message: &[u8]) -> Result<http_client::Request<AsyncBody>> {
        let mut request_builder = Request::builder()
            .method(Method::POST)
            .uri(&self.endpoint)
            .header("Content-Type", JSON_MIME_TYPE)
            .header(
                "Accept",
                format!("{}, {}", JSON_MIME_TYPE, EVENT_STREAM_MIME_TYPE),
            );

        for (key, value) in &self.headers {
            request_builder = request_builder.header(key.as_str(), value.as_str());
        }

        // Attach bearer token when a token provider is present.
        if let Some(token) = self.token_provider.as_ref().and_then(|p| p.access_token()) {
            request_builder = request_builder.header("Authorization", format!("Bearer {}", token));
        }

        // Add session ID if we have one (except for initialize).
        if let Some(ref session_id) = *self.session_id.lock() {
            request_builder = request_builder.header(HEADER_SESSION_ID, session_id.as_str());
        }

        Ok(request_builder.body(AsyncBody::from(message.to_vec()))?)
    }

    /// Send a message and handle the response based on content type.
    async fn send_message(&self, message: String) -> Result<()> {
        let is_notification =
            !message.contains("\"id\":") || message.contains("notifications/initialized");

        let request = self.build_request(message.as_bytes())?;
        let mut response = self.http_client.send(request).await?;

        // On 401, try refreshing the token and retry once.
        if response.status().as_u16() == 401 {
            let www_auth_header = response
                .headers()
                .get("www-authenticate")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("Bearer");

            let www_authenticate =
                oauth::parse_www_authenticate(www_auth_header).unwrap_or(WwwAuthenticate {
                    resource_metadata: None,
                    scope: None,
                    error: None,
                    error_description: None,
                });

            if let Some(ref provider) = self.token_provider {
                if provider.try_refresh().await.unwrap_or(false) {
                    // Retry with the refreshed token.
                    let retry_request = self.build_request(message.as_bytes())?;
                    response = self.http_client.send(retry_request).await?;

                    // If still 401 after refresh, give up.
                    if response.status().as_u16() == 401 {
                        return Err(TransportError::AuthRequired { www_authenticate }.into());
                    }
                } else {
                    return Err(TransportError::AuthRequired { www_authenticate }.into());
                }
            } else {
                return Err(TransportError::AuthRequired { www_authenticate }.into());
            }
        }

        // Handle different response types based on status and content-type.
        match response.status() {
            status if status.is_success() => {
                // Check content type
                let content_type = response
                    .headers()
                    .get("content-type")
                    .and_then(|v| v.to_str().ok());

                // Extract session ID from response headers if present
                if let Some(session_id) = response
                    .headers()
                    .get(HEADER_SESSION_ID)
                    .and_then(|v| v.to_str().ok())
                {
                    *self.session_id.lock() = Some(session_id.to_string());
                    log::debug!("Session ID set: {}", session_id);
                }

                match content_type {
                    Some(ct) if ct.starts_with(JSON_MIME_TYPE) => {
                        // JSON response - read and forward immediately
                        let mut body = String::new();
                        futures::AsyncReadExt::read_to_string(response.body_mut(), &mut body)
                            .await?;

                        // Only send non-empty responses
                        if !body.is_empty() {
                            self.response_tx
                                .send(body)
                                .await
                                .map_err(|_| anyhow!("Failed to send JSON response"))?;
                        }
                    }
                    Some(ct) if ct.starts_with(EVENT_STREAM_MIME_TYPE) => {
                        // SSE stream - set up streaming
                        self.setup_sse_stream(response).await?;
                    }
                    _ => {
                        // For notifications, 202 Accepted with no content type is ok
                        if is_notification && status.as_u16() == 202 {
                            log::debug!("Notification accepted");
                        } else {
                            return Err(anyhow!("Unexpected content type: {:?}", content_type));
                        }
                    }
                }
            }
            status if status.as_u16() == 202 => {
                // Accepted - notification acknowledged, no response needed
                log::debug!("Notification accepted");
            }
            _ => {
                let mut error_body = String::new();
                futures::AsyncReadExt::read_to_string(response.body_mut(), &mut error_body).await?;

                self.error_tx
                    .send(format!("HTTP {}: {}", response.status(), error_body))
                    .await
                    .map_err(|_| anyhow!("Failed to send error"))?;
            }
        }

        Ok(())
    }

    /// Set up SSE streaming from the response
    async fn setup_sse_stream(&self, mut response: Response<AsyncBody>) -> Result<()> {

```


**Tool Call: Read file `crates/context_server/src/oauth.rs` (lines 350-700)**
Status: Completed

```zed/crates/context_server/src/oauth.rs
/// Generate a PKCE code verifier and S256 challenge per RFC 7636.
///
/// The verifier is 43 base64url characters derived from 32 random bytes.
/// The challenge is `BASE64URL(SHA256(verifier))`.
pub fn generate_pkce_challenge() -> PkceChallenge {
    let random_bytes: Vec<u8> = (0..32).map(|_| rand::random::<u8>()).collect();
    let verifier = base64_url_encode(&random_bytes);

    let digest = simple_sha256(verifier.as_bytes());
    let challenge = base64_url_encode(&digest);

    PkceChallenge {
        verifier,
        challenge,
    }
}

// -- Authorization URL construction ------------------------------------------

/// Build the authorization URL for the OAuth Authorization Code + PKCE flow.
pub fn build_authorization_url(
    auth_server_metadata: &AuthServerMetadata,
    client_id: &str,
    redirect_uri: &str,
    scopes: &[String],
    resource: &str,
    pkce: &PkceChallenge,
    state: &str,
) -> Url {
    let mut url = auth_server_metadata.authorization_endpoint.clone();
    {
        let mut query = url.query_pairs_mut();
        query.append_pair("response_type", "code");
        query.append_pair("client_id", client_id);
        query.append_pair("redirect_uri", redirect_uri);
        if !scopes.is_empty() {
            query.append_pair("scope", &scopes.join(" "));
        }
        query.append_pair("resource", resource);
        query.append_pair("code_challenge", &pkce.challenge);
        query.append_pair("code_challenge_method", "S256");
        query.append_pair("state", state);
    }
    url
}

// -- Token endpoint request bodies -------------------------------------------

/// The JSON body returned by the token endpoint on success.
#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    #[serde(default)]
    pub refresh_token: Option<String>,
    #[serde(default)]
    pub expires_in: Option<u64>,
    #[serde(default)]
    pub token_type: Option<String>,
}

impl TokenResponse {
    /// Convert into `OAuthTokens`, computing `expires_at` from `expires_in`.
    pub fn into_tokens(self) -> OAuthTokens {
        let expires_at = self
            .expires_in
            .map(|secs| SystemTime::now() + Duration::from_secs(secs));
        OAuthTokens {
            access_token: self.access_token,
            refresh_token: self.refresh_token,
            expires_at,
        }
    }
}

/// Build the form-encoded body for an authorization code token exchange.
pub fn token_exchange_params(
    code: &str,
    client_id: &str,
    redirect_uri: &str,
    code_verifier: &str,
    resource: &str,
) -> Vec<(&'static str, String)> {
    vec![
        ("grant_type", "authorization_code".to_string()),
        ("code", code.to_string()),
        ("redirect_uri", redirect_uri.to_string()),
        ("client_id", client_id.to_string()),
        ("code_verifier", code_verifier.to_string()),
        ("resource", resource.to_string()),
    ]
}

/// Build the form-encoded body for a token refresh request.
pub fn token_refresh_params(
    refresh_token: &str,
    client_id: &str,
    resource: &str,
) -> Vec<(&'static str, String)> {
    vec![
        ("grant_type", "refresh_token".to_string()),
        ("refresh_token", refresh_token.to_string()),
        ("client_id", client_id.to_string()),
        ("resource", resource.to_string()),
    ]
}

// -- DCR request body (RFC 7591) ---------------------------------------------

/// Build the JSON body for a Dynamic Client Registration request.
pub fn dcr_registration_body() -> serde_json::Value {
    serde_json::json!({
        "client_name": "Zed",
        "redirect_uris": ["http://127.0.0.1/callback"],
        "grant_types": ["authorization_code"],
        "response_types": ["code"],
        "token_endpoint_auth_method": "none"
    })
}

// -- Helpers (vendored to avoid extra deps) ----------------------------------

/// Base64url-encode without padding, per RFC 4648 Section 5.
fn base64_url_encode(bytes: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let triple = (b0 << 16) | (b1 << 8) | b2;

        out.push(ALPHABET[((triple >> 18) & 0x3F) as usize] as char);
        out.push(ALPHABET[((triple >> 12) & 0x3F) as usize] as char);
        if chunk.len() > 1 {
            out.push(ALPHABET[((triple >> 6) & 0x3F) as usize] as char);
        }
        if chunk.len() > 2 {
            out.push(ALPHABET[(triple & 0x3F) as usize] as char);
        }
    }
    out
}

/// Minimal SHA-256 implementation (avoids pulling in a crypto crate just for
/// PKCE challenge derivation).
fn simple_sha256(data: &[u8]) -> [u8; 32] {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];

    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];

    // Pre-processing: pad the message.
    let bit_len = (data.len() as u64) * 8;
    let mut msg = data.to_vec();
    msg.push(0x80);
    while (msg.len() % 64) != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bit_len.to_be_bytes());

    // Process each 512-bit (64-byte) block.
    for block in msg.chunks_exact(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([
                block[i * 4],
                block[i * 4 + 1],
                block[i * 4 + 2],
                block[i * 4 + 3],
            ]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }

        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh] = h;

        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = hh
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(hh);
    }

    let mut result = [0u8; 32];
    for (i, val) in h.iter().enumerate() {
        result[i * 4..i * 4 + 4].copy_from_slice(&val.to_be_bytes());
    }
    result
}

// -- Discovery (async, hits real endpoints) ----------------------------------

/// Fetch Protected Resource Metadata from the MCP server.
///
/// Tries the `resource_metadata` URL from the `WWW-Authenticate` header first,
/// then falls back to well-known URIs constructed from `server_url`.
pub async fn fetch_protected_resource_metadata(
    http_client: &Arc<dyn HttpClient>,
    server_url: &Url,
    www_authenticate: &WwwAuthenticate,
) -> Result<ProtectedResourceMetadata> {
    let candidate_urls = if let Some(ref url) = www_authenticate.resource_metadata {
        vec![url.clone()]
    } else {
        protected_resource_metadata_urls(server_url)
    };

    for url in &candidate_urls {
        match fetch_json::<ProtectedResourceMetadataResponse>(http_client, url).await {
            Ok(response) => {
                if response.authorization_servers.is_empty() {
                    bail!(
                        "Protected Resource Metadata at {} has no authorization_servers",
                        url
                    );
                }
                return Ok(ProtectedResourceMetadata {
                    resource: response.resource.unwrap_or_else(|| server_url.clone()),
                    authorization_servers: response.authorization_servers,
                    scopes_supported: response.scopes_supported,
                });
            }
            Err(err) => {
                log::debug!(
                    "Failed to fetch Protected Resource Metadata from {}: {}",
                    url,
                    err
                );
            }
        }
    }

    bail!(
        "Could not fetch Protected Resource Metadata for {}",
        server_url
    )
}

/// Fetch Authorization Server Metadata, trying RFC 8414 and OIDC Discovery
/// endpoints in the priority order specified by the MCP spec.
pub async fn fetch_auth_server_metadata(
    http_client: &Arc<dyn HttpClient>,
    issuer: &Url,
) -> Result<AuthServerMetadata> {
    let candidate_urls = auth_server_metadata_urls(issuer);

    for url in &candidate_urls {
        match fetch_json::<AuthServerMetadataResponse>(http_client, url).await {
            Ok(response) => {
                let issuer_url = response
                    .issuer
                    .as_deref()
                    .and_then(|s| Url::parse(s).ok())
                    .unwrap_or_else(|| issuer.clone());

                let authorization_endpoint = response
                    .authorization_endpoint
                    .as_deref()
                    .map(Url::parse)
                    .transpose()
                    .context("invalid authorization_endpoint")?
                    .ok_or_else(|| anyhow!("missing authorization_endpoint"))?;

                let token_endpoint = response
                    .token_endpoint
                    .as_deref()
                    .map(Url::parse)
                    .transpose()
                    .context("invalid token_endpoint")?
                    .ok_or_else(|| anyhow!("missing token_endpoint"))?;

                let registration_endpoint = response
                    .registration_endpoint
                    .as_deref()
                    .map(Url::parse)
                    .transpose()
                    .context("invalid registration_endpoint")?;

                let code_challenge_methods = response.code_challenge_methods_supported;

                return Ok(AuthServerMetadata {
                    issuer: issuer_url,
                    authorization_endpoint,
                    token_endpoint,
                    registration_endpoint,
                    scopes_supported: response.scopes_supported,
                    code_challenge_methods_supported: code_challenge_methods,
                    client_id_metadata_document_supported: response
                        .client_id_metadata_document_supported
                        .unwrap_or(false),
                });
            }
            Err(err) => {
                log::debug!("Failed to fetch Auth Server Metadata from {}: {}", url, err);
            }
        }
    }

    bail!(
        "Could not fetch Authorization Server Metadata for {}",
        issuer
    )
}

/// Run the full discovery flow: fetch resource metadata, then auth server
/// metadata, then determine the client registration strategy and scopes.

```


**Tool Call: Read file `crates/context_server/src/oauth.rs` (lines 700-1050)**
Status: Completed

```zed/crates/context_server/src/oauth.rs
/// metadata, then determine the client registration strategy and scopes.
pub async fn discover(
    http_client: &Arc<dyn HttpClient>,
    server_url: &Url,
    www_authenticate: &WwwAuthenticate,
    cached_dcr_registration: Option<OAuthClientRegistration>,
) -> Result<OAuthDiscovery> {
    let resource_metadata =
        fetch_protected_resource_metadata(http_client, server_url, www_authenticate).await?;

    let auth_server_url = resource_metadata
        .authorization_servers
        .first()
        .ok_or_else(|| anyhow!("no authorization servers in resource metadata"))?;

    let auth_server_metadata = fetch_auth_server_metadata(http_client, auth_server_url).await?;

    // Verify PKCE S256 support (spec requirement).
    match &auth_server_metadata.code_challenge_methods_supported {
        Some(methods) if methods.iter().any(|m| m == "S256") => {}
        Some(_) => bail!("authorization server does not support S256 PKCE"),
        None => bail!("authorization server does not advertise code_challenge_methods_supported"),
    }

    let scopes = select_scopes(www_authenticate, &resource_metadata);

    let client_registration = match determine_registration_strategy(&auth_server_metadata) {
        ClientRegistrationStrategy::Cimd { client_id } => OAuthClientRegistration {
            client_id,
            client_secret: None,
        },
        ClientRegistrationStrategy::Dcr {
            registration_endpoint,
        } => {
            if let Some(cached) = cached_dcr_registration {
                log::info!(
                    "using cached DCR registration (client_id={})",
                    cached.client_id
                );
                cached
            } else {
                perform_dcr(http_client, &registration_endpoint).await?
            }
        }
        ClientRegistrationStrategy::Unavailable => {
            bail!("authorization server supports neither CIMD nor DCR")
        }
    };

    Ok(OAuthDiscovery {
        resource_metadata,
        auth_server_metadata,
        client_registration,
        scopes,
    })
}

// -- Dynamic Client Registration (RFC 7591) ----------------------------------

/// Perform Dynamic Client Registration with the authorization server.
pub async fn perform_dcr(
    http_client: &Arc<dyn HttpClient>,
    registration_endpoint: &Url,
) -> Result<OAuthClientRegistration> {
    let body = dcr_registration_body();
    let body_bytes = serde_json::to_vec(&body)?;

    let request = Request::builder()
        .method(http_client::http::Method::POST)
        .uri(registration_endpoint.as_str())
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(AsyncBody::from(body_bytes))?;

    let mut response = http_client.send(request).await?;

    if !response.status().is_success() {
        let mut error_body = String::new();
        response.body_mut().read_to_string(&mut error_body).await?;
        bail!(
            "DCR failed with status {}: {}",
            response.status(),
            error_body
        );
    }

    let mut response_body = String::new();
    response
        .body_mut()
        .read_to_string(&mut response_body)
        .await?;

    let dcr_response: DcrResponse =
        serde_json::from_str(&response_body).context("failed to parse DCR response")?;

    Ok(OAuthClientRegistration {
        client_id: dcr_response.client_id,
        client_secret: dcr_response.client_secret,
    })
}

// -- Token exchange and refresh (async) --------------------------------------

/// Exchange an authorization code for tokens at the token endpoint.
pub async fn exchange_code(
    http_client: &Arc<dyn HttpClient>,
    auth_server_metadata: &AuthServerMetadata,
    code: &str,
    client_id: &str,
    redirect_uri: &str,
    code_verifier: &str,
    resource: &str,
) -> Result<OAuthTokens> {
    let params = token_exchange_params(code, client_id, redirect_uri, code_verifier, resource);
    post_token_request(http_client, &auth_server_metadata.token_endpoint, &params).await
}

/// Refresh tokens using a refresh token.
pub async fn refresh_tokens(
    http_client: &Arc<dyn HttpClient>,
    auth_server_metadata: &AuthServerMetadata,
    refresh_token: &str,
    client_id: &str,
    resource: &str,
) -> Result<OAuthTokens> {
    let params = token_refresh_params(refresh_token, client_id, resource);
    post_token_request(http_client, &auth_server_metadata.token_endpoint, &params).await
}

/// POST form-encoded parameters to a token endpoint and parse the response.
async fn post_token_request(
    http_client: &Arc<dyn HttpClient>,
    token_endpoint: &Url,
    params: &[(&str, String)],
) -> Result<OAuthTokens> {
    let body = url::form_urlencoded::Serializer::new(String::new())
        .extend_pairs(params.iter().map(|(k, v)| (*k, v.as_str())))
        .finish();

    let request = Request::builder()
        .method(http_client::http::Method::POST)
        .uri(token_endpoint.as_str())
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Accept", "application/json")
        .body(AsyncBody::from(body.into_bytes()))?;

    let mut response = http_client.send(request).await?;

    if !response.status().is_success() {
        let mut error_body = String::new();
        response.body_mut().read_to_string(&mut error_body).await?;
        bail!(
            "token request failed with status {}: {}",
            response.status(),
            error_body
        );
    }

    let mut response_body = String::new();
    response
        .body_mut()
        .read_to_string(&mut response_body)
        .await?;

    let token_response: TokenResponse =
        serde_json::from_str(&response_body).context("failed to parse token response")?;

    Ok(token_response.into_tokens())
}

// -- Local callback server ---------------------------------------------------

/// Result of awaiting the OAuth callback on the local server.
#[derive(Debug)]
pub struct AuthorizationCallback {
    pub code: String,
    pub state: String,
}

/// Start a local HTTP server on an ephemeral port to receive the OAuth callback.
///
/// Returns the bound port and a future that resolves when the callback is
/// received. The server responds with a simple HTML page telling the user
/// they can close the tab, then shuts down.
pub async fn start_callback_server() -> Result<(u16, smol::Task<Result<AuthorizationCallback>>)> {
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let port = listener.local_addr()?.port();

    let task = smol::spawn(async move {
        let (mut stream, _) = listener.accept().await?;

        // Read the full HTTP request head into a buffer. We read byte-by-byte
        // looking for the "\r\n\r\n" delimiter so we never over-read past the
        // headers (there is no body in a GET callback request).
        let mut head = Vec::with_capacity(1024);
        let mut found_end = false;
        let mut buf = [0u8; 1];
        while smol::io::AsyncReadExt::read(&mut stream, &mut buf).await? > 0 {
            head.push(buf[0]);
            if head.len() >= 4 && head[head.len() - 4..] == *b"\r\n\r\n" {
                found_end = true;
                break;
            }
            // Safety valve — a real callback request should be well under 8 KiB.
            if head.len() > 8192 {
                bail!("OAuth callback request too large");
            }
        }
        if !found_end {
            bail!("OAuth callback connection closed before end of headers");
        }

        let head_str = String::from_utf8_lossy(&head);

        // Parse "GET /callback?code=...&state=... HTTP/1.1\r\n..."
        let request_line = head_str
            .lines()
            .next()
            .ok_or_else(|| anyhow!("empty HTTP request"))?;

        let path = request_line
            .split_whitespace()
            .nth(1)
            .ok_or_else(|| anyhow!("malformed HTTP request line"))?;

        let dummy_base = Url::parse("http://127.0.0.1")?;
        let full_url = dummy_base.join(path)?;
        let query_pairs: std::collections::HashMap<_, _> = full_url.query_pairs().collect();

        let code = query_pairs
            .get("code")
            .ok_or_else(|| anyhow!("missing 'code' parameter in OAuth callback"))?
            .to_string();
        let state = query_pairs
            .get("state")
            .ok_or_else(|| anyhow!("missing 'state' parameter in OAuth callback"))?
            .to_string();

        // Send a minimal response.
        let html = "<!DOCTYPE html><html><body><h1>Authorization complete</h1><p>You can close this tab and return to Zed.</p></body></html>";
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            html.len(),
            html
        );
        smol::io::AsyncWriteExt::write_all(&mut stream, response.as_bytes()).await?;

        Ok(AuthorizationCallback { code, state })
    });

    Ok((port, task))
}

// -- JSON fetch helper -------------------------------------------------------

async fn fetch_json<T: serde::de::DeserializeOwned>(
    http_client: &Arc<dyn HttpClient>,
    url: &Url,
) -> Result<T> {
    let request = Request::builder()
        .method(http_client::http::Method::GET)
        .uri(url.as_str())
        .header("Accept", "application/json")
        .body(AsyncBody::default())?;

    let mut response = http_client.send(request).await?;

    if !response.status().is_success() {
        bail!("HTTP {} fetching {}", response.status(), url);
    }

    let mut body = String::new();
    response.body_mut().read_to_string(&mut body).await?;
    serde_json::from_str(&body).with_context(|| format!("failed to parse JSON from {}", url))
}

// -- Serde response types for discovery --------------------------------------

#[derive(Debug, Deserialize)]
struct ProtectedResourceMetadataResponse {
    #[serde(default)]
    resource: Option<Url>,
    #[serde(default)]
    authorization_servers: Vec<Url>,
    #[serde(default)]
    scopes_supported: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct AuthServerMetadataResponse {
    #[serde(default)]
    issuer: Option<String>,
    #[serde(default)]
    authorization_endpoint: Option<String>,
    #[serde(default)]
    token_endpoint: Option<String>,
    #[serde(default)]
    registration_endpoint: Option<String>,
    #[serde(default)]
    scopes_supported: Option<Vec<String>>,
    #[serde(default)]
    code_challenge_methods_supported: Option<Vec<String>>,
    #[serde(default)]
    client_id_metadata_document_supported: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct DcrResponse {
    client_id: String,
    #[serde(default)]
    client_secret: Option<String>,
}

/// Provides OAuth tokens to the HTTP transport layer.
///
/// The transport calls `access_token()` before each request. On a 401 response
/// it calls `try_refresh()` and retries once if the refresh succeeds.
#[async_trait]
pub trait OAuthTokenProvider: Send + Sync {
    /// Returns the current access token, if one is available.
    fn access_token(&self) -> Option<String>;

    /// Attempts to refresh the access token. Returns `true` if a new token was
    /// obtained and the request should be retried.
    async fn try_refresh(&self) -> Result<bool>;
}

/// Concrete `OAuthTokenProvider` backed by in-memory tokens and an HTTP client
/// for token refresh. Created by the store after successful authentication or
/// when loading cached tokens from the keychain.
pub struct McpOAuthTokenProvider {
    tokens: SyncMutex<Option<OAuthTokens>>,
    discovery: Arc<OAuthDiscovery>,
    http_client: Arc<dyn HttpClient>,
    token_refresh_tx: Option<mpsc::UnboundedSender<OAuthTokens>>,
}

impl McpOAuthTokenProvider {
    pub fn new(
        tokens: OAuthTokens,
        discovery: Arc<OAuthDiscovery>,
        http_client: Arc<dyn HttpClient>,
        token_refresh_tx: Option<mpsc::UnboundedSender<OAuthTokens>>,
    ) -> Self {
        Self {
            tokens: SyncMutex::new(Some(tokens)),
            discovery,
            http_client,
            token_refresh_tx,
        }
    }

```


**Tool Call: Read file `crates/context_server/src/oauth.rs` (lines 1050-1200)**
Status: Completed

```zed/crates/context_server/src/oauth.rs
    }
}

#[async_trait]
impl OAuthTokenProvider for McpOAuthTokenProvider {
    fn access_token(&self) -> Option<String> {
        self.tokens.lock().as_ref().map(|t| t.access_token.clone())
    }

    async fn try_refresh(&self) -> Result<bool> {
        let refresh_token = {
            let guard = self.tokens.lock();
            match guard.as_ref().and_then(|t| t.refresh_token.clone()) {
                Some(rt) => rt,
                None => return Ok(false),
            }
        };

        let resource = canonical_server_uri(&self.discovery.resource_metadata.resource);

        match refresh_tokens(
            &self.http_client,
            &self.discovery.auth_server_metadata,
            &refresh_token,
            &self.discovery.client_registration.client_id,
            &resource,
        )
        .await
        {
            Ok(new_tokens) => {
                if let Some(ref tx) = self.token_refresh_tx {
                    tx.unbounded_send(new_tokens.clone()).ok();
                }
                *self.tokens.lock() = Some(new_tokens);
                Ok(true)
            }
            Err(err) => {
                log::warn!("OAuth token refresh failed: {}", err);
                Ok(false)
            }
        }
    }
}

/// A simple token provider that holds a static access token and never
/// refreshes. Used on startup when we have cached tokens from the keychain
/// but no discovery info yet. If the token is expired, the transport will
/// get a 401 and transition to `AuthRequired`, triggering a full discovery
/// and re-authentication flow.
pub struct StaticTokenProvider {
    access_token: String,
}

impl StaticTokenProvider {
    pub fn new(access_token: String) -> Self {
        Self { access_token }
    }
}

#[async_trait]
impl OAuthTokenProvider for StaticTokenProvider {
    fn access_token(&self) -> Option<String> {
        Some(self.access_token.clone())
    }

    async fn try_refresh(&self) -> Result<bool> {
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use http_client::Response;

    #[test]
    fn test_parse_www_authenticate_with_resource_metadata_and_scope() {
        let header = r#"Bearer resource_metadata="https://mcp.example.com/.well-known/oauth-protected-resource", scope="files:read user:profile""#;
        let result = parse_www_authenticate(header).unwrap();

        assert_eq!(
            result.resource_metadata.as_ref().map(|u| u.as_str()),
            Some("https://mcp.example.com/.well-known/oauth-protected-resource")
        );
        assert_eq!(
            result.scope,
            Some(vec!["files:read".to_string(), "user:profile".to_string()])
        );
        assert_eq!(result.error, None);
    }

    #[test]
    fn test_parse_www_authenticate_resource_metadata_only() {
        let header = r#"Bearer resource_metadata="https://mcp.example.com/.well-known/oauth-protected-resource""#;
        let result = parse_www_authenticate(header).unwrap();

        assert_eq!(
            result.resource_metadata.as_ref().map(|u| u.as_str()),
            Some("https://mcp.example.com/.well-known/oauth-protected-resource")
        );
        assert_eq!(result.scope, None);
    }

    #[test]
    fn test_parse_www_authenticate_bare_bearer() {
        let result = parse_www_authenticate("Bearer").unwrap();
        assert_eq!(result.resource_metadata, None);
        assert_eq!(result.scope, None);
    }

    #[test]
    fn test_parse_www_authenticate_with_error() {
        let header = r#"Bearer error="insufficient_scope", scope="files:read files:write", resource_metadata="https://mcp.example.com/.well-known/oauth-protected-resource", error_description="Additional file write permission required""#;
        let result = parse_www_authenticate(header).unwrap();

        assert_eq!(result.error.as_deref(), Some("insufficient_scope"));
        assert_eq!(
            result.error_description.as_deref(),
            Some("Additional file write permission required")
        );
        assert_eq!(
            result.scope,
            Some(vec!["files:read".to_string(), "files:write".to_string()])
        );
        assert!(result.resource_metadata.is_some());
    }

    #[test]
    fn test_parse_www_authenticate_rejects_non_bearer() {
        let result = parse_www_authenticate("Basic realm=\"example\"");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_www_authenticate_case_insensitive_scheme() {
        let header = r#"bearer resource_metadata="https://example.com/.well-known/oauth-protected-resource""#;
        let result = parse_www_authenticate(header).unwrap();
        assert!(result.resource_metadata.is_some());
    }

    #[test]
    fn test_parse_www_authenticate_multiline_style() {
        // Some servers emit the header spread across multiple lines joined by
        // whitespace, as shown in the spec examples.
        let header = "Bearer resource_metadata=\"https://mcp.example.com/.well-known/oauth-protected-resource\",\n                         scope=\"files:read\"";
        let result = parse_www_authenticate(header).unwrap();
        assert!(result.resource_metadata.is_some());
        assert_eq!(result.scope, Some(vec!["files:read".to_string()]));
    }

    #[test]

```


**Tool Call: Read file `crates/project/src/context_server_store.rs`**
Status: Completed

```zed/crates/project/src/context_server_store.rs
SUCCESS: File outline retrieved. This file is too large to read all at once, so the outline below shows the file's structure with line numbers.

IMPORTANT: Do NOT retry this call without line numbers - you will get the same outline.
Instead, use the line numbers below to read specific sections by calling this tool again with start_line and end_line parameters.

# File outline for /Users/tom/gh/zed-industries/zed/crates/project/src/context_server_store.rs

pub mod extension [L1]
pub mod registry [L2]
const MAX_TIMEOUT_SECS [L33]
pub fn init [L35-37]
pub enum ContextServerStatus [L48-57]
 Starting [L49]
 Running [L50]
 Stopped [L51]
 Error [L52]
 AuthRequired [L56]
impl PartialEq for ContextServerStatus [L59-70]
 fn eq [L60-69]
impl Eq for ContextServerStatus [L72]
impl std::hash::Hash for ContextServerStatus [L74-82]
 fn hash [L75-81]
impl ContextServerStatus [L84-96]
 fn from_state [L85-95]
enum ContextServerState [L98-124]
 Starting [L99-103]
  server [L100]
  configuration [L101]
  _task [L102]
 Running [L104-107]
  server [L105]
  configuration [L106]
 Stopped [L108-111]
  server [L109]
  configuration [L110]
 Error [L112-116]
  server [L113]
  configuration [L114]
  error [L115]
 AuthRequired [L119-123]
  server [L120]
  configuration [L121]
  discovery [L122]
impl ContextServerState [L126-146]
 pub fn server [L127-135]
 pub fn configuration [L137-145]
pub enum ContextServerConfiguration [L149-164]
 Custom [L150-153]
  command [L151]
  remote [L152]
 Extension [L154-158]
  command [L155]
  settings [L156]
  remote [L157]
 Http [L159-163]
  url [L160]
  headers [L161]
  timeout [L162]
impl ContextServerConfiguration [L166-233]
 pub fn command [L167-173]
 pub fn remote [L175-181]
 pub async fn from_settings [L183-232]
pub type ContextServerFactory [L235-236]
enum ContextServerStoreState [L238-247]
 Local [L239-242]
  downstream_client [L240]
  is_headless [L241]
 Remote [L243-246]
  project_id [L244]
  upstream_client [L245]
pub struct ContextServerStore [L249-261]
 state [L250]
 context_server_settings [L251]
 servers [L252]
 server_ids [L253]
 worktree_store [L254]
 project [L255]
 registry [L256]
 update_servers_task [L257]
 context_server_factory [L258]
 needs_server_update [L259]
 _subscriptions [L260]
pub struct ServerStatusChangedEvent [L263-266]
 pub server_id [L264]
 pub status [L265]
impl EventEmitter<ServerStatusChangedEvent> for ContextServerStore [L268]
impl ContextServerStore [L270-1465]
 pub fn local [L271-289]
 pub fn remote [L291-310]
 pub fn init_headless [L312-314]
 pub fn shared [L316-323]
 pub fn is_remote_project [L325-327]
 pub fn configured_server_ids [L330-336]
 pub fn test [L339-357]
 pub fn test_maintain_server_loop [L360-379]
 pub fn set_context_server_factory [L382-384]
 pub fn registry [L387-389]
 pub fn test_start_server [L392-403]
 fn new_internal [L405-451]
 pub fn get_server [L453-455]
 pub fn get_running_server [L457-463]
 pub fn status_for_server [L465-467]
 pub fn configuration_for_server [L469-474]
 pub fn server_ids [L479-481]
 fn populate_server_ids [L483-516]
  const MCP_PREFIX [L504]
 pub fn running_servers [L518-529]
 pub fn start_server [L531-563]
 pub fn stop_server [L565-596]
 fn run_server [L598-772]
 fn remove_server [L774-785]
 pub async fn create_context_server [L787-945]
 async fn handle_get_context_server_command [L947-1000]
 fn resolve_project_settings [L1002-1015]
 pub fn authenticate_server [L1023-1070]
 async fn run_oauth_flow [L1072-1200]
 async fn store_tokens [L1204-1215]
 async fn load_tokens [L1218-1231]
 async fn clear_tokens [L1234-1241]
 fn keychain_key [L1243-1245]
 fn dcr_keychain_key [L1247-1252]
 async fn store_dcr_registration [L1256-1267]
 async fn load_dcr_registration [L1270-1284]
 async fn clear_dcr_registration [L1287-1294]
 pub fn logout_server [L1298-1324]
 fn update_server_state [L1326-1338]
 fn available_context_servers_changed [L1340-1361]
 async fn maintain_servers [L1363-1464]

Showing symbols 1-121 (total symbols: 121)


NEXT STEPS: To read a specific symbol's implementation, call read_file with the same path plus start_line and end_line from the outline above.
For example, to read a function shown as [L100-150], use start_line: 100 and end_line: 150.
```


**Tool Call: Read file `crates/project/src/context_server_store.rs` (lines 48-146)**
Status: Completed

```zed/crates/project/src/context_server_store.rs
pub enum ContextServerStatus {
    Starting,
    Running,
    Stopped,
    Error(Arc<str>),
    /// The server returned 401 and OAuth authorization is needed. The
    /// `OAuthDiscovery` contains everything required to kick off the browser
    /// flow — the UI should show an "Authenticate" button.
    AuthRequired(Arc<OAuthDiscovery>),
}

impl PartialEq for ContextServerStatus {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Starting, Self::Starting) => true,
            (Self::Running, Self::Running) => true,
            (Self::Stopped, Self::Stopped) => true,
            (Self::Error(a), Self::Error(b)) => a == b,
            (Self::AuthRequired(_), Self::AuthRequired(_)) => true,
            _ => false,
        }
    }
}

impl Eq for ContextServerStatus {}

impl std::hash::Hash for ContextServerStatus {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
            Self::Error(e) => e.hash(state),
            _ => {}
        }
    }
}

impl ContextServerStatus {
    fn from_state(state: &ContextServerState) -> Self {
        match state {
            ContextServerState::Starting { .. } => ContextServerStatus::Starting,
            ContextServerState::Running { .. } => ContextServerStatus::Running,
            ContextServerState::Stopped { .. } => ContextServerStatus::Stopped,
            ContextServerState::Error { error, .. } => ContextServerStatus::Error(error.clone()),
            ContextServerState::AuthRequired { discovery, .. } => {
                ContextServerStatus::AuthRequired(discovery.clone())
            }
        }
    }
}

enum ContextServerState {
    Starting {
        server: Arc<ContextServer>,
        configuration: Arc<ContextServerConfiguration>,
        _task: Task<()>,
    },
    Running {
        server: Arc<ContextServer>,
        configuration: Arc<ContextServerConfiguration>,
    },
    Stopped {
        server: Arc<ContextServer>,
        configuration: Arc<ContextServerConfiguration>,
    },
    Error {
        server: Arc<ContextServer>,
        configuration: Arc<ContextServerConfiguration>,
        error: Arc<str>,
    },
    /// The server requires OAuth authorization before it can be used. The
    /// `OAuthDiscovery` holds everything needed to start the browser flow.
    AuthRequired {
        server: Arc<ContextServer>,
        configuration: Arc<ContextServerConfiguration>,
        discovery: Arc<OAuthDiscovery>,
    },
}

impl ContextServerState {
    pub fn server(&self) -> Arc<ContextServer> {
        match self {
            ContextServerState::Starting { server, .. }
            | ContextServerState::Running { server, .. }
            | ContextServerState::Stopped { server, .. }
            | ContextServerState::Error { server, .. }
            | ContextServerState::AuthRequired { server, .. } => server.clone(),
        }
    }

    pub fn configuration(&self) -> Arc<ContextServerConfiguration> {
        match self {
            ContextServerState::Starting { configuration, .. }
            | ContextServerState::Running { configuration, .. }
            | ContextServerState::Stopped { configuration, .. }
            | ContextServerState::Error { configuration, .. }
            | ContextServerState::AuthRequired { configuration, .. } => configuration.clone(),
        }
    }
}

```


**Tool Call: Read file `crates/project/src/context_server_store.rs` (lines 598-772)**
Status: Completed

```zed/crates/project/src/context_server_store.rs
    fn run_server(
        &mut self,
        server: Arc<ContextServer>,
        configuration: Arc<ContextServerConfiguration>,
        cx: &mut Context<Self>,
    ) {
        let id = server.id();
        if matches!(
            self.servers.get(&id),
            Some(ContextServerState::Starting { .. } | ContextServerState::Running { .. })
        ) {
            self.stop_server(&id, cx).log_err();
        }
        let task =
            cx.spawn({
                let id = server.id();
                let server = server.clone();
                let configuration = configuration.clone();

                async move |this, cx| {
                    match server.clone().start(cx).await {
                        Ok(_) => {
                            debug_assert!(server.client().is_some());

                            this.update(cx, |this, cx| {
                                this.update_server_state(
                                    id.clone(),
                                    ContextServerState::Running {
                                        server,
                                        configuration,
                                    },
                                    cx,
                                )
                            })
                            .log_err()
                        }
                        Err(err) => {
                            // Check if the error is an OAuth 401 — if so, run
                            // discovery and transition to AuthRequired instead of
                            // the generic Error state.
                            if let Some(TransportError::AuthRequired { www_authenticate }) =
                                err.downcast_ref::<TransportError>()
                            {
                                let server_url = match &configuration.as_ref() {
                                    ContextServerConfiguration::Http { url, .. } => url.clone(),
                                    _ => {
                                        log::error!("{} got OAuth 401 on a non-HTTP transport", id);
                                        this.update(cx, |this, cx| {
                                            this.update_server_state(
                                                id.clone(),
                                                ContextServerState::Error {
                                                    configuration,
                                                    server,
                                                    error: err.to_string().into(),
                                                },
                                                cx,
                                            )
                                        })
                                        .log_err();
                                        return;
                                    }
                                };

                                let http_client = cx.update(|cx| cx.http_client());

                                let credentials_provider =
                                    cx.update(|cx| <dyn CredentialsProvider>::global(cx));
                                let cached_dcr = Self::load_dcr_registration(
                                    &credentials_provider,
                                    &server_url,
                                    &cx,
                                )
                                .await
                                .ok()
                                .flatten();

                                match context_server::oauth::discover(
                                    &http_client,
                                    &server_url,
                                    www_authenticate,
                                    cached_dcr,
                                )
                                .await
                                {
                                    Ok(discovery) => {
                                        // Persist the DCR registration so we
                                        // don't mint a new client_id next time.
                                        if let Err(err) = Self::store_dcr_registration(
                                            &credentials_provider,
                                            &server_url,
                                            &discovery.client_registration,
                                            &cx,
                                        )
                                        .await
                                        {
                                            log::warn!(
                                                "{} failed to cache DCR registration: {}",
                                                id,
                                                err,
                                            );
                                        }
                                        log::info!(
                                            "{} requires OAuth authorization (auth server: {})",
                                            id,
                                            discovery.auth_server_metadata.issuer,
                                        );
                                        this.update(cx, |this, cx| {
                                            this.update_server_state(
                                                id.clone(),
                                                ContextServerState::AuthRequired {
                                                    server,
                                                    configuration,
                                                    discovery: Arc::new(discovery),
                                                },
                                                cx,
                                            )
                                        })
                                        .log_err();
                                        return;
                                    }
                                    Err(discovery_err) => {
                                        log::error!(
                                            "{} OAuth discovery failed: {}",
                                            id,
                                            discovery_err,
                                        );
                                        this.update(cx, |this, cx| {
                                            this.update_server_state(
                                                id.clone(),
                                                ContextServerState::Error {
                                                    configuration,
                                                    server,
                                                    error: format!(
                                                        "OAuth discovery failed: {}",
                                                        discovery_err
                                                    )
                                                    .into(),
                                                },
                                                cx,
                                            )
                                        })
                                        .log_err();
                                        return;
                                    }
                                }
                            }

                            log::error!("{} context server failed to start: {}", id, err);
                            this.update(cx, |this, cx| {
                                this.update_server_state(
                                    id.clone(),
                                    ContextServerState::Error {
                                        configuration,
                                        server,
                                        error: err.to_string().into(),
                                    },
                                    cx,
                                )
                            })
                            .log_err()
                        }
                    };
                }
            });

        self.update_server_state(
            id.clone(),
            ContextServerState::Starting {
                configuration,
                _task: task,
                server,
            },
            cx,
        );
    }

```


**Tool Call: Read file `crates/project/src/context_server_store.rs` (lines 1023-1325)**
Status: Completed

```zed/crates/project/src/context_server_store.rs
    pub fn authenticate_server(
        &mut self,
        id: &ContextServerId,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        let state = self.servers.get(id).context("Context server not found")?;

        let (discovery, server, configuration) = match state {
            ContextServerState::AuthRequired {
                discovery,
                server,
                configuration,
            } => (discovery.clone(), server.clone(), configuration.clone()),
            _ => anyhow::bail!("Server is not in AuthRequired state"),
        };

        let id = id.clone();

        cx.spawn(async move |this, cx| {
            let result = Self::run_oauth_flow(
                this.clone(),
                id.clone(),
                discovery.clone(),
                configuration.clone(),
                cx,
            )
            .await;

            if let Err(err) = &result {
                log::error!("{} OAuth authentication failed: {}", id, err);
                this.update(cx, |this, cx| {
                    this.update_server_state(
                        id.clone(),
                        ContextServerState::Error {
                            server,
                            configuration,
                            error: err.to_string().into(),
                        },
                        cx,
                    )
                })
                .log_err();
            }
        })
        .detach();

        Ok(())
    }

    async fn run_oauth_flow(
        this: WeakEntity<Self>,
        id: ContextServerId,
        discovery: Arc<OAuthDiscovery>,
        configuration: Arc<ContextServerConfiguration>,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        let (port, callback_task) = oauth::start_callback_server().await?;
        let redirect_uri = format!("http://127.0.0.1:{}/callback", port);
        let resource = oauth::canonical_server_uri(&discovery.resource_metadata.resource);
        let pkce = oauth::generate_pkce_challenge();

        let state_param: String = (0..32)
            .map(|_| rand::random::<u8>())
            .collect::<Vec<_>>()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect();

        let auth_url = oauth::build_authorization_url(
            &discovery.auth_server_metadata,
            &discovery.client_registration.client_id,
            &redirect_uri,
            &discovery.scopes,
            &resource,
            &pkce,
            &state_param,
        );

        cx.update(|cx| cx.open_url(auth_url.as_str()));

        let callback = callback_task.await?;

        if callback.state != state_param {
            anyhow::bail!("OAuth state parameter mismatch");
        }

        let http_client = cx.update(|cx| cx.http_client());

        let tokens = oauth::exchange_code(
            &http_client,
            &discovery.auth_server_metadata,
            &callback.code,
            &discovery.client_registration.client_id,
            &redirect_uri,
            &pkce.verifier,
            &resource,
        )
        .await?;

        // Persist tokens in the keychain.
        let credentials_provider = cx.update(|cx| <dyn CredentialsProvider>::global(cx));
        let server_url = match configuration.as_ref() {
            ContextServerConfiguration::Http { url, .. } => url.clone(),
            _ => anyhow::bail!("OAuth authentication only supported for HTTP servers"),
        };
        Self::store_tokens(&credentials_provider, &server_url, &tokens, cx).await?;

        // Create a token provider and restart the server with it. The channel
        // lets us persist refreshed tokens back to the keychain without the
        // token provider needing access to GPUI or the credentials provider.
        let (token_refresh_tx, token_refresh_rx) = futures::channel::mpsc::unbounded();
        let token_provider: Arc<dyn oauth::OAuthTokenProvider> =
            Arc::new(McpOAuthTokenProvider::new(
                tokens,
                discovery.clone(),
                http_client.clone(),
                Some(token_refresh_tx),
            ));

        // Spawn a fire-and-forget task that persists tokens whenever the
        // provider successfully refreshes them mid-session.
        {
            let credentials_provider = credentials_provider.clone();
            let server_url = server_url.clone();
            let id = id.clone();
            cx.spawn(async move |cx| {
                let mut token_refresh_rx = token_refresh_rx;
                while let Some(refreshed_tokens) = token_refresh_rx.next().await {
                    if let Err(err) = Self::store_tokens(
                        &credentials_provider,
                        &server_url,
                        &refreshed_tokens,
                        &cx,
                    )
                    .await
                    {
                        log::warn!("{} failed to persist refreshed tokens: {}", id, err);
                    }
                }
            })
            .detach();
        }

        let new_server = this.update(cx, |this, cx| {
            let global_timeout =
                Self::resolve_project_settings(&this.worktree_store, cx).context_server_timeout;

            match configuration.as_ref() {
                ContextServerConfiguration::Http {
                    url,
                    headers,
                    timeout,
                } => {
                    let transport = HttpTransport::new_with_token_provider(
                        http_client.clone(),
                        url.to_string(),
                        headers.clone(),
                        cx.background_executor().clone(),
                        Some(token_provider),
                    );
                    Ok(Arc::new(ContextServer::new_with_timeout(
                        id.clone(),
                        Arc::new(transport),
                        Some(Duration::from_secs(
                            timeout.unwrap_or(global_timeout).min(MAX_TIMEOUT_SECS),
                        )),
                    )))
                }
                _ => anyhow::bail!("OAuth authentication only supported for HTTP servers"),
            }
        })??;

        this.update(cx, |this, cx| {
            this.run_server(new_server, configuration, cx);
        })?;

        Ok(())
    }

    /// Store OAuth tokens in the system keychain, keyed by the server's
    /// canonical URI.
    async fn store_tokens(
        credentials_provider: &Arc<dyn CredentialsProvider>,
        server_url: &url::Url,
        tokens: &OAuthTokens,
        cx: &AsyncApp,
    ) -> Result<()> {
        let key = Self::keychain_key(server_url);
        let json = serde_json::to_string(tokens)?;
        credentials_provider
            .write_credentials(&key, "mcp-oauth", json.as_bytes(), cx)
            .await
    }

    /// Load OAuth tokens from the system keychain for the given server URL.
    async fn load_tokens(
        credentials_provider: &Arc<dyn CredentialsProvider>,
        server_url: &url::Url,
        cx: &AsyncApp,
    ) -> Result<Option<OAuthTokens>> {
        let key = Self::keychain_key(server_url);
        match credentials_provider.read_credentials(&key, cx).await? {
            Some((_username, password_bytes)) => {
                let tokens: OAuthTokens = serde_json::from_slice(&password_bytes)?;
                Ok(Some(tokens))
            }
            None => Ok(None),
        }
    }

    /// Clear stored OAuth tokens from the system keychain.
    async fn clear_tokens(
        credentials_provider: &Arc<dyn CredentialsProvider>,
        server_url: &url::Url,
        cx: &AsyncApp,
    ) -> Result<()> {
        let key = Self::keychain_key(server_url);
        credentials_provider.delete_credentials(&key, cx).await
    }

    fn keychain_key(server_url: &url::Url) -> String {
        format!("mcp-oauth:{}", oauth::canonical_server_uri(server_url))
    }

    fn dcr_keychain_key(server_url: &url::Url) -> String {
        format!(
            "mcp-oauth-dcr-client:{}",
            oauth::canonical_server_uri(server_url)
        )
    }

    /// Persist a DCR client registration in the system keychain so we reuse the
    /// same client_id across restarts instead of minting a new one each time.
    async fn store_dcr_registration(
        credentials_provider: &Arc<dyn CredentialsProvider>,
        server_url: &url::Url,
        registration: &OAuthClientRegistration,
        cx: &AsyncApp,
    ) -> Result<()> {
        let key = Self::dcr_keychain_key(server_url);
        let json = serde_json::to_string(registration)?;
        credentials_provider
            .write_credentials(&key, "mcp-oauth-dcr", json.as_bytes(), cx)
            .await
    }

    /// Load a previously cached DCR client registration from the keychain.
    async fn load_dcr_registration(
        credentials_provider: &Arc<dyn CredentialsProvider>,
        server_url: &url::Url,
        cx: &AsyncApp,
    ) -> Result<Option<OAuthClientRegistration>> {
        let key = Self::dcr_keychain_key(server_url);
        match credentials_provider.read_credentials(&key, cx).await? {
            Some((_username, password_bytes)) => {
                let registration: OAuthClientRegistration =
                    serde_json::from_slice(&password_bytes)?;
                Ok(Some(registration))
            }
            None => Ok(None),
        }
    }

    /// Clear the cached DCR client registration from the keychain.
    async fn clear_dcr_registration(
        credentials_provider: &Arc<dyn CredentialsProvider>,
        server_url: &url::Url,
        cx: &AsyncApp,
    ) -> Result<()> {
        let key = Self::dcr_keychain_key(server_url);
        credentials_provider.delete_credentials(&key, cx).await
    }

    /// Log out of an OAuth-authenticated MCP server: clear stored tokens from
    /// the keychain and stop the server.
    pub fn logout_server(&mut self, id: &ContextServerId, cx: &mut Context<Self>) -> Result<()> {
        let state = self.servers.get(id).context("Context server not found")?;
        let configuration = state.configuration();

        let server_url = match configuration.as_ref() {
            ContextServerConfiguration::Http { url, .. } => url.clone(),
            _ => anyhow::bail!("logout only applies to HTTP servers with OAuth"),
        };

        let id = id.clone();
        self.stop_server(&id, cx)?;

        cx.spawn(async move |_this, cx| {
            let credentials_provider = cx.update(|cx| <dyn CredentialsProvider>::global(cx));
            if let Err(err) = Self::clear_tokens(&credentials_provider, &server_url, &cx).await {
                log::error!("{} failed to clear OAuth tokens: {}", id, err);
            }
            if let Err(err) =
                Self::clear_dcr_registration(&credentials_provider, &server_url, &cx).await
            {
                log::error!("{} failed to clear cached DCR registration: {}", id, err);
            }
        })
        .detach();

        Ok(())
    }


```


## Assistant

Now I have a thorough understanding of both branches. Let me also quickly check the startup flow with cached tokens:

**Tool Call: Read file `crates/project/src/context_server_store.rs` (lines 787-945)**
Status: Completed

```zed/crates/project/src/context_server_store.rs
    pub async fn create_context_server(
        this: WeakEntity<Self>,
        id: ContextServerId,
        configuration: Arc<ContextServerConfiguration>,
        cx: &mut AsyncApp,
    ) -> Result<(Arc<ContextServer>, Arc<ContextServerConfiguration>)> {
        let remote = configuration.remote();
        let needs_remote_command = match configuration.as_ref() {
            ContextServerConfiguration::Custom { .. }
            | ContextServerConfiguration::Extension { .. } => remote,
            ContextServerConfiguration::Http { .. } => false,
        };

        let (remote_state, is_remote_project) = this.update(cx, |this, _| {
            let remote_state = match &this.state {
                ContextServerStoreState::Remote {
                    project_id,
                    upstream_client,
                } if needs_remote_command => Some((*project_id, upstream_client.clone())),
                _ => None,
            };
            (remote_state, this.is_remote_project())
        })?;

        let root_path: Option<Arc<Path>> = this.update(cx, |this, cx| {
            this.project
                .as_ref()
                .and_then(|project| {
                    project
                        .read_with(cx, |project, cx| project.active_project_directory(cx))
                        .ok()
                        .flatten()
                })
                .or_else(|| {
                    this.worktree_store.read_with(cx, |store, cx| {
                        store.visible_worktrees(cx).fold(None, |acc, item| {
                            if acc.is_none() {
                                item.read(cx).root_dir()
                            } else {
                                acc
                            }
                        })
                    })
                })
        })?;

        let configuration = if let Some((project_id, upstream_client)) = remote_state {
            let root_dir = root_path.as_ref().map(|p| p.display().to_string());

            let response = upstream_client
                .update(cx, |client, _| {
                    client
                        .proto_client()
                        .request(proto::GetContextServerCommand {
                            project_id,
                            server_id: id.0.to_string(),
                            root_dir: root_dir.clone(),
                        })
                })
                .await?;

            let remote_command = upstream_client.update(cx, |client, _| {
                client.build_command(
                    Some(response.path),
                    &response.args,
                    &response.env.into_iter().collect(),
                    root_dir,
                    None,
                )
            })?;

            let command = ContextServerCommand {
                path: remote_command.program.into(),
                args: remote_command.args,
                env: Some(remote_command.env.into_iter().collect()),
                timeout: None,
            };

            Arc::new(ContextServerConfiguration::Custom { command, remote })
        } else {
            configuration
        };

        // For HTTP servers, try to load cached tokens from the keychain so we
        // can attach a bearer token on the very first request.
        let cached_token_provider: Option<Arc<dyn oauth::OAuthTokenProvider>> =
            if let ContextServerConfiguration::Http { url, .. } = configuration.as_ref() {
                let credentials_provider = cx.update(|cx| <dyn CredentialsProvider>::global(cx));
                match Self::load_tokens(&credentials_provider, url, &cx).await {
                    Ok(Some(tokens)) => {
                        log::info!("{} loaded cached OAuth tokens from keychain", id);
                        Some(Arc::new(oauth::StaticTokenProvider::new(
                            tokens.access_token,
                        )))
                    }
                    Ok(None) => None,
                    Err(err) => {
                        log::warn!("{} failed to load cached OAuth tokens: {}", id, err);
                        None
                    }
                }
            } else {
                None
            };

        let server: Arc<ContextServer> = this.update(cx, |this, cx| {
            let global_timeout =
                Self::resolve_project_settings(&this.worktree_store, cx).context_server_timeout;

            if let Some(factory) = this.context_server_factory.as_ref() {
                return anyhow::Ok(factory(id.clone(), configuration.clone()));
            }

            match configuration.as_ref() {
                ContextServerConfiguration::Http {
                    url,
                    headers,
                    timeout,
                } => {
                    let transport = HttpTransport::new_with_token_provider(
                        cx.http_client(),
                        url.to_string(),
                        headers.clone(),
                        cx.background_executor().clone(),
                        cached_token_provider,
                    );
                    anyhow::Ok(Arc::new(ContextServer::new_with_timeout(
                        id,
                        Arc::new(transport),
                        Some(Duration::from_secs(
                            timeout.unwrap_or(global_timeout).min(MAX_TIMEOUT_SECS),
                        )),
                    )))
                }
                _ => {
                    let mut command = configuration
                        .command()
                        .context("Missing command configuration for stdio context server")?
                        .clone();
                    command.timeout = Some(
                        command
                            .timeout
                            .unwrap_or(global_timeout)
                            .min(MAX_TIMEOUT_SECS),
                    );

                    // Don't pass remote paths as working directory for locally-spawned processes
                    let working_directory = if is_remote_project { None } else { root_path };
                    anyhow::Ok(Arc::new(ContextServer::stdio(
                        id,
                        command,
                        working_directory,
                    )))
                }
            }
        })??;

        Ok((server, configuration))
    }

```


## Assistant

I now have a comprehensive picture. Here's the review:

---

## Branch comparison: `mcp-auth` (domain expert) vs. current branch (LLM-driven)

### Architecture: fundamentally different approaches to state ownership

The biggest difference between the two branches is **where the OAuth state machine lives**.

**mcp-auth** puts an `OAuthClient` struct (with a stateful `State` enum: `Unauthenticated → WaitingForCode → Authenticated`) directly inside `HttpTransport`, behind an `Arc<Mutex<Option<OAuthClient>>>`. The transport itself drives the full flow: `start_auth()` → `exchange_token()` → token refresh, and it persists the entire serializable `ContextServerCredentials` blob (registration + server metadata + tokens + state) to the keychain after every state change via an `on_auth_updated` callback.

**Current branch** separates concerns more sharply: the transport only holds a trait-object `Option<Arc<dyn OAuthTokenProvider>>` and knows nothing about the flow. The full OAuth lifecycle (discovery, PKCE, callback server, token exchange) is driven by `context_server_store.rs` in `run_oauth_flow()`, and the `oauth.rs` module is a bag of pure functions + async helpers with no mutable state.

#### Tradeoffs

| | mcp-auth | Current branch |
|---|---|---|
| **Stateful vs. functional** | Encapsulated state machine in `OAuthClient` — cleaner ownership, fewer moving parts | Stateless functions composed in the store — more testable in isolation, but the store's `run_oauth_flow` method is very procedural |
| **Keychain persistence** | Serializes the *entire* client state (registration + server metadata + auth state) as one blob. Simple, but couples keychain schema to internal representation. | Stores tokens and DCR registrations as separate keychain entries with canonical URI keys. More granular, but more surface area. |
| **Re-auth on restart** | Restores full `OAuthClient` from keychain → can refresh immediately because server metadata is cached | Uses `StaticTokenProvider` on startup (token only, no refresh capability). If the token is expired, falls through to 401 → full rediscovery. Slightly worse cold-start experience for expired tokens, but avoids persisting potentially stale server metadata. |
| **Callback mechanism** | Uses `zed://mcp/oauth/callback` — registered URL scheme, routed through the open_listener. Requires changes to `main.rs` and `open_listener.rs`. | Runs a local `127.0.0.1` TCP server on an ephemeral port. No changes to the URL scheme handler. |

### What mcp-auth does better

1. **The `zed://` callback scheme is arguably more robust.** The local TCP server on the current branch has failure modes: firewall rules, port conflicts, the browser might warn about `127.0.0.1`. The custom URL scheme is the idiomatic desktop OAuth approach. The mcp-auth branch registers `zed://mcp/oauth/callback` in `open_listener.rs` and wires it through `main.rs`. The current branch avoids this at the cost of robustness.

2. **Richer `_supported` field handling.** mcp-auth defines proper enums for `GrantType`, `ResponseType`, `CodeChallengeMethod`, `BearerMethod`, `TokenEndpointAuthMethod` — each with typed `Supports` wrappers that have correct default semantics per the relevant RFCs (e.g. `GrantTypesSupported` defaults to `[AuthorizationCode, Implicit]`). The current branch treats `code_challenge_methods_supported` as `Option<Vec<String>>` and checks it with `.iter().any(|m| m == "S256")`. The mcp-auth approach is more maintainable and catches more incompatible-server situations.

3. **The `AbsUri` newtype** on mcp-auth prevents accidentally passing a relative URI where an absolute one is required — a real footgun when dealing with multiple metadata URLs. The current branch uses `Url` everywhere, which is always absolute, so this is less of an issue in practice (since `url::Url` is always absolute), but the discipline is still good.

4. **`BearerError` and `invalid_token` handling.** mcp-auth parses the `error` field from `WWW-Authenticate` into a proper enum and, when it sees `invalid_token`, discards the existing `OAuthClient` to force re-registration. The current branch parses `error` as a raw string and doesn't act on it — if a server says your *client registration* is invalid (as opposed to the token), the current branch would keep trying to refresh with the stale credentials.

5. **`AwaitingAuthorization` as a visible status.** mcp-auth has `ContextServerAuthStatus::AwaitingAuthorization` that drives a spinning-loader icon in the UI while the user is in the browser. The current branch doesn't track this intermediate state.

6. **Fewer moving parts for token persistence.** mcp-auth's `on_auth_updated` callback fires from the transport whenever state changes, and a single channel + task in the store drains it to the keychain. The current branch has a separate `mpsc` channel from the `McpOAuthTokenProvider` for refresh persistence, plus separate `store_tokens` calls during the initial exchange — two persistence paths that must stay in sync.

7. **WWW-Authenticate parser quality.** mcp-auth's parser in `www_authenticate.rs` is more correct: it handles quoted-string escapes (`\\`, `\"`), stops at subsequent Bearer challenges, uses proper `tchar` matching per RFC 7230 for token parsing, and has thorough edge-case tests. The current branch's `parse_auth_params` is simpler but would misbehave on escaped characters in quoted strings (it handles `\\` and `\"` in the value but doesn't reset character-iteration state properly for all cases).

### What the current branch does better

1. **Much more testable.** 52 tests in `oauth.rs` alone, covering pure logic and async I/O separately. The functional decomposition means each piece (URL construction, scope selection, PKCE generation, auth URL building, token params) can be tested in isolation with `#[test]`. mcp-auth's tests are all `#[gpui::test]` requiring `TestAppContext` and `BackgroundExecutor` even for pure logic, because everything is methods on `OAuthClient`.

2. **`AuthRequired` as a first-class status variant** on `ContextServerStatus` (not nested inside `Running`). On mcp-auth, a server that needs auth shows as `Running(ContextServerAuthStatus::Required)`, which is conceptually odd — the server isn't really "running." The current branch's `ContextServerStatus::AuthRequired(Arc<OAuthDiscovery>)` is cleaner, and the discovery info is directly available to the UI handler.

3. **DCR client ID caching is explicit.** The current branch has separate keychain entries for DCR registrations (`mcp-oauth-dcr-client:<uri>`) and tokens (`mcp-oauth:<uri>`). On mcp-auth, the DCR registration is buried inside the serialized `ContextServerCredentials` blob. The current branch's `logout_server` explicitly clears both, and `discover()` takes `cached_dcr_registration` as an explicit parameter — easier to reason about.

4. **No changes to the URL scheme / open_listener.** This is a double-edged sword (see above), but it does mean fewer files touched and no coupling between the OAuth flow and Zed's global URL routing. For a feature that might not ship immediately, keeping the blast radius small has value.

5. **`resource` parameter in token requests.** The current branch includes the RFC 8707 `resource` parameter in both token exchange and refresh requests (`token_exchange_params` and `token_refresh_params`). mcp-auth does not include `resource` in its token requests. This is a spec compliance gap in mcp-auth — the MCP 2025-11-25 spec explicitly requires this.

6. **Clean separation of transport and OAuth logic.** The `OAuthTokenProvider` trait is a narrow interface (`access_token()` + `try_refresh()`). This means the transport code barely changed — it's a small, targeted addition. On mcp-auth, the transport's `send_message` method directly calls into `oauth_client.access_token().await` and handles auth state transitions inline, making the transport harder to reason about.

### Potential issues on both branches

- **Neither branch handles concurrent 401s well.** If multiple requests are in flight and they all get 401s, the current branch would try refreshing tokens multiple times in parallel (the `SyncMutex` doesn't prevent concurrent `try_refresh` calls). mcp-auth has the same issue but with `Mutex<Option<OAuthClient>>` which at least serializes access.

- **The hand-rolled SHA-256 in the current branch's `simple_sha256`** is a red flag. The comment says "avoids pulling in a crypto crate just for PKCE," but `sha2` is already in the dependency tree (mcp-auth uses it). This is a correctness risk — a subtle bug in a hand-rolled SHA-256 could produce valid-looking but wrong PKCE challenges that silently fail at the auth server. Use `sha2::Sha256` instead.

- **The hand-rolled base64url in the current branch** (`base64_url_encode`) is similar — `base64` crate with `URL_SAFE_NO_PAD` engine is already available and used by mcp-auth.

### Summary recommendation

The current branch has better test coverage, cleaner separation of concerns, and is closer to spec-complete (the `resource` parameter). But it should:

1. **Drop the hand-rolled SHA-256 and base64** in favor of the `sha2` and `base64` crates that are already in the dependency tree.
2. **Switch to `zed://mcp/oauth/callback`** instead of the local TCP server — the mcp-auth approach is more robust for real-world desktop OAuth.
3. **Port the richer `_supported` enum types** from mcp-auth (or at least `BearerError` handling for `invalid_token` → discard client).
4. **Consider persisting discovery/server metadata** alongside tokens so that restarts with expired tokens can attempt a refresh before falling back to full rediscovery.

The mcp-auth branch's core `OAuthClient` is well-structured domain modeling that would benefit from the current branch's testability approach. An ideal merge would take the current branch's functional decomposition and test harness, layer on mcp-auth's type safety for server metadata fields and its `zed://` callback scheme, and ditch the hand-rolled crypto.
