import chroma from "chroma-js";
import { createColorScheme } from "../common/ramps";
import { ThemeConfig } from "../common/themeConfig";

const colors = {
  dark0_hard: chroma(29, 32, 33, "rgb").hex().toString(),
  dark0: chroma(40, 40, 40, "rgb").hex().toString(),
  dark0_soft: chroma(50, 48, 47, "rgb").hex().toString(),
  dark1: chroma(60, 56, 54, "rgb").hex().toString(),
  dark2: chroma(80, 73, 69, "rgb").hex().toString(),
  dark3: chroma(102, 92, 84, "rgb").hex().toString(),
  dark4: chroma(124, 111, 100, "rgb").hex().toString(),
  dark4_256: chroma(124, 111, 100, "rgb").hex().toString(),

  gray_245: chroma(146, 131, 116, "rgb").hex().toString(),
  gray_244: chroma(146, 131, 116, "rgb").hex().toString(),

  light0_hard: chroma(249, 245, 215, "rgb").hex().toString(),
  light0: chroma(253, 244, 193, "rgb").hex().toString(),
  light0_soft: chroma(242, 229, 188, "rgb").hex().toString(),
  light1: chroma(235, 219, 178, "rgb").hex().toString(),
  light2: chroma(213, 196, 161, "rgb").hex().toString(),
  light3: chroma(189, 174, 147, "rgb").hex().toString(),
  light4: chroma(168, 153, 132, "rgb").hex().toString(),
  light4_256: chroma(168, 153, 132, "rgb").hex().toString(),

  bright_red: chroma(251, 73, 52, "rgb").hex().toString(),
  bright_green: chroma(184, 187, 38, "rgb").hex().toString(),
  bright_yellow: chroma(250, 189, 47, "rgb").hex().toString(),
  bright_blue: chroma(131, 165, 152, "rgb").hex().toString(),
  bright_purple: chroma(211, 134, 155, "rgb").hex().toString(),
  bright_aqua: chroma(142, 192, 124, "rgb").hex().toString(),
  bright_orange: chroma(254, 128, 25, "rgb").hex().toString(),

  neutral_red: chroma(204, 36, 29, "rgb").hex().toString(),
  neutral_green: chroma(152, 151, 26, "rgb").hex().toString(),
  neutral_yellow: chroma(215, 153, 33, "rgb").hex().toString(),
  neutral_blue: chroma(69, 133, 136, "rgb").hex().toString(),
  neutral_purple: chroma(177, 98, 134, "rgb").hex().toString(),
  neutral_aqua: chroma(104, 157, 106, "rgb").hex().toString(),
  neutral_orange: chroma(214, 93, 14, "rgb").hex().toString(),

  faded_red: chroma(157, 0, 6, "rgb").hex().toString(),
  faded_green: chroma(121, 116, 14, "rgb").hex().toString(),
  faded_yellow: chroma(181, 118, 20, "rgb").hex().toString(),
  faded_blue: chroma(7, 102, 120, "rgb").hex().toString(),
  faded_purple: chroma(143, 63, 113, "rgb").hex().toString(),
  faded_aqua: chroma(66, 123, 88, "rgb").hex().toString(),
  faded_orange: chroma(175, 58, 3, "rgb").hex().toString()
}

type Appearance = "Dark" | "Light"
type Contrast = "Hard" | "Medium" | "Soft"

