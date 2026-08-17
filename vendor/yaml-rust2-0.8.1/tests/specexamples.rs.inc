const EX2_1 : &str =
    "- Mark McGwire\n- Sammy Sosa\n- Ken Griffey";

const EX2_2 : &str =
    "hr:  65    # Home runs\navg: 0.278 # Batting average\nrbi: 147   # Runs Batted In";

const EX2_3 : &str =
    "american:\n- Boston Red Sox\n- Detroit Tigers\n- New York Yankees\nnational:\n- New York Mets\n- Chicago Cubs\n- Atlanta Braves";

const EX2_4 : &str =
    "-\n  name: Mark McGwire\n  hr:   65\n  avg:  0.278\n-\n  name: Sammy Sosa\n  hr:   63\n  avg:  0.288";

const EX2_5 : &str =
    "- [name        , hr, avg  ]\n- [Mark McGwire, 65, 0.278]\n- [Sammy Sosa  , 63, 0.288]";

const EX2_6 : &str =
    "Mark McGwire: {hr: 65, avg: 0.278}\nSammy Sosa: {\n    hr: 63,\n    avg: 0.288\n  }";

const EX2_7 : &str =
    "# Ranking of 1998 home runs\n---\n- Mark McGwire\n- Sammy Sosa\n- Ken Griffey\n\n# Team ranking\n---\n- Chicago Cubs\n- St Louis Cardinals";

const EX2_8 : &str =
    "---\ntime: 20:03:20\nplayer: Sammy Sosa\naction: strike (miss)\n...\n---\ntime: 20:03:47\nplayer: Sammy Sosa\naction: grand slam\n...";

const EX2_9 : &str =
    "---\nhr: # 1998 hr ranking\n  - Mark McGwire\n  - Sammy Sosa\nrbi:\n  # 1998 rbi ranking\n  - Sammy Sosa\n  - Ken Griffey";

const EX2_10 : &str =
    "---\nhr:\n  - Mark McGwire\n  # Following node labeled SS\n  - &SS Sammy Sosa\nrbi:\n  - *SS # Subsequent occurrence\n  - Ken Griffey";

const EX2_11 : &str =
    "? - Detroit Tigers\n  - Chicago cubs\n:\n  - 2001-07-23\n\n? [ New York Yankees,\n    Atlanta Braves ]\n: [ 2001-07-02, 2001-08-12,\n    2001-08-14 ]";

const EX2_12 : &str =
    "---\n# Products purchased\n- item    : Super Hoop\n  quantity: 1\n- item    : Basketball\n  quantity: 4\n- item    : Big Shoes\n  quantity: 1";

const EX2_13 : &str =
    "# ASCII Art\n--- |\n  \\//||\\/||\n  // ||  ||__";

const EX2_14 : &str =
    "--- >\n  Mark McGwire's\n  year was crippled\n  by a knee injury.";

const EX2_15 : &str =
    ">\n Sammy Sosa completed another\n fine season with great stats.\n \n   63 Home Runs\n   0.288 Batting Average\n \n What a year!";

const EX2_16 : &str =
    "name: Mark McGwire\naccomplishment: >\n  Mark set a major league\n  home run record in 1998.\nstats: |\n  65 Home Runs\n  0.278 Batting Average\n";

const EX2_17 : &str =
    "unicode: \"Sosa did fine.\\u263A\"\ncontrol: \"\\b1998\\t1999\\t2000\\n\"\nhex esc: \"\\x0d\\x0a is \\r\\n\"\n\nsingle: '\"Howdy!\" he cried.'\nquoted: ' # Not a ''comment''.'\ntie-fighter: '|\\-*-/|'";

const EX2_18 : &str =
    "plain:\n  This unquoted scalar\n  spans many lines.\n\nquoted: \"So does this\n  quoted scalar.\\n\"";

// TODO: 2.19 - 2.22 schema tags

const EX2_23 : &str =
    "---\nnot-date: !!str 2002-04-28\n\npicture: !!binary |\n R0lGODlhDAAMAIQAAP//9/X\n 17unp5WZmZgAAAOfn515eXv\n Pz7Y6OjuDg4J+fn5OTk6enp\n 56enmleECcgggoBADs=\n\napplication specific tag: !something |\n The semantics of the tag\n above may be different for\n different documents.";

