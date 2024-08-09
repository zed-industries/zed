// - status/channel - None (stable) | Preview | Nightly | Experiemental | Unstable
// - namespace?/key?
// - name
// - type (boolean, string, ...)
// - default_value
// - values[] | "See {name} for more &rarr;"
// - short_description
// - long_description? / examples? -> these ones get pulled out into sections

// Every row should have a #permalink
// Create a table from an array of settings

const settings = [
  {
    status: "Preview",
    key: "ui",
    name: "font_weight",
    type: "string",
    default_value: 400,
    values: [100, 200, 300, 400, 500, 600, 700, 800, 900],
    short_description: "The weight of the ui font",
    description: "The weight of the ui font",
  },
  {
    status: "Nightly",
    key: "ui",
    name: "font_weight",
    type: "number",
    default_value: 400,
    values: [100, 200, 300, 400, 500, 600, 700, 800, 900],
    short_description: "The weight of the ui font",
    description: "The weight of the ui font",
  },
];

const render_tag = (text, variant = "default") => {
  let classes = "tag";

  if (variant !== "default") {
    switch (variant) {
      case "info":
        classes += "tag-info;";
        break;
      case "warning":
        classes += "tag-warning;";
        break;
      case "error":
        classes += "tag-error";
        break;
      default:
        classes;
    }
  }

  return `<span class="${classes}">${text}</span>`;
};

function createSettingsTable(settings) {
  const container = document.createElement("div");
  container.className = "settings-list";

  settings.forEach((setting) => {
    const settingContainer = document.createElement("div");
    settingContainer.className = "setting-container";
    settingContainer.id = `setting-${setting.key}-${setting.name}`;

    // Header
    const header = document.createElement("div");
    header.className = "setting-header";

    const nameContainer = document.createElement("div");
    nameContainer.className = "setting-name";
    let nameText = `${setting.key}.${setting.name}`;
    nameContainer.textContent = nameText;

    const typeContainer = document.createElement("div");
    typeContainer.className = "setting-type";
    typeContainer.textContent = Array.isArray(setting.type)
      ? setting.type.join(", ")
      : setting.type;

    header.appendChild(nameContainer);
    header.appendChild(typeContainer);

    if (setting.status) {
      const statusContainer = document.createElement("div");
      statusContainer.className = "setting-status";
      statusContainer.innerHTML = render_tag(setting.status, "info");
      header.appendChild(statusContainer);
    }

    // Details table
    const detailsTable = document.createElement("table");
    detailsTable.className = "setting-details";

    const rows = [
      ["Default", setting.default_value],
      [
        "Values",
        Array.isArray(setting.values)
          ? setting.values.join(", ")
          : setting.values,
      ],
      ["Description", setting.short_description],
    ];

    rows.forEach(([label, value]) => {
      const row = detailsTable.insertRow();
      const labelCell = row.insertCell();
      labelCell.textContent = label;
      labelCell.className = "setting-label";
      const valueCell = row.insertCell();
      valueCell.textContent = value;
      valueCell.className = "setting-value";
    });

    settingContainer.appendChild(header);
    settingContainer.appendChild(detailsTable);
    container.appendChild(settingContainer);
  });

  return container;
}

// Usage
document.addEventListener("DOMContentLoaded", () => {
  const settingsContainer = document.getElementById("settings-container");
  if (settingsContainer) {
    const settingsList = createSettingsTable(settings);
    settingsContainer.appendChild(settingsList);
  }
});
