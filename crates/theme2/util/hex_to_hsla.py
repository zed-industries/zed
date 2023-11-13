import colorsys
import sys

def hex_to_rgb(hex):
    hex = hex.lstrip('#')
    return tuple(int(hex[i:i+2], 16) for i in (0, 2, 4))

def rgb_to_hsla(rgb):
    h, l, s = colorsys.rgb_to_hls(rgb[0]/255.0, rgb[1]/255.0, rgb[2]/255.0)
    return (round(h * 360, 1), round(s * 100, 1), round(l * 100, 1), 1.0)

def hex_to_hsla(hex):
    return rgb_to_hsla(hex_to_rgb(hex))

if len(sys.argv) != 2:
    print("Usage: python util/hex_to_hsla.py <hex>")
else:
    hex_color = sys.argv[1]
    h, s, l, a = hex_to_hsla(hex_color)
    print(f"hsla({h} / 360., {s} / 100., {l} / 100., {a})")
