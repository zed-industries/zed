module tracy
  use, intrinsic :: iso_c_binding, only: c_ptr, c_loc, c_char, c_null_char, &
    & c_size_t, c_int8_t, c_int16_t, c_int32_t, c_int64_t, c_int, c_float, c_double, c_null_ptr
  implicit none
  private

  integer(c_int32_t), parameter, public :: TRACY_PLOTFORMAT_NUMBER = 0
  integer(c_int32_t), parameter, public :: TRACY_PLOTFORMAT_MEMORY = 1
  integer(c_int32_t), parameter, public :: TRACY_PLOTFORMAT_PERCENTAGE = 2
  integer(c_int32_t), parameter, public :: TRACY_PLOTFORMAT_WATT = 3

  character(c_char), parameter, public :: tracy_null_char = c_null_char

  type, bind(C) :: TracyColors_t
    integer(c_int32_t) :: Snow = int(Z'fffafa', kind=c_int32_t)
    integer(c_int32_t) :: GhostWhite = int(Z'f8f8ff', kind=c_int32_t)
    integer(c_int32_t) :: WhiteSmoke = int(Z'f5f5f5', kind=c_int32_t)
    integer(c_int32_t) :: Gainsboro = int(Z'dcdcdc', kind=c_int32_t)
    integer(c_int32_t) :: FloralWhite = int(Z'fffaf0', kind=c_int32_t)
    integer(c_int32_t) :: OldLace = int(Z'fdf5e6', kind=c_int32_t)
    integer(c_int32_t) :: Linen = int(Z'faf0e6', kind=c_int32_t)
    integer(c_int32_t) :: AntiqueWhite = int(Z'faebd7', kind=c_int32_t)
    integer(c_int32_t) :: PapayaWhip = int(Z'ffefd5', kind=c_int32_t)
    integer(c_int32_t) :: BlanchedAlmond = int(Z'ffebcd', kind=c_int32_t)
    integer(c_int32_t) :: Bisque = int(Z'ffe4c4', kind=c_int32_t)
    integer(c_int32_t) :: PeachPuff = int(Z'ffdab9', kind=c_int32_t)
    integer(c_int32_t) :: NavajoWhite = int(Z'ffdead', kind=c_int32_t)
    integer(c_int32_t) :: Moccasin = int(Z'ffe4b5', kind=c_int32_t)
    integer(c_int32_t) :: Cornsilk = int(Z'fff8dc', kind=c_int32_t)
    integer(c_int32_t) :: Ivory = int(Z'fffff0', kind=c_int32_t)
    integer(c_int32_t) :: LemonChiffon = int(Z'fffacd', kind=c_int32_t)
    integer(c_int32_t) :: Seashell = int(Z'fff5ee', kind=c_int32_t)
    integer(c_int32_t) :: Honeydew = int(Z'f0fff0', kind=c_int32_t)
    integer(c_int32_t) :: MintCream = int(Z'f5fffa', kind=c_int32_t)
    integer(c_int32_t) :: Azure = int(Z'f0ffff', kind=c_int32_t)
    integer(c_int32_t) :: AliceBlue = int(Z'f0f8ff', kind=c_int32_t)
    integer(c_int32_t) :: Lavender = int(Z'e6e6fa', kind=c_int32_t)
    integer(c_int32_t) :: LavenderBlush = int(Z'fff0f5', kind=c_int32_t)
    integer(c_int32_t) :: MistyRose = int(Z'ffe4e1', kind=c_int32_t)
    integer(c_int32_t) :: White = int(Z'ffffff', kind=c_int32_t)
    integer(c_int32_t) :: Black = int(Z'000000', kind=c_int32_t)
    integer(c_int32_t) :: DarkSlateGray = int(Z'2f4f4f', kind=c_int32_t)
    integer(c_int32_t) :: DarkSlateGrey = int(Z'2f4f4f', kind=c_int32_t)
    integer(c_int32_t) :: DimGray = int(Z'696969', kind=c_int32_t)
    integer(c_int32_t) :: DimGrey = int(Z'696969', kind=c_int32_t)
    integer(c_int32_t) :: SlateGray = int(Z'708090', kind=c_int32_t)
    integer(c_int32_t) :: SlateGrey = int(Z'708090', kind=c_int32_t)
    integer(c_int32_t) :: LightSlateGray = int(Z'778899', kind=c_int32_t)
    integer(c_int32_t) :: LightSlateGrey = int(Z'778899', kind=c_int32_t)
    integer(c_int32_t) :: Gray = int(Z'bebebe', kind=c_int32_t)
    integer(c_int32_t) :: Grey = int(Z'bebebe', kind=c_int32_t)
    integer(c_int32_t) :: X11Gray = int(Z'bebebe', kind=c_int32_t)
    integer(c_int32_t) :: X11Grey = int(Z'bebebe', kind=c_int32_t)
    integer(c_int32_t) :: WebGray = int(Z'808080', kind=c_int32_t)
    integer(c_int32_t) :: WebGrey = int(Z'808080', kind=c_int32_t)
    integer(c_int32_t) :: LightGrey = int(Z'd3d3d3', kind=c_int32_t)
    integer(c_int32_t) :: LightGray = int(Z'd3d3d3', kind=c_int32_t)
    integer(c_int32_t) :: MidnightBlue = int(Z'191970', kind=c_int32_t)
    integer(c_int32_t) :: Navy = int(Z'000080', kind=c_int32_t)
    integer(c_int32_t) :: NavyBlue = int(Z'000080', kind=c_int32_t)
    integer(c_int32_t) :: CornflowerBlue = int(Z'6495ed', kind=c_int32_t)
    integer(c_int32_t) :: DarkSlateBlue = int(Z'483d8b', kind=c_int32_t)
    integer(c_int32_t) :: SlateBlue = int(Z'6a5acd', kind=c_int32_t)
    integer(c_int32_t) :: MediumSlateBlue = int(Z'7b68ee', kind=c_int32_t)
    integer(c_int32_t) :: LightSlateBlue = int(Z'8470ff', kind=c_int32_t)
    integer(c_int32_t) :: MediumBlue = int(Z'0000cd', kind=c_int32_t)
    integer(c_int32_t) :: RoyalBlue = int(Z'4169e1', kind=c_int32_t)
    integer(c_int32_t) :: Blue = int(Z'0000ff', kind=c_int32_t)
    integer(c_int32_t) :: DodgerBlue = int(Z'1e90ff', kind=c_int32_t)
    integer(c_int32_t) :: DeepSkyBlue = int(Z'00bfff', kind=c_int32_t)
    integer(c_int32_t) :: SkyBlue = int(Z'87ceeb', kind=c_int32_t)
    integer(c_int32_t) :: LightSkyBlue = int(Z'87cefa', kind=c_int32_t)
    integer(c_int32_t) :: SteelBlue = int(Z'4682b4', kind=c_int32_t)
    integer(c_int32_t) :: LightSteelBlue = int(Z'b0c4de', kind=c_int32_t)
    integer(c_int32_t) :: LightBlue = int(Z'add8e6', kind=c_int32_t)
    integer(c_int32_t) :: PowderBlue = int(Z'b0e0e6', kind=c_int32_t)
    integer(c_int32_t) :: PaleTurquoise = int(Z'afeeee', kind=c_int32_t)
    integer(c_int32_t) :: DarkTurquoise = int(Z'00ced1', kind=c_int32_t)
    integer(c_int32_t) :: MediumTurquoise = int(Z'48d1cc', kind=c_int32_t)
    integer(c_int32_t) :: Turquoise = int(Z'40e0d0', kind=c_int32_t)
    integer(c_int32_t) :: Cyan = int(Z'00ffff', kind=c_int32_t)
    integer(c_int32_t) :: Aqua = int(Z'00ffff', kind=c_int32_t)
    integer(c_int32_t) :: LightCyan = int(Z'e0ffff', kind=c_int32_t)
    integer(c_int32_t) :: CadetBlue = int(Z'5f9ea0', kind=c_int32_t)
    integer(c_int32_t) :: MediumAquamarine = int(Z'66cdaa', kind=c_int32_t)
    integer(c_int32_t) :: Aquamarine = int(Z'7fffd4', kind=c_int32_t)
    integer(c_int32_t) :: DarkGreen = int(Z'006400', kind=c_int32_t)
    integer(c_int32_t) :: DarkOliveGreen = int(Z'556b2f', kind=c_int32_t)
    integer(c_int32_t) :: DarkSeaGreen = int(Z'8fbc8f', kind=c_int32_t)
    integer(c_int32_t) :: SeaGreen = int(Z'2e8b57', kind=c_int32_t)
    integer(c_int32_t) :: MediumSeaGreen = int(Z'3cb371', kind=c_int32_t)
    integer(c_int32_t) :: LightSeaGreen = int(Z'20b2aa', kind=c_int32_t)
    integer(c_int32_t) :: PaleGreen = int(Z'98fb98', kind=c_int32_t)
    integer(c_int32_t) :: SpringGreen = int(Z'00ff7f', kind=c_int32_t)
    integer(c_int32_t) :: LawnGreen = int(Z'7cfc00', kind=c_int32_t)
    integer(c_int32_t) :: Green = int(Z'00ff00', kind=c_int32_t)
    integer(c_int32_t) :: Lime = int(Z'00ff00', kind=c_int32_t)
    integer(c_int32_t) :: X11Green = int(Z'00ff00', kind=c_int32_t)
    integer(c_int32_t) :: WebGreen = int(Z'008000', kind=c_int32_t)
    integer(c_int32_t) :: Chartreuse = int(Z'7fff00', kind=c_int32_t)
    integer(c_int32_t) :: MediumSpringGreen = int(Z'00fa9a', kind=c_int32_t)
    integer(c_int32_t) :: GreenYellow = int(Z'adff2f', kind=c_int32_t)
    integer(c_int32_t) :: LimeGreen = int(Z'32cd32', kind=c_int32_t)
    integer(c_int32_t) :: YellowGreen = int(Z'9acd32', kind=c_int32_t)
    integer(c_int32_t) :: ForestGreen = int(Z'228b22', kind=c_int32_t)
    integer(c_int32_t) :: OliveDrab = int(Z'6b8e23', kind=c_int32_t)
    integer(c_int32_t) :: DarkKhaki = int(Z'bdb76b', kind=c_int32_t)
    integer(c_int32_t) :: Khaki = int(Z'f0e68c', kind=c_int32_t)
    integer(c_int32_t) :: PaleGoldenrod = int(Z'eee8aa', kind=c_int32_t)
    integer(c_int32_t) :: LightGoldenrodYellow = int(Z'fafad2', kind=c_int32_t)
    integer(c_int32_t) :: LightYellow = int(Z'ffffe0', kind=c_int32_t)
    integer(c_int32_t) :: Yellow = int(Z'ffff00', kind=c_int32_t)
    integer(c_int32_t) :: Gold = int(Z'ffd700', kind=c_int32_t)
    integer(c_int32_t) :: LightGoldenrod = int(Z'eedd82', kind=c_int32_t)
    integer(c_int32_t) :: Goldenrod = int(Z'daa520', kind=c_int32_t)
    integer(c_int32_t) :: DarkGoldenrod = int(Z'b8860b', kind=c_int32_t)
    integer(c_int32_t) :: RosyBrown = int(Z'bc8f8f', kind=c_int32_t)
    integer(c_int32_t) :: IndianRed = int(Z'cd5c5c', kind=c_int32_t)
    integer(c_int32_t) :: SaddleBrown = int(Z'8b4513', kind=c_int32_t)
    integer(c_int32_t) :: Sienna = int(Z'a0522d', kind=c_int32_t)
    integer(c_int32_t) :: Peru = int(Z'cd853f', kind=c_int32_t)
    integer(c_int32_t) :: Burlywood = int(Z'deb887', kind=c_int32_t)
    integer(c_int32_t) :: Beige = int(Z'f5f5dc', kind=c_int32_t)
    integer(c_int32_t) :: Wheat = int(Z'f5deb3', kind=c_int32_t)
    integer(c_int32_t) :: SandyBrown = int(Z'f4a460', kind=c_int32_t)
    integer(c_int32_t) :: Tan = int(Z'd2b48c', kind=c_int32_t)
    integer(c_int32_t) :: Chocolate = int(Z'd2691e', kind=c_int32_t)
    integer(c_int32_t) :: Firebrick = int(Z'b22222', kind=c_int32_t)
    integer(c_int32_t) :: Brown = int(Z'a52a2a', kind=c_int32_t)
    integer(c_int32_t) :: DarkSalmon = int(Z'e9967a', kind=c_int32_t)
    integer(c_int32_t) :: Salmon = int(Z'fa8072', kind=c_int32_t)
    integer(c_int32_t) :: LightSalmon = int(Z'ffa07a', kind=c_int32_t)
    integer(c_int32_t) :: Orange = int(Z'ffa500', kind=c_int32_t)
    integer(c_int32_t) :: DarkOrange = int(Z'ff8c00', kind=c_int32_t)
    integer(c_int32_t) :: Coral = int(Z'ff7f50', kind=c_int32_t)
    integer(c_int32_t) :: LightCoral = int(Z'f08080', kind=c_int32_t)
    integer(c_int32_t) :: Tomato = int(Z'ff6347', kind=c_int32_t)
    integer(c_int32_t) :: OrangeRed = int(Z'ff4500', kind=c_int32_t)
    integer(c_int32_t) :: Red = int(Z'ff0000', kind=c_int32_t)
    integer(c_int32_t) :: HotPink = int(Z'ff69b4', kind=c_int32_t)
    integer(c_int32_t) :: DeepPink = int(Z'ff1493', kind=c_int32_t)
    integer(c_int32_t) :: Pink = int(Z'ffc0cb', kind=c_int32_t)
    integer(c_int32_t) :: LightPink = int(Z'ffb6c1', kind=c_int32_t)
    integer(c_int32_t) :: PaleVioletRed = int(Z'db7093', kind=c_int32_t)
    integer(c_int32_t) :: Maroon = int(Z'b03060', kind=c_int32_t)
    integer(c_int32_t) :: X11Maroon = int(Z'b03060', kind=c_int32_t)
    integer(c_int32_t) :: WebMaroon = int(Z'800000', kind=c_int32_t)
    integer(c_int32_t) :: MediumVioletRed = int(Z'c71585', kind=c_int32_t)
    integer(c_int32_t) :: VioletRed = int(Z'd02090', kind=c_int32_t)
    integer(c_int32_t) :: Magenta = int(Z'ff00ff', kind=c_int32_t)
    integer(c_int32_t) :: Fuchsia = int(Z'ff00ff', kind=c_int32_t)
    integer(c_int32_t) :: Violet = int(Z'ee82ee', kind=c_int32_t)
    integer(c_int32_t) :: Plum = int(Z'dda0dd', kind=c_int32_t)
    integer(c_int32_t) :: Orchid = int(Z'da70d6', kind=c_int32_t)
    integer(c_int32_t) :: MediumOrchid = int(Z'ba55d3', kind=c_int32_t)
    integer(c_int32_t) :: DarkOrchid = int(Z'9932cc', kind=c_int32_t)
    integer(c_int32_t) :: DarkViolet = int(Z'9400d3', kind=c_int32_t)
    integer(c_int32_t) :: BlueViolet = int(Z'8a2be2', kind=c_int32_t)
    integer(c_int32_t) :: Purple = int(Z'a020f0', kind=c_int32_t)
    integer(c_int32_t) :: X11Purple = int(Z'a020f0', kind=c_int32_t)
    integer(c_int32_t) :: WebPurple = int(Z'800080', kind=c_int32_t)
    integer(c_int32_t) :: MediumPurple = int(Z'9370db', kind=c_int32_t)
    integer(c_int32_t) :: Thistle = int(Z'd8bfd8', kind=c_int32_t)
    integer(c_int32_t) :: Snow1 = int(Z'fffafa', kind=c_int32_t)
    integer(c_int32_t) :: Snow2 = int(Z'eee9e9', kind=c_int32_t)
    integer(c_int32_t) :: Snow3 = int(Z'cdc9c9', kind=c_int32_t)
    integer(c_int32_t) :: Snow4 = int(Z'8b8989', kind=c_int32_t)
    integer(c_int32_t) :: Seashell1 = int(Z'fff5ee', kind=c_int32_t)
    integer(c_int32_t) :: Seashell2 = int(Z'eee5de', kind=c_int32_t)
    integer(c_int32_t) :: Seashell3 = int(Z'cdc5bf', kind=c_int32_t)
    integer(c_int32_t) :: Seashell4 = int(Z'8b8682', kind=c_int32_t)
    integer(c_int32_t) :: AntiqueWhite1 = int(Z'ffefdb', kind=c_int32_t)
    integer(c_int32_t) :: AntiqueWhite2 = int(Z'eedfcc', kind=c_int32_t)
    integer(c_int32_t) :: AntiqueWhite3 = int(Z'cdc0b0', kind=c_int32_t)
    integer(c_int32_t) :: AntiqueWhite4 = int(Z'8b8378', kind=c_int32_t)
    integer(c_int32_t) :: Bisque1 = int(Z'ffe4c4', kind=c_int32_t)
    integer(c_int32_t) :: Bisque2 = int(Z'eed5b7', kind=c_int32_t)
    integer(c_int32_t) :: Bisque3 = int(Z'cdb79e', kind=c_int32_t)
    integer(c_int32_t) :: Bisque4 = int(Z'8b7d6b', kind=c_int32_t)
    integer(c_int32_t) :: PeachPuff1 = int(Z'ffdab9', kind=c_int32_t)
    integer(c_int32_t) :: PeachPuff2 = int(Z'eecbad', kind=c_int32_t)
    integer(c_int32_t) :: PeachPuff3 = int(Z'cdaf95', kind=c_int32_t)
    integer(c_int32_t) :: PeachPuff4 = int(Z'8b7765', kind=c_int32_t)
    integer(c_int32_t) :: NavajoWhite1 = int(Z'ffdead', kind=c_int32_t)
    integer(c_int32_t) :: NavajoWhite2 = int(Z'eecfa1', kind=c_int32_t)
    integer(c_int32_t) :: NavajoWhite3 = int(Z'cdb38b', kind=c_int32_t)
    integer(c_int32_t) :: NavajoWhite4 = int(Z'8b795e', kind=c_int32_t)
    integer(c_int32_t) :: LemonChiffon1 = int(Z'fffacd', kind=c_int32_t)
    integer(c_int32_t) :: LemonChiffon2 = int(Z'eee9bf', kind=c_int32_t)
    integer(c_int32_t) :: LemonChiffon3 = int(Z'cdc9a5', kind=c_int32_t)
    integer(c_int32_t) :: LemonChiffon4 = int(Z'8b8970', kind=c_int32_t)
    integer(c_int32_t) :: Cornsilk1 = int(Z'fff8dc', kind=c_int32_t)
    integer(c_int32_t) :: Cornsilk2 = int(Z'eee8cd', kind=c_int32_t)
    integer(c_int32_t) :: Cornsilk3 = int(Z'cdc8b1', kind=c_int32_t)
    integer(c_int32_t) :: Cornsilk4 = int(Z'8b8878', kind=c_int32_t)
    integer(c_int32_t) :: Ivory1 = int(Z'fffff0', kind=c_int32_t)
    integer(c_int32_t) :: Ivory2 = int(Z'eeeee0', kind=c_int32_t)
    integer(c_int32_t) :: Ivory3 = int(Z'cdcdc1', kind=c_int32_t)
    integer(c_int32_t) :: Ivory4 = int(Z'8b8b83', kind=c_int32_t)
    integer(c_int32_t) :: Honeydew1 = int(Z'f0fff0', kind=c_int32_t)
    integer(c_int32_t) :: Honeydew2 = int(Z'e0eee0', kind=c_int32_t)
    integer(c_int32_t) :: Honeydew3 = int(Z'c1cdc1', kind=c_int32_t)
    integer(c_int32_t) :: Honeydew4 = int(Z'838b83', kind=c_int32_t)
    integer(c_int32_t) :: LavenderBlush1 = int(Z'fff0f5', kind=c_int32_t)
    integer(c_int32_t) :: LavenderBlush2 = int(Z'eee0e5', kind=c_int32_t)
    integer(c_int32_t) :: LavenderBlush3 = int(Z'cdc1c5', kind=c_int32_t)
    integer(c_int32_t) :: LavenderBlush4 = int(Z'8b8386', kind=c_int32_t)
    integer(c_int32_t) :: MistyRose1 = int(Z'ffe4e1', kind=c_int32_t)
    integer(c_int32_t) :: MistyRose2 = int(Z'eed5d2', kind=c_int32_t)
    integer(c_int32_t) :: MistyRose3 = int(Z'cdb7b5', kind=c_int32_t)
    integer(c_int32_t) :: MistyRose4 = int(Z'8b7d7b', kind=c_int32_t)
    integer(c_int32_t) :: Azure1 = int(Z'f0ffff', kind=c_int32_t)
    integer(c_int32_t) :: Azure2 = int(Z'e0eeee', kind=c_int32_t)
    integer(c_int32_t) :: Azure3 = int(Z'c1cdcd', kind=c_int32_t)
    integer(c_int32_t) :: Azure4 = int(Z'838b8b', kind=c_int32_t)
    integer(c_int32_t) :: SlateBlue1 = int(Z'836fff', kind=c_int32_t)
    integer(c_int32_t) :: SlateBlue2 = int(Z'7a67ee', kind=c_int32_t)
    integer(c_int32_t) :: SlateBlue3 = int(Z'6959cd', kind=c_int32_t)
    integer(c_int32_t) :: SlateBlue4 = int(Z'473c8b', kind=c_int32_t)
    integer(c_int32_t) :: RoyalBlue1 = int(Z'4876ff', kind=c_int32_t)
    integer(c_int32_t) :: RoyalBlue2 = int(Z'436eee', kind=c_int32_t)
    integer(c_int32_t) :: RoyalBlue3 = int(Z'3a5fcd', kind=c_int32_t)
    integer(c_int32_t) :: RoyalBlue4 = int(Z'27408b', kind=c_int32_t)
    integer(c_int32_t) :: Blue1 = int(Z'0000ff', kind=c_int32_t)
    integer(c_int32_t) :: Blue2 = int(Z'0000ee', kind=c_int32_t)
    integer(c_int32_t) :: Blue3 = int(Z'0000cd', kind=c_int32_t)
    integer(c_int32_t) :: Blue4 = int(Z'00008b', kind=c_int32_t)
    integer(c_int32_t) :: DodgerBlue1 = int(Z'1e90ff', kind=c_int32_t)
    integer(c_int32_t) :: DodgerBlue2 = int(Z'1c86ee', kind=c_int32_t)
    integer(c_int32_t) :: DodgerBlue3 = int(Z'1874cd', kind=c_int32_t)
    integer(c_int32_t) :: DodgerBlue4 = int(Z'104e8b', kind=c_int32_t)
    integer(c_int32_t) :: SteelBlue1 = int(Z'63b8ff', kind=c_int32_t)
    integer(c_int32_t) :: SteelBlue2 = int(Z'5cacee', kind=c_int32_t)
    integer(c_int32_t) :: SteelBlue3 = int(Z'4f94cd', kind=c_int32_t)
    integer(c_int32_t) :: SteelBlue4 = int(Z'36648b', kind=c_int32_t)
    integer(c_int32_t) :: DeepSkyBlue1 = int(Z'00bfff', kind=c_int32_t)
    integer(c_int32_t) :: DeepSkyBlue2 = int(Z'00b2ee', kind=c_int32_t)
    integer(c_int32_t) :: DeepSkyBlue3 = int(Z'009acd', kind=c_int32_t)
    integer(c_int32_t) :: DeepSkyBlue4 = int(Z'00688b', kind=c_int32_t)
    integer(c_int32_t) :: SkyBlue1 = int(Z'87ceff', kind=c_int32_t)
    integer(c_int32_t) :: SkyBlue2 = int(Z'7ec0ee', kind=c_int32_t)
    integer(c_int32_t) :: SkyBlue3 = int(Z'6ca6cd', kind=c_int32_t)
    integer(c_int32_t) :: SkyBlue4 = int(Z'4a708b', kind=c_int32_t)
    integer(c_int32_t) :: LightSkyBlue1 = int(Z'b0e2ff', kind=c_int32_t)
    integer(c_int32_t) :: LightSkyBlue2 = int(Z'a4d3ee', kind=c_int32_t)
    integer(c_int32_t) :: LightSkyBlue3 = int(Z'8db6cd', kind=c_int32_t)
    integer(c_int32_t) :: LightSkyBlue4 = int(Z'607b8b', kind=c_int32_t)
    integer(c_int32_t) :: SlateGray1 = int(Z'c6e2ff', kind=c_int32_t)
    integer(c_int32_t) :: SlateGray2 = int(Z'b9d3ee', kind=c_int32_t)
    integer(c_int32_t) :: SlateGray3 = int(Z'9fb6cd', kind=c_int32_t)
    integer(c_int32_t) :: SlateGray4 = int(Z'6c7b8b', kind=c_int32_t)
    integer(c_int32_t) :: LightSteelBlue1 = int(Z'cae1ff', kind=c_int32_t)
    integer(c_int32_t) :: LightSteelBlue2 = int(Z'bcd2ee', kind=c_int32_t)
    integer(c_int32_t) :: LightSteelBlue3 = int(Z'a2b5cd', kind=c_int32_t)
    integer(c_int32_t) :: LightSteelBlue4 = int(Z'6e7b8b', kind=c_int32_t)
    integer(c_int32_t) :: LightBlue1 = int(Z'bfefff', kind=c_int32_t)
    integer(c_int32_t) :: LightBlue2 = int(Z'b2dfee', kind=c_int32_t)
    integer(c_int32_t) :: LightBlue3 = int(Z'9ac0cd', kind=c_int32_t)
    integer(c_int32_t) :: LightBlue4 = int(Z'68838b', kind=c_int32_t)
    integer(c_int32_t) :: LightCyan1 = int(Z'e0ffff', kind=c_int32_t)
    integer(c_int32_t) :: LightCyan2 = int(Z'd1eeee', kind=c_int32_t)
    integer(c_int32_t) :: LightCyan3 = int(Z'b4cdcd', kind=c_int32_t)
    integer(c_int32_t) :: LightCyan4 = int(Z'7a8b8b', kind=c_int32_t)
    integer(c_int32_t) :: PaleTurquoise1 = int(Z'bbffff', kind=c_int32_t)
    integer(c_int32_t) :: PaleTurquoise2 = int(Z'aeeeee', kind=c_int32_t)
    integer(c_int32_t) :: PaleTurquoise3 = int(Z'96cdcd', kind=c_int32_t)
    integer(c_int32_t) :: PaleTurquoise4 = int(Z'668b8b', kind=c_int32_t)
    integer(c_int32_t) :: CadetBlue1 = int(Z'98f5ff', kind=c_int32_t)
    integer(c_int32_t) :: CadetBlue2 = int(Z'8ee5ee', kind=c_int32_t)
    integer(c_int32_t) :: CadetBlue3 = int(Z'7ac5cd', kind=c_int32_t)
    integer(c_int32_t) :: CadetBlue4 = int(Z'53868b', kind=c_int32_t)
    integer(c_int32_t) :: Turquoise1 = int(Z'00f5ff', kind=c_int32_t)
    integer(c_int32_t) :: Turquoise2 = int(Z'00e5ee', kind=c_int32_t)
    integer(c_int32_t) :: Turquoise3 = int(Z'00c5cd', kind=c_int32_t)
    integer(c_int32_t) :: Turquoise4 = int(Z'00868b', kind=c_int32_t)
    integer(c_int32_t) :: Cyan1 = int(Z'00ffff', kind=c_int32_t)
    integer(c_int32_t) :: Cyan2 = int(Z'00eeee', kind=c_int32_t)
    integer(c_int32_t) :: Cyan3 = int(Z'00cdcd', kind=c_int32_t)
    integer(c_int32_t) :: Cyan4 = int(Z'008b8b', kind=c_int32_t)
    integer(c_int32_t) :: DarkSlateGray1 = int(Z'97ffff', kind=c_int32_t)
    integer(c_int32_t) :: DarkSlateGray2 = int(Z'8deeee', kind=c_int32_t)
    integer(c_int32_t) :: DarkSlateGray3 = int(Z'79cdcd', kind=c_int32_t)
    integer(c_int32_t) :: DarkSlateGray4 = int(Z'528b8b', kind=c_int32_t)
    integer(c_int32_t) :: Aquamarine1 = int(Z'7fffd4', kind=c_int32_t)
    integer(c_int32_t) :: Aquamarine2 = int(Z'76eec6', kind=c_int32_t)
    integer(c_int32_t) :: Aquamarine3 = int(Z'66cdaa', kind=c_int32_t)
    integer(c_int32_t) :: Aquamarine4 = int(Z'458b74', kind=c_int32_t)
    integer(c_int32_t) :: DarkSeaGreen1 = int(Z'c1ffc1', kind=c_int32_t)
    integer(c_int32_t) :: DarkSeaGreen2 = int(Z'b4eeb4', kind=c_int32_t)
    integer(c_int32_t) :: DarkSeaGreen3 = int(Z'9bcd9b', kind=c_int32_t)
    integer(c_int32_t) :: DarkSeaGreen4 = int(Z'698b69', kind=c_int32_t)
    integer(c_int32_t) :: SeaGreen1 = int(Z'54ff9f', kind=c_int32_t)
    integer(c_int32_t) :: SeaGreen2 = int(Z'4eee94', kind=c_int32_t)
    integer(c_int32_t) :: SeaGreen3 = int(Z'43cd80', kind=c_int32_t)
    integer(c_int32_t) :: SeaGreen4 = int(Z'2e8b57', kind=c_int32_t)
    integer(c_int32_t) :: PaleGreen1 = int(Z'9aff9a', kind=c_int32_t)
    integer(c_int32_t) :: PaleGreen2 = int(Z'90ee90', kind=c_int32_t)
    integer(c_int32_t) :: PaleGreen3 = int(Z'7ccd7c', kind=c_int32_t)
    integer(c_int32_t) :: PaleGreen4 = int(Z'548b54', kind=c_int32_t)
    integer(c_int32_t) :: SpringGreen1 = int(Z'00ff7f', kind=c_int32_t)
    integer(c_int32_t) :: SpringGreen2 = int(Z'00ee76', kind=c_int32_t)
    integer(c_int32_t) :: SpringGreen3 = int(Z'00cd66', kind=c_int32_t)
    integer(c_int32_t) :: SpringGreen4 = int(Z'008b45', kind=c_int32_t)
    integer(c_int32_t) :: Green1 = int(Z'00ff00', kind=c_int32_t)
    integer(c_int32_t) :: Green2 = int(Z'00ee00', kind=c_int32_t)
    integer(c_int32_t) :: Green3 = int(Z'00cd00', kind=c_int32_t)
    integer(c_int32_t) :: Green4 = int(Z'008b00', kind=c_int32_t)
    integer(c_int32_t) :: Chartreuse1 = int(Z'7fff00', kind=c_int32_t)
    integer(c_int32_t) :: Chartreuse2 = int(Z'76ee00', kind=c_int32_t)
    integer(c_int32_t) :: Chartreuse3 = int(Z'66cd00', kind=c_int32_t)
    integer(c_int32_t) :: Chartreuse4 = int(Z'458b00', kind=c_int32_t)
    integer(c_int32_t) :: OliveDrab1 = int(Z'c0ff3e', kind=c_int32_t)
    integer(c_int32_t) :: OliveDrab2 = int(Z'b3ee3a', kind=c_int32_t)
    integer(c_int32_t) :: OliveDrab3 = int(Z'9acd32', kind=c_int32_t)
    integer(c_int32_t) :: OliveDrab4 = int(Z'698b22', kind=c_int32_t)
    integer(c_int32_t) :: DarkOliveGreen1 = int(Z'caff70', kind=c_int32_t)
    integer(c_int32_t) :: DarkOliveGreen2 = int(Z'bcee68', kind=c_int32_t)
    integer(c_int32_t) :: DarkOliveGreen3 = int(Z'a2cd5a', kind=c_int32_t)
    integer(c_int32_t) :: DarkOliveGreen4 = int(Z'6e8b3d', kind=c_int32_t)
    integer(c_int32_t) :: Khaki1 = int(Z'fff68f', kind=c_int32_t)
    integer(c_int32_t) :: Khaki2 = int(Z'eee685', kind=c_int32_t)
    integer(c_int32_t) :: Khaki3 = int(Z'cdc673', kind=c_int32_t)
    integer(c_int32_t) :: Khaki4 = int(Z'8b864e', kind=c_int32_t)
    integer(c_int32_t) :: LightGoldenrod1 = int(Z'ffec8b', kind=c_int32_t)
    integer(c_int32_t) :: LightGoldenrod2 = int(Z'eedc82', kind=c_int32_t)
    integer(c_int32_t) :: LightGoldenrod3 = int(Z'cdbe70', kind=c_int32_t)
    integer(c_int32_t) :: LightGoldenrod4 = int(Z'8b814c', kind=c_int32_t)
    integer(c_int32_t) :: LightYellow1 = int(Z'ffffe0', kind=c_int32_t)
    integer(c_int32_t) :: LightYellow2 = int(Z'eeeed1', kind=c_int32_t)
    integer(c_int32_t) :: LightYellow3 = int(Z'cdcdb4', kind=c_int32_t)
    integer(c_int32_t) :: LightYellow4 = int(Z'8b8b7a', kind=c_int32_t)
    integer(c_int32_t) :: Yellow1 = int(Z'ffff00', kind=c_int32_t)
    integer(c_int32_t) :: Yellow2 = int(Z'eeee00', kind=c_int32_t)
    integer(c_int32_t) :: Yellow3 = int(Z'cdcd00', kind=c_int32_t)
    integer(c_int32_t) :: Yellow4 = int(Z'8b8b00', kind=c_int32_t)
    integer(c_int32_t) :: Gold1 = int(Z'ffd700', kind=c_int32_t)
    integer(c_int32_t) :: Gold2 = int(Z'eec900', kind=c_int32_t)
    integer(c_int32_t) :: Gold3 = int(Z'cdad00', kind=c_int32_t)
    integer(c_int32_t) :: Gold4 = int(Z'8b7500', kind=c_int32_t)
    integer(c_int32_t) :: Goldenrod1 = int(Z'ffc125', kind=c_int32_t)
    integer(c_int32_t) :: Goldenrod2 = int(Z'eeb422', kind=c_int32_t)
    integer(c_int32_t) :: Goldenrod3 = int(Z'cd9b1d', kind=c_int32_t)
    integer(c_int32_t) :: Goldenrod4 = int(Z'8b6914', kind=c_int32_t)
    integer(c_int32_t) :: DarkGoldenrod1 = int(Z'ffb90f', kind=c_int32_t)
    integer(c_int32_t) :: DarkGoldenrod2 = int(Z'eead0e', kind=c_int32_t)
    integer(c_int32_t) :: DarkGoldenrod3 = int(Z'cd950c', kind=c_int32_t)
    integer(c_int32_t) :: DarkGoldenrod4 = int(Z'8b6508', kind=c_int32_t)
    integer(c_int32_t) :: RosyBrown1 = int(Z'ffc1c1', kind=c_int32_t)
    integer(c_int32_t) :: RosyBrown2 = int(Z'eeb4b4', kind=c_int32_t)
    integer(c_int32_t) :: RosyBrown3 = int(Z'cd9b9b', kind=c_int32_t)
    integer(c_int32_t) :: RosyBrown4 = int(Z'8b6969', kind=c_int32_t)
    integer(c_int32_t) :: IndianRed1 = int(Z'ff6a6a', kind=c_int32_t)
    integer(c_int32_t) :: IndianRed2 = int(Z'ee6363', kind=c_int32_t)
    integer(c_int32_t) :: IndianRed3 = int(Z'cd5555', kind=c_int32_t)
    integer(c_int32_t) :: IndianRed4 = int(Z'8b3a3a', kind=c_int32_t)
    integer(c_int32_t) :: Sienna1 = int(Z'ff8247', kind=c_int32_t)
    integer(c_int32_t) :: Sienna2 = int(Z'ee7942', kind=c_int32_t)
    integer(c_int32_t) :: Sienna3 = int(Z'cd6839', kind=c_int32_t)
    integer(c_int32_t) :: Sienna4 = int(Z'8b4726', kind=c_int32_t)
    integer(c_int32_t) :: Burlywood1 = int(Z'ffd39b', kind=c_int32_t)
    integer(c_int32_t) :: Burlywood2 = int(Z'eec591', kind=c_int32_t)
    integer(c_int32_t) :: Burlywood3 = int(Z'cdaa7d', kind=c_int32_t)
    integer(c_int32_t) :: Burlywood4 = int(Z'8b7355', kind=c_int32_t)
    integer(c_int32_t) :: Wheat1 = int(Z'ffe7ba', kind=c_int32_t)
    integer(c_int32_t) :: Wheat2 = int(Z'eed8ae', kind=c_int32_t)
    integer(c_int32_t) :: Wheat3 = int(Z'cdba96', kind=c_int32_t)
    integer(c_int32_t) :: Wheat4 = int(Z'8b7e66', kind=c_int32_t)
    integer(c_int32_t) :: Tan1 = int(Z'ffa54f', kind=c_int32_t)
    integer(c_int32_t) :: Tan2 = int(Z'ee9a49', kind=c_int32_t)
    integer(c_int32_t) :: Tan3 = int(Z'cd853f', kind=c_int32_t)
    integer(c_int32_t) :: Tan4 = int(Z'8b5a2b', kind=c_int32_t)
    integer(c_int32_t) :: Chocolate1 = int(Z'ff7f24', kind=c_int32_t)
    integer(c_int32_t) :: Chocolate2 = int(Z'ee7621', kind=c_int32_t)
    integer(c_int32_t) :: Chocolate3 = int(Z'cd661d', kind=c_int32_t)
    integer(c_int32_t) :: Chocolate4 = int(Z'8b4513', kind=c_int32_t)
    integer(c_int32_t) :: Firebrick1 = int(Z'ff3030', kind=c_int32_t)
    integer(c_int32_t) :: Firebrick2 = int(Z'ee2c2c', kind=c_int32_t)
    integer(c_int32_t) :: Firebrick3 = int(Z'cd2626', kind=c_int32_t)
    integer(c_int32_t) :: Firebrick4 = int(Z'8b1a1a', kind=c_int32_t)
    integer(c_int32_t) :: Brown1 = int(Z'ff4040', kind=c_int32_t)
    integer(c_int32_t) :: Brown2 = int(Z'ee3b3b', kind=c_int32_t)
    integer(c_int32_t) :: Brown3 = int(Z'cd3333', kind=c_int32_t)
    integer(c_int32_t) :: Brown4 = int(Z'8b2323', kind=c_int32_t)
    integer(c_int32_t) :: Salmon1 = int(Z'ff8c69', kind=c_int32_t)
    integer(c_int32_t) :: Salmon2 = int(Z'ee8262', kind=c_int32_t)
    integer(c_int32_t) :: Salmon3 = int(Z'cd7054', kind=c_int32_t)
    integer(c_int32_t) :: Salmon4 = int(Z'8b4c39', kind=c_int32_t)
    integer(c_int32_t) :: LightSalmon1 = int(Z'ffa07a', kind=c_int32_t)
    integer(c_int32_t) :: LightSalmon2 = int(Z'ee9572', kind=c_int32_t)
    integer(c_int32_t) :: LightSalmon3 = int(Z'cd8162', kind=c_int32_t)
    integer(c_int32_t) :: LightSalmon4 = int(Z'8b5742', kind=c_int32_t)
    integer(c_int32_t) :: Orange1 = int(Z'ffa500', kind=c_int32_t)
    integer(c_int32_t) :: Orange2 = int(Z'ee9a00', kind=c_int32_t)
    integer(c_int32_t) :: Orange3 = int(Z'cd8500', kind=c_int32_t)
    integer(c_int32_t) :: Orange4 = int(Z'8b5a00', kind=c_int32_t)
    integer(c_int32_t) :: DarkOrange1 = int(Z'ff7f00', kind=c_int32_t)
    integer(c_int32_t) :: DarkOrange2 = int(Z'ee7600', kind=c_int32_t)
    integer(c_int32_t) :: DarkOrange3 = int(Z'cd6600', kind=c_int32_t)
    integer(c_int32_t) :: DarkOrange4 = int(Z'8b4500', kind=c_int32_t)
    integer(c_int32_t) :: Coral1 = int(Z'ff7256', kind=c_int32_t)
    integer(c_int32_t) :: Coral2 = int(Z'ee6a50', kind=c_int32_t)
    integer(c_int32_t) :: Coral3 = int(Z'cd5b45', kind=c_int32_t)
    integer(c_int32_t) :: Coral4 = int(Z'8b3e2f', kind=c_int32_t)
    integer(c_int32_t) :: Tomato1 = int(Z'ff6347', kind=c_int32_t)
    integer(c_int32_t) :: Tomato2 = int(Z'ee5c42', kind=c_int32_t)
    integer(c_int32_t) :: Tomato3 = int(Z'cd4f39', kind=c_int32_t)
    integer(c_int32_t) :: Tomato4 = int(Z'8b3626', kind=c_int32_t)
    integer(c_int32_t) :: OrangeRed1 = int(Z'ff4500', kind=c_int32_t)
    integer(c_int32_t) :: OrangeRed2 = int(Z'ee4000', kind=c_int32_t)
    integer(c_int32_t) :: OrangeRed3 = int(Z'cd3700', kind=c_int32_t)
    integer(c_int32_t) :: OrangeRed4 = int(Z'8b2500', kind=c_int32_t)
    integer(c_int32_t) :: Red1 = int(Z'ff0000', kind=c_int32_t)
    integer(c_int32_t) :: Red2 = int(Z'ee0000', kind=c_int32_t)
    integer(c_int32_t) :: Red3 = int(Z'cd0000', kind=c_int32_t)
    integer(c_int32_t) :: Red4 = int(Z'8b0000', kind=c_int32_t)
    integer(c_int32_t) :: DeepPink1 = int(Z'ff1493', kind=c_int32_t)
    integer(c_int32_t) :: DeepPink2 = int(Z'ee1289', kind=c_int32_t)
    integer(c_int32_t) :: DeepPink3 = int(Z'cd1076', kind=c_int32_t)
    integer(c_int32_t) :: DeepPink4 = int(Z'8b0a50', kind=c_int32_t)
    integer(c_int32_t) :: HotPink1 = int(Z'ff6eb4', kind=c_int32_t)
    integer(c_int32_t) :: HotPink2 = int(Z'ee6aa7', kind=c_int32_t)
    integer(c_int32_t) :: HotPink3 = int(Z'cd6090', kind=c_int32_t)
    integer(c_int32_t) :: HotPink4 = int(Z'8b3a62', kind=c_int32_t)
    integer(c_int32_t) :: Pink1 = int(Z'ffb5c5', kind=c_int32_t)
    integer(c_int32_t) :: Pink2 = int(Z'eea9b8', kind=c_int32_t)
    integer(c_int32_t) :: Pink3 = int(Z'cd919e', kind=c_int32_t)
    integer(c_int32_t) :: Pink4 = int(Z'8b636c', kind=c_int32_t)
    integer(c_int32_t) :: LightPink1 = int(Z'ffaeb9', kind=c_int32_t)
    integer(c_int32_t) :: LightPink2 = int(Z'eea2ad', kind=c_int32_t)
    integer(c_int32_t) :: LightPink3 = int(Z'cd8c95', kind=c_int32_t)
    integer(c_int32_t) :: LightPink4 = int(Z'8b5f65', kind=c_int32_t)
    integer(c_int32_t) :: PaleVioletRed1 = int(Z'ff82ab', kind=c_int32_t)
    integer(c_int32_t) :: PaleVioletRed2 = int(Z'ee799f', kind=c_int32_t)
    integer(c_int32_t) :: PaleVioletRed3 = int(Z'cd6889', kind=c_int32_t)
    integer(c_int32_t) :: PaleVioletRed4 = int(Z'8b475d', kind=c_int32_t)
    integer(c_int32_t) :: Maroon1 = int(Z'ff34b3', kind=c_int32_t)
    integer(c_int32_t) :: Maroon2 = int(Z'ee30a7', kind=c_int32_t)
    integer(c_int32_t) :: Maroon3 = int(Z'cd2990', kind=c_int32_t)
    integer(c_int32_t) :: Maroon4 = int(Z'8b1c62', kind=c_int32_t)
    integer(c_int32_t) :: VioletRed1 = int(Z'ff3e96', kind=c_int32_t)
    integer(c_int32_t) :: VioletRed2 = int(Z'ee3a8c', kind=c_int32_t)
    integer(c_int32_t) :: VioletRed3 = int(Z'cd3278', kind=c_int32_t)
    integer(c_int32_t) :: VioletRed4 = int(Z'8b2252', kind=c_int32_t)
    integer(c_int32_t) :: Magenta1 = int(Z'ff00ff', kind=c_int32_t)
    integer(c_int32_t) :: Magenta2 = int(Z'ee00ee', kind=c_int32_t)
    integer(c_int32_t) :: Magenta3 = int(Z'cd00cd', kind=c_int32_t)
    integer(c_int32_t) :: Magenta4 = int(Z'8b008b', kind=c_int32_t)
    integer(c_int32_t) :: Orchid1 = int(Z'ff83fa', kind=c_int32_t)
    integer(c_int32_t) :: Orchid2 = int(Z'ee7ae9', kind=c_int32_t)
    integer(c_int32_t) :: Orchid3 = int(Z'cd69c9', kind=c_int32_t)
    integer(c_int32_t) :: Orchid4 = int(Z'8b4789', kind=c_int32_t)
    integer(c_int32_t) :: Plum1 = int(Z'ffbbff', kind=c_int32_t)
    integer(c_int32_t) :: Plum2 = int(Z'eeaeee', kind=c_int32_t)
    integer(c_int32_t) :: Plum3 = int(Z'cd96cd', kind=c_int32_t)
    integer(c_int32_t) :: Plum4 = int(Z'8b668b', kind=c_int32_t)
    integer(c_int32_t) :: MediumOrchid1 = int(Z'e066ff', kind=c_int32_t)
    integer(c_int32_t) :: MediumOrchid2 = int(Z'd15fee', kind=c_int32_t)
    integer(c_int32_t) :: MediumOrchid3 = int(Z'b452cd', kind=c_int32_t)
    integer(c_int32_t) :: MediumOrchid4 = int(Z'7a378b', kind=c_int32_t)
    integer(c_int32_t) :: DarkOrchid1 = int(Z'bf3eff', kind=c_int32_t)
    integer(c_int32_t) :: DarkOrchid2 = int(Z'b23aee', kind=c_int32_t)
    integer(c_int32_t) :: DarkOrchid3 = int(Z'9a32cd', kind=c_int32_t)
    integer(c_int32_t) :: DarkOrchid4 = int(Z'68228b', kind=c_int32_t)
    integer(c_int32_t) :: Purple1 = int(Z'9b30ff', kind=c_int32_t)
    integer(c_int32_t) :: Purple2 = int(Z'912cee', kind=c_int32_t)
    integer(c_int32_t) :: Purple3 = int(Z'7d26cd', kind=c_int32_t)
    integer(c_int32_t) :: Purple4 = int(Z'551a8b', kind=c_int32_t)
    integer(c_int32_t) :: MediumPurple1 = int(Z'ab82ff', kind=c_int32_t)
    integer(c_int32_t) :: MediumPurple2 = int(Z'9f79ee', kind=c_int32_t)
    integer(c_int32_t) :: MediumPurple3 = int(Z'8968cd', kind=c_int32_t)
    integer(c_int32_t) :: MediumPurple4 = int(Z'5d478b', kind=c_int32_t)
    integer(c_int32_t) :: Thistle1 = int(Z'ffe1ff', kind=c_int32_t)
    integer(c_int32_t) :: Thistle2 = int(Z'eed2ee', kind=c_int32_t)
    integer(c_int32_t) :: Thistle3 = int(Z'cdb5cd', kind=c_int32_t)
    integer(c_int32_t) :: Thistle4 = int(Z'8b7b8b', kind=c_int32_t)
    integer(c_int32_t) :: Gray0 = int(Z'000000', kind=c_int32_t)
    integer(c_int32_t) :: Grey0 = int(Z'000000', kind=c_int32_t)
    integer(c_int32_t) :: Gray1 = int(Z'030303', kind=c_int32_t)
    integer(c_int32_t) :: Grey1 = int(Z'030303', kind=c_int32_t)
    integer(c_int32_t) :: Gray2 = int(Z'050505', kind=c_int32_t)
    integer(c_int32_t) :: Grey2 = int(Z'050505', kind=c_int32_t)
    integer(c_int32_t) :: Gray3 = int(Z'080808', kind=c_int32_t)
    integer(c_int32_t) :: Grey3 = int(Z'080808', kind=c_int32_t)
    integer(c_int32_t) :: Gray4 = int(Z'0a0a0a', kind=c_int32_t)
    integer(c_int32_t) :: Grey4 = int(Z'0a0a0a', kind=c_int32_t)
    integer(c_int32_t) :: Gray5 = int(Z'0d0d0d', kind=c_int32_t)
    integer(c_int32_t) :: Grey5 = int(Z'0d0d0d', kind=c_int32_t)
    integer(c_int32_t) :: Gray6 = int(Z'0f0f0f', kind=c_int32_t)
    integer(c_int32_t) :: Grey6 = int(Z'0f0f0f', kind=c_int32_t)
    integer(c_int32_t) :: Gray7 = int(Z'121212', kind=c_int32_t)
    integer(c_int32_t) :: Grey7 = int(Z'121212', kind=c_int32_t)
    integer(c_int32_t) :: Gray8 = int(Z'141414', kind=c_int32_t)
    integer(c_int32_t) :: Grey8 = int(Z'141414', kind=c_int32_t)
    integer(c_int32_t) :: Gray9 = int(Z'171717', kind=c_int32_t)
    integer(c_int32_t) :: Grey9 = int(Z'171717', kind=c_int32_t)
    integer(c_int32_t) :: Gray10 = int(Z'1a1a1a', kind=c_int32_t)
    integer(c_int32_t) :: Grey10 = int(Z'1a1a1a', kind=c_int32_t)
    integer(c_int32_t) :: Gray11 = int(Z'1c1c1c', kind=c_int32_t)
    integer(c_int32_t) :: Grey11 = int(Z'1c1c1c', kind=c_int32_t)
    integer(c_int32_t) :: Gray12 = int(Z'1f1f1f', kind=c_int32_t)
    integer(c_int32_t) :: Grey12 = int(Z'1f1f1f', kind=c_int32_t)
    integer(c_int32_t) :: Gray13 = int(Z'212121', kind=c_int32_t)
    integer(c_int32_t) :: Grey13 = int(Z'212121', kind=c_int32_t)
    integer(c_int32_t) :: Gray14 = int(Z'242424', kind=c_int32_t)
    integer(c_int32_t) :: Grey14 = int(Z'242424', kind=c_int32_t)
    integer(c_int32_t) :: Gray15 = int(Z'262626', kind=c_int32_t)
    integer(c_int32_t) :: Grey15 = int(Z'262626', kind=c_int32_t)
    integer(c_int32_t) :: Gray16 = int(Z'292929', kind=c_int32_t)
    integer(c_int32_t) :: Grey16 = int(Z'292929', kind=c_int32_t)
    integer(c_int32_t) :: Gray17 = int(Z'2b2b2b', kind=c_int32_t)
    integer(c_int32_t) :: Grey17 = int(Z'2b2b2b', kind=c_int32_t)
    integer(c_int32_t) :: Gray18 = int(Z'2e2e2e', kind=c_int32_t)
    integer(c_int32_t) :: Grey18 = int(Z'2e2e2e', kind=c_int32_t)
    integer(c_int32_t) :: Gray19 = int(Z'303030', kind=c_int32_t)
    integer(c_int32_t) :: Grey19 = int(Z'303030', kind=c_int32_t)
    integer(c_int32_t) :: Gray20 = int(Z'333333', kind=c_int32_t)
    integer(c_int32_t) :: Grey20 = int(Z'333333', kind=c_int32_t)
    integer(c_int32_t) :: Gray21 = int(Z'363636', kind=c_int32_t)
    integer(c_int32_t) :: Grey21 = int(Z'363636', kind=c_int32_t)
    integer(c_int32_t) :: Gray22 = int(Z'383838', kind=c_int32_t)
    integer(c_int32_t) :: Grey22 = int(Z'383838', kind=c_int32_t)
    integer(c_int32_t) :: Gray23 = int(Z'3b3b3b', kind=c_int32_t)
    integer(c_int32_t) :: Grey23 = int(Z'3b3b3b', kind=c_int32_t)
    integer(c_int32_t) :: Gray24 = int(Z'3d3d3d', kind=c_int32_t)
    integer(c_int32_t) :: Grey24 = int(Z'3d3d3d', kind=c_int32_t)
    integer(c_int32_t) :: Gray25 = int(Z'404040', kind=c_int32_t)
    integer(c_int32_t) :: Grey25 = int(Z'404040', kind=c_int32_t)
    integer(c_int32_t) :: Gray26 = int(Z'424242', kind=c_int32_t)
    integer(c_int32_t) :: Grey26 = int(Z'424242', kind=c_int32_t)
    integer(c_int32_t) :: Gray27 = int(Z'454545', kind=c_int32_t)
    integer(c_int32_t) :: Grey27 = int(Z'454545', kind=c_int32_t)
    integer(c_int32_t) :: Gray28 = int(Z'474747', kind=c_int32_t)
    integer(c_int32_t) :: Grey28 = int(Z'474747', kind=c_int32_t)
    integer(c_int32_t) :: Gray29 = int(Z'4a4a4a', kind=c_int32_t)
    integer(c_int32_t) :: Grey29 = int(Z'4a4a4a', kind=c_int32_t)
    integer(c_int32_t) :: Gray30 = int(Z'4d4d4d', kind=c_int32_t)
    integer(c_int32_t) :: Grey30 = int(Z'4d4d4d', kind=c_int32_t)
    integer(c_int32_t) :: Gray31 = int(Z'4f4f4f', kind=c_int32_t)
    integer(c_int32_t) :: Grey31 = int(Z'4f4f4f', kind=c_int32_t)
    integer(c_int32_t) :: Gray32 = int(Z'525252', kind=c_int32_t)
    integer(c_int32_t) :: Grey32 = int(Z'525252', kind=c_int32_t)
    integer(c_int32_t) :: Gray33 = int(Z'545454', kind=c_int32_t)
    integer(c_int32_t) :: Grey33 = int(Z'545454', kind=c_int32_t)
    integer(c_int32_t) :: Gray34 = int(Z'575757', kind=c_int32_t)
    integer(c_int32_t) :: Grey34 = int(Z'575757', kind=c_int32_t)
    integer(c_int32_t) :: Gray35 = int(Z'595959', kind=c_int32_t)
    integer(c_int32_t) :: Grey35 = int(Z'595959', kind=c_int32_t)
    integer(c_int32_t) :: Gray36 = int(Z'5c5c5c', kind=c_int32_t)
    integer(c_int32_t) :: Grey36 = int(Z'5c5c5c', kind=c_int32_t)
    integer(c_int32_t) :: Gray37 = int(Z'5e5e5e', kind=c_int32_t)
    integer(c_int32_t) :: Grey37 = int(Z'5e5e5e', kind=c_int32_t)
    integer(c_int32_t) :: Gray38 = int(Z'616161', kind=c_int32_t)
    integer(c_int32_t) :: Grey38 = int(Z'616161', kind=c_int32_t)
    integer(c_int32_t) :: Gray39 = int(Z'636363', kind=c_int32_t)
    integer(c_int32_t) :: Grey39 = int(Z'636363', kind=c_int32_t)
    integer(c_int32_t) :: Gray40 = int(Z'666666', kind=c_int32_t)
    integer(c_int32_t) :: Grey40 = int(Z'666666', kind=c_int32_t)
    integer(c_int32_t) :: Gray41 = int(Z'696969', kind=c_int32_t)
    integer(c_int32_t) :: Grey41 = int(Z'696969', kind=c_int32_t)
    integer(c_int32_t) :: Gray42 = int(Z'6b6b6b', kind=c_int32_t)
    integer(c_int32_t) :: Grey42 = int(Z'6b6b6b', kind=c_int32_t)
    integer(c_int32_t) :: Gray43 = int(Z'6e6e6e', kind=c_int32_t)
    integer(c_int32_t) :: Grey43 = int(Z'6e6e6e', kind=c_int32_t)
    integer(c_int32_t) :: Gray44 = int(Z'707070', kind=c_int32_t)
    integer(c_int32_t) :: Grey44 = int(Z'707070', kind=c_int32_t)
    integer(c_int32_t) :: Gray45 = int(Z'737373', kind=c_int32_t)
    integer(c_int32_t) :: Grey45 = int(Z'737373', kind=c_int32_t)
    integer(c_int32_t) :: Gray46 = int(Z'757575', kind=c_int32_t)
    integer(c_int32_t) :: Grey46 = int(Z'757575', kind=c_int32_t)
    integer(c_int32_t) :: Gray47 = int(Z'787878', kind=c_int32_t)
    integer(c_int32_t) :: Grey47 = int(Z'787878', kind=c_int32_t)
    integer(c_int32_t) :: Gray48 = int(Z'7a7a7a', kind=c_int32_t)
    integer(c_int32_t) :: Grey48 = int(Z'7a7a7a', kind=c_int32_t)
    integer(c_int32_t) :: Gray49 = int(Z'7d7d7d', kind=c_int32_t)
    integer(c_int32_t) :: Grey49 = int(Z'7d7d7d', kind=c_int32_t)
    integer(c_int32_t) :: Gray50 = int(Z'7f7f7f', kind=c_int32_t)
    integer(c_int32_t) :: Grey50 = int(Z'7f7f7f', kind=c_int32_t)
    integer(c_int32_t) :: Gray51 = int(Z'828282', kind=c_int32_t)
    integer(c_int32_t) :: Grey51 = int(Z'828282', kind=c_int32_t)
    integer(c_int32_t) :: Gray52 = int(Z'858585', kind=c_int32_t)
    integer(c_int32_t) :: Grey52 = int(Z'858585', kind=c_int32_t)
    integer(c_int32_t) :: Gray53 = int(Z'878787', kind=c_int32_t)
    integer(c_int32_t) :: Grey53 = int(Z'878787', kind=c_int32_t)
    integer(c_int32_t) :: Gray54 = int(Z'8a8a8a', kind=c_int32_t)
    integer(c_int32_t) :: Grey54 = int(Z'8a8a8a', kind=c_int32_t)
    integer(c_int32_t) :: Gray55 = int(Z'8c8c8c', kind=c_int32_t)
    integer(c_int32_t) :: Grey55 = int(Z'8c8c8c', kind=c_int32_t)
    integer(c_int32_t) :: Gray56 = int(Z'8f8f8f', kind=c_int32_t)
    integer(c_int32_t) :: Grey56 = int(Z'8f8f8f', kind=c_int32_t)
    integer(c_int32_t) :: Gray57 = int(Z'919191', kind=c_int32_t)
    integer(c_int32_t) :: Grey57 = int(Z'919191', kind=c_int32_t)
    integer(c_int32_t) :: Gray58 = int(Z'949494', kind=c_int32_t)
    integer(c_int32_t) :: Grey58 = int(Z'949494', kind=c_int32_t)
    integer(c_int32_t) :: Gray59 = int(Z'969696', kind=c_int32_t)
    integer(c_int32_t) :: Grey59 = int(Z'969696', kind=c_int32_t)
    integer(c_int32_t) :: Gray60 = int(Z'999999', kind=c_int32_t)
    integer(c_int32_t) :: Grey60 = int(Z'999999', kind=c_int32_t)
    integer(c_int32_t) :: Gray61 = int(Z'9c9c9c', kind=c_int32_t)
    integer(c_int32_t) :: Grey61 = int(Z'9c9c9c', kind=c_int32_t)
    integer(c_int32_t) :: Gray62 = int(Z'9e9e9e', kind=c_int32_t)
    integer(c_int32_t) :: Grey62 = int(Z'9e9e9e', kind=c_int32_t)
    integer(c_int32_t) :: Gray63 = int(Z'a1a1a1', kind=c_int32_t)
    integer(c_int32_t) :: Grey63 = int(Z'a1a1a1', kind=c_int32_t)
    integer(c_int32_t) :: Gray64 = int(Z'a3a3a3', kind=c_int32_t)
    integer(c_int32_t) :: Grey64 = int(Z'a3a3a3', kind=c_int32_t)
    integer(c_int32_t) :: Gray65 = int(Z'a6a6a6', kind=c_int32_t)
    integer(c_int32_t) :: Grey65 = int(Z'a6a6a6', kind=c_int32_t)
    integer(c_int32_t) :: Gray66 = int(Z'a8a8a8', kind=c_int32_t)
    integer(c_int32_t) :: Grey66 = int(Z'a8a8a8', kind=c_int32_t)
    integer(c_int32_t) :: Gray67 = int(Z'ababab', kind=c_int32_t)
    integer(c_int32_t) :: Grey67 = int(Z'ababab', kind=c_int32_t)
    integer(c_int32_t) :: Gray68 = int(Z'adadad', kind=c_int32_t)
    integer(c_int32_t) :: Grey68 = int(Z'adadad', kind=c_int32_t)
    integer(c_int32_t) :: Gray69 = int(Z'b0b0b0', kind=c_int32_t)
    integer(c_int32_t) :: Grey69 = int(Z'b0b0b0', kind=c_int32_t)
    integer(c_int32_t) :: Gray70 = int(Z'b3b3b3', kind=c_int32_t)
    integer(c_int32_t) :: Grey70 = int(Z'b3b3b3', kind=c_int32_t)
    integer(c_int32_t) :: Gray71 = int(Z'b5b5b5', kind=c_int32_t)
    integer(c_int32_t) :: Grey71 = int(Z'b5b5b5', kind=c_int32_t)
    integer(c_int32_t) :: Gray72 = int(Z'b8b8b8', kind=c_int32_t)
    integer(c_int32_t) :: Grey72 = int(Z'b8b8b8', kind=c_int32_t)
    integer(c_int32_t) :: Gray73 = int(Z'bababa', kind=c_int32_t)
    integer(c_int32_t) :: Grey73 = int(Z'bababa', kind=c_int32_t)
    integer(c_int32_t) :: Gray74 = int(Z'bdbdbd', kind=c_int32_t)
    integer(c_int32_t) :: Grey74 = int(Z'bdbdbd', kind=c_int32_t)
    integer(c_int32_t) :: Gray75 = int(Z'bfbfbf', kind=c_int32_t)
    integer(c_int32_t) :: Grey75 = int(Z'bfbfbf', kind=c_int32_t)
    integer(c_int32_t) :: Gray76 = int(Z'c2c2c2', kind=c_int32_t)
    integer(c_int32_t) :: Grey76 = int(Z'c2c2c2', kind=c_int32_t)
    integer(c_int32_t) :: Gray77 = int(Z'c4c4c4', kind=c_int32_t)
    integer(c_int32_t) :: Grey77 = int(Z'c4c4c4', kind=c_int32_t)
    integer(c_int32_t) :: Gray78 = int(Z'c7c7c7', kind=c_int32_t)
    integer(c_int32_t) :: Grey78 = int(Z'c7c7c7', kind=c_int32_t)
    integer(c_int32_t) :: Gray79 = int(Z'c9c9c9', kind=c_int32_t)
    integer(c_int32_t) :: Grey79 = int(Z'c9c9c9', kind=c_int32_t)
    integer(c_int32_t) :: Gray80 = int(Z'cccccc', kind=c_int32_t)
    integer(c_int32_t) :: Grey80 = int(Z'cccccc', kind=c_int32_t)
    integer(c_int32_t) :: Gray81 = int(Z'cfcfcf', kind=c_int32_t)
    integer(c_int32_t) :: Grey81 = int(Z'cfcfcf', kind=c_int32_t)
    integer(c_int32_t) :: Gray82 = int(Z'd1d1d1', kind=c_int32_t)
    integer(c_int32_t) :: Grey82 = int(Z'd1d1d1', kind=c_int32_t)
    integer(c_int32_t) :: Gray83 = int(Z'd4d4d4', kind=c_int32_t)
    integer(c_int32_t) :: Grey83 = int(Z'd4d4d4', kind=c_int32_t)
    integer(c_int32_t) :: Gray84 = int(Z'd6d6d6', kind=c_int32_t)
    integer(c_int32_t) :: Grey84 = int(Z'd6d6d6', kind=c_int32_t)
    integer(c_int32_t) :: Gray85 = int(Z'd9d9d9', kind=c_int32_t)
    integer(c_int32_t) :: Grey85 = int(Z'd9d9d9', kind=c_int32_t)
    integer(c_int32_t) :: Gray86 = int(Z'dbdbdb', kind=c_int32_t)
    integer(c_int32_t) :: Grey86 = int(Z'dbdbdb', kind=c_int32_t)
    integer(c_int32_t) :: Gray87 = int(Z'dedede', kind=c_int32_t)
    integer(c_int32_t) :: Grey87 = int(Z'dedede', kind=c_int32_t)
    integer(c_int32_t) :: Gray88 = int(Z'e0e0e0', kind=c_int32_t)
    integer(c_int32_t) :: Grey88 = int(Z'e0e0e0', kind=c_int32_t)
    integer(c_int32_t) :: Gray89 = int(Z'e3e3e3', kind=c_int32_t)
    integer(c_int32_t) :: Grey89 = int(Z'e3e3e3', kind=c_int32_t)
    integer(c_int32_t) :: Gray90 = int(Z'e5e5e5', kind=c_int32_t)
    integer(c_int32_t) :: Grey90 = int(Z'e5e5e5', kind=c_int32_t)
    integer(c_int32_t) :: Gray91 = int(Z'e8e8e8', kind=c_int32_t)
    integer(c_int32_t) :: Grey91 = int(Z'e8e8e8', kind=c_int32_t)
    integer(c_int32_t) :: Gray92 = int(Z'ebebeb', kind=c_int32_t)
    integer(c_int32_t) :: Grey92 = int(Z'ebebeb', kind=c_int32_t)
    integer(c_int32_t) :: Gray93 = int(Z'ededed', kind=c_int32_t)
    integer(c_int32_t) :: Grey93 = int(Z'ededed', kind=c_int32_t)
    integer(c_int32_t) :: Gray94 = int(Z'f0f0f0', kind=c_int32_t)
    integer(c_int32_t) :: Grey94 = int(Z'f0f0f0', kind=c_int32_t)
    integer(c_int32_t) :: Gray95 = int(Z'f2f2f2', kind=c_int32_t)
    integer(c_int32_t) :: Grey95 = int(Z'f2f2f2', kind=c_int32_t)
    integer(c_int32_t) :: Gray96 = int(Z'f5f5f5', kind=c_int32_t)
    integer(c_int32_t) :: Grey96 = int(Z'f5f5f5', kind=c_int32_t)
    integer(c_int32_t) :: Gray97 = int(Z'f7f7f7', kind=c_int32_t)
    integer(c_int32_t) :: Grey97 = int(Z'f7f7f7', kind=c_int32_t)
    integer(c_int32_t) :: Gray98 = int(Z'fafafa', kind=c_int32_t)
    integer(c_int32_t) :: Grey98 = int(Z'fafafa', kind=c_int32_t)
    integer(c_int32_t) :: Gray99 = int(Z'fcfcfc', kind=c_int32_t)
    integer(c_int32_t) :: Grey99 = int(Z'fcfcfc', kind=c_int32_t)
    integer(c_int32_t) :: Gray100 = int(Z'ffffff', kind=c_int32_t)
    integer(c_int32_t) :: Grey100 = int(Z'ffffff', kind=c_int32_t)
    integer(c_int32_t) :: DarkGrey = int(Z'a9a9a9', kind=c_int32_t)
    integer(c_int32_t) :: DarkGray = int(Z'a9a9a9', kind=c_int32_t)
    integer(c_int32_t) :: DarkBlue = int(Z'00008b', kind=c_int32_t)
    integer(c_int32_t) :: DarkCyan = int(Z'008b8b', kind=c_int32_t)
    integer(c_int32_t) :: DarkMagenta = int(Z'8b008b', kind=c_int32_t)
    integer(c_int32_t) :: DarkRed = int(Z'8b0000', kind=c_int32_t)
    integer(c_int32_t) :: LightGreen = int(Z'90ee90', kind=c_int32_t)
    integer(c_int32_t) :: Crimson = int(Z'dc143c', kind=c_int32_t)
    integer(c_int32_t) :: Indigo = int(Z'4b0082', kind=c_int32_t)
    integer(c_int32_t) :: Olive = int(Z'808000', kind=c_int32_t)
    integer(c_int32_t) :: RebeccaPurple = int(Z'663399', kind=c_int32_t)
    integer(c_int32_t) :: Silver = int(Z'c0c0c0', kind=c_int32_t)
    integer(c_int32_t) :: Teal = int(Z'008080', kind=c_int32_t)
  end type

  interface
    subroutine impl_tracy_set_thread_name(name) bind(C, name="___tracy_set_thread_name")
      import
      type(c_ptr), intent(in), value :: name
    end subroutine impl_tracy_set_thread_name
  end interface

  type, bind(C) :: tracy_source_location_data
    type(c_ptr) :: name
    type(c_ptr) :: function
    type(c_ptr) :: file
    integer(c_int32_t) :: line
    integer(c_int32_t) :: color
  end type

  type, bind(C) :: tracy_zone_context
    integer(c_int32_t) :: id
    integer(c_int32_t) :: active
  end type

  type, bind(C) :: tracy_gpu_time_data
    integer(c_int64_t) :: gpuTime
    integer(c_int16_t) :: queryId
    integer(c_int8_t) :: context
  end type

  type, bind(C) :: tracy_gpu_zone_begin_data
    integer(c_int64_t) :: srcloc
    integer(c_int16_t) :: queryId
    integer(c_int8_t) :: context
  end type

  type, bind(C) :: tracy_gpu_zone_begin_callstack_data
    integer(c_int64_t) :: srcloc
    integer(c_int32_t) :: depth
    integer(c_int16_t) :: queryId
    integer(c_int8_t) :: context
  end type

  type, bind(C) :: tracy_gpu_zone_end_data
    integer(c_int16_t) :: queryId
    integer(c_int8_t) :: context
  end type

  type, bind(C) :: tracy_gpu_new_context_data
    integer(c_int64_t) :: gpuTime
    real(c_float) :: period
    integer(c_int8_t) :: context
    integer(c_int8_t) :: flags
    integer(c_int8_t) :: type
  end type

  type, bind(C) :: tracy_gpu_context_name_data
    integer(c_int8_t) :: context
    type(c_ptr) :: name
    integer(c_int16_t) :: len
  end type

  type, bind(C) :: tracy_gpu_calibration_data
    integer(c_int64_t) :: gpuTime
    integer(c_int64_t) :: cpuDelta
    integer(c_int8_t) :: context
  end type

  type, bind(C) :: tracy_gpu_time_sync_data
    integer(c_int64_t) :: gpuTime
    integer(c_int8_t) :: context
  end type

  ! tracy_lockable_context_data and related stuff is missed since Fortran does not have support of mutexes

  interface
    subroutine tracy_startup_profiler() bind(C, name="___tracy_startup_profiler")
    end subroutine tracy_startup_profiler
    subroutine tracy_shutdown_profiler() bind(C, name="___tracy_shutdown_profiler")
    end subroutine tracy_shutdown_profiler
    function impl_tracy_profiler_started() bind(C, name="___tracy_profiler_started")
      import
      integer(c_int32_t) :: impl_tracy_profiler_started
    end function impl_tracy_profiler_started
  end interface

  interface
    function impl_tracy_alloc_srcloc(line, source, sourceSz, function_name, functionSz, color) &
      bind(C, name="___tracy_alloc_srcloc")
      import
      integer(c_int64_t) :: impl_tracy_alloc_srcloc
      integer(c_int32_t), intent(in), value :: line
      type(c_ptr), intent(in), value :: source
      integer(c_size_t), intent(in), value :: sourceSz
      type(c_ptr), intent(in), value :: function_name
      integer(c_size_t), intent(in), value :: functionSz
      integer(c_int32_t), intent(in), value :: color
    end function impl_tracy_alloc_srcloc
    function impl_tracy_alloc_srcloc_name(line, source, sourceSz, function_name, functionSz, zone_name, nameSz, color) &
      bind(C, name="___tracy_alloc_srcloc_name")
      import
      integer(c_int64_t) :: impl_tracy_alloc_srcloc_name
      integer(c_int32_t), intent(in), value :: line
      type(c_ptr), intent(in), value :: source
      integer(c_size_t), intent(in), value :: sourceSz
      type(c_ptr), intent(in), value :: function_name
      integer(c_size_t), intent(in), value :: functionSz
      type(c_ptr), intent(in), value :: zone_name
      integer(c_size_t), intent(in), value :: nameSz
      integer(c_int32_t), intent(in), value :: color
    end function impl_tracy_alloc_srcloc_name
  end interface

  interface
    type(tracy_zone_context) function impl_tracy_emit_zone_begin_callstack(srcloc, depth, active) &
      bind(C, name="___tracy_emit_zone_begin_callstack")
      import
      type(tracy_source_location_data), intent(in) :: srcloc
      integer(c_int32_t), intent(in), value :: depth
      integer(c_int32_t), intent(in), value :: active
    end function impl_tracy_emit_zone_begin_callstack
    type(tracy_zone_context) function impl_tracy_emit_zone_begin_alloc_callstack(srcloc, depth, active) &
      bind(C, name="___tracy_emit_zone_begin_alloc_callstack")
      import
      integer(c_int64_t), intent(in), value :: srcloc
      integer(c_int32_t), intent(in), value :: depth
      integer(c_int32_t), intent(in), value :: active
    end function impl_tracy_emit_zone_begin_alloc_callstack
  end interface
  interface tracy_zone_begin
    module procedure tracy_emit_zone_begin_id, tracy_emit_zone_begin_type
  end interface tracy_zone_begin

  interface
    subroutine tracy_zone_end(ctx) bind(C, name="___tracy_emit_zone_end")
      import
      type(tracy_zone_context), intent(in), value :: ctx
    end subroutine tracy_zone_end
  end interface

  interface
    subroutine tracy_emit_zone_text(ctx, txt, size) bind(C, name="___tracy_emit_zone_text")
      import
      type(tracy_zone_context), intent(in), value :: ctx
      type(c_ptr), intent(in), value :: txt
      integer(c_size_t), intent(in), value :: size
    end subroutine tracy_emit_zone_text
    subroutine tracy_emit_zone_name(ctx, txt, size) bind(C, name="___tracy_emit_zone_name")
      import
      type(tracy_zone_context), intent(in), value :: ctx
      type(c_ptr), intent(in), value :: txt
      integer(c_size_t), intent(in), value :: size
    end subroutine tracy_emit_zone_name
    subroutine tracy_emit_zone_color(ctx, color) bind(C, name="___tracy_emit_zone_color")
      import
      type(tracy_zone_context), intent(in), value :: ctx
      integer(c_int32_t), intent(in), value :: color
    end subroutine tracy_emit_zone_color
    subroutine tracy_emit_zone_value(ctx, value) bind(C, name="___tracy_emit_zone_value")
      import
      type(tracy_zone_context), intent(in), value :: ctx
      integer(c_int64_t), intent(in), value :: value
    end subroutine tracy_emit_zone_value
  end interface

  ! GPU is not supported yet

  interface
    function impl_tracy_connected() bind(C, name="___tracy_connected")
      import
      integer(c_int32_t) :: impl_tracy_connected
    end function impl_tracy_connected
  end interface

  interface
    subroutine impl_tracy_emit_memory_alloc_callstack(ptr, size, depth, secure) &
      bind(C, name="___tracy_emit_memory_alloc_callstack")
      import
      type(c_ptr), intent(in), value :: ptr
      integer(c_size_t), intent(in), value :: size
      integer(c_int32_t), intent(in), value :: depth
      integer(c_int32_t), intent(in), value :: secure
    end subroutine impl_tracy_emit_memory_alloc_callstack
    subroutine impl_tracy_emit_memory_alloc_callstack_named(ptr, size, depth, secure, name) &
      bind(C, name="___tracy_emit_memory_alloc_callstack_named")
      import
      type(c_ptr), intent(in), value :: ptr
      integer(c_size_t), intent(in), value :: size
      integer(c_int32_t), intent(in), value :: depth
      integer(c_int32_t), intent(in), value :: secure
      type(c_ptr), intent(in), value :: name
    end subroutine impl_tracy_emit_memory_alloc_callstack_named
    subroutine impl_tracy_emit_memory_free_callstack(ptr, depth, secure) &
      bind(C, name="___tracy_emit_memory_free_callstack")
      import
      type(c_ptr), intent(in), value :: ptr
      integer(c_int32_t), intent(in), value :: depth
      integer(c_int32_t), intent(in), value :: secure
    end subroutine impl_tracy_emit_memory_free_callstack
    subroutine impl_tracy_emit_memory_free_callstack_named(ptr, depth, secure, name) &
      bind(C, name="___tracy_emit_memory_free_callstack_named")
      import
      type(c_ptr), intent(in), value :: ptr
      integer(c_int32_t), intent(in), value :: depth
      integer(c_int32_t), intent(in), value :: secure
      type(c_ptr), intent(in), value :: name
    end subroutine impl_tracy_emit_memory_free_callstack_named
    subroutine impl_tracy_emit_memory_discard_callstack(name, secure, depth) &
      bind(C, name="___tracy_emit_memory_discard_callstack")
      import
      type(c_ptr), intent(in), value :: name
      integer(c_int32_t), intent(in), value :: secure
      integer(c_int32_t), intent(in), value :: depth
    end subroutine impl_tracy_emit_memory_discard_callstack
  end interface

  interface
    subroutine impl_tracy_emit_message(txt, size, depth) &
      bind(C, name="___tracy_emit_message")
      import
      type(c_ptr), intent(in), value :: txt
      integer(c_size_t), value :: size
      integer(c_int32_t), value :: depth
    end subroutine impl_tracy_emit_message
    subroutine impl_tracy_emit_messageC(txt, size, color, depth) &
      bind(C, name="___tracy_emit_messageC")
      import
      type(c_ptr), intent(in), value :: txt
      integer(c_size_t), value :: size
      integer(c_int32_t), value :: color
      integer(c_int32_t), value :: depth
    end subroutine impl_tracy_emit_messageC
    subroutine impl_tracy_emit_message_appinfo(txt, size) &
      bind(C, name="___tracy_emit_message_appinfo")
      import
      type(c_ptr), intent(in), value :: txt
      integer(c_size_t), value :: size
    end subroutine impl_tracy_emit_message_appinfo
  end interface

  interface
    subroutine impl_tracy_emit_frame_mark(name) &
      bind(C, name="___tracy_emit_frame_mark")
      import
      type(c_ptr), intent(in), value :: name
    end subroutine impl_tracy_emit_frame_mark
    subroutine impl_tracy_emit_frame_mark_start(name) &
      bind(C, name="___tracy_emit_frame_mark_start")
      import
      type(c_ptr), intent(in), value :: name
    end subroutine impl_tracy_emit_frame_mark_start
    subroutine impl_tracy_emit_frame_mark_end(name) &
      bind(C, name="___tracy_emit_frame_mark_end")
      import
      type(c_ptr), intent(in), value :: name
    end subroutine impl_tracy_emit_frame_mark_end
  end interface

  interface
    subroutine impl_tracy_emit_frame_image(image, w, h, offset, flip) &
      bind(C, name="___tracy_emit_frame_image")
      import
      type(c_ptr), intent(in), value :: image
      integer(c_int16_t), intent(in), value :: w
      integer(c_int16_t), intent(in), value :: h
      integer(c_int8_t), intent(in), value :: offset
      integer(c_int32_t), intent(in), value :: flip
    end subroutine impl_tracy_emit_frame_image
  end interface

  interface
    subroutine impl_tracy_emit_plot_int8(name, val) &
      bind(C, name="___tracy_emit_plot_int")
      import
      type(c_ptr), intent(in), value :: name
      integer(c_int64_t), value :: val
    end subroutine impl_tracy_emit_plot_int8
    subroutine impl_tracy_emit_plot_real4(name, val) &
      bind(C, name="___tracy_emit_plot_float")
      import
      type(c_ptr), intent(in), value :: name
      real(c_float), value :: val
    end subroutine impl_tracy_emit_plot_real4
    subroutine impl_tracy_emit_plot_real8(name, val) &
      bind(C, name="___tracy_emit_plot")
      import
      type(c_ptr), intent(in), value :: name
      real(c_double), value :: val
    end subroutine impl_tracy_emit_plot_real8
  end interface
  interface tracy_plot
    module procedure tracy_plot_int8, tracy_plot_real4, tracy_plot_real8
  end interface tracy_plot
  interface
    subroutine impl_tracy_emit_plot_config(name, type, step, fill, color) &
      bind(C, name="___tracy_emit_plot_config")
      import
      type(c_ptr), intent(in), value :: name
      integer(c_int32_t), intent(in), value :: type
      integer(c_int32_t), intent(in), value :: step
      integer(c_int32_t), intent(in), value :: fill
      integer(c_int32_t), intent(in), value :: color
    end subroutine impl_tracy_emit_plot_config
  end interface

