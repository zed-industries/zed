import { ColorScheme } from "../theme/colorScheme"
import { background, border, borderColor, foreground, text } from "./components"
import { interactive, toggleable } from "../element"
export default function contactsPanel(colorScheme: ColorScheme) {
  const nameMargin = 8
  const sidePadding = 12

  let layer = colorScheme.middle

  const contactButton = {
    background: background(layer, "on"),
    color: foreground(layer, "on"),
    iconWidth: 8,
    buttonWidth: 16,
    cornerRadius: 8,
  }
  const projectRow = {
    guestAvatarSpacing: 4,
    height: 24,
    guestAvatar: {
      cornerRadius: 8,
      width: 14,
    },
    name: {
      ...text(layer, "mono", { size: "sm" }),
      margin: {
        left: nameMargin,
        right: 6,
      },
    },
    guests: {
      margin: {
        left: nameMargin,
        right: nameMargin,
      },
    },
    padding: {
      left: sidePadding,
      right: sidePadding,
    },
  }

  return {
    background: background(layer),
    padding: { top: 12 },
    userQueryEditor: {
      background: background(layer, "on"),
      cornerRadius: 6,
      text: text(layer, "mono", "on"),
      placeholderText: text(layer, "mono", "on", "disabled", {
        size: "xs",
      }),
      selection: colorScheme.players[0],
      border: border(layer, "on"),
      padding: {
        bottom: 4,
        left: 8,
        right: 8,
        top: 4,
      },
      margin: {
        left: 6,
      },
    },
    userQueryEditorHeight: 33,
    addContactButton: {
      margin: { left: 6, right: 12 },
      color: foreground(layer, "on"),
      buttonWidth: 28,
      iconWidth: 16,
    },
    rowHeight: 28,
    sectionIconSize: 8,
    headerRow: toggleable({
      base: interactive({
        base: {
          ...text(layer, "mono", { size: "sm" }),
          margin: { top: 14 },
          padding: {
            left: sidePadding,
            right: sidePadding,
          },
          background: background(layer, "default"), // posiewic: breaking change
        },
        state: {
          hovered: { background: background(layer, "default") },
        }, // hack, we want headerRow to be interactive for whatever reason. It probably shouldn't be interactive in the first place.
      }),
      state: {
        active: {
          default: {
            ...text(layer, "mono", "active", { size: "sm" }),
            background: background(layer, "active"),
          },
        }
      }
    }),
    leaveCall: interactive({
      base: {
        background: background(layer),
        border: border(layer),
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
        ...text(layer, "sans", "variant", { size: "xs" }),
      },
      state: {
        hovered: {
          ...text(layer, "sans", "hovered", { size: "xs" }),
          background: background(layer, "hovered"),
          border: border(layer, "hovered"),
        },
      },
    }),
    contactRow: {
      inactive: {
        default: {
          padding: {
            left: sidePadding,
            right: sidePadding,
          },
        },
      },
      active: {
        default: {
          background: background(layer, "active"),
          padding: {
            left: sidePadding,
            right: sidePadding,
          },
        },
      },
    },

    contactAvatar: {
      cornerRadius: 10,
      width: 18,
    },
    contactStatusFree: {
      cornerRadius: 4,
      padding: 4,
      margin: { top: 12, left: 12 },
      background: foreground(layer, "positive"),
    },
    contactStatusBusy: {
      cornerRadius: 4,
      padding: 4,
      margin: { top: 12, left: 12 },
      background: foreground(layer, "negative"),
    },
    contactUsername: {
      ...text(layer, "mono", { size: "sm" }),
      margin: {
        left: nameMargin,
      },
    },
    contactButtonSpacing: nameMargin,
    contactButton: interactive({
      base: { ...contactButton },
      state: {
        hovered: {
          background: background(layer, "hovered"),
        },
      },
    }),
    disabledButton: {
      ...contactButton,
      background: background(layer, "on"),
      color: foreground(layer, "on"),
    },
    callingIndicator: {
      ...text(layer, "mono", "variant", { size: "xs" }),
    },
    treeBranch: toggleable({
      base:
        interactive({
          base: {
            color: borderColor(layer),
            width: 1,
          },
          state: {
            hovered: {
              color: borderColor(layer),
            },
          },
        }),
      state: {
        active: {
          default: {
            color: borderColor(layer),
          },
        }
      }
    }
    ),
    projectRow: toggleable({
      base:
        interactive({
          base: {
            ...projectRow,
            background: background(layer),
            icon: {
              margin: { left: nameMargin },
              color: foreground(layer, "variant"),
              width: 12,
            },
            name: {
              ...projectRow.name,
              ...text(layer, "mono", { size: "sm" }),
            },
          },
          state: {
            hovered: {
              background: background(layer, "hovered"),
            },
          },
        }),
      state: {
        active: {
          default: { background: background(layer, "active") },
        }
      }
    }
    ),
  }
}
