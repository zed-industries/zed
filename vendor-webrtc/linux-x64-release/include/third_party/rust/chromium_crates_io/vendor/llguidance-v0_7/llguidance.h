#ifndef LLGUIDANCE_H
#define LLGUIDANCE_H

#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Do not include special tokens, and keep invalid UTF-8 as is.
 */
#define LLG_DECODE_NONE 0

/**
 * Include special tokens in the output.
 * They may look like <|something|>, <something_else>, or <[12345]> if they don't have a name.
 */
#define LLG_DECODE_INCLUDE_SPECIAL 1

/**
 * Replace invalid UTF-8 with the replacement character.
 */
#define LLG_DECODE_VALID_UTF8 2

typedef struct LlgConstraint LlgConstraint;

typedef struct LlgMatcher LlgMatcher;

typedef struct LlgStopController LlgStopController;

typedef struct LlgTokenizer LlgTokenizer;

typedef struct NodeRef NodeRef;

typedef struct LlgParserLimits {
  /**
   * For non-ambiguous grammars, this is the maximum "branching factor" of the grammar.
   * For ambiguous grammars, this might get hit much quicker.
   * Default: 2000
   */
  size_t max_items_in_row;
  /**
   * How much "fuel" are we willing to spend to build initial lexer regex AST nodes.
   * Default: 1_000_000
   * Speed: 50k/ms
   */
  uint64_t initial_lexer_fuel;
  /**
   * Maximum lexer fuel for computation of the whole token mask.
   * Default: 200_000
   * Speed: 14k/ms
   */
  uint64_t step_lexer_fuel;
  /**
   * Number of Earley items created for the whole token mask.
   * Default: 50_000
   * Speed: 20k/ms
   */
  size_t step_max_items;
  /**
   * Maximum number of lexer states.
   * Affects memory consumption, but not the speed for the most part.
   * Default: 250_000
   * Speed: ~1-2kB of memory per state
   */
  size_t max_lexer_states;
  /**
   * Maximum size of the grammar (symbols in productions)
   * Default: 500_000 (a few megabytes of JSON)
   */
  size_t max_grammar_size;
  /**
   * If true, we'll run any extremely large regexes against the whole
   * trie of the tokenizer while constructing the lexer.
   * This reduces future mask computation time, but increases
   * the time it takes to construct the lexer.
   * Default: true
   */
  bool precompute_large_lexemes;
} LlgParserLimits;

typedef struct LlgConstraintInit {
  /**
   * The tokenizer to use, created with llg_new_tokenizer()
   */
  const struct LlgTokenizer *tokenizer;
  /**
   * The log level for the buffer that is kept inside of the constraint
   * 0 - no logging, 1 - warnings only, 2 - info
   */
  uint32_t log_buffer_level;
  /**
   * The log level for writing to stderr
   */
  uint32_t log_stderr_level;
  /**
   * Does the engine support fast-forward tokens?
   * (Appending more than one token to output at once)
   */
  bool ff_tokens_ok;
  /**
   * Does the engine support backtracking?
   * (Removing tokens from the output)
   */
  bool backtrack_ok;
  /**
   * The resource limits for the parser
   * Default values will be used for all fields that are 0
   */
  struct LlgParserLimits limits;
} LlgConstraintInit;

typedef struct LlgMaskResult {
  /**
   * One bit per vocab token
   * This is valid until any call to llg_*() on the current constraint
   */
  const uint32_t *sample_mask;
  /**
   * Temperature to use for sampling
   */
  float temperature;
  /**
   * Should the sequence stop?
   */
  bool is_stop;
} LlgMaskResult;

typedef uint32_t LlgToken;

/**
 * Represents result from llg_commit_token()
 */
typedef struct LlgCommitResult {
  /**
   * The tokens to append to the output if any
   * This is valid until any call to llg_*() on the current constraint
   */
  const uint32_t *tokens;
  /**
   * The number of tokens in the tokens array (can be 0)
   */
  uint32_t n_tokens;
  /**
   * Should the sequence stop?
   */
  bool is_stop;
} LlgCommitResult;

typedef struct LlgConstraintStep {
  /**
   * The constraint to compute mask for.
   */
  struct LlgConstraint *constraint;
  /**
   * Pointer to memory where the mask should be written.
   */
  uint32_t *mask_dest;
  /**
   * The length of the mask_dest array in bytes (not elements).
   */
  size_t mask_byte_len;
} LlgConstraintStep;

/**
 * Function which llg calls when an operation is done.
 */
typedef void (*LlgCallback)(const void *user_data);

/**
 * Tokenization function
 * Will not write more than output_tokens_len tokens (which can be 0)
 * Returns the total number of tokens (which can be more than output_tokens_len)
 * This function has to be thread-safe!
 */