const EX2_24 : &str =
    "%TAG ! tag:clarkevans.com,2002:\n--- !shape\n  # Use the ! handle for presenting\n  # tag:clarkevans.com,2002:circle\n- !circle\n  center: &ORIGIN {x: 73, y: 129}\n  radius: 7\n- !line\n  start: *ORIGIN\n  finish: { x: 89, y: 102 }\n- !label\n  start: *ORIGIN\n  color: 0xFFEEBB\n  text: Pretty vector drawing.";

const EX2_25 : &str =
    "# Sets are represented as a\n# Mapping where each key is\n# associated with a null value\n--- !!set\n? Mark McGwire\n? Sammy Sosa\n? Ken Griffey";

const EX2_26 : &str =
    "# Ordered maps are represented as\n# A sequence of mappings, with\n# each mapping having one key\n--- !!omap\n- Mark McGwire: 65\n- Sammy Sosa: 63\n- Ken Griffey: 58";

const EX2_27 : &str =
    "--- !<tag:clarkevans.com,2002:invoice>\ninvoice: 34843\ndate   : 2001-01-23\nbill-to: &id001\n    given  : Chris\n    family : Dumars\n    address:\n        lines: |\n            458 Walkman Dr.\n            Suite #292\n        city    : Royal Oak\n        state   : MI\n        postal  : 48046\nship-to: *id001\nproduct:\n    - sku         : BL394D\n      quantity    : 4\n      description : Basketball\n      price       : 450.00\n    - sku         : BL4438H\n      quantity    : 1\n      description : Super Hoop\n      price       : 2392.00\ntax  : 251.42\ntotal: 4443.52\ncomments:\n    Late afternoon is best.\n    Backup contact is Nancy\n    Billsmer @ 338-4338.";

const EX2_28 : &str =
    "---\nTime: 2001-11-23 15:01:42 -5\nUser: ed\nWarning:\n  This is an error message\n  for the log file\n---\nTime: 2001-11-23 15:02:31 -5\nUser: ed\nWarning:\n  A slightly different error\n  message.\n---\nDate: 2001-11-23 15:03:17 -5\nUser: ed\nFatal:\n  Unknown variable \"bar\"\nStack:\n  - file: TopClass.py\n    line: 23\n    code: |\n      x = MoreObject(\"345\\n\")\n  - file: MoreClass.py\n    line: 58\n    code: |-\n      foo = bar";

// TODO: 5.1 - 5.2 BOM

const EX5_3 : &str =
    "sequence:\n- one\n- two\nmapping:\n  ? sky\n  : blue\n  sea : green";

const EX5_4 : &str =
    "sequence: [ one, two, ]\nmapping: { sky: blue, sea: green }";

const EX5_5 : &str = "# Comment only.";

const EX5_6 : &str =
    "anchored: !local &anchor value\nalias: *anchor";

const EX5_7 : &str =
    "literal: |\n  some\n  text\nfolded: >\n  some\n  text\n";

const EX5_8 : &str =
    "single: 'text'\ndouble: \"text\"";

// TODO: 5.9 directive
// TODO: 5.10 reserved indicator

const EX5_11 : &str =
    "|\n  Line break (no glyph)\n  Line break (glyphed)\n";

const EX5_12 : &str =
    "# Tabs and spaces\nquoted: \"Quoted\t\"\nblock:	|\n  void main() {\n  \tprintf(\"Hello, world!\\n\");\n  }";

const EX5_13 : &str =
    "\"Fun with \\\\\n\\\" \\a \\b \\e \\f \\\n\\n \\r \\t \\v \\0 \\\n\\  \\_ \\N \\L \\P \\\n\\x41 \\u0041 \\U00000041\"";

const EX5_14 : &str =
    "Bad escapes:\n  \"\\c\n  \\xq-\"";

const EX6_1 : &str =
    "  # Leading comment line spaces are\n   # neither content nor indentation.\n    \nNot indented:\n By one space: |\n    By four\n      spaces\n Flow style: [    # Leading spaces\n   By two,        # in flow style\n  Also by two,    # are neither\n  \tStill by two   # content nor\n    ]             # indentation.";

const EX6_2 : &str =
    "? a\n: -\tb\n  -  -\tc\n     - d";

const EX6_3 : &str =
    "- foo:\t bar\n- - baz\n  -\tbaz";

const EX6_4 : &str =
    "plain: text\n  lines\nquoted: \"text\n  \tlines\"\nblock: |\n  text\n   \tlines\n";

const EX6_5 : &str =
    "Folding:\n  \"Empty line\n   \t\n  as a line feed\"\nChomping: |\n  Clipped empty lines\n ";

