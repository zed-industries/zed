; `#fff`, `#ffffff`, `#ffffffff`
((color_value) @color.text @color)

; `rgb(..)`, `rgba(..)`, `hsl(..)`, `hsla(..)`
((call_expression
   (function_name) @_name
   (arguments)) @color @color.text
 (#any-of? @_name "rgb" "rgba" "hsl" "hsla"))

; Named colors such as `rebeccapurple`. Values that are not color names simply
; fail to parse and produce no swatch.
((plain_value) @color.text @color)
