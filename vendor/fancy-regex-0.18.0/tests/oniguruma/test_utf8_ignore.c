// Tests that currently fail when run against fancy-regex, see README.md.
//
// x2 tests check if a pattern matches against an input at the specified start/end positions.
// x3 tests have an additional argument which is the group number to check.


  // Compile failed: ParseError(0, InvalidEscape("\\c"))
  x2("\\ca", "\001", 0, 1);

  // Compile failed: ParseError(0, InvalidEscape("\\C"))
  x2("\\C-b", "\002", 0, 1);

  // Compile failed: ParseError(0, InvalidEscape("\\c"))
  x2("\\c\\\\", "\034", 0, 1);

  // Compile failed: ParseError(2, InvalidEscape("\\c"))
  x2("q[\\c\\\\]", "q\034", 0, 2);

  // Compile failed: CompileError(InvalidBackref(17))
  x2("\\17", "\017", 0, 1);

  // No match found
  x2("(?x)  G (o O(?-x)oO) g L", "GoOoOgLe", 0, 7);

  // Compile failed: ParseError(1, InvalidClass)
  x2("[\\044-\\047]", "\046", 0, 1);

  // Compile failed: CompileError(InnerError(BuildError { kind: Syntax { pid: PatternID(0), err: Parse(Error { kind: ClassRangeInvalid, pattern: "[a-&&-a]", span: Span(Position(o: 1, l: 1, c: 2), Position(o: 4, l: 1, c: 5)) }) } }))
  x2("[a-&&-a]", "-", 0, 1);

  // No match found
  x2("(?i:ss)", "\xc3\x9f", 0, 2);

  // No match found
  x2("(?i:ss)", "\xe1\xba\x9e", 0, 3);

  // No match found
  x2("(?i:xssy)", "x\xc3\x9fy", 0, 4);

  // No match found
  x2("(?i:xssy)", "x\xe1\xba\x9ey", 0, 5);

  // No match found
  x2("(?i:\xc3\x9f)", "ss", 0, 2);

  // No match found
  x2("(?i:\xc3\x9f)", "SS", 0, 2);

  // No match found
  x2("(?i:[\xc3\x9f])", "ss", 0, 2);

  // No match found
  x2("(?i:[\xc3\x9f])", "SS", 0, 2);

  // No match found
  x2("(?m:.)", "\n", 0, 1);

  // No match found
  x2("(?m:a.)", "a\n", 0, 2);

  // No match found
  x2("(?m:.b)", "a\nb", 1, 3);

  // Match found at start 1 and end 2 (expected 0 and 2)
  x2("a(?i)b|c", "aC", 0, 2);

  // No match found
  x2("(?:ab)?{2}", "", 0, 0);

  // No match found
  x2("(?:ab)?{2}", "ababa", 0, 4);

  // No match found
  x2("(?:ab)*{0}", "ababa", 0, 0);

  // Match found at start 0 and end 2 (expected 0 and 5)
  x2("(?:ab){,}", "ab{,}", 0, 5);

  // No match found
  x2("(?:abc)+?{2}", "abcabcabc", 0, 6);

  // No match found
  x3("((?m:a.c))", "a\nc", 0, 3, 1);

  // No match found
  x2("(?:(?<x>)|(?<x>efg))\\k<x>", "", 0, 0);

  // No match found
  x2("(?:(?<n1>.)|(?<n1>..)|(?<n1>...)|(?<n1>....)|(?<n1>.....)|(?<n1>......)|(?<n1>.......)|(?<n1>........)|(?<n1>.........)|(?<n1>..........)|(?<n1>...........)|(?<n1>............)|(?<n1>.............)|(?<n1>..............))\\k<n1>$", "a-pyumpyum", 2, 10);
  
  // No match found
  x2("(?:()|())*\\1\\2", "", 0, 0);

  // No match found
  x2("(?:()|()|()|()|()|())*\\2\\5", "", 0, 0);

  // No match found
  x2("(?:()|()|()|(x)|()|())*\\2b\\5", "b", 0, 1);
  // Compile failed: ParseError(0, InvalidEscape("\\o"))
  x2("\\o{101}", "A", 0, 1);

  // Compile failed: CompileError(FeatureNotYetSupported("Backref at recursion level"))
  x2("\\A(a|b\\g<1>c)\\k<1+3>\\z", "bbacca", 0, 6);

  // Compile failed: CompileError(FeatureNotYetSupported("Backref at recursion level"))
  x2("(?i)\\A(a|b\\g<1>c)\\k<1+2>\\z", "bBACcbac", 0, 8);

  // Compile failed: CompileError(FeatureNotYetSupported("Backref exists condition with relative recursion level"))
  x2("(a)(?(1+0)b|c)d", "abd", 0, 3);

  // No match found
  x2("(?:(?'name'a)|(?'name'b))(?('name')c|d)e", "ace", 0, 3);

  // No match found
  x2("(?:()|()|())*\\3\\1", "abc", 0, 0);

  // Match found at start 0 and end 3 (expected 0 and 6)
  x2("(?<x>a)(?<x>b)(\\k<x>)+", "abbaab", 0, 6);

  // Compile failed: ParseError(1, InvalidEscape("\\o"))
  x2("[\\o{101}]", "A", 0, 1);

  // Compile failed: CompileError(FeatureNotYetSupported("Nested absent operators"))
  x2("a(?~(?~)).", "abcdefghijklmnopqrstuvwxyz", 0, 26);

  // Compile failed: CompileError(FeatureNotYetSupported("Absent expression"))
  x2("(?~|78|\\d*)", "123456789", 0, 6);

  // Compile failed: CompileError(FeatureNotYetSupported("Absent expression"))
  x2("(?~|def|(?:abc|de|f){0,100})", "abcdedeabcfdefabc", 0, 11);

  // Compile failed: CompileError(FeatureNotYetSupported("Absent expression"))
  x2("(?~|ab|.*)", "ccc\nddd", 0, 3);

  // Compile failed: CompileError(FeatureNotYetSupported("Absent expression"))
  x2("(?~|ab|\\O*)", "ccc\ndab", 0, 5);

  // Compile failed: CompileError(FeatureNotYetSupported("Absent expression"))
  x2("(?~|ab|\\O{2,10})", "ccc\ndab", 0, 5);

  // Compile failed: CompileError(FeatureNotYetSupported("Absent expression"))
  x2("(?~|ab|\\O{1,10})", "ab", 1, 2);

  // Compile failed: CompileError(FeatureNotYetSupported("Absent expression"))
  x2("(?~|abc|\\O{1,10})", "abc", 1, 3);

  // Compile failed: CompileError(FeatureNotYetSupported("Absent expression"))
  x2("(?~|ab|\\O{5,10})|abc", "abc", 0, 3);

  // Compile failed: CompileError(FeatureNotYetSupported("Absent expression"))
  x2("(?~|ab|\\O{1,10})", "cccccccccccab", 0, 10);

  // Compile failed: CompileError(FeatureNotYetSupported("Absent expression"))
  x2("(?~|aaa|)", "aaa", 0, 0);

  // Compile failed: CompileError(FeatureNotYetSupported("Absent expression"))
  x2("(?~||a*)", "aaaaaa", 0, 0);

  // Compile failed: CompileError(FeatureNotYetSupported("Absent expression"))
  x2("(?~||a*?)", "aaaaaa", 0, 0);

  // Compile failed: CompileError(FeatureNotYetSupported("Absent expression"))
  x2("(a)(?~|b|\\1)", "aaaaaa", 0, 2);

  // Compile failed: CompileError(FeatureNotYetSupported("Absent expression"))
  x2("(a)(?~|bb|(?:a\\1)*)", "aaaaaa", 0, 5);

  // Compile failed: CompileError(FeatureNotYetSupported("Absent expression"))
  x2("(b|c)(?~|abac|(?:a\\1)*)", "abababacabab", 1, 4);

  // Compile failed: CompileError(FeatureNotYetSupported("Absent expression"))
  x2("(?~|aaaaa|a*+)", "aaaaa", 0, 0);

  // Compile failed: CompileError(FeatureNotYetSupported("Absent expression"))
  x2("(?~|aaaaaa|a*+)b", "aaaaaab", 1, 7);

  // Compile failed: CompileError(FeatureNotYetSupported("Absent expression"))
  x2("(?~|abcd|(?>))", "zzzabcd", 0, 0);

  // Compile failed: CompileError(FeatureNotYetSupported("Absent expression"))
  x2("(?~|abc|a*?)", "aaaabc", 0, 0);

  // Compile failed: CompileError(FeatureNotYetSupported("Absent stopper"))
  x2("(?~|abc)a*", "aaaaaabc", 0, 5);

  // Compile failed: CompileError(FeatureNotYetSupported("Absent stopper"))
  x2("(?~|abc)a*z|aaaaaabc", "aaaaaabc", 0, 8);

  // Compile failed: CompileError(FeatureNotYetSupported("Absent stopper"))
  x2("(?~|aaaaaa)a*", "aaaaaa", 0, 0);

  // Compile failed: CompileError(FeatureNotYetSupported("Absent stopper"))
  x2("(?~|abc)aaaa|aaaabc", "aaaabc", 0, 6);

  // Compile failed: CompileError(FeatureNotYetSupported("Absent stopper"))
  x2("(?>(?~|abc))aaaa|aaaabc", "aaaabc", 0, 6);

  // Compile failed: CompileError(FeatureNotYetSupported("Range clear"))
  x2("(?~|)a", "a", 0, 1);

  // Compile failed: CompileError(FeatureNotYetSupported("Absent stopper"))
  x2("(?~|a)(?~|)a", "a", 0, 1);

  // Compile failed: CompileError(FeatureNotYetSupported("Absent stopper"))
  x2("(?~|a).*(?~|)a", "bbbbbbbbbbbbbbbbbbbba", 0, 21);

  // Compile failed: CompileError(FeatureNotYetSupported("Absent stopper"))
  x2("(?~|abc).*(xyz|pqr)(?~|)abc", "aaaaxyzaaapqrabc", 0, 16);

  // Compile failed: CompileError(FeatureNotYetSupported("Absent stopper"))
  x2("(?~|abc).*(xyz|pqr)(?~|)abc", "aaaaxyzaaaabcpqrabc", 11, 19);

  // No match found
  x2("\\xca\\xb8", "\xca\xb8", 0, 2);

  // No match found
  x2("(?m:よ.)", "よ\n", 0, 4);

  // No match found
  x2("(?m:.め)", "ま\nめ", 3, 7);

  // No match found
  x2("(?:あい)?{2}", "", 0, 0);

  // No match found
  x2("(?:鬼車)?{2}", "鬼車鬼車鬼", 0, 12);

  // No match found
  x2("(?:鬼車)*{0}", "鬼車鬼車鬼", 0, 0);

  // Match found at start 0 and end 6 (expected 0 and 9)
  x2("(?:鬼車){,}", "鬼車{,}", 0, 9);

  // No match found
  x2("(?:かきく)+?{2}", "かきくかきくかきく", 0, 18);

  // No match found
  x3("((?m:あ.う))", "あ\nう", 0, 7, 1);

  // Compile failed: CompileError(InnerError(BuildError { kind: Syntax { pid: PatternID(0), err: Parse(Error { kind: ClassRangeInvalid, pattern: "[あ-&&-あ]", span: Span(Position(o: 1, l: 1, c: 2), Position(o: 6, l: 1, c: 5)) }) } }))
  x2("[あ-&&-あ]", "-", 0, 1);

  // No match found
  n("[\\p{^Word}]", "こ");

  // Compile failed: ParseError(2, UnknownFlag("(?-W"))
  x2("(?-W:\\p{Word})", "こ", 0, 3);

  // Compile failed: ParseError(2, UnknownFlag("(?W"))
  x2("(?W:\\p{Word})", "k", 0, 1);

  // Compile failed: ParseError(2, UnknownFlag("(?-W"))
  x2("(?-W:[[:word:]])", "こ", 0, 3);

  // Compile failed: ParseError(2, UnknownFlag("(?-D"))
  x2("(?-D:\\p{Digit})", "３", 0, 3);

  // Compile failed: ParseError(2, UnknownFlag("(?-S"))
  x2("(?-S:\\p{Space})", "\xc2\x85", 0, 2);

  // Compile failed: ParseError(2, UnknownFlag("(?-P"))
  x2("(?-P:\\p{Word})", "こ", 0, 3);

  // Compile failed: ParseError(2, UnknownFlag("(?-W"))
  x2("(?-W:\\w)", "こ", 0, 3);

  // Compile failed: ParseError(2, UnknownFlag("(?-W"))
  x2("(?-W:\\w)", "k", 0, 1);

  // Compile failed: ParseError(2, UnknownFlag("(?W"))
  x2("(?W:\\w)", "k", 0, 1);

  // Compile failed: ParseError(2, UnknownFlag("(?W"))
  x2("(?W:\\W)", "こ", 0, 3);

  // Compile failed: ParseError(2, UnknownFlag("(?-W"))
  x2("(?-W:\\b)", "こ", 0, 0);

  // Compile failed: ParseError(2, UnknownFlag("(?-W"))
  x2("(?-W:\\b)", "h", 0, 0);

  // Compile failed: ParseError(2, UnknownFlag("(?W"))
  x2("(?W:\\b)", "h", 0, 0);

  // Compile failed: ParseError(2, UnknownFlag("(?W"))
  x2("(?W:\\B)", "こ", 0, 0);

  // Compile failed: ParseError(2, UnknownFlag("(?-P"))
  x2("(?-P:\\b)", "こ", 0, 0);

  // Compile failed: ParseError(2, UnknownFlag("(?-P"))
  x2("(?-P:\\b)", "h", 0, 0);

  // Compile failed: ParseError(2, UnknownFlag("(?P"))
  x2("(?P:\\b)", "h", 0, 0);

  // Compile failed: ParseError(2, UnknownFlag("(?P"))
  x2("(?P:\\B)", "こ", 0, 0);

  // Compile failed: CompileError(InnerError(BuildError { kind: Syntax { pid: PatternID(0), err: Translate(Error { kind: UnicodePropertyNotFound, pattern: "\\p{inbasiclatin}", span: Span(Position(o: 0, l: 1, c: 1), Position(o: 16, l: 1, c: 17)) }) } }))
  x2("\\p{InBasicLatin}", "\x41", 0, 1);

  // Compile failed: ParseError(1, InvalidEscape("\\Y"))
  x2(".\\Y\\O", "\x0d\x0a", 0, 2);

  // Compile failed: ParseError(1, InvalidEscape("\\Y"))
  x2(".\\Y.", "\x67\xCC\x88", 0, 3);

  // Compile failed: ParseError(0, InvalidEscape("\\y"))
  x2("\\y.\\Y.\\y", "\x67\xCC\x88", 0, 3);

  // Compile failed: ParseError(0, InvalidEscape("\\y"))
  x2("\\y.\\y", "\xEA\xB0\x81", 0, 3);

  // Compile failed: ParseError(2, InvalidEscape("\\Y"))
  x2("^.\\Y.\\Y.$", "\xE1\x84\x80\xE1\x85\xA1\xE1\x86\xA8", 0, 9);

  // Compile failed: ParseError(1, InvalidEscape("\\Y"))
  x2(".\\Y.", "\xE0\xAE\xA8\xE0\xAE\xBF", 0, 6);

  // Compile failed: ParseError(1, InvalidEscape("\\Y"))
  x2(".\\Y.", "\xE0\xB8\x81\xE0\xB8\xB3", 0, 6);

  // Compile failed: ParseError(1, InvalidEscape("\\Y"))
  x2(".\\Y.", "\xE0\xA4\xB7\xE0\xA4\xBF", 0, 6);

  // Compile failed: ParseError(2, InvalidEscape("\\Y"))
  x2("..\\Y.", "\xE3\x80\xB0\xE2\x80\x8D\xE2\xAD\x95", 0, 9);

  // Compile failed: ParseError(3, InvalidEscape("\\Y"))
  x2("...\\Y.", "\xE3\x80\xB0\xCC\x82\xE2\x80\x8D\xE2\xAD\x95", 0, 11);

  // Compile failed: ParseError(1, InvalidEscape("\\X"))
  x2("^\\X$", "\x0d\x0a", 0, 2);

  // Compile failed: ParseError(1, InvalidEscape("\\X"))
  x2("^\\X$", "\x67\xCC\x88", 0, 3);

  // Compile failed: ParseError(1, InvalidEscape("\\X"))
  x2("^\\X$", "\xE1\x84\x80\xE1\x85\xA1\xE1\x86\xA8", 0, 9);

  // Compile failed: ParseError(1, InvalidEscape("\\X"))
  x2("^\\X$", "\xE0\xAE\xA8\xE0\xAE\xBF", 0, 6);

  // Compile failed: ParseError(1, InvalidEscape("\\X"))
  x2("^\\X$", "\xE0\xB8\x81\xE0\xB8\xB3", 0, 6);

  // Compile failed: ParseError(1, InvalidEscape("\\X"))
  x2("^\\X$", "\xE0\xA4\xB7\xE0\xA4\xBF", 0, 6);

  // Compile failed: ParseError(1, InvalidEscape("\\X"))
  x2("h\\Xllo", "ha\xCC\x80llo", 0, 7);

  // Compile failed: ParseError(2, UnknownFlag("(?y"))
  x2("(?y{g})\\yabc\\y", "abc", 0, 3);

  // Compile failed: ParseError(2, UnknownFlag("(?y"))
  x2("(?y{g})\\y\\X\\y", "abc", 0, 1);

  // Compile failed: ParseError(2, UnknownFlag("(?y"))
  x2("(?y{w})\\yabc\\y", "abc", 0, 3);

  // Compile failed: ParseError(2, UnknownFlag("(?y"))
  x2("(?y{w})\\X", "\r\n", 0, 2);

  // Compile failed: ParseError(2, UnknownFlag("(?y"))
  x2("(?y{w})\\X", "\x0cz", 0, 1);

  // Compile failed: ParseError(2, UnknownFlag("(?y"))
  x2("(?y{w})\\X", "q\x0c", 0, 1);

  // Compile failed: ParseError(2, UnknownFlag("(?y"))
  x2("(?y{w})\\X", "\xE2\x80\x8D\xE2\x9D\x87", 0, 6);

  // Compile failed: ParseError(2, UnknownFlag("(?y"))
  x2("(?y{w})\\X", "\x20\x20", 0, 2);

  // Compile failed: ParseError(2, UnknownFlag("(?y"))
  x2("(?y{w})\\X", "a\xE2\x80\x8D", 0, 4);

  // Compile failed: ParseError(2, UnknownFlag("(?y"))
  x2("(?y{w})\\y\\X\\y", "abc", 0, 3);

  // Compile failed: ParseError(2, UnknownFlag("(?y"))
  x2("(?y{w})\\y\\X\\y", "v\xCE\x87w", 0, 4);

  // Compile failed: ParseError(2, UnknownFlag("(?y"))
  x2("(?y{w})\\y\\X\\y", "\xD7\x93\x27", 0, 3);

  // Compile failed: ParseError(2, UnknownFlag("(?y"))
  x2("(?y{w})\\y\\X\\y", "\xD7\x93\x22\xD7\x93", 0, 5);

  // Compile failed: ParseError(2, UnknownFlag("(?y"))
  x2("(?y{w})\\X", "14 45", 0, 2);

  // Compile failed: ParseError(2, UnknownFlag("(?y"))
  x2("(?y{w})\\X", "a14", 0, 3);

  // Compile failed: ParseError(2, UnknownFlag("(?y"))
  x2("(?y{w})\\X", "832e", 0, 4);

  // Compile failed: ParseError(2, UnknownFlag("(?y"))
  x2("(?y{w})\\X", "8\xEF\xBC\x8C\xDB\xB0", 0, 6);

  // Compile failed: ParseError(2, UnknownFlag("(?y"))
  x2("(?y{w})\\y\\X\\y", "ケン", 0, 6);

  // Compile failed: ParseError(2, UnknownFlag("(?y"))
  x2("(?y{w})\\y\\X\\y", "ケン\xE2\x80\xAFタ", 0, 12);

  // Compile failed: ParseError(2, UnknownFlag("(?y"))
  x2("(?y{w})\\y\\X\\y", "\x21\x23", 0, 1);

  // Compile failed: ParseError(2, UnknownFlag("(?y"))
  x2("(?y{w})\\y\\X\\y", "山ア", 0, 3);

  // Compile failed: ParseError(2, UnknownFlag("(?y"))
  x2("(?y{w})\\X", "3.14", 0, 4);

  // Compile failed: ParseError(2, UnknownFlag("(?y"))
  x2("(?y{w})\\X", "3 14", 0, 1);

  // Compile failed: ParseError(2, InvalidHex)
  x2("\\x1", "\x01", 0, 1);

  // Compile failed: ParseError(10, TargetNotRepeatable)
  x2("((?()0+)+++(((0\\g<0>)0)|())++++((?(1)(0\\g<0>))++++++0*())++++((?(1)(0\\g<1>)+)++++++++++*())++++((?(1)((0)\\g<0>)+)++())+0++*+++(((0\\g<0>))*())++++((?(1)(0\\g<0>)+)++++++++++*|)++++*+++((?(1)((0)\\g<0>)+)+++++++++())++*|)++++((?()0))|", "abcde", 0, 0);

  // Compile failed: ParseError(9, TargetNotRepeatable)
  x2("(?:[ab]|(*MAX{2}).)*", "abcbaaccaaa", 0, 7);

  // Compile failed: ParseError(4, TargetNotRepeatable)
  x2("(?:(*COUNT[AB]{X})[ab]|(*COUNT[CD]{X})[cd])*(*CMP{AB,<,CD})",
     "abababcdab", 5, 8);

  // Compile failed: ParseError(3, TargetNotRepeatable)
  x2("(?(?{....})123|456)", "123", 0, 3);

  // Compile failed: CompileError(LeftRecursiveSubroutineCall("group 0"))
  x2("\\g'0'++{,0}",   "abcdefgh", 0, 0);

  // Compile failed: CompileError(LeftRecursiveSubroutineCall("group 0"))
  x2("\\g'0'++{,0}?",  "abcdefgh", 0, 0);

  // Compile failed: CompileError(LeftRecursiveSubroutineCall("group 0"))
  x2("\\g'0'++{,0}b",  "abcdefgh", 1, 2);

  // Compile failed: CompileError(LeftRecursiveSubroutineCall("group 0"))
  x2("\\g'0'++{,0}?def", "abcdefgh", 3, 6);

  // Compile failed: CompileError(InnerError(BuildError { kind: Syntax { pid: PatternID(0), err: Parse(Error { kind: RepetitionCountInvalid, pattern: "a{3,2}b", span: Span(Position(o: 1, l: 1, c: 2), Position(o: 6, l: 1, c: 7)) }) } }))
  x2("a{3,2}b", "aaab", 0, 4);

  // Compile failed: CompileError(InnerError(BuildError { kind: Syntax { pid: PatternID(0), err: Parse(Error { kind: RepetitionCountInvalid, pattern: "a{3,2}b", span: Span(Position(o: 1, l: 1, c: 2), Position(o: 6, l: 1, c: 7)) }) } }))
  x2("a{3,2}b", "aaaab", 1, 5);

  // Compile failed: CompileError(InnerError(BuildError { kind: Syntax { pid: PatternID(0), err: Parse(Error { kind: RepetitionCountInvalid, pattern: "a{3,2}b", span: Span(Position(o: 1, l: 1, c: 2), Position(o: 6, l: 1, c: 7)) }) } }))
  x2("a{3,2}b", "aab", 0, 3);

  // Compile failed: CompileError(InnerError(BuildError { kind: Syntax { pid: PatternID(0), err: Parse(Error { kind: RepetitionCountInvalid, pattern: "a{3,2}?", span: Span(Position(o: 1, l: 1, c: 2), Position(o: 7, l: 1, c: 8)) }) } }))
  x2("a{3,2}?", "", 0, 0);

  // No match found
  x2("a{2,3}+a", "aaa", 0, 3);

  // Compile failed: CompileError(InnerError(BuildError { kind: Syntax { pid: PatternID(0), err: Translate(Error { kind: UnicodePropertyNotFound, pattern: "\\p{in_enclosed_cjk_letters_and_months}", span: Span(Position(o: 0, l: 1, c: 1), Position(o: 38, l: 1, c: 39)) }) } }))
  x2("\\p{In_Enclosed_CJK_Letters_and_Months}", "\xe3\x8b\xbf", 0, 3);