const EX6_6 : &str =
    ">-\n  trimmed\n  \n \n\n  as\n  space";

const EX6_7 : &str =
    ">\n  foo \n \n  \t bar\n\n  baz\n";

const EX6_8 : &str =
    "\"\n  foo \n \n  \t bar\n\n  baz\n\"";

const EX6_9 : &str =
    "key:    # Comment\n  value";

const EX6_10 : &str =
    "  # Comment\n   \n\n";

const EX6_11 : &str =
    "key:    # Comment\n        # lines\n  value\n\n";

const EX6_12 : &str =
    "{ first: Sammy, last: Sosa }:\n# Statistics:\n  hr:  # Home runs\n     65\n  avg: # Average\n   0.278";

const EX6_13 : &str =
    "%FOO  bar baz # Should be ignored\n               # with a warning.\n--- \"foo\"";

const EX6_14 : &str =
    "%YAML 1.3 # Attempt parsing\n           # with a warning\n---\n\"foo\"";

const EX6_15 : &str =
    "%YAML 1.2\n%YAML 1.1\nfoo";

const EX6_16 : &str =
    "%TAG !yaml! tag:yaml.org,2002:\n---\n!yaml!str \"foo\"";

const EX6_17 : &str =
    "%TAG ! !foo\n%TAG ! !foo\nbar";

const EX6_18 : &str =
    "# Private\n!foo \"bar\"\n...\n# Global\n%TAG ! tag:example.com,2000:app/\n---\n!foo \"bar\"";

const EX6_19 : &str =
    "%TAG !! tag:example.com,2000:app/\n---\n!!int 1 - 3 # Interval, not integer";

const EX6_20 : &str =
    "%TAG !e! tag:example.com,2000:app/\n---\n!e!foo \"bar\"";

const EX6_21 : &str =
    "%TAG !m! !my-\n--- # Bulb here\n!m!light fluorescent\n...\n%TAG !m! !my-\n--- # Color here\n!m!light green";

const EX6_22 : &str =
    "%TAG !e! tag:example.com,2000:app/\n---\n- !e!foo \"bar\"";

const EX6_23 : &str =
    "!!str &a1 \"foo\":\n  !!str bar\n&a2 baz : *a1";

const EX6_24 : &str =
    "!<tag:yaml.org,2002:str> foo :\n  !<!bar> baz";

const EX6_25 : &str =
    "- !<!> foo\n- !<$:?> bar\n";

const EX6_26 : &str =
    "%TAG !e! tag:example.com,2000:app/\n---\n- !local foo\n- !!str bar\n- !e!tag%21 baz\n";

const EX6_27a : &str =
    "%TAG !e! tag:example,2000:app/\n---\n- !e! foo";

const EX6_27b : &str =
    "%TAG !e! tag:example,2000:app/\n---\n- !h!bar baz";

const EX6_28 : &str =
    "# Assuming conventional resolution:\n- \"12\"\n- 12\n- ! 12";

const EX6_29 : &str =
    "First occurrence: &anchor Value\nSecond occurrence: *anchor";

const EX7_1 : &str =
    "First occurrence: &anchor Foo\nSecond occurrence: *anchor\nOverride anchor: &anchor Bar\nReuse anchor: *anchor";

const EX7_2 : &str =
    "{\n  foo : !!str,\n  !!str : bar,\n}";

const EX7_3 : &str =
    "{\n  ? foo :,\n  : bar,\n}\n";

const EX7_4 : &str =
    "\"implicit block key\" : [\n  \"implicit flow key\" : value,\n ]";

const EX7_5 : &str =
    "\"folded \nto a space,\t\n \nto a line feed, or \t\\\n \\ \tnon-content\"";

const EX7_6 : &str =
    "\" 1st non-empty\n\n 2nd non-empty \n\t3rd non-empty \"";

const EX7_7 : &str = " 'here''s to \"quotes\"'";

const EX7_8 : &str =
    "'implicit block key' : [\n  'implicit flow key' : value,\n ]";

const EX7_9 : &str =
    "' 1st non-empty\n\n 2nd non-empty \n\t3rd non-empty '";

const EX7_10 : &str =
    "# Outside flow collection:\n- ::vector\n- \": - ()\"\n- Up, up, and away!\n- -123\n- http://example.com/foo#bar\n# Inside flow collection:\n- [ ::vector,\n  \": - ()\",\n  \"Up, up, and away!\",\n  -123,\n  http://example.com/foo#bar ]";