function buildNeutralRamp(appearance: Appearance, contrast: Contrast) {
  let neutralRamp: string[] = []
  let firstValue = ""

  if (appearance == "Light") {
    switch (contrast) {
      case ("Hard"): {
        firstValue = colors.light0_hard
        break;
      }
      case ("Soft"): {
        firstValue = colors.light0_soft
        break;
      }
      default: {
        firstValue = colors.light0
      }
    }

    neutralRamp = [
      firstValue, colors.light1, colors.light2, colors.light3, colors.light4,
      colors.dark4, colors.dark3, colors.dark2, colors.dark1, colors.dark0
    ].reverse()
  }

  if (appearance == "Dark") {
    switch (contrast) {
      case ("Hard"): {
        firstValue = colors.dark0_hard
        break;
      }
      case ("Soft"): {
        firstValue = colors.dark0_soft
        break;
      }
      default: {
        firstValue = colors.dark0
      }
    }

    neutralRamp = [
      firstValue, colors.dark1, colors.dark2, colors.dark3, colors.dark4,
      colors.light4, colors.light3, colors.light2, colors.light1, colors.light0
    ]
  }

  return neutralRamp
}
function buildRamps(appearance: Appearance, contrast: Contrast) {
  let neutral = buildNeutralRamp(appearance, contrast)

  return {
    neutral: chroma.scale(neutral),
    red: chroma.scale([
      "#4D150F",
      "#7D241A",
      "#A31C17",
      "#CC241D",
      "#C83A29",
      "#FB4934",
      "#F06D61",
      "#E6928E",
      "#FFFFFF",
    ]),
    orange: chroma.scale([
      "#462307",
      "#7F400C",
      "#AB4A0B",
      "#D65D0E",
      "#CB6614",
      "#FE8019",
      "#F49750",
      "#EBAE87",
      "#FFFFFF",
    ]),
    yellow: chroma.scale([
      "#3D2C05",
      "#7D5E17",
      "#AC7A1A",
      "#D79921",
      "#E8AB28",
      "#FABD2F",
      "#F2C45F",
      "#EBCC90",
      "#FFFFFF",
    ]),
    green: chroma.scale([
      "#32330A",
      "#5C5D13",
      "#797814",
      "#98971A",
      "#93951E",
      "#B8BB26",
      "#C2C359",
      "#CCCB8D",
      "#FFFFFF",
    ]),
    cyan: chroma.scale([
      "#283D20",
      "#47603E",
      "#537D54",
      "#689D6A",
      "#719963",
      "#8EC07C",
      "#A1C798",
      "#B4CEB5",
      "#FFFFFF",
    ]),
    blue: chroma.scale([
      "#103738",
      "#214C4D",
      "#376A6C",
      "#458588",
      "#688479",
      "#83A598",
      "#92B3AE",
      "#A2C2C4",
      "#FFFFFF",
    ]),
    violet: chroma.scale([
      "#392228",
      "#69434D",
      "#8D4E6B",
      "#B16286",
      "#A86B7C",
      "#D3869B",
      "#D59BAF",
      "#D8B1C3",
      "#FFFFFF",
    ]),
    magenta: chroma.scale([
      "#48402C",
      "#756D59",
      "#867A69",
      "#A89984",
      "#BCAF8E",
      "#EBDBB2",
      "#DFD3BA",
      "#D4CCC2",
      "#FFFFFF",
    ])
  }
}

function buildThemeConfig(appearance: Appearance, contrast: Contrast) {
  const light = appearance == "Light"

  const red = light ? colors.faded_red : colors.neutral_red
  const orange = light ? colors.faded_orange : colors.neutral_orange
  const yellow = light ? colors.faded_yellow : colors.neutral_yellow
  const green = light ? colors.faded_green : colors.neutral_green
  const lightblue = light ? colors.faded_blue : colors.neutral_blue
  const blue = light ? colors.neutral_blue : colors.faded_blue
  const aqua = light ? colors.faded_aqua : colors.neutral_aqua
  const purple = light ? colors.faded_purple : colors.neutral_purple

  const syntax = {
    comment: { color: colors.gray_245 },
    function: { color: yellow },
    type: { color: yellow },
    property: { color: lightblue },
    number: { color: light ? colors.faded_purple : colors.bright_purple },
    string: { color: green },
    keyword: { color: red },
    boolean: { color: orange },
    punctuation: { color: purple },
    operator: { color: aqua },
    enum: { color: lightblue },
    method: { color: lightblue }
  }

  const theme: ThemeConfig = {
    meta: {
      name: `Gruvbox ${appearance} ${contrast}`,
      author: "morhetz (Pavel Pertsev)",
      url: "https://github.com/morhetz/gruvbox",
      license: {
        type: "MIT/X11",
        url: "https://en.wikipedia.org/wiki/MIT_License",
      },
    },
    color: buildRamps(appearance, contrast),
    syntax,
    override: {},
  }

  const ramps = buildRamps(appearance, contrast)

  const colorScheme = createColorScheme(theme.meta.name, light, ramps, theme)

  return colorScheme
}

export const variants = [
  buildThemeConfig("Dark", "Hard"),
  buildThemeConfig("Dark", "Medium"),
  buildThemeConfig("Dark", "Soft"),
  buildThemeConfig("Light", "Hard"),
  buildThemeConfig("Light", "Medium"),
  buildThemeConfig("Light", "Soft")
]