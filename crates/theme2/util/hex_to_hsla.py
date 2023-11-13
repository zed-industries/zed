import colorsys
import sys

def hex_to_rgb(hex):
    hex = hex.lstrip('#')
    if len(hex) == 8: # 8 digit hex color
        r, g, b, a = (int(hex[i:i+2], 16) for i in (0, 2, 4, 6))
        return r, g, b, a / 255.0
    else: # 6 digit hex color
        return tuple(int(hex[i:i+2], 16) for i in (0, 2, 4)) + (1.0,)

def rgb_to_hsla(rgb):
    h, l, s = colorsys.rgb_to_hls(rgb[0]/255.0, rgb[1]/255.0, rgb[2]/255.0)
    a = rgb[3] # alpha value
    return (round(h * 360, 1), round(s * 100, 1), round(l * 100, 1), round(a, 3))

def hex_to_hsla(hex):
    return rgb_to_hsla(hex_to_rgb(hex))

if len(sys.argv) != 2:
    print("Usage: python util/hex_to_hsla.py <6 or 8 digit hex color or comma-separated list of colors>")
else:
    input_arg = sys.argv[1]
    if ',' in input_arg: # comma-separated list of colors
        hex_colors = input_arg.split(',')
        hslas = [] # output array
        for hex_color in hex_colors:
            hex_color = hex_color.strip("'\" ")
            h, s, l, a = hex_to_hsla(hex_color)
            hslas.append(f"hsla({h} / 360., {s} / 100., {l} / 100., {a})")
        print(hslas)
    else: # single color
        hex_color = input_arg.strip("'\"")
        h, s, l, a = hex_to_hsla(hex_color)
        print(f"hsla({h} / 360., {s} / 100., {l} / 100., {a})")