const EX7_11 : &str =
    "implicit block key : [\n  implicit flow key : value,\n ]";

const EX7_12 : &str =
    "1st non-empty\n\n 2nd non-empty \n\t3rd non-empty";

const EX7_13 : &str =
    "- [ one, two, ]\n- [three ,four]";

const EX7_14 : &str =
    "[\n\"double\n quoted\", 'single\n           quoted',\nplain\n text, [ nested ],\nsingle: pair,\n]";

const EX7_15 : &str =
    "- { one : two , three: four , }\n- {five: six,seven : eight}";

const EX7_16 : &str =
    "{\n? explicit: entry,\nimplicit: entry,\n?\n}";

const EX7_17 : &str =
    "{\nunquoted : \"separate\",\nhttp://foo.com,\nomitted value:,\n: omitted key,\n}";

const EX7_18 : &str =
    "{\n\"adjacent\":value,\n\"readable\":value,\n\"empty\":\n}";

const EX7_19 : &str =
    "[\nfoo: bar\n]";

const EX7_20 : &str =
    "[\n? foo\n bar : baz\n]";

const EX7_21 : &str =
    "- [ YAML : separate ]\n- [ : empty key entry ]\n- [ {JSON: like}:adjacent ]";

const EX7_22 : &str =
    "[ foo\n bar: invalid,";  // Note: we don't check (on purpose) the >1K chars for an
                       // implicit key

const EX7_23 : &str =
    "- [ a, b ]\n- { a: b }\n- \"a\"\n- 'b'\n- c";

const EX7_24 : &str =
    "- !!str \"a\"\n- 'b'\n- &anchor \"c\"\n- *anchor\n- !!str";

const EX8_1 : &str =
    "- | # Empty header\n literal\n- >1 # Indentation indicator\n  folded\n- |+ # Chomping indicator\n keep\n\n- >1- # Both indicators\n  strip\n";

const EX8_2 : &str =
    "- |\n detected\n- >\n \n  \n  # detected\n- |1\n  explicit\n- >\n \t\n detected\n";

const EX8_3a : &str =
    "- |\n  \n text";

const EX8_3b : &str =
    "- >\n  text\n text";

const EX8_3c : &str =
    "- |2\n text";

const EX8_4 : &str =
    "strip: |-\n  text\nclip: |\n  text\nkeep: |+\n  text\n";

const EX8_5 : &str =
    " # Strip\n  # Comments:\nstrip: |-\n  # text\n  \n # Clip\n  # comments:\n\nclip: |\n  # text\n \n # Keep\n  # comments:\n\nkeep: |+\n  # text\n\n # Trail\n  # Comments\n";

const EX8_6 : &str =
    "strip: >-\n\nclip: >\n\nkeep: |+\n\n";

const EX8_7 : &str =
    "|\n literal\n \ttext\n\n";

const EX8_8 : &str =
    "|\n \n  \n  literal\n   \n  \n  text\n\n # Comment\n";

const EX8_9 : &str =
    ">\n folded\n text\n\n";

const EX8_10 : &str =
    ">\n\n folded\n line\n\n next\n line\n   * bullet\n\n   * list\n   * lines\n\n last\n line\n\n# Comment\n";

const EX8_11 : &str = EX8_10;
const EX8_12 : &str = EX8_10;
const EX8_13 : &str = EX8_10;

const EX8_14 : &str =
    "block sequence:\n  - one\n  - two : three\n";

const EX8_15 : &str =
    "- # Empty\n- |\n block node\n- - one # Compact\n  - two # sequence\n- one: two # Compact mapping\n";

const EX8_16 : &str =
    "block mapping:\n key: value\n";

const EX8_17 : &str =
    "? explicit key # Empty value\n? |\n  block key\n: - one # Explicit compact\n  - two # block value\n";

// XXX libyaml failed this test
const EX8_18 : &str =
    "plain key: in-line value\n:  # Both empty\n\"quoted key\":\n- entry\n";

const EX8_19 : &str =
    "- sun: yellow\n- ? earth: blue\n  : moon: white\n";

const EX8_20 : &str =
    "-\n  \"flow in block\"\n- >\n Block scalar\n- !!map # Block collection\n  foo : bar\n";

const EX8_21 : &str =
    "literal: |2\n  value\nfolded:\n   !foo\n  >1\n value\n";

const EX8_22 : &str =
    "sequence: !!seq\n- entry\n- !!seq\n - nested\nmapping: !!map\n foo: bar\n";
