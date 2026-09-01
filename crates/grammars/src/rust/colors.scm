; Constructors that take one argument per channel, as used by Bevy, egui and
; friends: `Color::srgb(0.2, 0.9, 0.4)`, `Srgba::new(0.2, 0.9, 0.4, 1.0)`,
; `Color::srgb_u8(51, 230, 102)`.
;
; The anchors pin each channel to a specific argument position so that a call
; with an alpha argument matches only the four-channel pattern.
((call_expression
   function: (scoped_identifier
     path: (identifier) @_type
     name: (identifier) @_constructor)
   arguments: (arguments
     . [(float_literal) (integer_literal)] @color.red
     . [(float_literal) (integer_literal)] @color.green
     . [(float_literal) (integer_literal)] @color.blue
     . [(float_literal) (integer_literal)] @color.alpha
     .)) @color
 (#any-of? @_type "Color" "Color32" "Rgba" "Srgba" "LinearRgba")
 (#any-of? @_constructor
   "rgba" "srgba" "linear_rgba" "rgba_u8" "srgba_u8" "linear_rgba_u8"
   "new" "from_rgba_premultiplied" "from_rgba_unmultiplied"))

((call_expression
   function: (scoped_identifier
     path: (identifier) @_type
     name: (identifier) @_constructor)
   arguments: (arguments
     . [(float_literal) (integer_literal)] @color.red
     . [(float_literal) (integer_literal)] @color.green
     . [(float_literal) (integer_literal)] @color.blue
     .)) @color
 (#any-of? @_type "Color" "Color32" "Rgba" "Srgba" "LinearRgba")
 (#any-of? @_constructor
   "rgb" "srgb" "linear_rgb" "rgb_u8" "srgb_u8" "linear_rgb_u8"
   "new" "from_rgb" "from_rgb_additive"))

; Bevy's HSL constructors take a hue in degrees but keep saturation and
; lightness as 0.0-1.0 fractions, so the hue needs a scale of its own.
;
; `Color` and the `Hsl`/`Hsla` types are matched separately so that no call
; matches two patterns and produces two swatches for one color.
((call_expression
   function: (scoped_identifier
     path: (identifier) @_type
     name: (identifier) @_constructor)
   arguments: (arguments
     . [(float_literal) (integer_literal)] @color.hue
     . [(float_literal) (integer_literal)] @color.saturation
     . [(float_literal) (integer_literal)] @color.lightness
     . [(float_literal) (integer_literal)] @color.alpha
     .)) @color
 (#eq? @_type "Color")
 (#eq? @_constructor "hsla")
 (#set! color.scale "unit")
 (#set! color.hue.scale "degrees"))

((call_expression
   function: (scoped_identifier
     path: (identifier) @_type
     name: (identifier) @_constructor)
   arguments: (arguments
     . [(float_literal) (integer_literal)] @color.hue
     . [(float_literal) (integer_literal)] @color.saturation
     . [(float_literal) (integer_literal)] @color.lightness
     . [(float_literal) (integer_literal)] @color.alpha
     .)) @color
 (#eq? @_type "Hsla")
 (#any-of? @_constructor "new" "hsla")
 (#set! color.scale "unit")
 (#set! color.hue.scale "degrees"))

((call_expression
   function: (scoped_identifier
     path: (identifier) @_type
     name: (identifier) @_constructor)
   arguments: (arguments
     . [(float_literal) (integer_literal)] @color.hue
     . [(float_literal) (integer_literal)] @color.saturation
     . [(float_literal) (integer_literal)] @color.lightness
     .)) @color
 (#eq? @_type "Color")
 (#eq? @_constructor "hsl")
 (#set! color.scale "unit")
 (#set! color.hue.scale "degrees"))

((call_expression
   function: (scoped_identifier
     path: (identifier) @_type
     name: (identifier) @_constructor)
   arguments: (arguments
     . [(float_literal) (integer_literal)] @color.hue
     . [(float_literal) (integer_literal)] @color.saturation
     . [(float_literal) (integer_literal)] @color.lightness
     .)) @color
 (#eq? @_type "Hsl")
 (#any-of? @_constructor "new" "hsl")
 (#set! color.scale "unit")
 (#set! color.hue.scale "degrees"))

; Hex colors written as strings, as in theme and config files:
;   const ACCENT: &str = "#ff00aa";
((string_literal) @color.text @color
 (#match? @color.text "^\"#[0-9a-fA-F]{3,8}\"$"))
