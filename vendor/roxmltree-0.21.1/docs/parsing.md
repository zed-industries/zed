# roxmltree parsing strategy

XML parsing is hard. Everyone knows that. But the other problem is that it
can be represented in very different ways:

- You can preserve comment or ignore them completely or partially.
- You can represent text data as a separated node or embed it into the element node.
- You can keep CDATA as a separated node or merge it into the text node.
- You can preserve XML declaration or ignore it completely.
- ... and many more.

This document explains how *roxmltree* parses and represents the XML document.

## XML declaration

[XML declaration](https://www.w3.org/TR/xml/#NT-XMLDecl) is completely ignored.
Mostly because it doesn't contain any valuable information for us.

- `version` is expected to be `1.*`. Otherwise an error will occur.
- `encoding` is irrelevant since we are parsing only valid UTF-8 strings.
- And no one really follow the `standalone` constraints.

## DTD

Only `ENTITY` objects will be resolved. Everything else will be ignored
at the moment.

```xml
<!DOCTYPE test [
    <!ENTITY a 'text<p/>text'>
]>
<e>&a;</e>
```

will be parsed into:

```xml
<e>text<p/>text</e>
```

Were `p` is an element, not a text.

## Comments

All comment will be preserved.

## Processing instructions

All processing instructions will be preserved.

## Whitespaces

All whitespaces inside the root element will be preserved.

```xml
<p>
    text
</p>
```

it will be parsed as `\n␣␣␣␣text\n`.

Same goes to an escaped one:

```xml
<p>&#x20;&#x20;text&#x20;&#x20;</p>
```

it will be parsed as `␣␣text␣␣`.

## CDATA

CDATA will be embedded to a text node:

```xml
<p>t<![CDATA[e&#x20;]]>&#x20;x<![CDATA[t]]></p>
```

it will be parsed as `te&#x20; xt`.

## Text

Text will be unescaped. All entity references will be resolved.

```xml
<!DOCTYPE test [
    <!ENTITY b 'Some&#x20;text'>
]>
<p>&b;</p>
```

it will be parsed as `Some text`.

## Attribute-Value Normalization

[Attribute-Value Normalization](https://www.w3.org/TR/xml/#AVNormalize) works
as explained in the spec.

## Namespaces resolving

*roxmltree* has a complete support for XML namespaces.
