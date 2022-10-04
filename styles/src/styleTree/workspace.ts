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
      height: 33, // 32px + 1px for overlaid border
      background: background(layer),
      border: border(layer, { bottom: true, overlay: true }),
      padding: {
        left: 80,
        right: titlebarPadding,
      },

      // Project
      title: text(layer, "sans", "variant"),

      // Collaborators
      avatarWidth: 18,
      avatarMargin: 8,
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
        // TODO: Chore: Make avatarRibbon colors driven by the theme rather than being hard coded.
      },

      // Sign in buttom
      // FlatButton, Variant
      signInPrompt: {
        ...text(layer, "sans", { size: "xs" }),
        background: background(layer),
        border: border(layer),
        cornerRadius: 6,
        padding: {
          top: 1,
          bottom: 1,
          left: 8,
          right: 8,
        },
        hover: {
          ...text(layer, "sans", "hovered", { size: "xs" }),
          background: background(layer, "hovered"),
        },
      },

      // Offline Indicator
      offlineIcon: {
        color: foreground(layer, "variant"),
        width: 16,
        margin: {
          left: titlebarPadding,
        },
        padding: {
          right: 4,
        },
      },

      // Notice that the collaboration server is out of date
      outdatedWarning: {
        ...text(layer, "sans", "warning", { size: "xs" }),
        background: withOpacity(background(layer, "warning"), 0.3),
        border: border(layer, "warning"),
        margin: {
          left: titlebarPadding,
        },
        padding: {
          left: 8,
          right: 8,
        },
        cornerRadius: 6,
      },
    },

    toolbar: {
      height: 34,
      background: background(elevation.top),
      border: border(elevation.top, { bottom: true }),
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
      ...text(layer, "mono", "variant"),
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
