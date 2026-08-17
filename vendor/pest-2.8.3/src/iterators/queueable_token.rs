// pest. The Elegant Parser
// Copyright (c) 2018 Dragoș Tiselice
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

// This structure serves to improve performance over Token objects in two ways:
//
//   * it is smaller than a Token, leading to both less memory use when stored in the queue but also
//     increased speed when pushing to the queue
//   * it finds its pair in O(1) time instead of O(N), since pair positions are known at parse time
//     and can easily be stored instead of recomputed
#[derive(Debug)]
pub enum QueueableToken<'i, R> {
    Start {
        /// Queue (as a vec) contains both `Start` token and `End` for the same rule.
        /// This field is an index of corresponding `End` token in vec.
        end_token_index: usize,
        /// Position from which rule was tried to parse (or successfully parsed).
        input_pos: usize,
    },
    End {
        /// Queue (as a vec) contains both `Start` token and `End` for the same rule.
        /// This filed is an index of corresponding `Start` token in vec.
        start_token_index: usize,
        rule: R,
        tag: Option<&'i str>,
        /// Position at which successfully parsed rule finished (ended).
        input_pos: usize,
    },
}
