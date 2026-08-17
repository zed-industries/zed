mod vm_tests {
    use crate::base::SharedEncoding;
    use crate::errors::RewritingError;
    use crate::html::Namespace;
    use crate::memory::SharedMemoryLimiter;
    use crate::rewritable_units::{DocumentEnd, Token, TokenCaptureFlags};
    use crate::rewriter::AsciiCompatibleEncoding;
    use crate::selectors_vm::*;
    use crate::transform_stream::{
        StartTagHandlingResult, TransformController, TransformStream, TransformStreamSettings,
    };
    use encoding_rs::{Encoding, UTF_8};
    use hashbrown::HashMap;

    struct Expectation {
        should_bailout: bool,
        should_match_with_content: bool,
        /// The indexes of selectors that match this tag
        matched_ids: DenseHashSet,
    }

    struct TestElementData(DenseHashSet);

    impl ElementData for TestElementData {
        fn matched_ids_mut(&mut self) -> &mut DenseHashSet {
            &mut self.0
        }
        fn new() -> Self {
            Self(DenseHashSet::new())
        }
    }

    struct TestTransformController<T: FnMut(&mut Token<'_>)>(T);

    impl<T: FnMut(&mut Token<'_>)> TransformController for TestTransformController<T> {
        fn initial_capture_flags(&self) -> TokenCaptureFlags {
            TokenCaptureFlags::all()
        }

        fn handle_start_tag(
            &mut self,
            _: LocalName<'_>,
            _: Namespace,
        ) -> StartTagHandlingResult<Self> {
            Ok(TokenCaptureFlags::NEXT_START_TAG)
        }

        fn handle_end_tag(&mut self, _: LocalName<'_>) -> TokenCaptureFlags {
            TokenCaptureFlags::all()
        }

        fn handle_end(&mut self, _: &mut DocumentEnd<'_>) -> Result<(), RewritingError> {
            Ok(())
        }

        fn handle_token(&mut self, token: &mut Token<'_>) -> Result<(), RewritingError> {
            (self.0)(token);
            Ok(())
        }

        fn should_emit_content(&self) -> bool {
            true
        }
    }

    pub fn test_with_token(
        html: &str,
        encoding: &'static Encoding,
        test_fn: impl FnMut(&mut Token<'_>),
    ) {
        let (html, _, has_unmappable_characters) = encoding.encode(html);

        // NOTE: there is no character in existence outside of ASCII range
        // that can be represented in all the ASCII-compatible encodings.
        // So, if test cases contains some non-ASCII characters that can't
        // be represented in the given encoding then we just skip it.
        // It is OK to do so, because our intention is not to test the
        // encoding library itself (it is already well tested), but test
        // how our own code works with non-ASCII characters.
        if has_unmappable_characters {
            return;
        }

        let mut transform_stream = TransformStream::new(TransformStreamSettings {
            transform_controller: TestTransformController(test_fn),
            output_sink: |_: &[u8]| {},
            preallocated_parsing_buffer_size: 0,
            encoding: SharedEncoding::new(AsciiCompatibleEncoding::new(encoding).unwrap()),
            memory_limiter: SharedMemoryLimiter::new(2048),
            strict: true,
        });

        transform_stream.write(&html).unwrap();
        transform_stream.end().unwrap();
    }

    macro_rules! map {
        ($($items:expr),*) => {
            vec![$($items),*].into_iter().collect::<HashMap<_, _>>()
        };
    }

    macro_rules! local_name {
        ($t:expr) => {
            LocalName::from_str_without_replacements($t.name(), UTF_8).unwrap()
        };
    }

    // NOTE: these are macroses to preserve callsites on fails.
    macro_rules! create_vm {
        ($selectors:expr) => {{
            let mut ast = Ast::default();

            for (selector, match_id) in $selectors.into_iter().zip(0..) {
                ast.add_selector(&selector.parse().unwrap(), match_id);
            }

            let memory_limiter = SharedMemoryLimiter::new(2048);
            let enable_esi_tags = false;
            let vm: SelectorMatchingVm<TestElementData> =
                SelectorMatchingVm::new(ast, UTF_8, memory_limiter, enable_esi_tags);

            vm
        }};
    }

    macro_rules! exec_for_start_tag_and_assert {
        ($vm:expr, $tag_html:expr, $ns:expr, $expectation:expr) => {
            test_with_token($tag_html, UTF_8, |t| {
                match t {
                    Token::StartTag(t) => {
                        let mut matched_ids = DenseHashSet::new();

                        {
                            let mut match_handler = |m: MatchInfo| {
                                assert_eq!(
                                    m.with_content, $expectation.should_match_with_content,
                                    "with_content"
                                );
                                matched_ids.insert(m.match_id);
                            };

                            let result =
                                $vm.exec_for_start_tag(local_name!(t), $ns, &mut match_handler);

                            if $expectation.should_bailout {
                                let aux_info_req = result.expect_err("Bailout expected");
                                let (input, attr_buffer) = t.raw_attributes();

                                match aux_info_req {
                                    VmError::InfoRequest(f) => f(
                                        &mut $vm,
                                        AuxStartTagInfo {
                                            input,
                                            attr_buffer,
                                            self_closing: t.self_closing(),
                                        },
                                        &mut match_handler,
                                    )
                                    .unwrap(),
                                    VmError::MemoryLimitExceeded(e) => panic!("{}", e),
                                }
                            } else {
                                // NOTE: can't use unwrap() or expect() here, because
                                // Debug is not implemented for the closure in the error type.
                                #[allow(clippy::match_wild_err_arm)]
                                match result {
                                    Ok(_) => (),
                                    Err(_) => panic!("Should match without bailout"),
                                }
                            }
                        }

                        assert_eq!(
                            matched_ids,
                            $expectation.matched_ids,
                            "{:?}",
                            matched_ids.iter().collect::<Vec<_>>()
                        );
                    }
                    _ => panic!("Start tag expected"),
                }
            });
        };
    }

    macro_rules! exec_for_end_tag_and_assert {
        ($vm:expr, $tag_html:expr, $expected_unmatched_ids:expr) => {
            test_with_token($tag_html, UTF_8, |t| match t {
                Token::EndTag(t) => {
                    let mut unmatched_ids = HashMap::default();

                    $vm.exec_for_end_tag(local_name!(t), |elem_data: TestElementData| {
                        for match_id in elem_data.0.iter() {
                            unmatched_ids
                                .entry(match_id)
                                .and_modify(|c| *c += 1)
                                .or_insert(1);
                        }
                    });

                    assert_eq!(unmatched_ids, $expected_unmatched_ids);
                }
                _ => panic!("End tag expected"),
            });
        };
    }

    #[test]
    fn html_elements() {
        let mut vm = create_vm!(&["a", "img.c1", ":not(a).c2"]);

        // Stack after:
        // - <a> (0)
        exec_for_start_tag_and_assert!(
            vm,
            "<a>",
            Namespace::Html,
            Expectation {
                should_bailout: false,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([0]),
            }
        );

        // Void element.
        // Stack after:
        // - <a> (0)
        exec_for_start_tag_and_assert!(
            vm,
            "<img>",
            Namespace::Html,
            Expectation {
                should_bailout: true,
                should_match_with_content: false,
                matched_ids: DenseHashSet::from([]),
            }
        );

        // Void element.
        // Stack after:
        // - <a> (0)
        exec_for_start_tag_and_assert!(
            vm,
            "<img class='c2 c1'>",
            Namespace::Html,
            Expectation {
                should_bailout: true,
                should_match_with_content: false,
                matched_ids: DenseHashSet::from([1, 2]),
            }
        );

        // Stack after:
        // - <a> (0)
        // - <a class='c2 c1'> (0)
        exec_for_start_tag_and_assert!(
            vm,
            "<a class='c1 c2'>",
            Namespace::Html,
            Expectation {
                should_bailout: false,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([0]),
            }
        );

        // Stack after:
        // - <a> (0)
        // - <a class='c2 c1'> (0)
        // - <div class=c2> (2)
        exec_for_start_tag_and_assert!(
            vm,
            "<div class='c2'>",
            Namespace::Html,
            Expectation {
                should_bailout: true,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([2]),
            }
        );

        // Stack after:
        // - <a> (0)
        // - <a class='c2 c1'> (0)
        // - <div class=c2> (2)
        // - <h1 class='c1 c2 c3'> (2)
        exec_for_start_tag_and_assert!(
            vm,
            "<h1 class='c1 c2 c3'>",
            Namespace::Html,
            Expectation {
                should_bailout: true,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([2]),
            }
        );

        // Stack after:
        // Stack after:
        // - <a> (0)
        // - <a class='c2 c1'> (0)
        // - <div class=c2> (2)
        // - <h1 class='c1 c2 c3'> (2)
        exec_for_end_tag_and_assert!(vm, "</span>", map![]);

        // Stack after:
        // - <a> (0)
        exec_for_end_tag_and_assert!(vm, "</a>", map![(0, 1), (2, 2)]);

        // Stack after:
        // - <a> (0)
        exec_for_end_tag_and_assert!(vm, "</div>", map![]);

        // Stack after: empty
        exec_for_end_tag_and_assert!(vm, "</a>", map![(0, 1)]);
    }

    #[test]
    fn foreign_elements() {
        let mut vm = create_vm!(&["circle", "#foo"]);

        // Stack after:
        // - <svg>
        exec_for_start_tag_and_assert!(
            vm,
            "<svg>",
            Namespace::Svg,
            Expectation {
                should_bailout: true,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([]),
            }
        );

        // Self-closing.
        // Stack after:
        // - <svg>
        exec_for_start_tag_and_assert!(
            vm,
            "<circle id=foo />",
            Namespace::Svg,
            Expectation {
                should_bailout: true,
                should_match_with_content: false,
                matched_ids: DenseHashSet::from([0, 1]),
            }
        );

        // Self-closing.
        // Stack after:
        // - <svg>
        // - <circle> (0)
        exec_for_start_tag_and_assert!(
            vm,
            "<circle>",
            Namespace::Svg,
            Expectation {
                should_bailout: true,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([0]),
            }
        );

        // Stack after: empty
        exec_for_end_tag_and_assert!(vm, "</svg>", map![(0, 1)]);
    }

    #[test]
    fn entry_points() {
        let mut vm = create_vm!(&["div", "span[foo=bar]", "span#test"]);

        exec_for_start_tag_and_assert!(
            vm,
            "<div>",
            Namespace::Html,
            Expectation {
                should_bailout: false,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([0]),
            }
        );

        exec_for_start_tag_and_assert!(
            vm,
            "<span>",
            Namespace::Html,
            Expectation {
                should_bailout: true,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([]),
            }
        );

        exec_for_start_tag_and_assert!(
            vm,
            "<span foo=bar>",
            Namespace::Html,
            Expectation {
                should_bailout: true,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([1]),
            }
        );

        exec_for_start_tag_and_assert!(
            vm,
            "<span foo=bar id=test>",
            Namespace::Html,
            Expectation {
                should_bailout: true,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([1, 2]),
            }
        );

        exec_for_end_tag_and_assert!(vm, "</div>", map![(0, 1), (1, 2), (2, 1)]);
    }

    #[test]
    fn entry_points_bailout_on_last_addr_in_set() {
        let mut vm = create_vm!(&["*", "span", "span[foo=bar]"]);

        exec_for_start_tag_and_assert!(
            vm,
            "<span>",
            Namespace::Html,
            Expectation {
                should_bailout: true,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([0, 1]),
            }
        );

        exec_for_start_tag_and_assert!(
            vm,
            "<span foo=bar>",
            Namespace::Html,
            Expectation {
                should_bailout: true,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([0, 1, 2]),
            }
        );
    }

    #[test]
    fn nth_child() {
        let mut vm = create_vm!(&["div:first-child", "div:nth-child(2n+1)"]);

        // Stack:
        // 0: html
        exec_for_start_tag_and_assert!(
            vm,
            "<html>",
            Namespace::Html,
            Expectation {
                should_bailout: false,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([]),
            }
        );

        // Stack:
        // 0: html
        // 1: div
        exec_for_start_tag_and_assert!(
            vm,
            "<div>",
            Namespace::Html,
            Expectation {
                should_bailout: false,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([0, 1]),
            }
        );

        // Stack:
        // 0: html
        exec_for_end_tag_and_assert!(vm, "</div>", map![(0, 1), (1, 1)]);

        // Stack:
        // 0: html
        // 1: div
        exec_for_start_tag_and_assert!(
            vm,
            "<div>",
            Namespace::Html,
            Expectation {
                should_bailout: false,
                should_match_with_content: false,
                matched_ids: DenseHashSet::from([]),
            }
        );

        // Stack:
        // 0: html
        exec_for_end_tag_and_assert!(vm, "</div>", map![]);

        // Stack:
        // 0: html
        // 1: div
        exec_for_start_tag_and_assert!(
            vm,
            "<div>",
            Namespace::Html,
            Expectation {
                should_bailout: false,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([1]),
            }
        );

        // Stack:
        // 0: html
        // 1: div
        // 2: div
        exec_for_start_tag_and_assert!(
            vm,
            "<div>",
            Namespace::Html,
            Expectation {
                should_bailout: false,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([0, 1]),
            }
        );

        // Stack:
        // 0: html
        // 1: div
        exec_for_end_tag_and_assert!(vm, "</div>", map![(0, 1), (1, 1)]);

        // Stack:
        // 0: html
        exec_for_end_tag_and_assert!(vm, "</div>", map![(1, 1)]);

        // Stack:
        exec_for_end_tag_and_assert!(vm, "</html>", map![]);
    }

    #[test]
    fn nth_of_type() {
        let mut vm = create_vm!(&["div:first-of-type", "div:nth-of-type(2n+1)"]);

        // Stack:
        // 0: html
        exec_for_start_tag_and_assert!(
            vm,
            "<html>",
            Namespace::Html,
            Expectation {
                should_bailout: false,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([]),
            }
        );

        // Stack:
        // 0: html
        // 1: div
        exec_for_start_tag_and_assert!(
            vm,
            "<div>",
            Namespace::Html,
            Expectation {
                should_bailout: false,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([0, 1]),
            }
        );

        // Stack:
        // 0: html
        exec_for_end_tag_and_assert!(vm, "</div>", map![(0, 1), (1, 1)]);

        // Stack:
        // 0: html
        // 1: div
        exec_for_start_tag_and_assert!(
            vm,
            "<div>",
            Namespace::Html,
            Expectation {
                should_bailout: false,
                should_match_with_content: false,
                matched_ids: DenseHashSet::from([]),
            }
        );

        // Stack:
        // 0: html
        exec_for_end_tag_and_assert!(vm, "</div>", map![]);

        // Stack:
        // 0: html
        // 1: a
        exec_for_start_tag_and_assert!(
            vm,
            "<a>",
            Namespace::Html,
            Expectation {
                should_bailout: false,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([]),
            }
        );

        // Stack:
        // 0: html
        // 1: a
        // 2: div
        exec_for_start_tag_and_assert!(
            vm,
            "<div>",
            Namespace::Html,
            Expectation {
                should_bailout: false,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([0, 1]),
            }
        );

        // Stack:
        // 0: html
        // 1: a
        exec_for_end_tag_and_assert!(vm, "</div>", map![(0, 1), (1, 1)]);

        // Stack:
        // 0: html
        exec_for_end_tag_and_assert!(vm, "</a>", map![]);

        // Stack:
        // 0: html
        // 1: div
        exec_for_start_tag_and_assert!(
            vm,
            "<div>",
            Namespace::Html,
            Expectation {
                should_bailout: false,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([1]),
            }
        );

        // Stack:
        // 0: html
        // 1: div
        // 2: div
        exec_for_start_tag_and_assert!(
            vm,
            "<div>",
            Namespace::Html,
            Expectation {
                should_bailout: false,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([0, 1]),
            }
        );

        // Stack:
        // 0: html
        // 1: div
        exec_for_end_tag_and_assert!(vm, "</div>", map![(0, 1), (1, 1)]);

        // Stack:
        // 0: html
        exec_for_end_tag_and_assert!(vm, "</div>", map![(1, 1)]);

        // Stack:
        exec_for_end_tag_and_assert!(vm, "</html>", map![]);
    }

    #[test]
    fn jumps() {
        let mut vm = create_vm!(&["div > span", "div > #foo", ":not(span) > .c2 > .c3"]);

        // Stack after:
        // - <div>
        exec_for_start_tag_and_assert!(
            vm,
            "<div>",
            Namespace::Html,
            Expectation {
                should_bailout: false,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([]),
            }
        );

        // Stack after:
        // - <div>
        // - <span> (0)
        exec_for_start_tag_and_assert!(
            vm,
            "<span>",
            Namespace::Html,
            Expectation {
                should_bailout: true,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([0]),
            }
        );

        // Stack after:
        // - <div>
        // - <span> (0)
        // - <span>
        exec_for_start_tag_and_assert!(
            vm,
            "<span>",
            Namespace::Html,
            Expectation {
                should_bailout: false,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([]),
            }
        );

        // Stack after:
        // - <div>
        // - <span> (0)
        exec_for_end_tag_and_assert!(vm, "</span>", map![]);

        // Stack after:
        // - <div>
        exec_for_end_tag_and_assert!(vm, "</span>", map![(0, 1)]);

        // Stack after:
        // - <div>
        // - <div id=foo class=c2> (1)
        exec_for_start_tag_and_assert!(
            vm,
            "<div id=foo class=c2>",
            Namespace::Html,
            Expectation {
                should_bailout: true,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([1]),
            }
        );

        // Stack after:
        // - <div>
        // - <div id=foo class=c2> (1)
        // - <span class=c3> (0, 2)
        exec_for_start_tag_and_assert!(
            vm,
            "<span class=c3>",
            Namespace::Html,
            Expectation {
                should_bailout: true,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([0, 2]),
            }
        );

        // Stack after is empty
        exec_for_end_tag_and_assert!(vm, "</div>", map![(0, 1), (1, 1), (2, 1)]);
    }

    #[test]
    fn bailout_in_jumps_on_last_addr_in_set() {
        let mut vm = create_vm!(&["div > span", "div > *", "#foo > span", "#foo > ul.c1"]);

        // Stack after:
        // - <div id=foo>
        exec_for_start_tag_and_assert!(
            vm,
            "<div id=foo>",
            Namespace::Html,
            Expectation {
                should_bailout: true,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([]),
            }
        );

        // Stack after:
        // - <div id=foo>
        // - <span> (0, 1, 2)
        exec_for_start_tag_and_assert!(
            vm,
            "<span>",
            Namespace::Html,
            Expectation {
                should_bailout: true,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([0, 1, 2]),
            }
        );

        // Stack after:
        // - <div id=foo>
        exec_for_end_tag_and_assert!(vm, "</span>", map![(0, 1), (1, 1), (2, 1)]);

        // Stack after:
        // - <div id=foo>
        // - <ul> (1)
        exec_for_start_tag_and_assert!(
            vm,
            "<ul>",
            Namespace::Html,
            Expectation {
                should_bailout: true,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([1]),
            }
        );

        // Stack after:
        // - <div id=foo>
        exec_for_end_tag_and_assert!(vm, "</ul>", map![(1, 1)]);

        // Stack after:
        // - <div id=foo>
        // - <ul class=c1> (1, 3)
        exec_for_start_tag_and_assert!(
            vm,
            "<ul class=c1>",
            Namespace::Html,
            Expectation {
                should_bailout: true,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([1, 3]),
            }
        );
    }

    #[test]
    fn hereditary_jumps() {
        let mut vm = create_vm!(&["div .c1", "#foo .c2 .c3"]);

        // Stack after:
        // - <div id=foo>
        exec_for_start_tag_and_assert!(
            vm,
            "<div id=foo>",
            Namespace::Html,
            Expectation {
                should_bailout: true,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([]),
            }
        );

        // Stack after:
        // - <div id=foo>
        // - <div class='c1 c2'> (0)
        exec_for_start_tag_and_assert!(
            vm,
            "<div class='c1 c2'>",
            Namespace::Html,
            Expectation {
                should_bailout: true,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([0]),
            }
        );

        // Stack after:
        // - <div id=foo>
        // - <div class='c1 c2'> (0)
        // - <div class='c1 c2 c3'> (0, 1)
        exec_for_start_tag_and_assert!(
            vm,
            "<div class='c1 c2 c3'>",
            Namespace::Html,
            Expectation {
                should_bailout: true,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([0, 1]),
            }
        );

        // Stack after:
        // - <div id=foo>
        // - <div class='c1 c2'> (0)
        // - <div class='c1 c2 c3'> (0, 1)
        // - <span class='c3'> (1)
        exec_for_start_tag_and_assert!(
            vm,
            "<span class='c3'>",
            Namespace::Html,
            Expectation {
                should_bailout: true,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([1]),
            }
        );

        // Stack after:
        // - <div id=foo>
        // - <div class='c1 c2'> (0)
        // - <div class='c1 c2 c3'> (0, 1)
        // - <span class='c3'> (1)
        // - <span class='c1 c3'> (0, 1)
        exec_for_start_tag_and_assert!(
            vm,
            "<span class='c1 c3'>",
            Namespace::Html,
            Expectation {
                should_bailout: true,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([0, 1]),
            }
        );

        // Stack after:
        // - <div id=foo>
        // - <div class='c1 c2'> (0)
        exec_for_end_tag_and_assert!(vm, "</div>", map![(0, 2), (1, 3)]);

        // Stack after:
        // - <div id=foo>
        // - <div class='c1'> (0)
        exec_for_start_tag_and_assert!(
            vm,
            "<span class='c1'>",
            Namespace::Html,
            Expectation {
                should_bailout: true,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([0]),
            }
        );
    }

    #[test]
    fn bailout_in_hereditary_jumps_in_first_ancestor() {
        let mut vm = create_vm!(&[
            "body div *",
            "body div span#foo",
            "body div span",
            "body * #foo"
        ]);

        // Stack after:
        // - <body>
        exec_for_start_tag_and_assert!(
            vm,
            "<body>",
            Namespace::Html,
            Expectation {
                should_bailout: false,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([]),
            }
        );

        // Stack after:
        // - <body>
        // - <div>
        exec_for_start_tag_and_assert!(
            vm,
            "<div>",
            Namespace::Html,
            Expectation {
                should_bailout: false,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([]),
            }
        );

        // Stack after:
        // - <body>
        // - <div>
        exec_for_start_tag_and_assert!(
            vm,
            "<img>",
            Namespace::Html,
            Expectation {
                should_bailout: true,
                should_match_with_content: false,
                matched_ids: DenseHashSet::from([0]),
            }
        );

        // Stack after:
        // - <body>
        // - <div>
        // - <span id=foo> (0, 1, 2, 3)
        exec_for_start_tag_and_assert!(
            vm,
            "<span id=foo>",
            Namespace::Html,
            Expectation {
                should_bailout: true,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([0, 1, 2, 3]),
            }
        );

        // Stack after:
        // - <body>
        // - <div>
        // - <span id=foo> (0, 1, 2, 3)
        // - <span> (0, 2)
        exec_for_start_tag_and_assert!(
            vm,
            "<span>",
            Namespace::Html,
            Expectation {
                should_bailout: true,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([0, 2]),
            }
        );

        // Stack after is empty
        exec_for_end_tag_and_assert!(vm, "</body>", map![(0, 2), (1, 1), (2, 2), (3, 1)]);
    }

    #[test]
    fn bailout_in_hereditary_jumps_on_last_addr_in_last_set_of_last_ancestor() {
        let mut vm = create_vm!(&["body *", "body span#foo", "div *"]);

        // Stack after:
        // - <body>
        exec_for_start_tag_and_assert!(
            vm,
            "<body>",
            Namespace::Html,
            Expectation {
                should_bailout: false,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([]),
            }
        );

        // Stack after:
        // - <body>
        // - <div> (0)
        exec_for_start_tag_and_assert!(
            vm,
            "<div>",
            Namespace::Html,
            Expectation {
                should_bailout: false,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([0]),
            }
        );

        // Stack after:
        // - <body>
        // - <div> (0)
        // - <div> (0, 1, 2)
        exec_for_start_tag_and_assert!(
            vm,
            "<span id=foo>",
            Namespace::Html,
            Expectation {
                should_bailout: true,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([0, 1, 2]),
            }
        );

        // Stack after:
        // - <body>
        // - <div> (0)
        // - <span> (0, 1, 2)
        // - <span> (0, 2)
        exec_for_start_tag_and_assert!(
            vm,
            "<span>",
            Namespace::Html,
            Expectation {
                should_bailout: true,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([0, 2]),
            }
        );

        // Stack after is empty
        exec_for_end_tag_and_assert!(vm, "</body>", map![(0, 3), (1, 1), (2, 2)]);
    }

    #[test]
    fn compound_selector() {
        let mut vm = create_vm!(&["body > span#foo .c1 .c2"]);

        // Stack after:
        // - <body>
        exec_for_start_tag_and_assert!(
            vm,
            "<body>",
            Namespace::Html,
            Expectation {
                should_bailout: false,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([]),
            }
        );

        // Stack after:
        // - <body>
        // - <span>
        exec_for_start_tag_and_assert!(
            vm,
            "<span>",
            Namespace::Html,
            Expectation {
                should_bailout: true,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([]),
            }
        );

        // Stack after:
        // - <body>
        exec_for_end_tag_and_assert!(vm, "</span>", map![]);

        // Stack after:
        // - <body>
        // - <span id=foo>
        exec_for_start_tag_and_assert!(
            vm,
            "<span id=foo>",
            Namespace::Html,
            Expectation {
                should_bailout: true,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([]),
            }
        );

        // Stack after:
        // - <body>
        // - <span id=foo>
        // - <div>
        exec_for_start_tag_and_assert!(
            vm,
            "<div>",
            Namespace::Html,
            Expectation {
                should_bailout: true,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([]),
            }
        );

        // Stack after:
        // - <body>
        // - <span id=foo>
        // - <div>
        // - <ul class='bar c1'>
        exec_for_start_tag_and_assert!(
            vm,
            "<ul class='bar c1'>",
            Namespace::Html,
            Expectation {
                should_bailout: true,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([]),
            }
        );

        // Stack after:
        // - <body>
        // - <span id=foo>
        // - <div>
        // - <ul class='bar c1'>
        // - <li class='c3'>
        exec_for_start_tag_and_assert!(
            vm,
            "<li class='c3'>",
            Namespace::Html,
            Expectation {
                should_bailout: true,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([]),
            }
        );

        // Stack after:
        // - <body>
        // - <span id=foo>
        // - <div>
        // - <ul class='bar c1'>
        // - <li class='c3'>
        // - <span class=c2>
        exec_for_start_tag_and_assert!(
            vm,
            "<span class=c2>",
            Namespace::Html,
            Expectation {
                should_bailout: true,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([0]),
            }
        );
    }

    #[test]
    fn nested_not_matching() {
        let mut vm = create_vm!(&[":not(:not(div))"]);

        // Stack after: - <div>
        exec_for_start_tag_and_assert!(
            vm,
            "<div>",
            Namespace::Html,
            Expectation {
                should_bailout: false,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([0]),
            }
        );

        exec_for_end_tag_and_assert!(vm, "</div>", map![(0, 1)]);

        let mut vm2 = create_vm!(&[":not(:not(:not(div)))"]);

        // <div> should NOT match :not(:not(:not(div)))
        exec_for_start_tag_and_assert!(
            vm2,
            "<div>",
            Namespace::Html,
            Expectation {
                should_bailout: false,
                should_match_with_content: false,
                matched_ids: DenseHashSet::from([]),
            }
        );
    }

    #[test]
    fn lots_of_selectors() {
        let mut vm = create_vm!(
            ('a'..='z')
                .map(|c| c.to_string())
                .chain(('A'..='Z').map(|c| format!("#{c}")))
                .chain(('0'..='9').map(|c| format!("[data{c}]")))
        );

        exec_for_start_tag_and_assert!(
            vm,
            "<a id=Z>",
            Namespace::Html,
            Expectation {
                should_bailout: true,
                should_match_with_content: true,
                matched_ids: DenseHashSet::from([0, 51]),
            }
        );
    }

    /// Test that match_handler is called exactly once per matched id,
    /// even when the same match_ids can be reached through multiple instruction paths.
    /// For example, `"span, [foo$=bar]"` is one selector (match_id 0) with two
    /// comma-separated entries — both match `<span foo=bar>`, but match_id 0
    /// must be reported only once.
    #[test]
    fn no_duplicate_match_handler_calls() {
        let mut ast = Ast::default();
        ast.add_selector(&"span, [foo$=bar]".parse().unwrap(), 0);
        // Add a second selector "span" with match_id 1 — should also match <span foo=bar>.
        ast.add_selector(&"span".parse().unwrap(), 1);

        let memory_limiter = SharedMemoryLimiter::new(2048);
        let mut vm: SelectorMatchingVm<TestElementData> =
            SelectorMatchingVm::new(ast, UTF_8, memory_limiter, false);

        test_with_token("<span foo=bar>", UTF_8, |t| {
            let Token::StartTag(t) = t else {
                panic!("must be start tag");
            };
            let mut calls = Vec::new();

            let mut match_handler = |m: MatchInfo| {
                calls.push(m.match_id);
            };

            let result = vm.exec_for_start_tag(local_name!(t), Namespace::Html, &mut match_handler);

            if let Err(VmError::InfoRequest(inforeq)) = result {
                let (input, attr_buffer) = t.raw_attributes();
                inforeq(
                    &mut vm,
                    AuxStartTagInfo {
                        input,
                        attr_buffer,
                        self_closing: t.self_closing(),
                    },
                    &mut match_handler,
                )
                .unwrap();
            }

            // // Sort for deterministic comparison (HashSet iteration order varies)
            // calls.sort();

            assert_eq!(
                calls,
                [0, 1],
                "Each match_id should be reported exactly once"
            );
        });
    }
}

mod compiler {
    use super::vm_tests::test_with_token;
    use crate::html::Namespace;
    use crate::rewritable_units::Token;
    use crate::selectors_vm::*;
    use crate::test_utils::ASCII_COMPATIBLE_ENCODINGS;
    use encoding_rs::UTF_8;

    macro_rules! assert_instr_res {
        ($res:expr, $should_match:expr, $selector:expr, $input:expr, $encoding:expr) => {{
            let expected_ids = if *$should_match {
                Some(DenseHashSet::from([0]))
            } else {
                None
            };

            assert_eq!(
                $res.map(|b| b.matched_ids.to_owned()),
                expected_ids,
                "Instruction didn't produce expected matching result\n\
                 selector: {:#?}\n\
                 input: {:#?}\n\
                 encoding: {:?}\n\
                 ",
                $selector,
                $input,
                $encoding.name()
            );
        }};
    }

    fn test_compile(
        selectors: &[&str],
        encoding: &'static Encoding,
        expected_entry_point_count: usize,
    ) -> Program {
        let mut ast = Ast::default();

        for (selector, match_id) in selectors.iter().zip(0..) {
            ast.add_selector(&selector.parse().unwrap(), match_id);
        }

        let program = Compiler::new(encoding).compile(ast);

        assert_eq!(
            program.entry_points.end - program.entry_points.start,
            expected_entry_point_count
        );

        program
    }

    fn with_negated<'i>(
        selector: &str,
        test_cases: &[(&'i str, bool)],
    ) -> Vec<(String, Vec<(&'i str, bool)>)> {
        vec![
            (selector.to_string(), test_cases.to_owned()),
            (
                format!(":not({selector})"),
                test_cases
                    .iter()
                    .map(|(input, should_match)| (*input, !should_match))
                    .collect(),
            ),
        ]
    }

    fn with_start_tag(
        html: &str,
        encoding: &'static Encoding,
        mut action: impl FnMut(LocalName<'_>, AttributeMatcher<'_>),
    ) {
        test_with_token(html, encoding, |t| match t {
            Token::StartTag(t) => {
                let (input, attrs) = t.raw_attributes();
                let attr_matcher = AttributeMatcher::new(*input, attrs, Namespace::Html);
                let local_name =
                    LocalName::from_str_without_replacements(t.name(), encoding).unwrap();

                action(local_name, attr_matcher);
            }
            _ => panic!("Start tag expected"),
        });
    }

    fn for_each_test_case<T>(
        test_cases: &[(&str, T)],
        encoding: &'static Encoding,
        action: impl Fn(&str, &T, &SelectorState<'_>, LocalName<'_>, AttributeMatcher<'_>),
    ) {
        for (input, matching_data) in test_cases {
            with_start_tag(input, encoding, |local_name, attr_matcher| {
                let counter = Default::default();
                let state = SelectorState {
                    cumulative: &counter,
                    typed: None,
                };
                action(input, matching_data, &state, local_name, attr_matcher);
            });
        }
    }

    fn assert_attr_expr_matches(
        selector: &str,
        encoding: &'static Encoding,
        test_cases: &[(&str, bool)],
    ) {
        let program = test_compile(&[selector], encoding, 1);
        let instr = &program.instructions[program.entry_points.start];

        for_each_test_case(
            test_cases,
            encoding,
            |input, should_match, state, local_name, attr_matcher| {
                assert!(
                    matches!(
                        instr.try_exec_without_attrs(state, &local_name),
                        TryExecResult::AttributesRequired
                    ),
                    "Instruction should not execute without attributes"
                );

                let multi_step_res = instr.complete_exec_with_attrs(state, &attr_matcher);
                let res = instr.exec(state, &local_name, &attr_matcher);

                assert_eq!(multi_step_res, res);
                assert_instr_res!(res, should_match, selector, input, encoding);
            },
        );
    }

    fn assert_non_attr_expr_matches_and_negation_reverses_match(
        selector: &str,
        encoding: &'static Encoding,
        test_cases: &[(&str, bool)],
    ) {
        for (selector, test_cases) in with_negated(selector, test_cases) {
            let program = test_compile(&[&selector], encoding, 1);
            let instr = &program.instructions[program.entry_points.start];

            for_each_test_case(
                &test_cases,
                encoding,
                |input, should_match, state, local_name, attr_matcher| {
                    #[allow(clippy::match_wild_err_arm)]
                    let multi_step_res = match instr.try_exec_without_attrs(state, &local_name) {
                        TryExecResult::Branch(b) => Some(b),
                        TryExecResult::Fail => None,
                        TryExecResult::AttributesRequired => {
                            panic!("Should match without attribute request")
                        }
                    };

                    let res = instr.exec(state, &local_name, &attr_matcher);

                    assert_eq!(multi_step_res, res);

                    assert_instr_res!(res, should_match, selector, input, encoding);
                },
            );
        }
    }

    fn assert_attr_expr_matches_and_negation_reverses_match(
        selector: &str,
        encoding: &'static Encoding,
        test_cases: &[(&str, bool)],
    ) {
        for (selector, test_cases) in &with_negated(selector, test_cases) {
            assert_attr_expr_matches(selector, encoding, test_cases);
        }
    }

    macro_rules! exec_generic_instr {
        ($instr:expr, $state:expr, $local_name:expr, $attr_matcher:expr) => {{
            let res = $instr.exec($state, &$local_name, &$attr_matcher);

            let multi_step_res = match $instr.try_exec_without_attrs($state, &$local_name) {
                TryExecResult::Branch(b) => Some(b),
                TryExecResult::Fail => None,
                TryExecResult::AttributesRequired => {
                    $instr.complete_exec_with_attrs(&*$state, &$attr_matcher)
                }
            };

            assert_eq!(res, multi_step_res);

            res
        }};
    }

    fn assert_generic_expr_matches(
        selector: &str,
        encoding: &'static Encoding,
        test_cases: &[(&str, bool)],
    ) {
        let program = test_compile(&[selector], encoding, 1);
        let instr = &program.instructions[program.entry_points.start];

        for_each_test_case(
            test_cases,
            encoding,
            |input, should_match, state, local_name, attr_matcher| {
                let res = exec_generic_instr!(instr, state, local_name, attr_matcher);

                assert_instr_res!(res, should_match, selector, input, encoding);
            },
        );
    }

    macro_rules! exec_instr_range {
        ($range:expr, $program:expr, $state:expr, $local_name:expr, $attr_matcher:expr) => {{
            let mut matched_ids = DenseHashSet::new();
            let mut jumps = Vec::default();
            let mut hereditary_jumps = Vec::default();

            for addr in $range.clone() {
                let res = exec_generic_instr!(
                    $program.instructions[addr],
                    $state,
                    $local_name,
                    $attr_matcher
                );

                if let Some(res) = res {
                    matched_ids.union(&res.matched_ids);

                    if let Some(ref j) = res.jumps {
                        jumps.push(j.to_owned());
                    }

                    if let Some(ref j) = res.hereditary_jumps {
                        hereditary_jumps.push(j.to_owned());
                    }
                }
            }

            (matched_ids, jumps, hereditary_jumps)
        }};
    }

    macro_rules! assert_payload {
        ($actual:expr, $expected:expr, $selectors:expr, $input:expr) => {
            assert_eq!(
                $actual,
                DenseHashSet::from($expected.iter().cloned()),
                "Instructions didn't produce expected payload\n\
                 selectors: {:#?}\n\
                 input: {:#?}\n\
                 ",
                $selectors,
                $input
            );
        };
    }

    fn assert_entry_points_match(
        selectors: &[&str],
        expected_entry_point_count: usize,
        test_cases: &[(&str, Vec<MatchId>)],
    ) {
        let program = test_compile(selectors, UTF_8, expected_entry_point_count);

        // NOTE: encoding of the individual components is tested by other tests,
        // so we use only UTF-8 here.
        for_each_test_case(
            test_cases,
            UTF_8,
            |input, expected_ids, state, local_name, attr_matcher| {
                let (matched_ids, _, _) = exec_instr_range!(
                    program.entry_points,
                    program,
                    state,
                    local_name,
                    attr_matcher
                );

                assert_payload!(matched_ids, expected_ids, selectors, input);
            },
        );
    }

    #[test]
    fn compiled_non_attr_expression() {
        for encoding in &ASCII_COMPATIBLE_ENCODINGS {
            assert_non_attr_expr_matches_and_negation_reverses_match(
                "*",
                encoding,
                &[
                    ("<div>", true),
                    ("<span>", true),
                    ("<anything-else>", true),
                    ("<FуБар>", true),
                ],
            );

            assert_non_attr_expr_matches_and_negation_reverses_match(
                "div",
                encoding,
                &[
                    ("<div>", true),
                    ("<Div>", true),
                    ("<divnotdiv>", false),
                    ("<span>", false),
                    ("<anything-else>", false),
                ],
            );

            // NOTE: case-insensitivity for local names that can't be represented by a hash.
            assert_non_attr_expr_matches_and_negation_reverses_match(
                "fooƔ",
                encoding,
                &[("<fooƔ", true), ("<FooƔ>", true)],
            );

            assert_non_attr_expr_matches_and_negation_reverses_match(
                "span",
                encoding,
                &[
                    ("<div>", false),
                    ("<span>", true),
                    ("<spanϰ>", false),
                    ("<anything-else>", false),
                ],
            );

            assert_non_attr_expr_matches_and_negation_reverses_match(
                "anything-else",
                encoding,
                &[
                    ("<div>", false),
                    ("<span>", false),
                    ("<anything-else>", true),
                ],
            );
        }
    }

    #[test]
    fn compiled_attr_expression() {
        for encoding in &ASCII_COMPATIBLE_ENCODINGS {
            assert_attr_expr_matches_and_negation_reverses_match(
                "#foo⾕",
                encoding,
                &[
                    ("<div bar=baz qux id='foo⾕'>", true),
                    ("<div iD='foo⾕'>", true),
                    ("<div bar=baz qux id='foo1'>", false),
                    ("<div bar=baz qux>", false),
                ],
            );

            assert_attr_expr_matches_and_negation_reverses_match(
                ".c2",
                encoding,
                &[
                    ("<div bar=baz class='c1 c2 c3 c4' qux>", true),
                    ("<div CLASS='c1 c2 c3 c4'>", true),
                    ("<div class='c1 c23 c4'>", false),
                    ("<div bar=baz qux>", false),
                ],
            );

            assert_attr_expr_matches_and_negation_reverses_match(
                "[foo⽅]",
                encoding,
                &[
                    ("<div foo1 foo2 foo⽅>", true),
                    ("<div FOo⽅=123>", true),
                    ("<div id='baz'>", false),
                    ("<div bar=baz qux>", false),
                ],
            );

            assert_attr_expr_matches_and_negation_reverses_match(
                r#"[foo="barα"]"#,
                encoding,
                &[
                    ("<div fOo='barα'>", true),
                    ("<div foo=barα>", true),
                    ("<div foo='BaRα'>", false),
                    ("<div foo='42'>", false),
                    ("<div bar=baz qux>", false),
                ],
            );

            assert_attr_expr_matches_and_negation_reverses_match(
                r#"[foo="barα" i]"#,
                encoding,
                &[
                    ("<div fOo='barα'>", true),
                    ("<div foo=barα>", true),
                    ("<div foo='BaRα'>", true),
                    ("<div foo='42'>", false),
                    ("<div bar=baz qux>", false),
                ],
            );

            assert_attr_expr_matches_and_negation_reverses_match(
                r#"[foo*=""]"#,
                encoding,
                &[
                    ("<div>", false),
                    ("<span>", false),
                    ("<anything-else>", false),
                ],
            );

            assert_attr_expr_matches_and_negation_reverses_match(
                r#"[foo~="μbar3"]"#,
                encoding,
                &[
                    ("<div fOo='bar1\nbar2 μbar3\tbar4'>", true),
                    ("<div foo='μbar3'>", true),
                    ("<div foo='bar1 bar2 μBAR3'>", false),
                    ("<div foo='42'>", false),
                    ("<div bar=baz qux>", false),
                ],
            );

            assert_attr_expr_matches_and_negation_reverses_match(
                r#"[foo~="μbar3" i]"#,
                encoding,
                &[
                    ("<div fOo='bar1\nbar2 μbar3\tbar4'>", true),
                    ("<div foo='μbar3'>", true),
                    ("<div foo='bar1 bar2 μBAR3'>", true),
                    ("<div foo='42'>", false),
                    ("<div bar=baz qux>", false),
                ],
            );

            // NOTE: "lang" attribute always case-insesitive for HTML elements.
            assert_attr_expr_matches_and_negation_reverses_match(
                r#"[lang|="en" s]"#,
                encoding,
                &[
                    ("<div lang='en-GB'>", true),
                    ("<div lang='en-US'>", true),
                    ("<div lang='en'>", true),
                    ("<div lang='En'>", false),
                    ("<div lang='En-GB'>", false),
                    ("<div lang='fr'>", false),
                    ("<div bar=baz qux>", false),
                ],
            );

            assert_attr_expr_matches_and_negation_reverses_match(
                r#"[lang|="en"]"#,
                encoding,
                &[
                    ("<div lang='en-GB'>", true),
                    ("<div lang='en-US'>", true),
                    ("<div lang='en'>", true),
                    ("<div lang='En'>", true),
                    ("<div lang='En-GB'>", true),
                    ("<div lang='fr'>", false),
                    ("<div bar=baz qux>", false),
                ],
            );

            assert_attr_expr_matches_and_negation_reverses_match(
                r#"[foo^="barБ"]"#,
                encoding,
                &[
                    ("<div fOo='barБ1\nbar2 bar3\tbar4'>", true),
                    ("<div foo='barБ'>", true),
                    ("<div foo='BaRБ'>", false),
                    ("<div foo='bazbarБ'>", false),
                    ("<div foo='42'>", false),
                    ("<div barБ=baz qux>", false),
                ],
            );

            assert_attr_expr_matches_and_negation_reverses_match(
                r#"[foo^="bar㕦" i]"#,
                encoding,
                &[
                    ("<div fOo='bar㕦1\nbar2 bar3\tbar4'>", true),
                    ("<div foo='bar㕦'>", true),
                    ("<div foo='BaR㕦'>", true),
                    ("<div foo='bazbar㕦'>", false),
                    ("<div foo='42'>", false),
                    ("<div bar㕦=baz qux>", false),
                ],
            );

            assert_attr_expr_matches_and_negation_reverses_match(
                r#"[foo*="barφ"]"#,
                encoding,
                &[
                    ("<div fOo='bar1\nbarφ2 bar3\tbar4'>", true),
                    ("<div foo='barφ'>", true),
                    ("<div foo='Barφ'>", false),
                    ("<div foo='42BaRφ42'>", false),
                    ("<div foo='bazbatbarφ'>", true),
                    ("<div foo='42'>", false),
                    ("<div barφ=baz qux>", false),
                ],
            );

            assert_attr_expr_matches_and_negation_reverses_match(
                r#"[foo*="barφ" i]"#,
                encoding,
                &[
                    ("<div fOo='bar1\nbarφ2 bar3\tbar4'>", true),
                    ("<div Foo='barφ'>", true),
                    ("<div foo='42BaRφ42'>", true),
                    ("<div foo='bazbatbarφ'>", true),
                    ("<div foo='42'>", false),
                    ("<div barφ=baz qux>", false),
                ],
            );

            assert_attr_expr_matches_and_negation_reverses_match(
                r#"[foo$="barЫ"]"#,
                encoding,
                &[
                    ("<div fOo='bar1\nbar2 bar3\tbarЫ'>", true),
                    ("<div foo='barЫ'>", true),
                    ("<div foo='bazbarЫ'>", true),
                    ("<div foo='BaRЫ'>", false),
                    ("<div foo='42'>", false),
                    ("<div barЫ=baz qux>", false),
                    ("<div foo='bar'>", false),
                ],
            );

            assert_attr_expr_matches_and_negation_reverses_match(
                r#"[foo$="barЫ" i]"#,
                encoding,
                &[
                    ("<div fOo='bar1\nbar2 bar3\tbarЫ'>", true),
                    ("<div foo='barЫ'>", true),
                    ("<div foo='bazbarЫ'>", true),
                    ("<div foo='BaRЫ'>", true),
                    ("<div foo='42'>", false),
                    ("<div barЫ=baz qux>", false),
                ],
            );

            assert_attr_expr_matches(
                r#"#foo1.c1.c2[foo3][foo2$="barЫ"]"#,
                encoding,
                &[
                    (
                        "<div id='foo1' class='c4 c2 c3 c1' foo3 foo2=heybarЫ>",
                        true,
                    ),
                    (
                        "<div ID='foo1' class='c4 c2 c3 c1' foo3=test foo2=barЫ>",
                        true,
                    ),
                    ("<div id='foo1' class='c4 c2 c3 c1' foo3>", false),
                    (
                        "<div id='foo1' class='c4 c2 c3 c5' foo3 foo2=heybarЫ>",
                        false,
                    ),
                    (
                        "<div id='foo12' class='c4 c2 c3 c5' foo3 foo2=heybarЫ>",
                        false,
                    ),
                ],
            );
        }
    }

    #[test]
    fn generic_expressions() {
        for encoding in &ASCII_COMPATIBLE_ENCODINGS {
            assert_generic_expr_matches(
                r#"div#foo1.c1.c2[foo3੦][foo2$="bar"]"#,
                encoding,
                &[
                    (
                        "<div id='foo1' class='c4 c2 c3 c1' foo3੦ foo2=heybar>",
                        true,
                    ),
                    (
                        "<span id='foo1' class='c4 c2 c3 c1' foo3੦ foo2=heybar>",
                        false,
                    ),
                    (
                        "<div ID='foo1' class='c4 c2 c3 c1' foo3੦=test foo2=bar>",
                        true,
                    ),
                    ("<div id='foo1' class='c4 c2 c3 c1' foo3੦>", false),
                    (
                        "<div id='foo1' class='c4 c2 c3 c5' foo3੦ foo2=heybar>",
                        false,
                    ),
                    (
                        "<div id='foo12' class='c4 c2 c3 c5' foo3੦ foo2=heybar>",
                        false,
                    ),
                ],
            );

            assert_generic_expr_matches(
                r"some-thing[lang|=en]",
                encoding,
                &[
                    ("<some-thing lang='en-GB'", true),
                    ("<some-thing lang='en-US'", true),
                    ("<some-thing lang='fr'>", false),
                    ("<some-thing lang>", false),
                    ("<span lang='en-GB'", false),
                ],
            );
        }
    }

    #[test]
    fn multiple_entry_points() {
        assert_entry_points_match(
            &["div", "div.c1.c2", "#foo", ".c1#foo"],
            4,
            &[
                ("<div>", vec![0]),
                ("<div class='c3 c2  c1'>", vec![0, 1]),
                ("<div class='c1 c2' id=foo>", vec![0, 1, 2, 3]),
                ("<div class='c1' id=foo>", vec![0, 2, 3]),
                ("<span class='c1 c2'>", vec![]),
            ],
        );

        assert_entry_points_match(
            &["span, [foo$=bar]"],
            2,
            &[
                ("<span>", vec![0]),
                ("<div fOo=testbar>", vec![0]),
                ("<span foo=bar>", vec![0]),
            ],
        );
    }

    #[test]
    fn jumps() {
        let selectors = [
            "div > .c1",
            "div > .c2",
            "div #d1",
            "div #d2",
            "[foo=bar] #id1 > #id2",
        ];

        let program = test_compile(&selectors, UTF_8, 2);

        macro_rules! exec {
            ($html:expr, $add_range:expr, $expected_ids:expr) => {{
                let mut jumps = Vec::default();
                let mut hereditary_jumps = Vec::default();
                let counter = Default::default();
                let state = SelectorState {
                    cumulative: &counter,
                    typed: None,
                };

                with_start_tag($html, UTF_8, |local_name, attr_matcher| {
                    let res =
                        exec_instr_range!($add_range, program, &state, local_name, attr_matcher);

                    assert_payload!(res.0, $expected_ids, selectors, $html);

                    jumps = res.1;
                    hereditary_jumps = res.2;
                });

                (jumps, hereditary_jumps)
            }};
        }

        {
            let (jumps, hereditary_jumps) = exec!("<div>", program.entry_points, []);

            assert_eq!(jumps.len(), 1);
            assert_eq!(hereditary_jumps.len(), 1);

            {
                let (jumps, hereditary_jumps) = exec!("<span class='c1 c2'>", jumps[0], [0, 1]);

                assert_eq!(jumps.len(), 0);
                assert_eq!(hereditary_jumps.len(), 0);
            }

            {
                let (jumps, hereditary_jumps) = exec!("<span class='c2'>", jumps[0], [1]);

                assert_eq!(jumps.len(), 0);
                assert_eq!(hereditary_jumps.len(), 0);
            }

            {
                let (jumps, hereditary_jumps) = exec!("<h1 id=d2>", hereditary_jumps[0], [3]);

                assert_eq!(jumps.len(), 0);
                assert_eq!(hereditary_jumps.len(), 0);
            }
        }

        {
            let (jumps, hereditary_jumps) = exec!("<div foo=bar>", program.entry_points, []);

            assert_eq!(jumps.len(), 1);
            assert_eq!(hereditary_jumps.len(), 2);
        }

        {
            let (jumps, hereditary_jumps) = exec!("<span foo=bar>", program.entry_points, []);

            assert_eq!(jumps.len(), 0);
            assert_eq!(hereditary_jumps.len(), 1);

            {
                let (jumps, hereditary_jumps) = exec!("<table id=id1>", hereditary_jumps[0], []);

                assert_eq!(jumps.len(), 1);
                assert_eq!(hereditary_jumps.len(), 0);

                {
                    let (jumps, hereditary_jumps) = exec!("<span id=id2>", jumps[0], [4]);

                    assert_eq!(jumps.len(), 0);
                    assert_eq!(hereditary_jumps.len(), 0);
                }
            }
        }
    }
}
