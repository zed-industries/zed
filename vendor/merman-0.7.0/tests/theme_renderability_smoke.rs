#![cfg(feature = "render")]

use merman::render::HeadlessRenderer;

fn render_svg(name: &str, source: &str) -> String {
    HeadlessRenderer::new()
        .with_vendored_text_measurer()
        .with_diagram_id(name)
        .render_svg_sync(source)
        .unwrap_or_else(|err| panic!("{name}: render failed: {err}"))
        .unwrap_or_else(|| panic!("{name}: no diagram detected"))
}

fn assert_renderable_theme_signals(
    name: &str,
    svg: &str,
    expected_labels: &[&str],
    expected_colors: &[&str],
) {
    assert!(svg.starts_with("<svg"), "{name}: expected SVG output");
    assert!(!svg.contains("NaN"), "{name}: SVG leaked NaN geometry");

    let svg_without_upstream_placeholder_classes = svg
        .replace(r#"class="cluster undefined "#, r#"class="cluster "#)
        .replace(r#"class="node undefined"#, r#"class="node"#)
        .replace(r#"class="node-bkg node-undefined""#, r#"class="node-bkg""#);
    assert!(
        !svg_without_upstream_placeholder_classes.contains("undefined"),
        "{name}: SVG leaked undefined tokens"
    );

    for label in expected_labels {
        assert!(
            svg.contains(label),
            "{name}: expected rendered label {label:?}"
        );
    }

    for color in expected_colors {
        assert!(
            svg.contains(color),
            "{name}: expected visible theme color {color:?}"
        );
    }
}

#[test]
fn representative_dark_theme_diagrams_keep_visible_theme_signals() {
    let cases: &[(&str, &str, &[&str], &[&str])] = &[
        (
            "theme-flowchart",
            r##"%%{init: {"themeVariables": {"mainBkg": "#111827", "primaryTextColor": "#f8fafc", "nodeBorder": "#38bdf8", "lineColor": "#f59e0b", "edgeLabelBackground": "#0f172a", "nodeTextColor": "#f8fafc", "strokeWidth": 4}}}%%
flowchart TD
  A[Dark Node] -->|Readable Edge| B[Other]
"##,
            &["Dark Node", "Readable Edge", "Other"],
            &[
                "#111827",
                "#f8fafc",
                "#38bdf8",
                "#f59e0b",
                r#"#theme-flowchart .edge-thickness-normal{stroke-width:4px;}"#,
                r#"class="edge-thickness-normal edge-pattern-solid edge-thickness-normal edge-pattern-solid flowchart-link""#,
            ],
        ),
        (
            "theme-sequence",
            r##"%%{init: {"themeVariables": {"actorBkg": "#111827", "actorBorder": "#38bdf8", "actorTextColor": "#f8fafc", "signalColor": "#22c55e", "signalTextColor": "#facc15", "noteBkgColor": "#1f2937", "noteBorderColor": "#f97316", "noteTextColor": "#fef3c7"}}}%%
sequenceDiagram
  participant A as Alpha
  participant B as Beta
  A->>B: Request
  Note over A,B: Dark note
"##,
            &["Alpha", "Beta", "Request", "Dark note"],
            &[
                "#111827", "#38bdf8", "#f8fafc", "#22c55e", "#facc15", "#1f2937", "#f97316",
            ],
        ),
        (
            "theme-class",
            r##"%%{init: {"themeVariables": {"classText": "#f8fafc", "mainBkg": "#111827", "nodeBorder": "#38bdf8", "lineColor": "#f59e0b", "noteBkgColor": "#1f2937", "noteBorderColor": "#f97316", "noteTextColor": "#fde68a", "strokeWidth": 4}}}%%
classDiagram
  Animal <|-- Dog
  class Animal {
    +bark()
  }
  note for Animal "Dark note"
"##,
            &["Animal", "Dog", "bark()", "Dark note"],
            &[
                "#f8fafc", "#111827", "#38bdf8", "#f59e0b", "#1f2937", "#f97316", "#fde68a",
            ],
        ),
        (
            "theme-state",
            r##"%%{init: {"themeVariables": {"transitionColor": "#22c55e", "lineColor": "#38bdf8", "stateLabelColor": "#f8fafc", "transitionLabelColor": "#facc15", "stateBkg": "#111827", "stateBorder": "#38bdf8", "specialStateColor": "#f97316", "strokeWidth": 4}}}%%
stateDiagram-v2
  [*] --> Idle: start
  Idle --> Done: finish
"##,
            &["Idle", "Done", "start", "finish"],
            &[
                "#22c55e", "#38bdf8", "#f8fafc", "#facc15", "#111827", "#f97316",
            ],
        ),
        (
            "theme-er",
            r##"%%{init: {"look": "neo", "themeVariables": {"textColor": "#f8fafc", "lineColor": "#22c55e", "mainBkg": "#111827", "nodeBorder": "#38bdf8", "nodeTextColor": "#fde68a", "tertiaryColor": "#172554", "edgeLabelBackground": "#334155", "strokeWidth": 3}}}%%
erDiagram
  CUSTOMER ||--o{ ORDER : places
  CUSTOMER {
    string name
  }
"##,
            &["CUSTOMER", "ORDER", "places", "name"],
            &[
                "#22c55e",
                "#111827",
                "#38bdf8",
                "#fde68a",
                "rgba(23, 37, 84, 0.5)",
                "#334155",
            ],
        ),
        (
            "theme-kanban",
            r##"%%{init: {"themeVariables": {"background": "#0f172a", "nodeBorder": "#38bdf8", "textColor": "#f8fafc", "git0": "#22c55e", "gitBranchLabel0": "#020617", "cScale0": "hsl(160, 80%, 40%)", "cScaleLabel0": "#f8fafc", "cScaleInv0": "#111827"}}}%%
kanban
  todo[Todo]
    card[Dark Card]@{ assigned: "Core", priority: "High" }
"##,
            &["Todo", "Dark Card", "Core"],
            &["#0f172a", "#38bdf8", "#f8fafc", "#22c55e", "#020617"],
        ),
        (
            "theme-mindmap",
            r##"%%{init: {"theme": "redux", "themeVariables": {"THEME_COLOR_LIMIT": 2, "git0": "#22c55e", "nodeBorder": "#facc15", "cScale1": "#172554", "cScaleLabel1": "#f8fafc", "cScaleInv1": "#334155"}}}%%
mindmap
  Root
    Child
"##,
            &["Root", "Child"],
            &["#22c55e", "#facc15", "#172554", "#f8fafc", "#334155"],
        ),
        (
            "theme-gitgraph",
            r##"%%{init: {"themeVariables": {"git0": "#22c55e", "gitBranchLabel0": "#020617", "git1": "#38bdf8", "gitBranchLabel1": "#0f172a", "commitLabelColor": "#f8fafc", "commitLabelBackground": "#111827", "commitLineColor": "#f59e0b"}}}%%
gitGraph
  commit id: "A"
  branch dev
  checkout dev
  commit id: "B"
"##,
            &["main", "dev"],
            &[
                "#22c55e", "#020617", "#38bdf8", "#0f172a", "#f8fafc", "#111827", "#f59e0b",
            ],
        ),
        (
            "theme-c4",
            r##"%%{init: {"c4": {"person_bg_color": "#172554", "person_border_color": "#38bdf8", "system_bg_color": "#111827", "system_border_color": "#facc15"}}}%%
C4Context
title Theme C4
Person(customer, "Customer", "Uses the system")
System(system, "Internet Banking", "Core system")
Rel(customer, system, "Uses", "HTTPS")
"##,
            &[
                "Theme C4",
                "Customer",
                "Internet Banking",
                "Core system",
                "Uses",
            ],
            &["#172554", "#38bdf8", "#111827", "#facc15"],
        ),
        (
            "theme-architecture",
            r##"%%{init: {"themeVariables": {"archEdgeColor": "#22c55e", "archEdgeArrowColor": "#facc15", "archEdgeWidth": 5, "archGroupBorderColor": "#38bdf8", "archGroupBorderWidth": "4px"}}}%%
architecture-beta
  group core(cloud)[Core]
  service api(server)[API] in core
  service db(database)[DB] in core
  api:R --> L:db
"##,
            &["Core", "API", "DB"],
            &[
                "#22c55e",
                "#facc15",
                "#38bdf8",
                "stroke-width:5",
                "stroke-width:4px",
            ],
        ),
        (
            "theme-block",
            r##"%%{init: {"themeVariables": {"nodeTextColor": "#f8fafc", "clusterBkg": "#172554", "clusterBorder": "#38bdf8", "lineColor": "#f59e0b", "strokeWidth": 4}}}%%
block
  block:Core
    A["Alpha"]
    B["Beta"]
  end
  A --> B
"##,
            &["Alpha", "Beta"],
            &[
                "#f8fafc",
                "rgba(23, 37, 84, 0.5)",
                "rgba(56, 189, 248, 0.2)",
                "#f59e0b",
                r#"#theme-block .edge-thickness-normal{stroke-width:4px;}"#,
                r#"class="edge-thickness-normal edge-pattern-solid edge-thickness-normal edge-pattern-solid flowchart-link LS-a1 LE-b1""#,
            ],
        ),
        (
            "theme-journey",
            r##"%%{init: {"themeVariables": {"textColor": "#f8fafc", "faceColor": "#111827", "fillType0": "#172554", "actor0": "#f97316"}}}%%
journey
  title Theme Journey
  section Checkout
    Sign Up: 5: Alice
    Pay: 3: Alice
"##,
            &["Theme Journey", "Checkout", "Sign Up", "Pay", "Alice"],
            &["#f8fafc", "#111827", "#172554", "#f97316"],
        ),
        (
            "theme-quadrantchart",
            r##"%%{init: {"themeVariables": {"quadrant1Fill": "#172554", "quadrant2Fill": "#1e3a8a", "quadrant3Fill": "#0f172a", "quadrant4Fill": "#111827", "quadrant1TextFill": "#f8fafc", "quadrant2TextFill": "#f8fafc", "quadrant3TextFill": "#f8fafc", "quadrant4TextFill": "#f8fafc", "quadrantPointFill": "#facc15", "quadrantPointTextFill": "#111827", "quadrantTitleFill": "#f8fafc", "quadrantXAxisTextFill": "#bfdbfe", "quadrantYAxisTextFill": "#fde68a"}}}%%
quadrantChart
  title Priority Matrix
  x-axis Low Effort --> High Effort
  y-axis Low Impact --> High Impact
  quadrant-1 Invest
  quadrant-2 Watch
  quadrant-3 Park
  quadrant-4 Delegate
  Feature: [0.7, 0.8]
"##,
            &[
                "Priority Matrix",
                "Low Effort",
                "High Impact",
                "Invest",
                "Feature",
            ],
            &[
                "#172554", "#1e3a8a", "#0f172a", "#111827", "#f8fafc", "#facc15", "#bfdbfe",
                "#fde68a",
            ],
        ),
        (
            "theme-packet",
            r##"%%{init: {"packet": {"startByteColor": "#22c55e", "endByteColor": "#f97316", "labelColor": "#f8fafc", "titleColor": "#fde68a", "blockStrokeColor": "#38bdf8", "blockStrokeWidth": 2, "blockFillColor": "#111827"}}}%%
packet
title Theme Packet
+8: "Byte"
+16: "Word"
"##,
            &["Theme Packet", "Byte", "Word"],
            &[
                "#22c55e", "#f97316", "#f8fafc", "#fde68a", "#38bdf8", "#111827",
            ],
        ),
        (
            "theme-sankey",
            r##"%%{init: {"themeVariables": {"textColor": "#f8fafc", "mainBkg": "#111827"}, "sankey": {"labelStyle": "outlined", "nodeColors": {"Source": "#22c55e", "Target": "#38bdf8"}, "showValues": true, "prefix": "$", "suffix": " units"}}}%%
sankey
Source,Target,10
Target,Done,2
"##,
            &["Source", "Target", "Done", "$10 units"],
            &["#f8fafc", "#111827", "#22c55e", "#38bdf8"],
        ),
        (
            "theme-radar",
            r##"%%{init: {"themeVariables": {"titleColor": "#f8fafc", "cScale0": "#22c55e", "radar": {"axisColor": "#38bdf8", "graticuleColor": "#1f2937"}}, "radar": {"axisColor": "#facc15", "axisStrokeWidth": 4, "axisLabelFontSize": 14, "graticuleColor": "#334155", "graticuleOpacity": 0.8, "curveOpacity": 0.9, "curveStrokeWidth": 5}}}%%
radar-beta
  title Theme Radar
  axis Speed, Quality, Cost
  curve Team{8, 7, 4}
"##,
            &["Theme Radar", "Speed", "Quality", "Cost", "Team"],
            &[
                "#f8fafc",
                "#22c55e",
                "#facc15",
                "#334155",
                "stroke-width:4",
                "stroke-width:5",
            ],
        ),
        (
            "theme-requirement",
            r##"%%{init: {"look": "neo", "themeVariables": {"textColor": "#f8fafc", "relationColor": "#22c55e", "edgeLabelBackground": "#0f172a", "nodeBorder": "#f97316", "strokeWidth": 3}}}%%
requirementDiagram
  requirement req1 {
    id: 1
    text: Dark requirement
    risk: high
    verifymethod: analysis
  }
  element sys {
    type: system
  }
  sys - satisfies -> req1
"##,
            &[
                "req1",
                "Dark requirement",
                "Risk: High",
                "Verification: Analysis",
                "sys",
                "satisfies",
            ],
            &["#22c55e", "#0f172a", "#f97316", "stroke-width:3"],
        ),
        (
            "theme-timeline",
            r##"%%{init: {"themeVariables": {"cScale0": "#172554", "cScaleLabel0": "#f8fafc", "cScaleInv0": "#38bdf8"}}}%%
timeline
  title Theme Timeline
  section Release
    2026 : Ship
"##,
            &["Theme Timeline", "Release", "2026", "Ship"],
            &["#172554", "#f8fafc", "#38bdf8"],
        ),
        (
            "theme-gantt",
            r##"%%{init: {"themeVariables": {"textColor": "#f8fafc", "sectionBkgColor": "#172554", "sectionBkgColor2": "#1e3a8a", "titleColor": "#fde68a", "gridColor": "#38bdf8", "taskTextColor": "#f8fafc", "taskBkgColor": "#111827", "taskBorderColor": "#facc15", "taskTextOutsideColor": "#fb923c", "doneTaskBkgColor": "#22c55e", "doneTaskBorderColor": "#16a34a"}}}%%
gantt
  title Theme Plan
  dateFormat YYYY-MM-DD
  section Core
  Build : 2026-01-01, 15d
  Outside Label : 2026-01-16, 1d
  Ship :done, 2026-01-17, 3d
"##,
            &["Theme Plan", "Core", "Build", "Outside Label", "Ship"],
            &[
                "#f8fafc", "#172554", "#1e3a8a", "#fde68a", "#38bdf8", "#111827", "#facc15",
                "#fb923c", "#22c55e", "#16a34a",
            ],
        ),
        (
            "theme-treemap",
            r##"%%{init: {"themeVariables": {"textColor": "#f8fafc", "titleColor": "#fde68a"}, "treemap": {"sectionStrokeColor": "#38bdf8", "sectionFillColor": "#172554", "leafStrokeColor": "#facc15", "leafFillColor": "#111827", "labelColor": "#f8fafc", "valueColor": "#fb923c", "titleColor": "#fde68a"}}}%%
treemap-beta
  "Theme Section"
    "Theme Leaf": 42
"##,
            &["Theme Section", "Theme Leaf", "42"],
            &[
                "#f8fafc", "#fde68a", "#38bdf8", "#172554", "#facc15", "#111827", "#fb923c",
            ],
        ),
        (
            "theme-pie",
            r##"%%{init: {"themeVariables": {"pieTitleTextColor": "#f8fafc", "pieSectionTextColor": "#111827", "pieLegendTextColor": "#fde68a", "pieStrokeColor": "#38bdf8", "pieStrokeWidth": "3px"}}}%%
pie title Theme Pie
  "Alpha": 40
  "Beta": 60
"##,
            &["Theme Pie", "Alpha", "Beta"],
            &["#f8fafc", "#111827", "#fde68a", "#38bdf8", "3px"],
        ),
        (
            "theme-xychart",
            r##"%%{init: {"themeVariables": {"xyChart": {"backgroundColor": "#0f172a", "titleColor": "#f8fafc", "xAxisLabelColor": "#93c5fd", "xAxisTitleColor": "#bfdbfe", "xAxisTickColor": "#38bdf8", "xAxisLineColor": "#60a5fa", "yAxisLabelColor": "#fde68a", "yAxisTitleColor": "#facc15", "yAxisTickColor": "#f97316", "yAxisLineColor": "#fb923c", "plotColorPalette": "#22c55e,#e879f9"}}}}%%
xychart
  title "Theme Chart"
  x-axis Months [Jan, Feb]
  y-axis Revenue 0 --> 10
  bar [3, 7]
  line [3, 7]
"##,
            &["Theme Chart", "Months", "Revenue", "Jan"],
            &[
                "#0f172a", "#f8fafc", "#93c5fd", "#bfdbfe", "#38bdf8", "#60a5fa", "#fde68a",
                "#facc15", "#f97316", "#fb923c", "#22c55e", "#e879f9",
            ],
        ),
        (
            "theme-venn",
            r##"%%{init: {"themeVariables": {"vennTitleTextColor": "#fde68a", "vennSetTextColor": "#f8fafc", "venn1": "#22c55e", "venn2": "#38bdf8"}}}%%
venn-beta
  title "Theme Venn"
  set A["Core"]:10
  set B["Editor"]:8
  union A,B["Shared"]:3
"##,
            &["Theme Venn", "Core", "Editor", "Shared"],
            &["#fde68a", "#f8fafc", "#22c55e", "#38bdf8"],
        ),
        (
            "theme-tree-view",
            r##"%%{init: {"themeVariables": {"treeView": {"labelFontSize": "20px", "labelColor": "#f8fafc", "lineColor": "#38bdf8"}}}}%%
treeView-beta
  "packages"
    "mermaid"
      "src"
    "parser"
"##,
            &["packages", "mermaid", "src", "parser"],
            &["font-size: 20px", "#f8fafc", "#38bdf8"],
        ),
        (
            "theme-ishikawa",
            r##"%%{init: {"fontFamily": "Inter, sans-serif", "themeVariables": {"lineColor": "#38bdf8", "mainBkg": "#111827", "textColor": "#f8fafc"}}}%%
ishikawa-beta
  Blurry Photo
    Process
      Out of focus
    User
      Shaky hands
"##,
            &[
                "Blurry",
                "Photo",
                "Process",
                "Out of focus",
                "User",
                "Shaky hands",
            ],
            &["#38bdf8", "#111827", "#f8fafc", "Inter, sans-serif"],
        ),
        (
            "theme-eventmodeling",
            r##"%%{init: {"themeVariables": {"textColor": "#f8fafc", "emUiFill": "#111827", "emUiStroke": "#475569", "emEventFill": "#f59e0b", "emEventStroke": "#fbbf24", "emSwimlaneBackgroundOdd": "#0f172a", "emSwimlaneBackgroundStroke": "#334155", "emRelationStroke": "#22c55e", "emArrowhead": "#22c55e"}}}%%
eventmodeling
tf 01 ui Shop.Cart
tf 02 cmd Ordering.AddItem ->> 01 { sku: "SKU-1" }
tf 03 evt Cart.ItemAdded ->> 02 [[ItemAddedData]]
rf 04 rmo Read.CartSummary
tf 05 evt Checkout.CheckedOut

data ItemAddedData {
  sku: "SKU-1"
  quantity: 1
}
"##,
            &[
                "UI/A: Shop",
                "C/RM: Ordering",
                "Stream: Cart",
                "Stream: Checkout",
                "Cart",
                "AddItem",
                "ItemAdded",
                "CheckedOut",
            ],
            &[
                "#f8fafc", "#111827", "#475569", "#f59e0b", "#fbbf24", "#0f172a", "#334155",
                "#22c55e",
            ],
        ),
    ];

    for (name, source, expected_labels, expected_colors) in cases {
        let svg = render_svg(name, source);
        assert_renderable_theme_signals(name, &svg, expected_labels, expected_colors);
    }
}

#[test]
fn c4_theme_smoke_counts_inline_config_and_style_macros_as_visible() {
    let svg = render_svg(
        "c4-visible-audit",
        r##"%%{init: {"themeVariables": {"personBkg": "#0ea5e9", "personBorder": "#ec4899"}, "c4": {"person_bg_color": "#172554", "person_border_color": "#38bdf8", "system_bg_color": "#111827", "system_border_color": "#facc15"}}}%%
C4Context
title C4 Visible Audit
Person(customer, "Customer", "Uses the system")
System(system, "System", "Core system")
Rel(customer, system, "Uses", "HTTPS")
UpdateElementStyle(customer, $bgColor="#334155", $fontColor="#fde68a", $borderColor="#f97316")
UpdateRelStyle(customer, system, $textColor="#a7f3d0", $lineColor="#facc15")
"##,
    );

    assert!(
        svg.contains(r#"#c4-visible-audit .person{stroke:#ec4899;fill:#0ea5e9;}"#),
        "Mermaid 11.15 still emits C4 .person provider CSS: {svg}"
    );
    assert!(
        svg.contains(r#"class="person-man""#),
        "C4 current output should expose the current shape group DOM: {svg}"
    );
    assert!(
        !svg.contains(r#"class="person""#),
        "C4 should not count .person provider CSS as visible while current DOM has no .person element: {svg}"
    );
    assert!(
        svg.contains(r##"fill="#334155" stroke="#f97316""##),
        "UpdateElementStyle colors should reach the visible C4 person shape: {svg}"
    );
    assert!(
        svg.contains(r##"dominant-baseline="middle" fill="#fde68a""##),
        "UpdateElementStyle fontColor should reach visible C4 person labels: {svg}"
    );
    assert!(
        svg.contains(r##"fill="#111827" stroke="#facc15""##),
        "C4 config colors should reach the visible system shape: {svg}"
    );
    assert!(
        svg.contains(r##"stroke-width="1" stroke="#facc15""##),
        "UpdateRelStyle lineColor should reach the visible C4 relationship line: {svg}"
    );
    assert!(
        svg.contains(r##"dominant-baseline="middle" fill="#a7f3d0""##),
        "UpdateRelStyle textColor should reach visible C4 relationship labels: {svg}"
    );
}

#[test]
fn packet_and_sankey_theme_smoke_count_dom_consumed_selectors_as_visible() {
    let packet = render_svg(
        "packet-visible-audit",
        r##"%%{init: {"packet": {"startByteColor": "#22c55e", "endByteColor": "#f97316", "labelColor": "#f8fafc", "titleColor": "#fde68a", "blockStrokeColor": "#38bdf8", "blockStrokeWidth": 3, "blockFillColor": "#111827"}}}%%
packet
title Packet Visible Audit
+8: "Byte"
+16: "Word"
"##,
    );

    assert!(
        packet.contains(r#"class="packetBlock""#),
        "Packet block colors should only count with packetBlock DOM: {packet}"
    );
    assert!(
        packet.contains(r#"class="packetLabel""#),
        "Packet label colors should only count with packetLabel DOM: {packet}"
    );
    assert!(
        packet.contains(r#"class="packetByte start""#),
        "Packet start byte colors should only count with packetByte start DOM: {packet}"
    );
    assert!(
        packet.contains(r#"class="packetByte end""#),
        "Packet end byte colors should only count with packetByte end DOM: {packet}"
    );
    assert!(
        packet.contains(r#"class="packetTitle""#),
        "Packet title colors should only count with packetTitle DOM: {packet}"
    );
    assert!(
        packet.contains(r##"#packet-visible-audit .packetByte.start{fill:#22c55e;}"##),
        "Packet startByteColor should reach a current DOM selector: {packet}"
    );
    assert!(
        packet.contains(r##"#packet-visible-audit .packetByte.end{fill:#f97316;}"##),
        "Packet endByteColor should reach a current DOM selector: {packet}"
    );
    assert!(
        packet.contains(r##"#packet-visible-audit .packetLabel{fill:#f8fafc;font-size:12px;}"##),
        "Packet labelColor should reach a current DOM selector: {packet}"
    );
    assert!(
        packet.contains(r##"#packet-visible-audit .packetTitle{fill:#fde68a;font-size:14px;}"##),
        "Packet titleColor should reach a current DOM selector: {packet}"
    );
    assert!(
        packet.contains(
            r##"#packet-visible-audit .packetBlock{stroke:#38bdf8;stroke-width:3;fill:#111827;}"##
        ),
        "Packet block colors should reach a current DOM selector: {packet}"
    );

    let sankey = render_svg(
        "sankey-visible-audit",
        r##"%%{init: {"themeVariables": {"textColor": "#f8fafc", "mainBkg": "#111827"}, "sankey": {"labelStyle": "outlined", "nodeColors": {"Source": "#22c55e", "Target": "#38bdf8", "Done": "#facc15"}, "showValues": true, "prefix": "$", "suffix": " units"}}}%%
sankey
Source,Target,10
Target,Done,2
"##,
    );

    assert!(
        sankey.contains(r#"class="sankey-label-bg""#),
        "Sankey outlined label background should only count with sankey-label-bg DOM: {sankey}"
    );
    assert!(
        sankey.contains(r#"class="sankey-label-fg""#),
        "Sankey label foreground should only count with sankey-label-fg DOM: {sankey}"
    );
    assert!(
        sankey.contains(r#"class="node""#),
        "Sankey node colors should only count with node DOM: {sankey}"
    );
    assert!(
        sankey.contains(r#"class="link""#),
        "Sankey link styling should only count with link DOM: {sankey}"
    );
    assert!(
        sankey.contains(
            r##"#sankey-visible-audit .sankey-label-bg{stroke:#111827;stroke-width:4px;"##
        ),
        "Sankey mainBkg should reach the outlined label background selector: {sankey}"
    );
    assert!(
        sankey.contains(r##"#sankey-visible-audit .sankey-label-fg{fill:#f8fafc;}"##),
        "Sankey textColor should reach the outlined label foreground selector: {sankey}"
    );
    assert!(
        sankey.contains(r##"<rect height=""##) && sankey.contains(r##"fill="#22c55e""##),
        "Sankey nodeColors should reach visible node rect fills: {sankey}"
    );
    assert!(
        sankey.contains(r##"fill="#38bdf8""##) && sankey.contains(r##"fill="#facc15""##),
        "Sankey nodeColors should reach all configured node rect fills: {sankey}"
    );
}

#[test]
fn mindmap_theme_smoke_counts_current_span_and_child_section_dom_as_visible() {
    let svg = render_svg(
        "mindmap-visible-audit",
        r##"%%{init: {"theme": "redux", "themeVariables": {"THEME_COLOR_LIMIT": 2, "git0": "#22c55e", "gitBranchLabel0": "#020617", "nodeBorder": "#facc15", "cScale0": "#ef4444", "cScaleLabel0": "#e879f9", "cScale1": "#172554", "cScaleLabel1": "#f8fafc", "cScaleInv1": "#334155"}}}%%
mindmap
  Root
    Child
"##,
    );

    assert!(
        svg.contains(r#"class="node mindmap-node section-root section--1""#),
        "Mindmap root colors should only count with the current root node DOM: {svg}"
    );
    assert!(
        svg.contains(r#"class="node mindmap-node section-0""#),
        "Mindmap child section colors should only count with current section node DOM: {svg}"
    );
    assert!(
        svg.contains(r##"#mindmap-visible-audit .section-root rect,#mindmap-visible-audit .section-root path,#mindmap-visible-audit .section-root circle,#mindmap-visible-audit .section-root polygon{fill:#22c55e;}"##),
        "Mindmap git0 should reach the current root shape selector: {svg}"
    );
    assert!(
        svg.contains(r##"#mindmap-visible-audit .section-root span{color:#facc15;}"##),
        "Mindmap redux nodeBorder should reach current XHTML root label spans: {svg}"
    );
    assert!(
        svg.contains(r##"#mindmap-visible-audit .section-0 rect,#mindmap-visible-audit .section-0 path,#mindmap-visible-audit .section-0 circle,#mindmap-visible-audit .section-0 polygon,#mindmap-visible-audit .section-0 path{fill:#172554;}"##),
        "Mindmap cScale1 should reach current child section shape selectors: {svg}"
    );
    assert!(
        svg.contains(r##"#mindmap-visible-audit .section-0 span{color:#f8fafc;}"##),
        "Mindmap cScaleLabel1 should reach current child XHTML label spans: {svg}"
    );
    assert!(
        svg.contains(r##"#mindmap-visible-audit .section-0 line{stroke:#334155;stroke-width:3;}"##),
        "Mindmap cScaleInv1 should reach current child divider line DOM: {svg}"
    );

    let root_section_rule = r##"#mindmap-visible-audit .section--1 rect,#mindmap-visible-audit .section--1 path,#mindmap-visible-audit .section--1 circle,#mindmap-visible-audit .section--1 polygon,#mindmap-visible-audit .section--1 path{fill:#ef4444;}"##;
    let root_override_rule = r##"#mindmap-visible-audit .section-root rect,#mindmap-visible-audit .section-root path,#mindmap-visible-audit .section-root circle,#mindmap-visible-audit .section-root polygon{fill:#22c55e;}"##;
    let root_section_pos = svg
        .find(root_section_rule)
        .unwrap_or_else(|| panic!("Mindmap should still emit section--1 provider CSS: {svg}"));
    let root_override_pos = svg
        .find(root_override_rule)
        .unwrap_or_else(|| panic!("Mindmap should still emit section-root override CSS: {svg}"));
    assert!(
        root_section_pos < root_override_pos,
        "Mindmap cScale0 root-section fill is followed by the section-root override, so it should not be counted as the compact sample's visible root fill: {svg}"
    );
    assert!(
        svg.contains(r##"#mindmap-visible-audit .section-root text{fill:#020617;}"##),
        "Mermaid 11.15 still emits root text CSS for gitBranchLabel0: {svg}"
    );
    assert!(
        !svg.contains("<text"),
        "Mindmap should not count section-root text CSS as visible while current labels are XHTML spans: {svg}"
    );
}

#[test]
fn mindmap_neo_theme_smoke_counts_data_look_dom_and_neo_css_as_visible() {
    let svg = render_svg(
        "mindmap-neo-visible-audit",
        r##"%%{init: {"theme": "redux", "look": "neo", "themeVariables": {"THEME_COLOR_LIMIT": 2, "mainBkg": "#111827", "nodeBorder": "#38bdf8", "strokeWidth": 3, "dropShadow": "drop-shadow(1px 2px 2px rgba(0,0,0,.4))", "useGradient": true, "gradientStart": "#112233", "gradientStop": "#445566"}}}%%
mindmap
  Root
    Child
"##,
    );

    assert!(
        svg.contains(
            r#"class="node mindmap-node section-root section--1" id="mindmap-neo-visible-audit-node_0""#
        )
            && svg.contains(r#"data-look="neo""#),
        "Mindmap neo node CSS should only count when current node DOM exposes data-look: {svg}"
    );
    assert!(
        svg.contains(
            r#"class="edge-thickness-normal edge-pattern-solid edge section-edge-0 edge-depth-1""#
        ) && svg.contains(r#"data-look="neo""#),
        "Mindmap neo edge CSS should only count when current edge DOM exposes data-look: {svg}"
    );
    assert!(
        svg.contains(r##"#mindmap-neo-visible-audit [data-look="neo"].mindmap-node.section-0 rect,#mindmap-neo-visible-audit [data-look="neo"].mindmap-node.section-0 path,#mindmap-neo-visible-audit [data-look="neo"].mindmap-node.section-0 circle,#mindmap-neo-visible-audit [data-look="neo"].mindmap-node.section-0 polygon{fill:#111827;stroke:#38bdf8;stroke-width:3px;}"##),
        "Mindmap neo child node shape CSS should reach current data-look node DOM: {svg}"
    );
    assert!(
        svg.contains(
            r##"#mindmap-neo-visible-audit [data-look="neo"].section-edge-0{stroke:#38bdf8;}"##
        ),
        "Mindmap neo edge CSS should reach current data-look edge DOM: {svg}"
    );
    assert!(
        svg.contains(
            r##"#mindmap-neo-visible-audit [data-look="neo"].mindmap-node{filter:drop-shadow(1px 2px 2px rgba(0,0,0,.4));}"##
        ),
        "Mindmap neo drop-shadow CSS should be emitted for current data-look node DOM: {svg}"
    );
    assert!(
        svg.contains(r#"<defs><linearGradient id="mindmap-neo-visible-audit-gradient""#)
            && svg.contains(r##"stop-color="#112233""##)
            && svg.contains(r##"stop-color="#445566""##),
        "Mindmap neo gradient defs should exist when gradient CSS references them: {svg}"
    );
    assert!(
        svg.contains(r##"#mindmap-neo-visible-audit [data-look="neo"].mindmap-node.section-0 rect,#mindmap-neo-visible-audit [data-look="neo"].mindmap-node.section-0 path,#mindmap-neo-visible-audit [data-look="neo"].mindmap-node.section-0 circle,#mindmap-neo-visible-audit [data-look="neo"].mindmap-node.section-0 polygon{stroke:url(#mindmap-neo-visible-audit-gradient);fill:#111827;}"##),
        "Mindmap neo gradient CSS should target current data-look node DOM: {svg}"
    );
}

#[test]
fn er_theme_smoke_counts_current_xhtml_label_and_edge_dom_as_visible() {
    let svg = render_svg(
        "er-visible-audit",
        r##"%%{init: {"look": "neo", "themeVariables": {"textColor": "#f8fafc", "lineColor": "#22c55e", "mainBkg": "#111827", "nodeBorder": "#38bdf8", "nodeTextColor": "#fde68a", "tertiaryColor": "#172554", "edgeLabelBackground": "#334155", "strokeWidth": 3}}}%%
erDiagram
  CUSTOMER ||--o{ ORDER : places
  CUSTOMER {
    string name
  }
"##,
    );

    assert!(
        svg.contains(r#"class="edge-thickness-normal edge-pattern-solid relationshipLine""#),
        "ER line colors should only count with current relationshipLine DOM: {svg}"
    );
    assert!(
        svg.contains(r#"class="labelBkg""#),
        "ER tertiary fade should only count with current labelBkg DOM: {svg}"
    );
    assert!(
        svg.contains(r#"<span class="edgeLabel"><p>places</p></span>"#),
        "ER edgeLabelBackground should only count with current XHTML edge label DOM: {svg}"
    );
    assert!(
        svg.contains(r#"<span class="nodeLabel"><p>CUSTOMER</p></span>"#),
        "ER nodeTextColor should only count with current XHTML node label DOM: {svg}"
    );
    assert!(
        svg.contains(
            r##"#er-visible-audit .relationshipLine{stroke:#22c55e;stroke-width:3;fill:none;}"##
        ),
        "ER lineColor should reach the visible relationshipLine selector: {svg}"
    );
    assert!(
        svg.contains(r##"#er-visible-audit .node rect,#er-visible-audit .node circle,#er-visible-audit .node ellipse,#er-visible-audit .node polygon{fill:#111827;stroke:#38bdf8;stroke-width:3;}"##),
        "ER mainBkg/nodeBorder should reach current simple node shape selectors: {svg}"
    );
    assert!(
        svg.contains(r##"fill="#111827""##) && svg.contains(r##"stroke="#38bdf8""##),
        "ER rough entity shapes should also carry mainBkg/nodeBorder inline colors: {svg}"
    );
    assert!(
        svg.contains(r##"#er-visible-audit .label{font-family:"trebuchet ms",verdana,arial,sans-serif;color:#fde68a;}"##),
        "ER nodeTextColor should reach current XHTML label containers: {svg}"
    );
    assert!(
        svg.contains(r##"#er-visible-audit .labelBkg{background-color:rgba(23, 37, 84, 0.5);}"##),
        "ER tertiaryColor should be counted through the current labelBkg fade, not as a direct fill: {svg}"
    );
    assert!(
        svg.contains(r##"#er-visible-audit .edgeLabel{background-color:#334155;}"##),
        "ER edgeLabelBackground should reach the current XHTML edge label class: {svg}"
    );
    assert!(
        svg.contains(r##"#er-visible-audit .relationshipLabelBox{fill:#172554;opacity:0.7;background-color:#172554;}"##),
        "Mermaid 11.15 still emits relationshipLabelBox provider CSS: {svg}"
    );
    assert!(
        !svg.contains(r#"class="relationshipLabelBox""#),
        "ER should not count direct tertiaryColor relationshipLabelBox CSS as visible without matching DOM: {svg}"
    );
    assert!(
        svg.contains(r##"#er-visible-audit .edgeLabel .label text{fill:#f8fafc;}"##),
        "Mermaid 11.15 still emits edge-label native text CSS: {svg}"
    );
    assert!(
        !svg.contains("<text"),
        "ER should not count textColor native text CSS as visible while current labels are XHTML spans: {svg}"
    );
}

#[test]
fn gantt_theme_smoke_counts_normal_and_done_task_dom_as_visible() {
    let svg = render_svg(
        "gantt-visible-audit",
        r##"%%{init: {"themeVariables": {"textColor": "#f8fafc", "taskTextColor": "#f8fafc", "taskBkgColor": "#111827", "taskBorderColor": "#facc15", "taskTextOutsideColor": "#fb923c", "doneTaskBkgColor": "#22c55e", "doneTaskBorderColor": "#16a34a"}}}%%
gantt
  title Visible Task Audit
  dateFormat YYYY-MM-DD
  section Core
  Build : 2026-01-01, 15d
  Outside Label : 2026-01-16, 1d
  Ship :done, 2026-01-17, 3d
"##,
    );

    assert!(
        svg.contains(r#"class="task task0""#),
        "Gantt taskBkgColor/taskBorderColor should only be counted with normal task DOM: {svg}"
    );
    assert!(
        svg.contains(r#"class="task done0""#),
        "Gantt doneTask* colors should only be counted with done task DOM: {svg}"
    );
    assert!(
        svg.contains(r#"class="taskTextOutsideRight taskTextOutside0"#),
        "Gantt taskTextOutsideColor should only be counted with outside-label DOM: {svg}"
    );
    assert!(
        svg.contains(r#"#gantt-visible-audit .task0,#gantt-visible-audit .task1,#gantt-visible-audit .task2,#gantt-visible-audit .task3{fill:#111827;stroke:#facc15;}"#),
        "normal task colors should reach Gantt task state selectors: {svg}"
    );
    assert!(
        svg.contains(r#"#gantt-visible-audit .done0,#gantt-visible-audit .done1,#gantt-visible-audit .done2,#gantt-visible-audit .done3{stroke:#16a34a;fill:#22c55e;stroke-width:2;}"#),
        "done task colors should reach Gantt done state selectors: {svg}"
    );
    assert!(
        svg.contains(r#"#gantt-visible-audit .taskTextOutside0,#gantt-visible-audit .taskTextOutside2{fill:#fb923c;}"#),
        "outside task text color should still be emitted as a Gantt state selector: {svg}"
    );
}

#[test]
fn gitgraph_official_themes_use_mermaid_11_15_color_generation() {
    let redux = render_svg(
        "gitgraph-redux-visible",
        r##"%%{init: {"theme": "redux"}}%%
gitGraph
  commit id: "A"
  branch dev
  checkout dev
  commit id: "B"
  checkout main
  merge dev
"##,
    );

    assert!(
        redux.contains(r#"#gitgraph-redux-visible .branch-label0{fill:#28253D;font-weight:600;}"#),
        "redux GitGraph should use Mermaid 11.15 nodeBorder branch label rules: {redux}"
    );
    assert!(
        redux.contains(r#"#gitgraph-redux-visible .label0{fill:#ffffff;stroke:#28253D;stroke-width:2;font-weight:600;}"#),
        "redux GitGraph branch label backgrounds should use mainBkg/nodeBorder geometry rules: {redux}"
    );
    assert!(
        redux.contains(r#"#gitgraph-redux-visible .branch{stroke-width:2;stroke:#BDBCCC;stroke-dasharray:4 2;}"#),
        "redux GitGraph branches should use Mermaid 11.15 redux stroke width and dash pattern: {redux}"
    );
    assert!(
        redux.contains(
            r#"#gitgraph-redux-visible .arrow{stroke-width:2;stroke-linecap:round;fill:none;}"#
        ),
        "redux GitGraph arrows should use redux geometry stroke width: {redux}"
    );
    assert!(
        redux.contains(r#"#gitgraph-redux-visible .commit-merge{stroke:#ffffff;fill:#ffffff;}"#),
        "redux GitGraph merge commits should use mainBkg for the inner merge mark: {redux}"
    );

    let neo = render_svg(
        "gitgraph-neo-visible",
        r##"%%{init: {"theme": "neo"}}%%
gitGraph
  commit id: "A"
  branch dev
  checkout dev
  commit id: "B"
"##,
    );

    assert!(
        neo.contains(r#"<defs><linearGradient id="gitgraph-neo-visible-gradient""#),
        "neo GitGraph should emit the gradient defs consumed by branch label backgrounds: {neo}"
    );
    assert!(
        neo.contains(r#"#gitgraph-neo-visible .label0{fill:#ffffff;stroke:url(#gitgraph-neo-visible-gradient);stroke-width:2;}"#),
        "neo GitGraph branch label backgrounds should consume the scoped gradient: {neo}"
    );
    assert!(
        neo.contains(
            r#"#gitgraph-neo-visible .branch{stroke-width:2;stroke:#000000;stroke-dasharray:4 2;}"#
        ),
        "neo GitGraph branches should use Mermaid 11.15 color-generation dash pattern: {neo}"
    );
    assert!(
        neo.contains(
            r#"#gitgraph-neo-visible .arrow{stroke-width:8;stroke-linecap:round;fill:none;}"#
        ),
        "neo GitGraph should keep Mermaid's bold classic arrow width: {neo}"
    );
}

#[test]
fn requirement_default_visible_rough_stroke_uses_node_border() {
    let svg = render_svg(
        "requirement-default-stroke",
        r##"requirementDiagram
  requirement req1 {
    id: 1
    text: Default stroke
    risk: high
    verifymethod: analysis
  }
"##,
    );

    assert!(
        svg.contains(
            r#"#requirement-default-stroke .reqBox{fill:#ECECFF;fill-opacity:1.0;stroke:hsl(240, 60%, 86.2745098039%);"#
        ),
        "Requirement legacy CSS should keep Mermaid's requirementBorderColor rule: {svg}"
    );
    assert!(
        svg.contains(
            r##"stroke="#9370DB" stroke-width="1.3" fill="none" stroke-dasharray="0 0""##,
        ),
        "Requirement visible rough shape/divider strokes should use nodeBorder by default: {svg}"
    );
}

#[test]
fn requirement_theme_smoke_counts_dom_consumed_neo_and_edge_signals() {
    let svg = render_svg(
        "requirement-visible-audit",
        r##"%%{init: {"look": "neo", "themeVariables": {"textColor": "#f8fafc", "relationColor": "#22c55e", "edgeLabelBackground": "#0f172a", "nodeBorder": "#f97316", "requirementBackground": "#111827", "requirementTextColor": "#fde68a", "relationLabelBackground": "#1f2937", "relationLabelColor": "#facc15", "strokeWidth": 3}}}%%
requirementDiagram
  requirement req1 {
    id: 1
    text: Dark requirement
    risk: high
    verifymethod: analysis
  }
  element sys {
    type: system
  }
  sys - satisfies -> req1
"##,
    );

    assert!(
        svg.contains(r#"data-look="neo""#),
        "Requirement nodes and edges should expose the current look for Mermaid 11.15 CSS: {svg}"
    );
    assert!(
        svg.contains(r#"class="basic label-container outer-path""#),
        "Requirement node containers should expose the current neo outer path surface: {svg}"
    );
    assert!(
        svg.contains(r#"#requirement-visible-audit [data-look="neo"].node path{stroke:#f97316;stroke-width:3px;}"#),
        "nodeBorder should reach a selector consumed by current Requirement node DOM: {svg}"
    );
    assert!(
        svg.contains(
            r#"#requirement-visible-audit .relationshipLine{stroke:#22c55e;stroke-width:3;}"#
        ),
        "relationColor should reach the visible Requirement relationship path: {svg}"
    );
    assert!(
        svg.contains(r#"class="labelBkg""#),
        "edgeLabelBackground should have current edge label DOM to style: {svg}"
    );
    assert!(
        svg.contains(r#"#requirement-visible-audit .reqBox{fill:#111827"#),
        "Mermaid 11.15 still emits legacy Requirement provider rules: {svg}"
    );
    assert!(
        !svg.contains(r#"class="reqBox""#),
        "Requirement should not count .reqBox colors as visible while current DOM has no reqBox element: {svg}"
    );
    assert!(
        !svg.contains(r#"class="reqTitle""#),
        "Requirement should not count .reqTitle colors as visible while current DOM has no reqTitle element: {svg}"
    );
    assert!(
        !svg.contains(r#"class="relationshipLabel""#),
        "Requirement should not count .relationshipLabel colors as visible while current edge labels are XHTML spans: {svg}"
    );
}

#[test]
fn journey_theme_smoke_does_not_count_inert_flowchart_rules_as_visible() {
    let svg = render_svg(
        "journey-line-audit",
        r##"%%{init: {"themeVariables": {"textColor": "#f8fafc", "lineColor": "#22c55e", "edgeLabelBackground": "#0f172a", "mainBkg": "#1f2937", "nodeBorder": "#38bdf8", "titleColor": "#fde68a", "arrowheadColor": "#facc15"}}}%%
journey
  title Inert Rule Audit
  section Checkout
    Sign Up: 5: Alice
"##,
    );

    assert!(
        svg.contains(r#"#journey-line-audit line{stroke:#f8fafc;}"#),
        "Journey's current plain line DOM is visibly driven by themeVariables.textColor: {svg}"
    );
    assert!(
        svg.contains(
            r#"stroke-width="4" stroke="black" marker-end="url(#journey-line-audit-arrowhead)""#
        ),
        "Mermaid 11.15 still emits a black presentation attribute on the activity line: {svg}"
    );
    assert!(
        svg.contains(r#"#journey-line-audit .flowchart-link{stroke:#22c55e;fill:none;}"#),
        "Mermaid 11.15 emits this inherited provider rule even though Journey does not render matching DOM: {svg}"
    );
    assert!(
        svg.contains(
            r#"#journey-line-audit .edgeLabel{background-color:#0f172a;text-align:center;}"#
        ),
        "Mermaid 11.15 emits this inherited provider rule even though Journey does not render matching DOM: {svg}"
    );
    assert!(
        !svg.contains(r#"class="flowchart-link""#),
        "Journey should not count .flowchart-link styling as a visible theme signal: {svg}"
    );
    assert!(
        !svg.contains(r#"class="edgeLabel""#),
        "Journey should not count .edgeLabel styling as a visible theme signal: {svg}"
    );
    assert!(
        !svg.contains(r#"class="arrowheadPath""#),
        "Journey's marker path still does not consume Mermaid's .arrowheadPath rule: {svg}"
    );
}

#[test]
fn timeline_theme_smoke_counts_section_dom_not_disabled_css_as_visible() {
    let svg = render_svg(
        "timeline-visible-audit",
        r##"%%{init: {"themeVariables": {"cScale0": "#172554", "cScaleLabel0": "#f8fafc", "cScaleInv0": "#38bdf8", "tertiaryColor": "#334155", "clusterBorder": "#f97316"}}}%%
timeline
  title Visible Rule Audit
  section Release
    2026 : Ship
"##,
    );

    assert!(
        svg.contains(
            r#"#timeline-visible-audit .section--1 rect,#timeline-visible-audit .section--1 path,#timeline-visible-audit .section--1 circle,#timeline-visible-audit .section--1 path{fill:#172554;}"#
        ),
        "Timeline's first visible section should consume cScale0: {svg}"
    );
    assert!(
        svg.contains(r#"#timeline-visible-audit .section--1 text{fill:#f8fafc;}"#),
        "Timeline's first visible section text should consume cScaleLabel0: {svg}"
    );
    assert!(
        svg.contains(r#"#timeline-visible-audit .section--1 line{stroke:#38bdf8;stroke-width:3;}"#),
        "Timeline's first visible section line should consume cScaleInv0: {svg}"
    );
    assert!(
        svg.contains(r#"#timeline-visible-audit .disabled,#timeline-visible-audit .disabled circle,#timeline-visible-audit .disabled text{fill:#334155;}"#),
        "Mermaid 11.15 emits disabled CSS even when this source has no disabled DOM: {svg}"
    );
    assert!(
        svg.contains(r#"#timeline-visible-audit .disabled text{fill:#f97316;}"#),
        "Mermaid 11.15 emits disabled text CSS even when this source has no disabled DOM: {svg}"
    );
    assert!(
        !svg.contains(r#"class="disabled""#),
        "Timeline should not count disabled styling as a visible theme signal without disabled DOM: {svg}"
    );
}

#[test]
fn timeline_redux_theme_smoke_counts_current_node_and_line_dom_as_visible() {
    let svg = render_svg(
        "timeline-redux-visible-audit",
        r##"%%{init: {"theme": "redux", "themeVariables": {"THEME_COLOR_LIMIT": 2, "mainBkg": "#111827", "nodeBorder": "#38bdf8", "strokeWidth": 5, "cScale0": "#ef4444", "cScaleLabel0": "#e879f9", "cScaleInv0": "#334155", "cScale1": "#172554", "cScaleLabel1": "#f8fafc", "cScaleInv1": "#334155"}}}%%
timeline
    section Release
        Plan : Build
        Ship : Done
"##,
    );

    assert!(
        svg.contains(
            r#"#timeline-redux-visible-audit .section--1 rect,#timeline-redux-visible-audit .section--1 path,#timeline-redux-visible-audit .section--1 circle{fill:#111827;stroke:#38bdf8;stroke-width:5;filter:url(#timeline-redux-visible-audit-drop-shadow);}"#
        ),
        "Timeline redux node path CSS should consume mainBkg/nodeBorder/strokeWidth with matching DOM: {svg}"
    );
    assert!(
        svg.contains(
            r#"#timeline-redux-visible-audit .section--1 text{fill:#38bdf8;font-weight:600;}"#
        ),
        "Timeline redux text CSS should consume nodeBorder/fontWeight with matching DOM: {svg}"
    );
    assert!(
        svg.contains(
            r#"#timeline-redux-visible-audit .lineWrapper line{stroke:#38bdf8;stroke-width:5;}"#
        ),
        "Timeline redux lineWrapper CSS should consume nodeBorder/strokeWidth with matching DOM: {svg}"
    );
    assert!(
        svg.contains(r#"class="lineWrapper"><line"#),
        "Timeline redux lineWrapper CSS should only be counted when line DOM exists: {svg}"
    );
    assert!(
        svg.contains(r#"class="timeline-node section--1""#),
        "Timeline redux node CSS should only be counted when current node DOM exists: {svg}"
    );
    assert!(
        !svg.contains(r#"class="node-line--1""#),
        "Timeline redux smoke should not rely on classic node divider DOM: {svg}"
    );
}
