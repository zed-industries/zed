**What We Learned About Testing with LLMs**

1. **Some problems need deterministic tests**
   - Parsing, algorithms, indentation
   - Use traditional randomized testing

2. **Some problems need statistical tests**
   - LLM behavior, prompt effectiveness
   - Accept thresholds, not perfection

3. **Accept imperfection, build resilience**
   - 95% success might be the best you get
   - Build systems that handle failures gracefully
   - Clear error messages when things go wrong

3. **Prompt tweaking without evals is flying blind**
   - Every instruction exists because an eval failed
   - Measure impact statistically
   - You can't reason about LLM behavior
