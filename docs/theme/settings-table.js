// - status/channel - None (stable) | Preview | Nightly | Experiemental | Unstable
// - namespace?/key?
// - name
// - type (boolean, string, ...)
// - default_value
// - values[] | "See {name} for more &rarr;"
// - short_description
// - long_description? / examples? -> these ones get pulled out into sections

// Every row should have a #peralink
// Create a table from an array of settings

const settings = [
  {
    status: null,
    key: "ui",
    name: "font_weight",
    type: ["string" | "number"],
    default_value: 400,
    values: [100, 200, 300, 400, 500, 600, 700, 800, 900],
    short_description: "The weight of the ui font",
    description: "The weight of the ui font",
  },
  {
    status: null,
    key: "ui",
    name: "font_size",
    type: ["string", "number"],
    default_value: 16,
    values: [12, 14, 16, 18, 20, 24],
    short_description: "The size of the ui font",
    description: "The size of the ui font in pixels",
  },
  {
    status: "Preview",
    key: "ui",
    name: "theme",
    type: "string",
    default_value: "light",
    values: ["light", "dark", "auto"],
    short_description: "The UI theme",
    description: "The overall theme for the user interface",
  },
  {
    status: null,
    key: "editor",
    name: "line_numbers",
    type: "boolean",
    default_value: true,
    values: [true, false],
    short_description: "Show line numbers",
    description: "Whether to display line numbers in the editor",
  },
];

function createSettingsTable(settings) {
  const table = document.createElement("table");
  table.className = "settings-table";

  // Create table header
  const thead = document.createElement("thead");
  const headerRow = document.createElement("tr");
  ["Name", "Type", "Default", "Values", "Description"].forEach((text) => {
    const th = document.createElement("th");
    th.textContent = text;
    headerRow.appendChild(th);
  });
  thead.appendChild(headerRow);
  table.appendChild(thead);

  // Create table body
  const tbody = document.createElement("tbody");
  settings.forEach((setting) => {
    const row = document.createElement("tr");
    row.id = `setting-${setting.key}-${setting.name}`;

    // Name (including key and status if available)
    const nameCell = document.createElement("td");
    let nameText = `${setting.key}.${setting.name}`;
    if (setting.status) {
      nameText += ` - ${setting.status}`;
    }
    nameCell.textContent = nameText;
    row.appendChild(nameCell);

    // Type
    const typeCell = document.createElement("td");
    typeCell.textContent = Array.isArray(setting.type)
      ? setting.type.join(", ")
      : setting.type;
    row.appendChild(typeCell);

    // Default
    const defaultCell = document.createElement("td");
    defaultCell.textContent = setting.default_value;
    row.appendChild(defaultCell);

    // Values
    const valuesCell = document.createElement("td");
    valuesCell.textContent = Array.isArray(setting.values)
      ? setting.values.join(", ")
      : setting.values;
    row.appendChild(valuesCell);

    // Description
    const descCell = document.createElement("td");
    descCell.textContent = setting.short_description;
    row.appendChild(descCell);

    tbody.appendChild(row);
  });
  table.appendChild(tbody);

  return table;
}

// Usage
document.addEventListener("DOMContentLoaded", () => {
  const tableContainer = document.getElementById("settings-table-container");
  if (tableContainer) {
    const table = createSettingsTable(settings);
    tableContainer.appendChild(table);
  }
});
