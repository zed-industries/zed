//! A full color palette derived from the
//! [Material Design 2014 Color Palette](https://material.io/design/color/the-color-system.html).
//! Colors are chosen to go well with each other, and each color is available in several tints,
//! ranging from 50 (very light) to 900 (very dark). A tint of 500 is considered "standard". Color's whose tint starts
//! with an 'A' (for example [`RED_A400`]) are *accent* colors and are more saturated than their
//! standard counterparts.
//!
//! See the full list of colors defined in this module:
//!
//! <img src="https://plotters-rs.github.io/plotters-doc-data/full_palette.png"></img>
use super::RGBColor;

/*
Colors were auto-generated from the Material-UI color palette using the following
Javascript code. It can be run in a code sandbox here: https://codesandbox.io/s/q9nj9o6o44?file=/index.js

///////////////////////////////////////////////////////
import React from "react";
import { render } from "react-dom";
import * as c from "material-ui/colors";

function capitalize(name) {
  return name.charAt(0).toUpperCase() + name.slice(1);
}

function kebabize(str) {
  return str
    .split("")
    .map((letter, idx) => {
      return letter.toUpperCase() === letter
        ? `${idx !== 0 ? " " : ""}${letter.toLowerCase()}`
        : letter;
    })
    .join("");
}

function hexToRgb(hex) {
  var result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
  return result
    ? {
        r: parseInt(result[1], 16),
        g: parseInt(result[2], 16),
        b: parseInt(result[3], 16)
      }
    : null;
}

function ColorList() {
  const colorNames = Object.keys(c);

  return (
    <pre>
      {colorNames.map((name, i) => (
        <div key={i}>
          {"//"} {name}
          <div>
            {(() => {
              const rustName = name.toUpperCase();
              const cvalue = c[name][500];
              const color = hexToRgb(cvalue);
              if (color == null) {
                return "";
              }
              let docComment = `*${capitalize(kebabize(name))}*; same as [\`${rustName}_500\`]`;
              return `define_color!(${rustName}, ${color.r}, ${color.g}, ${color.b}, "${docComment}");`;
            })()}
          </div>
          {Object.entries(c[name]).map(([cname, cvalue]) => {
            const color = hexToRgb(cvalue);
            if (color == null) {
              return "";
            }
            const rustName = `${name.toUpperCase()}_${cname}`;
            const adjective =
              cname > 500
                ? cname >= 800
                  ? "Dark "
                  : "Darker "
                : cname < 500
                ? cname <= 100
                  ? "Light "
                  : "Lighter "
                : "";
            const readableName = kebabize(name);
            let docComment = `${adjective}*${
              adjective ? readableName : capitalize(readableName)
            }* with a tint of ${cname}`;
            if (cname.charAt(0) === "A") {
              docComment =
                "Accent *" +
                docComment.charAt(1).toLowerCase() +
                docComment.slice(2);
            }
            return (
              <div key={cname}>
                define_color!({rustName}, {color.r}, {color.g}, {color.b}, "
                {docComment}");
              </div>
            );
          })}
        </div>
      ))}
    </pre>
  );
}

render(<ColorList />, document.querySelector("#root"));
///////////////////////////////////////////////////////
*/