typedef size_t (*LlgTokenizeFn)(const void *user_data,
                                const uint8_t *bytes,
                                size_t bytes_len,
                                uint32_t *output_tokens,
                                size_t output_tokens_len);

typedef struct LlgTokenizerInit {
  /**
   * The number of tokens in the vocabulary
   */
  uint32_t vocab_size;
  /**
   * The token ID for the end of sentence token
   * For chat mode, set it to end-of-turn token
   */
  LlgToken tok_eos;
  /**
   * An array of the lengths of the token strings (vocab_size elements)
   */
  const uint32_t *token_lens;
  /**
   * A pointer to the token strings
   * The length of this the sum of all token_lens
   */
  const uint8_t *token_bytes;
  /**
   * Instead of passing token_lens and token_bytes, this can be set to
   * the contents of HF tokenizer.json file.
   */
  const char *tokenizer_json;
  /**
   * Set to true to enable hack that works around the tokenize_fn only
   * accepting valid UTF-8 strings and possibly adding <BOS> etc.
   * TODO: the <BOS> bit not implemented yet
   */
  bool tokenize_assumes_string;
  /**
   * Tokenization function, see LlgTokenizeFn docs.
   * It should only tokenize the bytes and not add
   * any <BOS> etc. It should also work on any byte sequence, including
   * invalid UTF-8. If this is not the case, set tokenize_assumes_string to true.
   * Either way, this function has to be thread-safe!
   */
  LlgTokenizeFn tokenize_fn;
  /**
   * Set to true to not use tokenize_fn and instead tokenize greedily,
   * which is often incorrect and may reduce accuracy.
   */
  bool use_approximate_greedy_tokenize_fn;
  /**
   * User data to pass to the tokenize_fn
   */
  const void *tokenize_user_data;
  /**
   * Tokenizer partitions for the slicer optimization.
   * This is array of pointers to strings, terminated with NULL (argv style).
   * Pass NULL to use defaults. Pass empty array to disable.
   */
  const char *const *slices;
} LlgTokenizerInit;



#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * Set the default values for the ConstraintInit
 * Disables ff_tokens and backtracking, enables warnings on stderr
 * and all logging to the buffer (get with llg_flush_logs()).
 * You need to set the tokenizer field manually.
 */
void llg_constraint_init_set_defaults(struct LlgConstraintInit *init,
                                      const struct LlgTokenizer *tokenizer);

/**
 * Create a new constraint from a grammar JSON string
 * Always returns a non-null value. Call llg_get_error() on the result to check for errors.
 */
struct LlgConstraint *llg_new_constraint(const struct LlgConstraintInit *init,
                                         const char *llguidance);

/**
 * Create a new constraint from a given regular expression
 * Always returns a non-null value. Call llg_get_error() on the result to check for errors.
 */
struct LlgConstraint *llg_new_constraint_regex(const struct LlgConstraintInit *init,
                                               const char *regex);

/**
 * Create a new constraint from a given JSON schema
 * Always returns a non-null value. Call llg_get_error() on the result to check for errors.
 */
struct LlgConstraint *llg_new_constraint_json(const struct LlgConstraintInit *init,
                                              const char *json_schema);

/**
 * Create a new constraint from a given lark grammar
 * Always returns a non-null value. Call llg_get_error() on the result to check for errors.
 */
struct LlgConstraint *llg_new_constraint_lark(const struct LlgConstraintInit *init,
                                              const char *lark);

/**
 * Create a new constraint with specified type
 * Type can be one of "regex", "json_schema" (or "json"), "lark", "llguidance" (or "guidance")
 * Always returns a non-null value. Call llg_get_error() on the result to check for errors.
 */
struct LlgConstraint *llg_new_constraint_any(const struct LlgConstraintInit *init,
                                             const char *constraint_type,
                                             const char *data);

/**
 * Get the error message from the constraint or null if there is no error.
 * After it returns a non-null value, it will always return it until the constraint is freed
 * using llg_free_constraint() (at which point the pointer will be invalid).
 */
const char *llg_get_error(const struct LlgConstraint *cc);

/**
 * Get the current temperature of the constraint.
 * It is updated by mask computation.
 */
float llg_get_temperature(const struct LlgConstraint *cc);

/**
 * Check if constraint is stopped (cannot be extended further).
 */
bool llg_is_stopped(const struct LlgConstraint *cc);

/**
 * Compute mask for the next token sampling
 * It typically takes up to a millisecond for a 100k tokenizer, so should be called in background.
 * Returns 0 on success and -1 on error (use llg_get_error() to get the exact error).
 * When 0 is returned, the result is written to *res_p.
 */
