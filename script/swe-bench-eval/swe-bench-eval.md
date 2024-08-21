# SWE-Bench eval
## Zed "agent" flow
- Spin up Zed
  - Open project with repo at `base_commit`
  - Open assistant panel
- Add /workflow to context (e.g. in System message)
- (Out of band) LLM call to rephrase SWE-bench `problem_statement` into a Zed Assistant user query/prompt
  - Trying to simulate user prompt here
  - `user_query = rephrase(problem_statement)`
- Add `/auto {user_query}` to populate context
- Store benchmark outputs:
  - `/file` calls + `/search` output
  - Overlap of these files/snippets with `patch`
  - [Stretch]: Overlap of these files/snippets within one-hop of `patch` (hop resolved via LSP go-to-impl call)
- Add `user_query` at end of assistant context
- Run assistant on context
- Apply workflow step resolution
- Apply inline-edits
- Store benchmark outputs:
  - Success/failure of step resolution
  - Success/failure of "proper" indentation of inline edit
  - Success/failure of "overgeneration" of inline edit
- Finally, run tests from test_patch, observe results of `PASS_TO_PASS` + `FAIL_TO_PASS` tests
- Store benchmark outputs:
  - Number of patch files modified: all/any/none
  - Success/failure of `PASS_TO_PASS` + `FAIL_TO_PASS` tests

  ## Things to Report
  - Rephrased user query (for test case validity)

  ### /workflow
  - Step resolution: OK/fail
  - Proper indents in inline edits: OK/fail per edit
  - Overgeneration in inline edits: OK/fail per edit
  - Number of patch files modified: all/any/none
  - Success/failure of `PASS_TO_PASS` + `FAIL_TO_PASS` tests: OK/fail
  ### /auto {problem_statement}:
  - Overlap of `/file` + `/search` outputs with `patch` snippets
  - Overlap of these files/snippets within one-hop of `patch`
