Tree Construction Tests
=======================

Each file containing tree construction tests consists of any number of
tests separated by two newlines (LF) and a single newline before the end
of the file. For instance:

    [TEST]LF
    LF
    [TEST]LF
    LF
    [TEST]LF

Where [TEST] is the following format:

Each test must begin with a string "\#data" followed by a newline (LF).
All subsequent lines until a line that says "\#errors" are the test data
and must be passed to the system being tested unchanged, except with the
final newline (on the last line) removed.

Then there must be a line that says "\#errors". It must be followed by
one line per parse error that a conformant checker would return. It
doesn't matter what those lines are, although they can't be
"\#document-fragment", "\#document", "\#script-off", "\#script-on", or
empty, the only thing that matters is that there be the right number
of parse errors.

Then there \*may\* be a line that says "\#document-fragment", which must
be followed by a newline (LF), followed by a string of characters that
indicates the context element, followed by a newline (LF). If the string 
of characters starts with "svg ", the context element is in the SVG
namespace and the substring after "svg " is the local name. If the
string of characters starts with "math ", the context element is in the
MathML namespace and the substring after "math " is the local name.
Otherwise, the context element is in the HTML namespace and the string
is the local name. If this line is present the "\#data" must be parsed
using the HTML fragment parsing algorithm with the context element as
context.

Then there \*may\* be a line that says "\#script-off" or
"\#script-in". If a line that says "\#script-off" is present, the
parser must set the scripting flag to disabled. If a line that says
"\#script-on" is present, it must set it to enabled. Otherwise, the
test should be run in both modes.

Then there must be a line that says "\#document", which must be followed
by a dump of the tree of the parsed DOM. Each node must be represented
by a single line. Each line must start with "| ", followed by two spaces
per parent node that the node has before the root document node.

-   Element nodes must be represented by a "`<`" then the *tag name
    string* "`>`", and all the attributes must be given, sorted
    lexicographically by UTF-16 code unit according to their *attribute
    name string*, on subsequent lines, as if they were children of the
    element node.
-   Attribute nodes must have the *attribute name string*, then an "="
    sign, then the attribute value in double quotes (").
-   Text nodes must be the string, in double quotes. Newlines aren't
    escaped.
-   Comments must be "`<`" then "`!-- `" then the data then "` -->`".
-   DOCTYPEs must be "`<!DOCTYPE `" then the name then if either of the
    system id or public id is non-empty a space, public id in
    double-quotes, another space an the system id in double-quotes, and
    then in any case "`>`".
-   Processing instructions must be "`<?`", then the target, then a
    space, then the data and then "`>`". (The HTML parser cannot emit
    processing instructions, but scripts can, and the WebVTT to DOM
    rules can emit them.)
-   Template contents are represented by the string "content" with the
    children below it.

The *tag name string* is the local name prefixed by a namespace
designator. For the HTML namespace, the namespace designator is the
empty string, i.e. there's no prefix. For the SVG namespace, the
namespace designator is "svg ". For the MathML namespace, the namespace
designator is "math ".

The *attribute name string* is the local name prefixed by a namespace
designator. For no namespace, the namespace designator is the empty
string, i.e. there's no prefix. For the XLink namespace, the namespace
designator is "xlink ". For the XML namespace, the namespace designator
is "xml ". For the XMLNS namespace, the namespace designator is "xmlns
". Note the difference between "xlink:href" which is an attribute in no
namespace with the local name "xlink:href" and "xlink href" which is an
attribute in the xlink namespace with the local name "href".

If there is also a "\#document-fragment" the bit following "\#document"
must be a representation of the HTML fragment serialization for the
context element given by "\#document-fragment".

For example:

    #data
    <p>One<p>Two
    #errors
    3: Missing document type declaration
    #document
    | <html>
    |   <head>
    |   <body>
    |     <p>
    |       "One"
    |     <p>
    |       "Two"