// common
define_color!(WHITE, 255, 255, 255, "*White*");
define_color!(BLACK, 0, 0, 0, "*Black*");
// red
define_color!(RED, 244, 67, 54, "*Red*; same as [`RED_500`]");
define_color!(RED_50, 255, 235, 238, "Light *red* with a tint of 50");
define_color!(RED_100, 255, 205, 210, "Light *red* with a tint of 100");
define_color!(RED_200, 239, 154, 154, "Lighter *red* with a tint of 200");
define_color!(RED_300, 229, 115, 115, "Lighter *red* with a tint of 300");
define_color!(RED_400, 239, 83, 80, "Lighter *red* with a tint of 400");
define_color!(RED_500, 244, 67, 54, "*Red* with a tint of 500");
define_color!(RED_600, 229, 57, 53, "Darker *red* with a tint of 600");
define_color!(RED_700, 211, 47, 47, "Darker *red* with a tint of 700");
define_color!(RED_800, 198, 40, 40, "Dark *red* with a tint of 800");
define_color!(RED_900, 183, 28, 28, "Dark *red* with a tint of 900");
define_color!(RED_A100, 255, 138, 128, "Accent *red* with a tint of A100");
define_color!(RED_A200, 255, 82, 82, "Accent *red* with a tint of A200");
define_color!(RED_A400, 255, 23, 68, "Accent *red* with a tint of A400");
define_color!(RED_A700, 213, 0, 0, "Accent *red* with a tint of A700");
// pink
define_color!(PINK, 233, 30, 99, "*Pink*; same as [`PINK_500`]");
define_color!(PINK_50, 252, 228, 236, "Light *pink* with a tint of 50");
define_color!(PINK_100, 248, 187, 208, "Light *pink* with a tint of 100");
define_color!(PINK_200, 244, 143, 177, "Lighter *pink* with a tint of 200");
define_color!(PINK_300, 240, 98, 146, "Lighter *pink* with a tint of 300");
define_color!(PINK_400, 236, 64, 122, "Lighter *pink* with a tint of 400");
define_color!(PINK_500, 233, 30, 99, "*Pink* with a tint of 500");
define_color!(PINK_600, 216, 27, 96, "Darker *pink* with a tint of 600");
define_color!(PINK_700, 194, 24, 91, "Darker *pink* with a tint of 700");
define_color!(PINK_800, 173, 20, 87, "Dark *pink* with a tint of 800");
define_color!(PINK_900, 136, 14, 79, "Dark *pink* with a tint of 900");
define_color!(
    PINK_A100,
    255,
    128,
    171,
    "Accent *pink* with a tint of A100"
);
define_color!(PINK_A200, 255, 64, 129, "Accent *pink* with a tint of A200");
define_color!(PINK_A400, 245, 0, 87, "Accent *pink* with a tint of A400");
define_color!(PINK_A700, 197, 17, 98, "Accent *pink* with a tint of A700");
// purple
define_color!(PURPLE, 156, 39, 176, "*Purple*; same as [`PURPLE_500`]");
define_color!(PURPLE_50, 243, 229, 245, "Light *purple* with a tint of 50");
define_color!(
    PURPLE_100,
    225,
    190,
    231,
    "Light *purple* with a tint of 100"
);
define_color!(
    PURPLE_200,
    206,
    147,
    216,
    "Lighter *purple* with a tint of 200"
);
define_color!(
    PURPLE_300,
    186,
    104,
    200,
    "Lighter *purple* with a tint of 300"
);
define_color!(
    PURPLE_400,
    171,
    71,
    188,
    "Lighter *purple* with a tint of 400"
);
define_color!(PURPLE_500, 156, 39, 176, "*Purple* with a tint of 500");
define_color!(
    PURPLE_600,
    142,
    36,
    170,
    "Darker *purple* with a tint of 600"
);
define_color!(
    PURPLE_700,
    123,
    31,
    162,
    "Darker *purple* with a tint of 700"
);
define_color!(PURPLE_800, 106, 27, 154, "Dark *purple* with a tint of 800");
define_color!(PURPLE_900, 74, 20, 140, "Dark *purple* with a tint of 900");
define_color!(
    PURPLE_A100,
    234,
    128,
    252,
    "Accent *purple* with a tint of A100"
);
define_color!(
    PURPLE_A200,
    224,
    64,
    251,
    "Accent *purple* with a tint of A200"
);
define_color!(
    PURPLE_A400,
    213,
    0,
    249,
    "Accent *purple* with a tint of A400"
);
define_color!(
    PURPLE_A700,
    170,
    0,
    255,
    "Accent *purple* with a tint of A700"
);
// deepPurple
define_color!(
    DEEPPURPLE,
    103,
    58,
    183,
    "*Deep purple*; same as [`DEEPPURPLE_500`]"
);
define_color!(
    DEEPPURPLE_50,
    237,
    231,
    246,
    "Light *deep purple* with a tint of 50"
);
define_color!(
    DEEPPURPLE_100,
    209,
    196,
    233,
    "Light *deep purple* with a tint of 100"
);
define_color!(
    DEEPPURPLE_200,
    179,
    157,
    219,
    "Lighter *deep purple* with a tint of 200"
);
define_color!(
    DEEPPURPLE_300,
    149,
    117,
    205,
    "Lighter *deep purple* with a tint of 300"
);
define_color!(
    DEEPPURPLE_400,
    126,
    87,
    194,
    "Lighter *deep purple* with a tint of 400"
);
define_color!(
    DEEPPURPLE_500,
    103,
    58,
    183,
    "*Deep purple* with a tint of 500"
);
define_color!(
    DEEPPURPLE_600,
    94,
    53,
    177,
    "Darker *deep purple* with a tint of 600"
);
define_color!(
    DEEPPURPLE_700,
    81,
    45,
    168,
    "Darker *deep purple* with a tint of 700"
);
define_color!(
    DEEPPURPLE_800,
    69,
    39,
    160,
    "Dark *deep purple* with a tint of 800"
);
define_color!(
    DEEPPURPLE_900,
    49,
    27,
    146,
    "Dark *deep purple* with a tint of 900"
);
define_color!(
    DEEPPURPLE_A100,
    179,
    136,
    255,
    "Accent *deep purple* with a tint of A100"
);
define_color!(
    DEEPPURPLE_A200,
    124,
    77,
    255,
    "Accent *deep purple* with a tint of A200"
);
define_color!(
    DEEPPURPLE_A400,
    101,
    31,
    255,
    "Accent *deep purple* with a tint of A400"
);
define_color!(
    DEEPPURPLE_A700,
    98,
    0,
    234,
    "Accent *deep purple* with a tint of A700"
);
// indigo
define_color!(INDIGO, 63, 81, 181, "*Indigo*; same as [`INDIGO_500`]");
define_color!(INDIGO_50, 232, 234, 246, "Light *indigo* with a tint of 50");
define_color!(
    INDIGO_100,
    197,
    202,
    233,
    "Light *indigo* with a tint of 100"
);
define_color!(
    INDIGO_200,
    159,
    168,
    218,
    "Lighter *indigo* with a tint of 200"
);
define_color!(
    INDIGO_300,
    121,
    134,
    203,
    "Lighter *indigo* with a tint of 300"
);
define_color!(
    INDIGO_400,
    92,
    107,
    192,
    "Lighter *indigo* with a tint of 400"
);
define_color!(INDIGO_500, 63, 81, 181, "*Indigo* with a tint of 500");
define_color!(
    INDIGO_600,
    57,
    73,
    171,
    "Darker *indigo* with a tint of 600"
);
define_color!(
    INDIGO_700,
    48,
    63,
    159,
    "Darker *indigo* with a tint of 700"
);
define_color!(INDIGO_800, 40, 53, 147, "Dark *indigo* with a tint of 800");
define_color!(INDIGO_900, 26, 35, 126, "Dark *indigo* with a tint of 900");
define_color!(
    INDIGO_A100,
    140,
    158,
    255,
    "Accent *indigo* with a tint of A100"
);
define_color!(
    INDIGO_A200,
    83,
    109,
    254,
    "Accent *indigo* with a tint of A200"
);
define_color!(
    INDIGO_A400,
    61,
    90,
    254,
    "Accent *indigo* with a tint of A400"
);
define_color!(
    INDIGO_A700,
    48,
    79,
    254,
    "Accent *indigo* with a tint of A700"
);
// blue
define_color!(BLUE, 33, 150, 243, "*Blue*; same as [`BLUE_500`]");
define_color!(BLUE_50, 227, 242, 253, "Light *blue* with a tint of 50");
define_color!(BLUE_100, 187, 222, 251, "Light *blue* with a tint of 100");
define_color!(BLUE_200, 144, 202, 249, "Lighter *blue* with a tint of 200");
define_color!(BLUE_300, 100, 181, 246, "Lighter *blue* with a tint of 300");
define_color!(BLUE_400, 66, 165, 245, "Lighter *blue* with a tint of 400");
define_color!(BLUE_500, 33, 150, 243, "*Blue* with a tint of 500");
define_color!(BLUE_600, 30, 136, 229, "Darker *blue* with a tint of 600");
define_color!(BLUE_700, 25, 118, 210, "Darker *blue* with a tint of 700");
define_color!(BLUE_800, 21, 101, 192, "Dark *blue* with a tint of 800");
define_color!(BLUE_900, 13, 71, 161, "Dark *blue* with a tint of 900");
define_color!(
    BLUE_A100,
    130,
    177,
    255,
    "Accent *blue* with a tint of A100"
);
define_color!(BLUE_A200, 68, 138, 255, "Accent *blue* with a tint of A200");
define_color!(BLUE_A400, 41, 121, 255, "Accent *blue* with a tint of A400");
define_color!(BLUE_A700, 41, 98, 255, "Accent *blue* with a tint of A700");
// lightBlue
define_color!(
    LIGHTBLUE,
    3,
    169,
    244,
    "*Light blue*; same as [`LIGHTBLUE_500`]"
);
define_color!(
    LIGHTBLUE_50,
    225,
    245,
    254,
    "Light *light blue* with a tint of 50"
);
define_color!(
    LIGHTBLUE_100,
    179,
    229,
    252,
    "Light *light blue* with a tint of 100"
);
define_color!(
    LIGHTBLUE_200,
    129,
    212,
    250,
    "Lighter *light blue* with a tint of 200"
);
define_color!(
    LIGHTBLUE_300,
    79,
    195,
    247,
    "Lighter *light blue* with a tint of 300"
);
define_color!(
    LIGHTBLUE_400,
    41,
    182,
    246,
    "Lighter *light blue* with a tint of 400"
);
define_color!(
    LIGHTBLUE_500,
    3,
    169,
    244,
    "*Light blue* with a tint of 500"
);
define_color!(
    LIGHTBLUE_600,
    3,
    155,
    229,
    "Darker *light blue* with a tint of 600"
);
define_color!(
    LIGHTBLUE_700,
    2,
    136,
    209,
    "Darker *light blue* with a tint of 700"
);
define_color!(
    LIGHTBLUE_800,
    2,
    119,
    189,
    "Dark *light blue* with a tint of 800"
);
define_color!(
    LIGHTBLUE_900,
    1,
    87,
    155,
    "Dark *light blue* with a tint of 900"
);
define_color!(
    LIGHTBLUE_A100,
    128,
    216,
    255,
    "Accent *light blue* with a tint of A100"
);
define_color!(
    LIGHTBLUE_A200,
    64,
    196,
    255,
    "Accent *light blue* with a tint of A200"
);
define_color!(
    LIGHTBLUE_A400,
    0,
    176,
    255,
    "Accent *light blue* with a tint of A400"
);
define_color!(
    LIGHTBLUE_A700,
    0,
    145,
    234,
    "Accent *light blue* with a tint of A700"
);
// cyan
define_color!(CYAN, 0, 188, 212, "*Cyan*; same as [`CYAN_500`]");
define_color!(CYAN_50, 224, 247, 250, "Light *cyan* with a tint of 50");
define_color!(CYAN_100, 178, 235, 242, "Light *cyan* with a tint of 100");
define_color!(CYAN_200, 128, 222, 234, "Lighter *cyan* with a tint of 200");
define_color!(CYAN_300, 77, 208, 225, "Lighter *cyan* with a tint of 300");
define_color!(CYAN_400, 38, 198, 218, "Lighter *cyan* with a tint of 400");
define_color!(CYAN_500, 0, 188, 212, "*Cyan* with a tint of 500");
define_color!(CYAN_600, 0, 172, 193, "Darker *cyan* with a tint of 600");
define_color!(CYAN_700, 0, 151, 167, "Darker *cyan* with a tint of 700");
define_color!(CYAN_800, 0, 131, 143, "Dark *cyan* with a tint of 800");
define_color!(CYAN_900, 0, 96, 100, "Dark *cyan* with a tint of 900");
define_color!(
    CYAN_A100,
    132,
    255,
    255,
    "Accent *cyan* with a tint of A100"
);
define_color!(CYAN_A200, 24, 255, 255, "Accent *cyan* with a tint of A200");
define_color!(CYAN_A400, 0, 229, 255, "Accent *cyan* with a tint of A400");
define_color!(CYAN_A700, 0, 184, 212, "Accent *cyan* with a tint of A700");
// teal
define_color!(TEAL, 0, 150, 136, "*Teal*; same as [`TEAL_500`]");
define_color!(TEAL_50, 224, 242, 241, "Light *teal* with a tint of 50");
define_color!(TEAL_100, 178, 223, 219, "Light *teal* with a tint of 100");
define_color!(TEAL_200, 128, 203, 196, "Lighter *teal* with a tint of 200");
define_color!(TEAL_300, 77, 182, 172, "Lighter *teal* with a tint of 300");
define_color!(TEAL_400, 38, 166, 154, "Lighter *teal* with a tint of 400");
define_color!(TEAL_500, 0, 150, 136, "*Teal* with a tint of 500");
define_color!(TEAL_600, 0, 137, 123, "Darker *teal* with a tint of 600");
define_color!(TEAL_700, 0, 121, 107, "Darker *teal* with a tint of 700");
define_color!(TEAL_800, 0, 105, 92, "Dark *teal* with a tint of 800");
define_color!(TEAL_900, 0, 77, 64, "Dark *teal* with a tint of 900");
define_color!(
    TEAL_A100,
    167,
    255,
    235,
    "Accent *teal* with a tint of A100"
);
define_color!(
    TEAL_A200,
    100,
    255,
    218,
    "Accent *teal* with a tint of A200"
);
define_color!(TEAL_A400, 29, 233, 182, "Accent *teal* with a tint of A400");
define_color!(TEAL_A700, 0, 191, 165, "Accent *teal* with a tint of A700");
// green
define_color!(GREEN, 76, 175, 80, "*Green*; same as [`GREEN_500`]");
define_color!(GREEN_50, 232, 245, 233, "Light *green* with a tint of 50");
define_color!(GREEN_100, 200, 230, 201, "Light *green* with a tint of 100");
define_color!(
    GREEN_200,
    165,
    214,
    167,
    "Lighter *green* with a tint of 200"
);
define_color!(
    GREEN_300,
    129,
    199,
    132,
    "Lighter *green* with a tint of 300"
);
define_color!(
    GREEN_400,
    102,
    187,
    106,
    "Lighter *green* with a tint of 400"
);
define_color!(GREEN_500, 76, 175, 80, "*Green* with a tint of 500");
define_color!(GREEN_600, 67, 160, 71, "Darker *green* with a tint of 600");
define_color!(GREEN_700, 56, 142, 60, "Darker *green* with a tint of 700");
define_color!(GREEN_800, 46, 125, 50, "Dark *green* with a tint of 800");
define_color!(GREEN_900, 27, 94, 32, "Dark *green* with a tint of 900");
define_color!(
    GREEN_A100,
    185,
    246,
    202,
    "Accent *green* with a tint of A100"
);
define_color!(
    GREEN_A200,
    105,
    240,
    174,
    "Accent *green* with a tint of A200"
);
define_color!(
    GREEN_A400,
    0,
    230,
    118,
    "Accent *green* with a tint of A400"
);
define_color!(GREEN_A700, 0, 200, 83, "Accent *green* with a tint of A700");
// lightGreen
define_color!(
    LIGHTGREEN,
    139,
    195,
    74,
    "*Light green*; same as [`LIGHTGREEN_500`]"
);
define_color!(
    LIGHTGREEN_50,
    241,
    248,
    233,
    "Light *light green* with a tint of 50"
);
define_color!(
    LIGHTGREEN_100,
    220,
    237,
    200,
    "Light *light green* with a tint of 100"
);
define_color!(
    LIGHTGREEN_200,
    197,
    225,
    165,
    "Lighter *light green* with a tint of 200"
);
define_color!(
    LIGHTGREEN_300,
    174,
    213,
    129,
    "Lighter *light green* with a tint of 300"
);
define_color!(
    LIGHTGREEN_400,
    156,
    204,
    101,
    "Lighter *light green* with a tint of 400"
);
define_color!(
    LIGHTGREEN_500,
    139,
    195,
    74,
    "*Light green* with a tint of 500"
);
define_color!(
    LIGHTGREEN_600,
    124,
    179,
    66,
    "Darker *light green* with a tint of 600"
);
define_color!(
    LIGHTGREEN_700,
    104,
    159,
    56,
    "Darker *light green* with a tint of 700"
);
define_color!(
    LIGHTGREEN_800,
    85,
    139,
    47,
    "Dark *light green* with a tint of 800"
);
define_color!(
    LIGHTGREEN_900,
    51,
    105,
    30,
    "Dark *light green* with a tint of 900"
);
define_color!(
    LIGHTGREEN_A100,
    204,
    255,
    144,
    "Accent *light green* with a tint of A100"
);
define_color!(
    LIGHTGREEN_A200,
    178,
    255,
    89,
    "Accent *light green* with a tint of A200"
);
define_color!(
    LIGHTGREEN_A400,
    118,
    255,
    3,
    "Accent *light green* with a tint of A400"
);
define_color!(
    LIGHTGREEN_A700,
    100,
    221,
    23,
    "Accent *light green* with a tint of A700"
);
// lime
define_color!(LIME, 205, 220, 57, "*Lime*; same as [`LIME_500`]");
define_color!(LIME_50, 249, 251, 231, "Light *lime* with a tint of 50");
define_color!(LIME_100, 240, 244, 195, "Light *lime* with a tint of 100");
define_color!(LIME_200, 230, 238, 156, "Lighter *lime* with a tint of 200");
define_color!(LIME_300, 220, 231, 117, "Lighter *lime* with a tint of 300");
define_color!(LIME_400, 212, 225, 87, "Lighter *lime* with a tint of 400");
define_color!(LIME_500, 205, 220, 57, "*Lime* with a tint of 500");
define_color!(LIME_600, 192, 202, 51, "Darker *lime* with a tint of 600");
define_color!(LIME_700, 175, 180, 43, "Darker *lime* with a tint of 700");
define_color!(LIME_800, 158, 157, 36, "Dark *lime* with a tint of 800");
define_color!(LIME_900, 130, 119, 23, "Dark *lime* with a tint of 900");
define_color!(
    LIME_A100,
    244,
    255,
    129,
    "Accent *lime* with a tint of A100"
);
define_color!(LIME_A200, 238, 255, 65, "Accent *lime* with a tint of A200");
define_color!(LIME_A400, 198, 255, 0, "Accent *lime* with a tint of A400");
define_color!(LIME_A700, 174, 234, 0, "Accent *lime* with a tint of A700");
// yellow
define_color!(YELLOW, 255, 235, 59, "*Yellow*; same as [`YELLOW_500`]");
define_color!(YELLOW_50, 255, 253, 231, "Light *yellow* with a tint of 50");
define_color!(
    YELLOW_100,
    255,
    249,
    196,
    "Light *yellow* with a tint of 100"
);
define_color!(
    YELLOW_200,
    255,
    245,
    157,
    "Lighter *yellow* with a tint of 200"
);
define_color!(
    YELLOW_300,
    255,
    241,
    118,
    "Lighter *yellow* with a tint of 300"
);
define_color!(
    YELLOW_400,
    255,
    238,
    88,
    "Lighter *yellow* with a tint of 400"
);
define_color!(YELLOW_500, 255, 235, 59, "*Yellow* with a tint of 500");
define_color!(
    YELLOW_600,
    253,
    216,
    53,
    "Darker *yellow* with a tint of 600"
);
define_color!(
    YELLOW_700,
    251,
    192,
    45,
    "Darker *yellow* with a tint of 700"
);
define_color!(YELLOW_800, 249, 168, 37, "Dark *yellow* with a tint of 800");
define_color!(YELLOW_900, 245, 127, 23, "Dark *yellow* with a tint of 900");
define_color!(
    YELLOW_A100,
    255,
    255,
    141,
    "Accent *yellow* with a tint of A100"
);
define_color!(
    YELLOW_A200,
    255,
    255,
    0,
    "Accent *yellow* with a tint of A200"
);
define_color!(
    YELLOW_A400,
    255,
    234,
    0,
    "Accent *yellow* with a tint of A400"
);
define_color!(
    YELLOW_A700,
    255,
    214,
    0,
    "Accent *yellow* with a tint of A700"
);
// amber
define_color!(AMBER, 255, 193, 7, "*Amber*; same as [`AMBER_500`]");
define_color!(AMBER_50, 255, 248, 225, "Light *amber* with a tint of 50");
define_color!(AMBER_100, 255, 236, 179, "Light *amber* with a tint of 100");
define_color!(
    AMBER_200,
    255,
    224,
    130,
    "Lighter *amber* with a tint of 200"
);
define_color!(
    AMBER_300,
    255,
    213,
    79,
    "Lighter *amber* with a tint of 300"
);
define_color!(
    AMBER_400,
    255,
    202,
    40,
    "Lighter *amber* with a tint of 400"
);
define_color!(AMBER_500, 255, 193, 7, "*Amber* with a tint of 500");
define_color!(AMBER_600, 255, 179, 0, "Darker *amber* with a tint of 600");
define_color!(AMBER_700, 255, 160, 0, "Darker *amber* with a tint of 700");
define_color!(AMBER_800, 255, 143, 0, "Dark *amber* with a tint of 800");
define_color!(AMBER_900, 255, 111, 0, "Dark *amber* with a tint of 900");
define_color!(
    AMBER_A100,
    255,
    229,
    127,
    "Accent *amber* with a tint of A100"
);
define_color!(
    AMBER_A200,
    255,
    215,
    64,
    "Accent *amber* with a tint of A200"
);
define_color!(
    AMBER_A400,
    255,
    196,
    0,
    "Accent *amber* with a tint of A400"
);
define_color!(
    AMBER_A700,
    255,
    171,
    0,
    "Accent *amber* with a tint of A700"
);
// orange
define_color!(ORANGE, 255, 152, 0, "*Orange*; same as [`ORANGE_500`]");
define_color!(ORANGE_50, 255, 243, 224, "Light *orange* with a tint of 50");
define_color!(
    ORANGE_100,
    255,
    224,
    178,
    "Light *orange* with a tint of 100"
);
define_color!(
    ORANGE_200,
    255,
    204,
    128,
    "Lighter *orange* with a tint of 200"
);
define_color!(
    ORANGE_300,
    255,
    183,
    77,
    "Lighter *orange* with a tint of 300"
);
define_color!(
    ORANGE_400,
    255,
    167,
    38,
    "Lighter *orange* with a tint of 400"
);
define_color!(ORANGE_500, 255, 152, 0, "*Orange* with a tint of 500");
define_color!(
    ORANGE_600,
    251,
    140,
    0,
    "Darker *orange* with a tint of 600"
);
define_color!(
    ORANGE_700,
    245,
    124,
    0,
    "Darker *orange* with a tint of 700"
);
define_color!(ORANGE_800, 239, 108, 0, "Dark *orange* with a tint of 800");
define_color!(ORANGE_900, 230, 81, 0, "Dark *orange* with a tint of 900");
define_color!(
    ORANGE_A100,
    255,
    209,
    128,
    "Accent *orange* with a tint of A100"
);
define_color!(
    ORANGE_A200,
    255,
    171,
    64,
    "Accent *orange* with a tint of A200"
);
define_color!(
    ORANGE_A400,
    255,
    145,
    0,
    "Accent *orange* with a tint of A400"
);
define_color!(
    ORANGE_A700,
    255,
    109,
    0,
    "Accent *orange* with a tint of A700"
);
// deepOrange
define_color!(
    DEEPORANGE,
    255,
    87,
    34,
    "*Deep orange*; same as [`DEEPORANGE_500`]"
);
define_color!(
    DEEPORANGE_50,
    251,
    233,
    231,
    "Light *deep orange* with a tint of 50"
);
define_color!(
    DEEPORANGE_100,
    255,
    204,
    188,
    "Light *deep orange* with a tint of 100"
);
define_color!(
    DEEPORANGE_200,
    255,
    171,
    145,
    "Lighter *deep orange* with a tint of 200"
);
define_color!(
    DEEPORANGE_300,
    255,
    138,
    101,
    "Lighter *deep orange* with a tint of 300"
);
define_color!(
    DEEPORANGE_400,
    255,
    112,
    67,
    "Lighter *deep orange* with a tint of 400"
);
define_color!(
    DEEPORANGE_500,
    255,
    87,
    34,
    "*Deep orange* with a tint of 500"
);
define_color!(
    DEEPORANGE_600,
    244,
    81,
    30,
    "Darker *deep orange* with a tint of 600"
);
define_color!(
    DEEPORANGE_700,
    230,
    74,
    25,
    "Darker *deep orange* with a tint of 700"
);
define_color!(
    DEEPORANGE_800,
    216,
    67,
    21,
    "Dark *deep orange* with a tint of 800"
);
define_color!(
    DEEPORANGE_900,
    191,
    54,
    12,
    "Dark *deep orange* with a tint of 900"
);
define_color!(
    DEEPORANGE_A100,
    255,
    158,
    128,
    "Accent *deep orange* with a tint of A100"
);
define_color!(
    DEEPORANGE_A200,
    255,
    110,
    64,
    "Accent *deep orange* with a tint of A200"
);
define_color!(
    DEEPORANGE_A400,
    255,
    61,
    0,
    "Accent *deep orange* with a tint of A400"
);
define_color!(
    DEEPORANGE_A700,
    221,
    44,
    0,
    "Accent *deep orange* with a tint of A700"
);
// brown
define_color!(BROWN, 121, 85, 72, "*Brown*; same as [`BROWN_500`]");
define_color!(BROWN_50, 239, 235, 233, "Light *brown* with a tint of 50");
define_color!(BROWN_100, 215, 204, 200, "Light *brown* with a tint of 100");
define_color!(
    BROWN_200,
    188,
    170,
    164,
    "Lighter *brown* with a tint of 200"
);
define_color!(
    BROWN_300,
    161,
    136,
    127,
    "Lighter *brown* with a tint of 300"
);
define_color!(
    BROWN_400,
    141,
    110,
    99,
    "Lighter *brown* with a tint of 400"
);
define_color!(BROWN_500, 121, 85, 72, "*Brown* with a tint of 500");
define_color!(BROWN_600, 109, 76, 65, "Darker *brown* with a tint of 600");
define_color!(BROWN_700, 93, 64, 55, "Darker *brown* with a tint of 700");
define_color!(BROWN_800, 78, 52, 46, "Dark *brown* with a tint of 800");
define_color!(BROWN_900, 62, 39, 35, "Dark *brown* with a tint of 900");
define_color!(
    BROWN_A100,
    215,
    204,
    200,
    "Accent *brown* with a tint of A100"
);
define_color!(
    BROWN_A200,
    188,
    170,
    164,
    "Accent *brown* with a tint of A200"
);
define_color!(
    BROWN_A400,
    141,
    110,
    99,
    "Accent *brown* with a tint of A400"
);
define_color!(BROWN_A700, 93, 64, 55, "Accent *brown* with a tint of A700");
// grey
define_color!(GREY, 158, 158, 158, "*Grey*; same as [`GREY_500`]");
define_color!(GREY_50, 250, 250, 250, "Light *grey* with a tint of 50");
define_color!(GREY_100, 245, 245, 245, "Light *grey* with a tint of 100");
define_color!(GREY_200, 238, 238, 238, "Lighter *grey* with a tint of 200");
define_color!(GREY_300, 224, 224, 224, "Lighter *grey* with a tint of 300");
define_color!(GREY_400, 189, 189, 189, "Lighter *grey* with a tint of 400");
define_color!(GREY_500, 158, 158, 158, "*Grey* with a tint of 500");
define_color!(GREY_600, 117, 117, 117, "Darker *grey* with a tint of 600");
define_color!(GREY_700, 97, 97, 97, "Darker *grey* with a tint of 700");
define_color!(GREY_800, 66, 66, 66, "Dark *grey* with a tint of 800");
define_color!(GREY_900, 33, 33, 33, "Dark *grey* with a tint of 900");
define_color!(
    GREY_A100,
    213,
    213,
    213,
    "Accent *grey* with a tint of A100"
);
define_color!(
    GREY_A200,
    170,
    170,
    170,
    "Accent *grey* with a tint of A200"
);
define_color!(GREY_A400, 48, 48, 48, "Accent *grey* with a tint of A400");
define_color!(GREY_A700, 97, 97, 97, "Accent *grey* with a tint of A700");
// blueGrey
define_color!(
    BLUEGREY,
    96,
    125,
    139,
    "*Blue grey*; same as [`BLUEGREY_500`]"
);
define_color!(
    BLUEGREY_50,
    236,
    239,
    241,
    "Light *blue grey* with a tint of 50"
);
define_color!(
    BLUEGREY_100,
    207,
    216,
    220,
    "Light *blue grey* with a tint of 100"
);
define_color!(
    BLUEGREY_200,
    176,
    190,
    197,
    "Lighter *blue grey* with a tint of 200"
);
define_color!(
    BLUEGREY_300,
    144,
    164,
    174,
    "Lighter *blue grey* with a tint of 300"
);
define_color!(
    BLUEGREY_400,
    120,
    144,
    156,
    "Lighter *blue grey* with a tint of 400"
);
define_color!(BLUEGREY_500, 96, 125, 139, "*Blue grey* with a tint of 500");
define_color!(
    BLUEGREY_600,
    84,
    110,
    122,
    "Darker *blue grey* with a tint of 600"
);
define_color!(
    BLUEGREY_700,
    69,
    90,
    100,
    "Darker *blue grey* with a tint of 700"
);
define_color!(
    BLUEGREY_800,
    55,
    71,
    79,
    "Dark *blue grey* with a tint of 800"
);
define_color!(
    BLUEGREY_900,
    38,
    50,
    56,
    "Dark *blue grey* with a tint of 900"
);
define_color!(
    BLUEGREY_A100,
    207,
    216,
    220,
    "Accent *blue grey* with a tint of A100"
);
define_color!(
    BLUEGREY_A200,
    176,
    190,
    197,
    "Accent *blue grey* with a tint of A200"
);
define_color!(
    BLUEGREY_A400,
    120,
    144,
    156,
    "Accent *blue grey* with a tint of A400"
);
define_color!(
    BLUEGREY_A700,
    69,
    90,
    100,
    "Accent *blue grey* with a tint of A700"
);
