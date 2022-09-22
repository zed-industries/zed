import { ColorScheme } from "../themes/common/colorScheme";
import { withOpacity } from "../utils/color";
import {
  background,
  border,
  borderColor,
  foreground,
  text
} from "./components";
import statusBar from "./statusBar";
import tabBar from "./tabBar";

export default function workspace(colorScheme: ColorScheme) {
  const elevation = colorScheme.lowest;
  const layer = elevation.bottom;
  const titlebarPadding = 6;

  return {
    background: background(layer),
    joiningProjectAvatar: {
      cornerRadius: 40,
      width: 80,
    },
    joiningProjectMessage: {
      padding: 12,
      ...text(layer, "sans", { size: "lg" }),
    },
    leaderBorderOpacity: 0.7,
    leaderBorderWidth: 2.0,
    tabBar: tabBar(colorScheme),
    modal: {
      margin: {
        bottom: 52,
        top: 52,
      },
      cursor: "Arrow",
    },
    sidebar: {
      initialSize: 240,
      border: border(
        layer,
        { left: true, right: true }
      ),
    },
    paneDivider: {
      color: borderColor(layer),
      width: 1,
    },
    statusBar: statusBar(colorScheme),
    titlebar: {
      avatarWidth: 18,
      avatarMargin: 8,
      height: 33,
      background: background(layer),
      padding: {
        left: 80,
        right: titlebarPadding,
      },
      title: text(layer, "sans"),
      avatar: {
        cornerRadius: 10,
        border: {
          color: "#00000088",
          width: 1,
        },
      },
      avatarRibbon: {
        height: 3,
        width: 12,
        // TODO: The background for this ideally should be
        // set with a token, not hardcoded in rust
      },
      border: border(layer, { bottom: true, overlay: true }),
      signInPrompt: {
        background: background(layer, "on", "default"),
        border: border(layer, "on", "default"),
        cornerRadius: 6,
        margin: {
          top: 1,
        },
        padding: {
          top: 1,
          bottom: 1,
          left: 7,
          right: 7,
        },
        ...text(layer, "sans", { size: "xs" }),
        hover: {
          ...text(layer, "sans", "on", "hovered", { size: "xs" }),
          background: background(layer, "on", "hovered"),
          border: border(layer, "on", "hovered"),
        },
      },
      offlineIcon: {
        color: foreground(layer, "on"),
        width: 16,
        margin: {
          left: titlebarPadding,
        },
        padding: {
          right: 4,
        },
      },
      outdatedWarning: {
        ...text(layer, "sans", "warning", { size: "xs" }),
        background: background(layer, "warning"),
        border: border(layer, "warning"),
        margin: {
          left: titlebarPadding,
        },
        padding: {
          left: 6,
          right: 6,
        },
        cornerRadius: 6,
      },
    },
    toolbar: {
      height: 34,
      background: background(elevation.top),
      border: border(elevation.top, "base", "variant", { bottom: true }),
      itemSpacing: 8,
      navButton: {
        color: foreground(elevation.top, "on"),
        iconWidth: 12,
        buttonWidth: 24,
        cornerRadius: 6,
        hover: {
          color: foreground(elevation.top, "on", "hovered"),
          background: background(elevation.top, "on", "hovered"),
        },
        disabled: {
          color: foreground(elevation.top, "on", "disabled"),
        },
      },
      padding: { left: 8, right: 8, top: 4, bottom: 4 },
    },
    breadcrumbs: {
      ...text(layer, "mono", "on", "variant"),
      padding: { left: 6 },
    },
    disconnectedOverlay: {
      ...text(layer, "sans"),
      background: withOpacity(background(layer), 0.8),
    },
    notification: {
      margin: { top: 10 },
      background: background(elevation.above.middle),
      cornerRadius: 6,
      padding: 12,
      border: border(elevation.above.middle),
      shadow: elevation.above.shadow,
    },
    notifications: {
      width: 400,
      margin: { right: 10, bottom: 10 },
    },
    dock: {
      initialSizeRight: 640,
      initialSizeBottom: 480,
      wash_color: withOpacity(background(elevation.top), 0.5),
      panel: {
        border: border(elevation.top),
      },
      maximized: {
        margin: 32,
        border: border(elevation.above.top, { "overlay": true }),
        shadow: elevation.above.shadow,
      }
    }
  };
}
