import { backgroundColor } from "./components";
import selectorModal from "./selector-modal";
import workspace from "./workspace";
import Theme from "./theme";

export default function app(theme: Theme): Object {
  return {
    selector: selectorModal(theme),
    workspace: workspace(theme),
    chat_panel: {
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
    editor: {
      active_line_background: "$state.active_line",
      background: backgroundColor(theme, 300),
      code_actions_indicator: "$text.muted.color",
      diff_background_deleted: "$state.deleted_line",
      diff_background_inserted: "$state.inserted_line",
      document_highlight_read_background: "#99999920",
      document_highlight_write_background: "#99999916",
      error_color: "$status.bad",
      guest_selections: "$selection.guests",
      gutter_background: backgroundColor(theme, 300),
      gutter_padding_factor: 2.5,
      highlighted_line_background: "$state.highlighted_line",
      line_number: "$text.muted.color",
      line_number_active: "$text.primary.color",
      rename_fade: 0.6,
      selection: "$selection.host",
      text_color: "$text.secondary.color",
      unnecessary_code_fade: 0.5,
      autocomplete: {
        background: "$surface.100",
        corner_radius: 6,
        padding: 6,
        border: {
          color: "$border.secondary",
          width: 2,
        },
        hovered_item: {
          background: "$state.hover",
          extends: "$editor.autocomplete.item",
        },
        item: {
          corner_radius: 6,
          padding: {
            bottom: 2,
            left: 6,
            right: 6,
            top: 2,
          },
        },
        margin: {
          left: -14,
        },
        match_highlight: {
          color: "$editor.syntax.keyword.color",
          weight: "$editor.syntax.keyword.weight",
        },
        selected_item: {
          background: "$state.selected",
          extends: "$editor.autocomplete.item",
        },
      },
      diagnostic_header: {
        background: "$editor.background",
        icon_width_factor: 1.5,
        text_scale_factor: 0.857,
        border: {
          bottom: true,
          color: "$border.secondary",
          top: true,
          width: 1,
        },
        code: {
          extends: "$text.muted",
          size: 14,
          margin: {
            left: 10,
          },
        },
        message: {
          highlight_text: {
            extends: "$text.primary",
            size: 14,
            weight: "bold",
          },
          text: {
            extends: "$text.secondary",
            size: 14,
          },
        },
      },
      diagnostic_path_header: {
        background: "$state.active_line",
        text_scale_factor: 0.857,
        filename: {
          extends: "$text.primary",
          size: 14,
        },
        path: {
          extends: "$text.muted",
          size: 14,
          margin: {
            left: 12,
          },
        },
      },
      error_diagnostic: {
        text_scale_factor: 0.857,
        header: {
          border: {
            color: "$border.primary",
            top: true,
            width: 1,
          },
        },
        message: {
          highlight_text: {
            color: "$status.bad",
            extends: "$text.secondary",
            size: 14,
            weight: "bold",
          },
          text: {
            color: "$status.bad",
            extends: "$text.secondary",
            size: 14,
          },
        },
      },
      hint_diagnostic: {
        extends: "$editor.error_diagnostic",
        message: {
          highlight_text: {
            color: "$status.info",
          },
          text: {
            color: "$status.info",
          },
        },
      },
      information_diagnostic: {
        extends: "$editor.error_diagnostic",
        message: {
          highlight_text: {
            color: "$status.info",
          },
          text: {
            color: "$status.info",
          },
        },
      },
      invalid_error_diagnostic: {
        extends: "$editor.error_diagnostic",
        message: {
          highlight_text: {
            color: "$text.muted.color",
          },
          text: {
            color: "$text.muted.color",
          },
        },
      },
      invalid_hint_diagnostic: {
        extends: "$editor.hint_diagnostic",
        message: {
          highlight_text: {
            color: "$text.muted.color",
          },
          text: {
            color: "$text.muted.color",
          },
        },
      },
      invalid_information_diagnostic: {
        extends: "$editor.information_diagnostic",
        message: {
          highlight_text: {
            color: "$text.muted.color",
          },
          text: {
            color: "$text.muted.color",
          },
        },
      },
      invalid_warning_diagnostic: {
        extends: "$editor.warning_diagnostic",
        message: {
          highlight_text: {
            color: "$text.muted.color",
          },
          text: {
            color: "$text.muted.color",
          },
        },
      },
      warning_diagnostic: {
        extends: "$editor.error_diagnostic",
        message: {
          highlight_text: {
            color: "$status.warn",
          },
          text: {
            color: "$status.warn",
          },
        },
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
