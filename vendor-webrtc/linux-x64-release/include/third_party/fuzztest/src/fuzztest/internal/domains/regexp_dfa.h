// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_REGEXP_DFA_H_
#define FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_REGEXP_DFA_H_

#include <algorithm>
#include <cstddef>
#include <cstdint>
#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "absl/random/bit_gen_ref.h"
#include "absl/random/discrete_distribution.h"
#include "absl/random/distributions.h"
#include "absl/status/statusor.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/string_view.h"
#include "./fuzztest/internal/logging.h"
#include "re2/re2.h"

namespace fuzztest::internal {
// Represents the deterministic finite automaton (DFA) of a regular expression.
class RegexpDFA {
 public:
  struct State {
    bool is_end_state() const { return next.empty(); }
    // The special character `256` (kEndOfString) indicates the end of the input
    // string.
    struct StateTransition {
      std::vector<std::int16_t> chars_to_match;
      int next_state_id;
    };
    std::vector<StateTransition> next;
    // The weight reprensents the probablity of an edge being chosen during
    // random walk. The larger the weight, the higher chance the edge gets
    // picked. An edge transitioning to a node that is more likely to reach the
    // end state has larger weight than that doesn't.
    absl::discrete_distribution<int> edge_weight_distribution;
  };

  // `edge_index` is the index for the outgoing edge in State::next
  struct Edge {
    int from_state_id;
    int edge_index;
  };

  static RegexpDFA Create(absl::string_view regexp);

  std::string GenerateString(absl::BitGenRef prng) {
    std::string result;
    State* state = &states_[0];
    while (true) {
      FUZZTEST_INTERNAL_CHECK(!state->next.empty(), "Empty next state!");

      // Pick a random next state by weight.
      int rand_index = state->edge_weight_distribution(prng);
      auto& [next_string, next_id] = state->next[rand_index];
      state = &states_[next_id];
      result.insert(result.end(), next_string.begin(), next_string.end());
      if (next_string.back() == kEndOfString) {
        FUZZTEST_INTERNAL_CHECK(state->is_end_state(),
                                "EOS should lead to end state!");
        result.pop_back();
        break;
      }
    }
    return result;
  }

  // Randomly walk from the state of `from_state_id` to any state of
  // `to_state_ids` or an end state.
  std::vector<Edge> FindPath(
      absl::BitGenRef prng, int from_state_id,
      const std::vector<std::optional<int>>& to_state_ids) {
    std::vector<Edge> path;
    int cur_state_id = from_state_id;
    State* cur_state = &states_[cur_state_id];
    while (true) {
      FUZZTEST_INTERNAL_CHECK(!cur_state->next.empty(), "Empty next state!");

      // Pick a random next state.
      int offset = cur_state->edge_weight_distribution(prng);
      auto& [next_char, next_state_id] = cur_state->next[offset];
      path.push_back({cur_state_id, offset});

      cur_state_id = next_state_id;
      cur_state = &states_[cur_state_id];
      // Reached an end state or found a state in the original path?
      if (cur_state->is_end_state() ||
          to_state_ids[next_state_id].has_value()) {
        path.push_back({next_state_id, -1});
        break;
      }
    }
    return path;
  }

