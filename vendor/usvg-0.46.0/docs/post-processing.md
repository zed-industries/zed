# XML Post-processing Steps

## No namespaces

In an SVG tree all elements and attributes belong to the SVG namespace.

## No non-SVG elements and attributes

Only SVG elements and attributes are preserved.

And their names are stored as `enum`s and not strings.
This increases performance and makes typos impossible.

## Only elements and text nodes

XML can contain elements, text nodes, comments and processing instructions.
Our tree contains only elements and text nodes inside the `text` element.

## Whitespaces trimming

Not only text nodes can be present only inside the `text` element,
but they are also trimmed according to the SVG rules, including `xml:space`.

For example:

```xml
<text>
    Text
</text>
```

becomes

```xml
<text>Text</text>
```

And

```xml
<text>
    <tspan>
        Text
    </tspan>
    <tspan>
        Text
    </tspan>
</text>
```

becomes

```xml
<text><tspan>Text</tspan> <tspan>Text</tspan></text>
```

## `style` attribute splitting

The `style` attribute content will be converted into normal attributes.

```xml
<rect style="fill:green"/>
```

will become

```xml
<rect fill="green"/>
```

The produced SVG tree never has `style` attributes.

## CSS will be applied

All _supported_ CSS rules will be applied.

```xml
<style>rect { fill:green } </style>
<rect/>
```

will become

```xml
<rect fill="green"/>
```

The produced SVG tree never has `style` elements and `class` attributes.

## `inherit` will be resolved

SVG allows setting some attribute values to `inherit`,
in which case the actual value should be taken from a parent element.

Not only it applies only to some attributes.
But some attributes also allow `inherit` only from the direct parent.

`rosvgtree` handles this for us.

## Recursive links removal

SVG supports referencing other elements via IRI and FuncIRI value types.
IRI is `xlink:href="#id"` and FuncIRI is `url(#id)`.

As in any link-based system this could lead to recursive references,
which when handled incorrectly can crash your app.

We're trying to detect all common cases, but it's
not 100% guarantee that there will be no recursive links left, but we're pretty close.

This includes simple cases like

```xml
<use id="use1" xlink:href="#use1"/>
```

and more complex one like

```xml
<clipPath id="clip1">
    <rect clip-path="url(#clip2)"/>
</clipPath>
<clipPath id="clip2">
    <rect clip-path="url(#clip1)"/>
</clipPath>
```

## Remember all elements with an ID

As mentioned above, SVG supports references. And it can reference any element in the document.<br>
Instead of checking each element in the tree each time, which would be pretty slow,
we have an ID<->Node HashMap to quickly retrieve a requested element.

## Links are groups

The `<a>` element in SVG is just a `<g>` with a URL.<br>
Since we really support only the static SVG subset, we can replace `<a>` with `<g>`.

## `tref` resolving

[`tref`](https://www.w3.org/TR/SVG11/text.html#TRefElement) is a pretty weird SVG element.
It's basically a way to reference text nodes.

We resolve them automatically and replace them with `tspan`.

```xml
<defs>
    <text id="text1">Text</text>
</defs>
<text><tref xlink:href="#text1"/></text>
```

will become

```xml
<text><tspan>Text</tspan></text>
```

## `use` will be resolved

This is probably the only breaking change to the SVG structure.

The way the `use` works, is that it creates a shadow tree of nodes
that it's referencing. This is a great way to save space,
but it makes style properties resolving way harder.

This is because when you want to get a parent element from inside the `use`,
the tree should return `use`'s parent and not the referenced element parent.

To illustrate:

```xml
<g fill="red">
    <rect id="rect1"/>
</g>
<g fill="green">
    <!-- rect's fill should be resolved to green -->
    <use href="#rect"/>
</g>
```

If you simply call `node.parent().attribute("fill")` it will return `red`, not `green`.
Because the current node is `rect1`.

As you can imagine, this is pretty hard to handle using a typical DOM model.
So instead we're simply coping referenced elements inside
the `use` so it can be treated as a regular group.

```xml
<rect id="rect1"/>
<use href="#rect"/>
```

will become

```xml
<rect id="rect1"/>
<use href="#rect">
    <rect/>
</use>
```

<br>

The main limitation of this approach, excluding the fact we're creating way more elements
that we had initially, is that copied elements must not have an `id` attribute,
otherwise we would end up with multiple duplicates.