int32_t llg_compute_mask(struct LlgConstraint *cc, struct LlgMaskResult *res_p);

/**
 * Commit the token sampled with the mask returned from llg_compute_mask().
 * Can be run on the critical path of sampling (is fast).
 * Returns 0 on success and -1 on error (use llg_get_error() to get the exact error).
 * When 0 is returned, the result is written to *res_p.
 */
int32_t llg_commit_token(struct LlgConstraint *cc, LlgToken token, struct LlgCommitResult *res_p);

/**
 * Compute mask for several constraints in parallel.
 */
void llg_par_compute_mask(const struct LlgConstraintStep *steps,
                          size_t n_steps,
                          const void *user_data,
                          LlgCallback done_cb);

/**
 * Clone the constraint
 */
struct LlgConstraint *llg_clone_constraint(const struct LlgConstraint *cc);

/**
 * Construct a new tokenizer from the given TokenizerInit
 */
struct LlgTokenizer *llg_new_tokenizer(const struct LlgTokenizerInit *tok_init,
                                       char *error_string,
                                       size_t error_string_len);

/**
 * Clone a tokenizer.
 * This increments a reference count and does a small allocation.
 */
struct LlgTokenizer *llg_clone_tokenizer(const struct LlgTokenizer *tok);

/**
 * Tokenize the given bytes and return the tokens.
 * Always returns the number of tokens that would be written to output_tokens
 * if output_tokens_len was large enough.
 */
size_t llg_tokenize_bytes(const struct LlgTokenizer *tok,
                          const uint8_t *bytes,
                          size_t bytes_len,
                          uint32_t *output_tokens,
                          size_t output_tokens_len);

/**
 * Tokenize the given bytes and return the tokens.
 * Special tokens will be tokenized, if they follow 0xFF byte prefix.
 * Always returns the number of tokens that would be written to output_tokens
 * if output_tokens_len was large enough.
 */
size_t llg_tokenize_bytes_marker(const struct LlgTokenizer *tok,
                                 const uint8_t *bytes,
                                 size_t bytes_len,
                                 uint32_t *output_tokens,
                                 size_t output_tokens_len);

/**
 * Return a string representation of the tokens, useful for debugging.
 * The output is NUL-terminated.
 * Returns the number of bytes that would be written to output if output_len was large enough.
 */
size_t llg_stringify_tokens(const struct LlgTokenizer *tok,
                            const uint32_t *tokens,
                            size_t n_tokens,
                            char *output,
                            size_t output_len);

/**
 * Return a string representation of the tokens, useful for debugging.
 * The output is NUL-terminated.
 * Returns the number of bytes that would be written to output if output_len was large enough.
 * flags is one of LLG_DECODE_*
 */
size_t llg_decode_tokens(const struct LlgTokenizer *tok,
                         const uint32_t *tokens,
                         size_t n_tokens,
                         char *output,
                         size_t output_len,
                         uint32_t flags);

/**
 * Free the tokenizer. Should *NOT* be called while there are still constraints using it.
 */
void llg_free_tokenizer(struct LlgTokenizer *tok);

/**
 * Free the constraint
 */
void llg_free_constraint(struct LlgConstraint *cc);

/**
 * Get the logs from the constraint, since last call to this function.
 * The logs are null-terminated.
 * The logs are kept in the constraint until the next call to this function
 * or until the constraint is freed.
 */
const char *llg_flush_logs(struct LlgConstraint *cc);

/**
 * Create a new stop-sequence controller
 */
struct LlgStopController *llg_new_stop_controller(const struct LlgTokenizer *tokenizer,
                                                  const uint32_t *stop_tokens,
                                                  size_t stop_tokens_len,
                                                  const char *stop_rx,
                                                  char *error_string,
                                                  size_t error_string_len);

/**
 * Commit a token to the stop-sequence controller.
 * Returns a valid utf8 string to be returned to the user (which can be empty)
 * and whether the sequence should be then finished.
 * The string is valid until the next call to this function, or until the stop-sequence controller is freed.
 */
const char *llg_stop_commit_token(struct LlgStopController *stop_ctrl,
                                  uint32_t token,
                                  size_t *output_len_p,
                                  bool *is_stopped_p);

/**
 * Free the stop-sequence controller
 */
void llg_free_stop_controller(struct LlgStopController *stop_ctrl);