  // Randomly DFS from the state of `from_state_id` to the state of
  // `to_state_id`, trying to find a path of length less than, if no, equal to
  // `length`. If we have multiple such paths, randomly pick one of them. Since
  // this (nearly) fully exlpores all the paths, there is no big difference in
  // the efficiency. And we prefer DFS to BFS for better readability.
  std::vector<Edge> FindPathWithinLengthDFS(absl::BitGenRef prng,
                                            int from_state_id, int to_state_id,
                                            int length) {
    // Each state maintains an edge and a counter for each possible length. The
    // edge is the last edge in the path from `from_state` to the state and can
    // be used to reconstruct the path. And the counter is the number of paths
    // to the state, which can be used for reservoir sampling.
    struct LastEdgeAndCounter {
      std::optional<Edge> edge;
      int counter;
    };
    std::vector<std::vector<LastEdgeAndCounter>> last_edges_and_counters(
        states_.size(), std::vector<LastEdgeAndCounter>(length + 1));

    // Randomness for DFS. Instead of always starting to explore from edge index
    // 0, we start with a different random offset for each state.
    std::vector<int> rand_edge_offsets(states_.size(), 0);
    for (int& i : rand_edge_offsets) i = absl::Uniform<int>(prng, 0u, 256);

    std::vector<Edge> stack{Edge{from_state_id, 0}};
    do {
      auto [current_state_id, edge_index] = stack.back();
      if (edge_index == states_[current_state_id].next.size()) {
        stack.pop_back();
        continue;
      }
      ++stack.back().edge_index;
      const int current_path_length = static_cast<int>(stack.size());
      const int real_edge_index =
          (edge_index + rand_edge_offsets[current_state_id]) %
          static_cast<int>(states_[current_state_id].next.size());
      const int next_state_id =
          states_[current_state_id].next[real_edge_index].next_state_id;
      const int n_path_of_current_length =
          ++last_edges_and_counters[next_state_id][current_path_length].counter;
      // Reservoir Sampling.
      if (absl::Bernoulli(prng, 1.0 / n_path_of_current_length)) {
        last_edges_and_counters[next_state_id][current_path_length].edge =
            Edge{current_state_id, real_edge_index};
      }
      if (n_path_of_current_length == 1 && current_path_length != length) {
        stack.push_back(Edge{next_state_id, 0});
      }
    } while (!stack.empty());

    std::vector<int> candidate_lens;
    for (int len = 1; len <= length; ++len) {
      if (last_edges_and_counters[to_state_id][len].edge.has_value()) {
        candidate_lens.push_back(len);
      }
    }
    FUZZTEST_INTERNAL_CHECK(!candidate_lens.empty(), "Cannot find a path!");

    int state_id = to_state_id;
    std::vector<Edge> result;
    for (int len =
             candidate_lens[absl::Uniform<int>(prng, 0, candidate_lens.size())];
         len > 0; --len) {
      result.push_back(*(last_edges_and_counters[state_id][len].edge));
      state_id = last_edges_and_counters[state_id][len].edge->from_state_id;
    }
    FUZZTEST_INTERNAL_CHECK(state_id == from_state_id,
                            "Cannot find a path from from_state");
    std::reverse(result.begin(), result.end());
    return result;
  }

  absl::StatusOr<std::vector<Edge>> StringToDFAPath(absl::string_view s) const;

  absl::StatusOr<std::string> DFAPathToString(
      const std::vector<Edge>& path, size_t start_offset = 0,
      std::optional<size_t> end = std::nullopt) const;

  size_t state_count() const { return states_.size(); }

  int end_state_id() const { return end_state_id_; }

 private:
  RegexpDFA() {}

  // Given a state and the next input character, try to match the character and
  // return the index in State::next. Return `nullopt` if the matching fails.
  std::optional<int> NextState(const State& cur_state,
                               const std::vector<std::int16_t>& input_chars,
                               size_t& cur_index) const;
  static std::unique_ptr<re2::Prog> CompileRegexp(absl::string_view regexp);
  void BuildEntireDFA(std::unique_ptr<re2::Prog> compiled_regexp);

  // Assign weights (the probability of being picked during random walk)
  // for edges of the DFA so that very long strings are less likely.  All the
  // edge weights of a state sums to 1.  A node is "safe" if it has an high
  // chance (currently at least 50%, defined by `kProbToSafeNode`) to reach
  // closer to the end state. We separate the edges into safe and unsafe ones.
  // The safe edges transition to safe nodes. First we mark every end
  // states/nodes as safe nodes. Next we start BFS from the safe nodes. For each
  // node to be handled during the exploration, we assign
  // `kProbToSafeNode/num_safe_edges` to each safe edge and `(1 -
  // kProbToSafeNode)/num_of_unsafe_edges` to the unsafe ones. After that we can
  // mark the node as safe because it has now at least 50% chance to go to
  // another safe node, which is closer to the end nodes.
  void ComputeEdgeWeights();

  // Compress the DFA so that every state except the end states have at least
  // two outgoing states. With this condition, every non-ending states are good
  // candidates for mutation.
  void CompressStates();

  // We need a special character representing "end of string". This is necessary
  // to make sure that we have exact matches: i.e., that we always reach end
  // states with the "end of string".
  static constexpr std::int16_t kEndOfString = 256;

  std::vector<State> states_;
  int end_state_id_;
};

}  // namespace fuzztest::internal

#endif  // FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_REGEXP_DFA_H_