#ifdef TRACY_FIBERS
  interface
    subroutine impl_tracy_fiber_enter(fiber_name) &
      bind(C, name="___tracy_fiber_enter")
      import
      type(c_ptr), intent(in), value :: fiber_name
    end subroutine impl_tracy_fiber_enter
    subroutine tracy_fiber_leave() &
      bind(C, name="___tracy_fiber_leave")
    end subroutine tracy_fiber_leave
  end interface
#endif
  !
  public :: tracy_zone_context
  public :: tracy_source_location_data
  !
#ifndef __SUNPRO_F90
  type(TracyColors_t), public, parameter :: TracyColors = TracyColors_t()
#endif
  !
  public :: tracy_set_thread_name
  public :: tracy_startup_profiler, tracy_shutdown_profiler, tracy_profiler_started
  public :: tracy_connected
  public :: tracy_appinfo
  public :: tracy_alloc_srcloc
  public :: tracy_zone_begin, tracy_zone_end
  public :: tracy_zone_set_properties
  public :: tracy_frame_mark, tracy_frame_start, tracy_frame_end
  public :: tracy_memory_alloc, tracy_memory_free, tracy_memory_discard
  public :: tracy_message
  public :: tracy_image
  public :: tracy_plot_config, tracy_plot
#ifdef TRACY_FIBERS
  public :: tracy_fiber_enter, tracy_fiber_leave