/**
 * Create a new matcher from the given ConstraintInit
 * Always returns a non-null value. Call llg_matcher_get_error() on the result to check for errors.
 * init.ff_tokens_ok and init.backtrack_ok are ignored
 * (backtracking is always disabled, and ff_tokens can be retrieved using llg_matcher_compute_ff_tokens()).
 * The data is of different format, depending on constraint_type:
 * - "regex" - data is regular expression in rust regex format
 *   see https://docs.rs/regex/latest/regex/#syntax
 * - "json" or "json_schema" - data is (stringifed) JSON schema
 *   see https://github.com/guidance-ai/llguidance/blob/main/docs/json_schema.md
 * - "json_object" - equivalent to JSON schema: {"type":"object"}
 * - "lark" - data is grammar in a variant of Lark syntax
 *   see https://github.com/guidance-ai/llguidance/blob/main/docs/syntax.md
 * - "llguidance" or "guidance" - data is a list of Lark or JSON schemas in JSON format
 */
struct LlgMatcher *llg_new_matcher(const struct LlgConstraintInit *init,
                                   const char *constraint_type,
                                   const char *data);

/**
 * Check if given grammar is valid.
 * This about twice as fast as creating a matcher (which also validates).
 * See llg_new_matcher() for the grammar format.
 * Returns 0 on success and -1 on error and 1 on warning.
 * The error message or warning is written to message, which is message_len bytes long.
 * It's always NUL-terminated.
 */
int32_t llg_validate_grammar(const struct LlgConstraintInit *init,
                             const char *constraint_type,
                             const char *data,
                             char *message,
                             size_t message_len);

/**
 * Compute the set of allowed tokens for the current state.
 * The result is written to mask_dest.
 * mask_byte_len must be equal to llg_matcher_get_mask_byte_size().
 * Returns 0 on success and -1 on error.
 */
int32_t llg_matcher_compute_mask_into(struct LlgMatcher *matcher,
                                      uint32_t *mask_dest,
                                      size_t mask_byte_len);

/**
 * Compute the set of allowed tokens for the current state.
 * Use llg_matcher_get_mask() to get the result.
 * Returns 0 on success and -1 on error.
 */
int32_t llg_matcher_compute_mask(struct LlgMatcher *matcher);

/**
 * Return pointer to the mask computed by llg_matcher_compute_mask(), if any.
 */
const uint32_t *llg_matcher_get_mask(struct LlgMatcher *matcher);

/**
 * Return pointer to the mask computed by llg_matcher_compute_mask(), if any.
 */
size_t llg_matcher_get_mask_byte_size(struct LlgMatcher *matcher);

/**
 * Advance the matcher by one token.
 * Returns 0 on success and -1 on error.
 */
int32_t llg_matcher_consume_token(struct LlgMatcher *matcher, uint32_t token);

/**
 * Advance the matcher by several tokens.
 * Returns 0 on success and -1 on error.
 */
int32_t llg_matcher_consume_tokens(struct LlgMatcher *matcher,
                                   const uint32_t *tokens,
                                   size_t n_tokens);

/**
 * Get the error message from the matcher or null if there is no error.
 * After it returns a non-null value, it will always return it until the matcher is freed
 * using llg_free_matcher() (at which point the pointer will be invalid).
 */
const char *llg_matcher_get_error(struct LlgMatcher *matcher);

/**
 * Check if the matcher is in an error state.
 */
bool llg_matcher_is_error(struct LlgMatcher *matcher);

/**
 * Free the matcher.
 */
void llg_free_matcher(struct LlgMatcher *matcher);

/**
 * Backtracks the matcher states by num_tokens.
 * Returns 0 on success and -1 on error.
 */
int32_t llg_matcher_rollback(struct LlgMatcher *matcher, size_t num_tokens);

/**
 * Resets the matcher to the initial state.
 * A matcher in error state cannot be reset.
 * Returns 0 on success and -1 on error.
 */
int32_t llg_matcher_reset(struct LlgMatcher *matcher);

/**
 * Check if the grammar can fully accept the input.
 */
bool llg_matcher_is_accepting(struct LlgMatcher *matcher);

/**
 * Check if the matcher will force EOS token.
 * This returns true also in error state, as that is a forced stop.
 */
bool llg_matcher_is_stopped(const struct LlgMatcher *matcher);

/**
 * Check how many tokens can be consumed from the given tokens.
 * Returns the number of tokens that can be consumed, or -1 on error.
 */
int32_t llg_matcher_validate_tokens(struct LlgMatcher *matcher,
                                    const uint32_t *tokens,
                                    size_t n_tokens);

/**
 * Compute the fast-forward (forced) tokens for the current state.
 * The result is written to output.
 * Returns the number of tokens written to output (which can be 0) or -1 on error.
 */
int32_t llg_matcher_compute_ff_tokens(struct LlgMatcher *matcher,
                                      uint32_t *output,
                                      size_t output_len);

/**
 * Clone the matcher.
 */
struct LlgMatcher *llg_clone_matcher(const struct LlgMatcher *matcher);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  /* LLGUIDANCE_H */
