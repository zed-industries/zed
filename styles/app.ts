import { backgroundColor } from "./components";
import selectorModal from "./selector-modal";
import workspace from "./workspace";
import editor from "./editor";
import Theme from "./theme";

export default function app(theme: Theme): Object {
  return {
    selector: selectorModal(theme),
    workspace: workspace(theme),
    editor: editor(theme),
    chatPanel: {
      extends: "$panel",
      channel_name: {
        extends: "$text.primary",
        weight: "bold",
      },
      channel_name_hash: {
        text: "$text.muted",
        padding: {
          right: 8,
        },
      },
      channel_select: {
        active_item: {
          extends: "$chat_panel.channel_select.item",
          name: "$text.primary",
        },
        header: {
          extends: "$chat_panel.channel_select.active_item",
          padding: {
            bottom: 4,
            left: 0,
          },
        },
        hovered_active_item: {
          extends: "$chat_panel.channel_select.hovered_item",
          name: "$text.primary",
        },
        hovered_item: {
          background: "$state.hover",
          corner_radius: 6,
          extends: "$chat_panel.channel_select.item",
        },
        item: {
          name: "$text.secondary",
          padding: 4,
          hash: {
            extends: "$text.muted",
            margin: {
              right: 8,
            },
          },
        },
        menu: {
          background: "$surface.500",
          corner_radius: 6,
          padding: 4,
          border: {
            color: "$border.primary",
            width: 1,
          },
          shadow: {
            blur: 16,
            color: "$shadow.0",
            offset: [0, 2],
          },
        },
      },
      hovered_sign_in_prompt: {
        color: "$text.secondary.color",
        extends: "$chat_panel.sign_in_prompt",
      },
      input_editor: {
        background: backgroundColor(theme, 300),
        corner_radius: 6,
        placeholder_text: "$text.muted",
        selection: "$selection.host",
        text: "$text.primary",
        border: {
          color: "$border.primary",
          width: 1,
        },
        padding: {
          bottom: 7,
          left: 8,
          right: 8,
          top: 7,
        },
      },
      message: {
        body: "$text.secondary",
        timestamp: "$text.muted",
        padding: {
          bottom: 6,
        },
        sender: {
          extends: "$text.primary",
          weight: "bold",
          margin: {
            right: 8,
          },
        },
      },
      pending_message: {
        extends: "$chat_panel.message",
        body: {
          color: "$text.muted.color",
        },
        sender: {
          color: "$text.muted.color",
        },
        timestamp: {
          color: "$text.muted.color",
        },
      },
      sign_in_prompt: {
        extends: "$text.primary",
        underline: true,
      },
    },
    contacts_panel: {
      extends: "$panel",
      host_row_height: 28,
      tree_branch_color: "$surface.100",
      tree_branch_width: 1,
      host_avatar: {
        corner_radius: 10,
        width: 18,
      },
      host_username: {
        extends: "$text.primary",
        padding: {
          left: 8,
        },
      },
      hovered_shared_project: {
        background: "$state.hover",
        corner_radius: 6,
        extends: "$contacts_panel.shared_project",
      },
      hovered_unshared_project: {
        background: "$state.hover",
        corner_radius: 6,
        extends: "$contacts_panel.unshared_project",
      },
      project: {
        guest_avatar_spacing: 4,
        height: 24,
        guest_avatar: {
          corner_radius: 8,
          width: 14,
        },
        name: {
          extends: "$text.secondary",
          margin: {
            right: 6,
          },
        },
        padding: {
          left: 8,
        },
      },
      shared_project: {
        extends: "$contacts_panel.project",
        name: {
          color: "$text.primary.color",
        },
      },
      unshared_project: {
        extends: "$contacts_panel.project",
      },
    },
    project_diagnostics: {
      background: backgroundColor(theme, 300),
      tab_icon_spacing: 4,
      tab_icon_width: 13,
      tab_summary_spacing: 10,
      empty_message: {
        extends: "$text.primary",
        size: 18,
      },
      status_bar_item: {
        extends: "$text.muted",
        margin: {
          right: 10,
        },
      },
    },
    project_panel: {
      extends: "$panel",
      entry: {
        height: 22,
        icon_color: "$text.muted.color",
        icon_size: 8,
        icon_spacing: 8,
        text: "$text.secondary",
      },
      hovered_entry: {
        background: "$state.hover",
        extends: "$project_panel.entry",
      },
      hovered_selected_entry: {
        extends: "$project_panel.hovered_entry",
        text: {
          extends: "$text.primary",
        },
      },
      padding: {
        top: 6,
      },
      selected_entry: {
        extends: "$project_panel.entry",
        text: {
          extends: "$text.primary",
        },
      },
    },
    search: {
      background: backgroundColor(theme, 300),
      match_background: "$state.highlighted_line",
      tab_icon_spacing: 4,
      tab_icon_width: 14,
      active_hovered_option_button: {
        background: "$surface.100",
        extends: "$search.option_button",
      },
      active_option_button: {
        background: "$surface.100",
        extends: "$search.option_button",
      },
      editor: {
        background: "$surface.500",
        corner_radius: 6,
        max_width: 400,
        placeholder_text: "$text.muted",
        selection: "$selection.host",
        text: "$text.primary",
        border: {
          color: "$border.primary",
          width: 1,
        },
        margin: {
          bottom: 5,
          left: 5,
          right: 5,
          top: 5,
        },
        padding: {
          bottom: 3,
          left: 13,
          right: 13,
          top: 3,
        },
      },
      hovered_option_button: {
        background: "$surface.100",
        extends: "$search.option_button",
      },
      invalid_editor: {
        extends: "$search.editor",
        border: {
          color: "$status.bad",
          width: 1,
        },
      },
      match_index: {
        extends: "$text.secondary",
        padding: 6,
      },
      option_button: {
        background: backgroundColor(theme, 300),
        corner_radius: 6,
        extends: "$text.secondary",
        border: {
          color: "$border.primary",
          width: 1,
        },
        margin: {
          left: 1,
          right: 1,
        },
        padding: {
          bottom: 1,
          left: 6,
          right: 6,
          top: 1,
        },
      },
      option_button_group: {
        padding: {
          left: 2,
          right: 2,
        },
      },
      results_status: {
        extends: "$text.primary",
        size: 18,
      },
    },
  };
}