#endif
contains
  subroutine tracy_set_thread_name(name)
    character(kind=c_char, len=*), intent(in) :: name
    character(kind=c_char, len=:), allocatable, target :: alloc_name
    allocate (character(kind=c_char, len=len(name) + 1) :: alloc_name)
    alloc_name = name//c_null_char
    call impl_tracy_set_thread_name(c_loc(alloc_name))
  end subroutine tracy_set_thread_name

  logical(1) function tracy_profiler_started()
    tracy_profiler_started = impl_tracy_profiler_started() /= 0_c_int
  end function tracy_profiler_started

  integer(c_int64_t) function tracy_alloc_srcloc(line, source, function_name, zone_name, color)
    integer(c_int32_t), intent(in) :: line
    character(kind=c_char, len=*), target, intent(in) :: source, function_name
    character(kind=c_char, len=*), target, intent(in), optional :: zone_name
    integer(c_int32_t), intent(in), optional :: color
    !
    integer(c_int32_t) :: color_
    !
    color_ = 0_c_int32_t
    if (present(color)) color_ = color
    if (present(zone_name)) then
      tracy_alloc_srcloc = impl_tracy_alloc_srcloc_name(line, &
                                                        c_loc(source), len(source, kind=c_size_t), &
                                                        c_loc(function_name), len(function_name, kind=c_size_t), &
                                                        c_loc(zone_name), len(zone_name, kind=c_size_t), &
                                                        color_)
    else
      tracy_alloc_srcloc = impl_tracy_alloc_srcloc(line, &
                                                   c_loc(source), len(source, kind=c_size_t), &
                                                   c_loc(function_name), len(function_name, kind=c_size_t), &
                                                   color_)
    end if
  end function tracy_alloc_srcloc

  type(tracy_zone_context) function tracy_emit_zone_begin_id(srcloc, depth, active)
    integer(c_int64_t), intent(inout) :: srcloc
    integer(c_int32_t), intent(in), optional :: depth
    logical(1), intent(in), optional :: active
    !
    integer(c_int32_t) :: depth_
    integer(c_int32_t) :: active_
    active_ = 1_c_int32_t
    depth_ = 0_c_int32_t
    if (present(active)) then
      if (active) then
        active_ = 1_c_int32_t
      else
        active_ = 0_c_int32_t
      end if
    end if
    if (present(depth)) depth_ = depth
    tracy_emit_zone_begin_id = impl_tracy_emit_zone_begin_alloc_callstack(srcloc, depth_, active_)
    srcloc = 0_c_int64_t
  end function tracy_emit_zone_begin_id
  type(tracy_zone_context) function tracy_emit_zone_begin_type(srcloc, depth, active)
    type(tracy_source_location_data), intent(inout) :: srcloc
    integer(c_int32_t), intent(in), optional :: depth
    logical(1), intent(in), optional :: active
    !
    integer(c_int32_t) :: depth_
    integer(c_int32_t) :: active_
    active_ = 1_c_int32_t
    depth_ = 0_c_int32_t
    if (present(active)) then
      if (active) then
        active_ = 1_c_int32_t
      else
        active_ = 0_c_int32_t
      end if
    end if
    if (present(depth)) depth_ = depth
    tracy_emit_zone_begin_type = impl_tracy_emit_zone_begin_callstack(srcloc, depth_, active_)
    srcloc = tracy_source_location_data(c_null_ptr, c_null_ptr, c_null_ptr, 0_c_int32_t, 0_c_int32_t)
  end function tracy_emit_zone_begin_type

  subroutine tracy_zone_set_properties(ctx, text, name, color, value)
    type(tracy_zone_context), intent(in), value :: ctx
    character(kind=c_char, len=*), target, intent(in), optional :: text
    character(kind=c_char, len=*), target, intent(in), optional :: name
    integer(c_int32_t), target, intent(in), optional :: color
    integer(c_int64_t), target, intent(in), optional :: value
    if (present(text)) then
      call tracy_emit_zone_text(ctx, c_loc(text), len(text, kind=c_size_t))
    end if
    if (present(name)) then
      call tracy_emit_zone_name(ctx, c_loc(name), len(name, kind=c_size_t))
    end if
    if (present(color)) then
      call tracy_emit_zone_color(ctx, color)
    end if
    if (present(value)) then
      call tracy_emit_zone_value(ctx, value)
    end if
  end subroutine tracy_zone_set_properties

  logical(1) function tracy_connected()
    tracy_connected = impl_tracy_connected() /= 0_c_int32_t
  end function tracy_connected

  subroutine tracy_memory_alloc(ptr, size, name, depth, secure)
    type(c_ptr), intent(in) :: ptr
    integer(c_size_t), intent(in) :: size
    character(kind=c_char, len=*), target, intent(in), optional :: name
    integer(c_int32_t), intent(in), optional :: depth
    logical(1), intent(in), optional :: secure
    !
    integer(c_int32_t) :: depth_, secure_
    secure_ = 0_c_int32_t
    depth_ = 0_c_int32_t
    if (present(secure)) then
      if (secure) secure_ = 1_c_int32_t
    end if
    if (present(depth)) depth_ = depth
    if (present(name)) then
      call impl_tracy_emit_memory_alloc_callstack_named(ptr, size, depth_, secure_, c_loc(name))
    else
      call impl_tracy_emit_memory_alloc_callstack(ptr, size, depth_, secure_)
    end if
  end subroutine tracy_memory_alloc
  subroutine tracy_memory_free(ptr, name, depth, secure)
    type(c_ptr), intent(in) :: ptr
    character(kind=c_char, len=*), target, intent(in), optional :: name
    integer(c_int32_t), intent(in), optional :: depth
    logical(1), intent(in), optional :: secure
    !
    integer(c_int32_t) :: depth_, secure_
    secure_ = 0_c_int32_t
    depth_ = 0_c_int32_t
    if (present(secure)) then
      if (secure) secure_ = 1_c_int32_t
    end if
    if (present(depth)) depth_ = depth
    if (present(name)) then
      call impl_tracy_emit_memory_free_callstack_named(ptr, depth_, secure_, c_loc(name))
    else
      call impl_tracy_emit_memory_free_callstack(ptr, depth_, secure_)
    end if
  end subroutine tracy_memory_free
  subroutine tracy_memory_discard(name, depth, secure)
    character(kind=c_char, len=*), target, intent(in) :: name
    integer(c_int32_t), intent(in), optional :: depth
    logical(1), intent(in), optional :: secure
    !
    integer(c_int32_t) :: depth_, secure_
    secure_ = 0_c_int32_t
    depth_ = 0_c_int32_t
    if (present(secure)) then
      if (secure) secure_ = 1_c_int32_t
    end if
    if (present(depth)) depth_ = depth
    call impl_tracy_emit_memory_discard_callstack(c_loc(name), depth_, secure_)
  end subroutine tracy_memory_discard

  subroutine tracy_message(msg, color, depth)
    character(kind=c_char, len=*), target, intent(in) :: msg
    integer(c_int32_t), intent(in), optional :: color
    integer(c_int32_t), intent(in), optional :: depth
    !
    integer(c_int32_t) :: depth_
    depth_ = 0_c_int32_t
    if (present(depth)) depth_ = depth
    if (present(color)) then
      call impl_tracy_emit_messageC(c_loc(msg), len(msg, kind=c_size_t), color, depth_)
    else
      call impl_tracy_emit_message(c_loc(msg), len(msg, kind=c_size_t), depth_)
    end if
  end subroutine tracy_message

  subroutine tracy_appinfo(info)
    character(kind=c_char, len=*), target, intent(in) :: info
    call impl_tracy_emit_message_appinfo(c_loc(info), len(info, kind=c_size_t))
  end subroutine tracy_appinfo

  subroutine tracy_frame_mark(name)
    character(kind=c_char, len=*), target, intent(in), optional :: name
    if (present(name)) then
      call impl_tracy_emit_frame_mark(c_loc(name))
    else
      call impl_tracy_emit_frame_mark(c_null_ptr)
    end if
  end subroutine tracy_frame_mark
  subroutine tracy_frame_start(name)
    character(kind=c_char, len=*), target, intent(in), optional :: name
    if (present(name)) then
      call impl_tracy_emit_frame_mark_start(c_loc(name))
    else
      call impl_tracy_emit_frame_mark_start(c_null_ptr)
    end if
  end subroutine tracy_frame_start
  subroutine tracy_frame_end(name)
    character(kind=c_char, len=*), target, intent(in), optional :: name
    if (present(name)) then
      call impl_tracy_emit_frame_mark_end(c_loc(name))
    else
      call impl_tracy_emit_frame_mark_end(c_null_ptr)
    end if
  end subroutine tracy_frame_end

  subroutine tracy_image(image, w, h, offset, flip)
    type(c_ptr), intent(in) :: image
    integer(c_int16_t), intent(in) :: w, h
    integer(c_int8_t), intent(in), optional :: offset
    logical(1), intent(in), optional :: flip
    !
    integer(c_int32_t) :: flip_
    integer(c_int8_t) :: offset_
    flip_ = 0_c_int32_t
    offset_ = 0_c_int8_t
    if (present(flip)) then
      if (flip) flip_ = 1_c_int32_t
    end if
    if (present(offset)) offset_ = offset
    call impl_tracy_emit_frame_image(image, w, h, offset_, flip_)
  end subroutine tracy_image

  subroutine tracy_plot_int8(name, val)
    character(kind=c_char, len=*), target, intent(in) :: name
    integer(c_int64_t) :: val
    call impl_tracy_emit_plot_int8(c_loc(name), val)
  end subroutine tracy_plot_int8
  subroutine tracy_plot_real4(name, val)
    character(kind=c_char, len=*), target, intent(in) :: name
    real(c_float) :: val
    call impl_tracy_emit_plot_real4(c_loc(name), val)
  end subroutine tracy_plot_real4
  subroutine tracy_plot_real8(name, val)
    character(kind=c_char, len=*), target, intent(in) :: name
    real(c_double) :: val
    call impl_tracy_emit_plot_real8(c_loc(name), val)
  end subroutine tracy_plot_real8

  subroutine tracy_plot_config(name, type, step, fill, color)
    character(kind=c_char, len=*), target, intent(in) :: name
    integer(c_int32_t), intent(in), optional :: type
    logical(1), intent(in), optional :: step
    logical(1), intent(in), optional :: fill
    integer(c_int32_t), intent(in), optional :: color
    !
    integer(c_int32_t) :: type_, step_, fill_, color_
    type_ = 0_c_int32_t
    step_ = 0_c_int32_t
    fill_ = 1_c_int32_t
    color_ = 0_c_int32_t
    if (present(type)) type_ = type
    if (present(step)) then
      if (step) step_ = 1_c_int32_t
    end if
    if (present(fill)) then
      if (.not. fill) fill_ = 0_c_int32_t
    end if
    if (present(color)) color_ = color
    call impl_tracy_emit_plot_config(c_loc(name), type_, step_, fill_, color_)
  end subroutine tracy_plot_config

#ifdef TRACY_FIBERS
  subroutine tracy_fiber_enter(fiber_name)
    character(kind=c_char, len=*), target, intent(in) :: fiber_name
    call impl_tracy_fiber_enter(c_loc(fiber_name))
  end subroutine tracy_fiber_enter
#endif
end module tracy
