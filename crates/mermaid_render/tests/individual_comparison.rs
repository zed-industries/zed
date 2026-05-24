use gpui::Hsla;
use mermaid_render::MermaidTheme;

fn hex(value: u32) -> Hsla {
    gpui::rgb(value).into()
}
use std::path::PathBuf;

const DIAGRAMS: &[(&str, &str)] = &[
    (
        "flowchart",
        r#"flowchart TD
    A([Customer Places Order]) --> B[Validate Cart]
    B --> C{Items In Stock?}
    C -- No --> D[Notify Customer]
    C -- Yes --> E[Charge Payment]
    E --> F{Payment OK?}
    F -- No --> D
    F -- Yes --> G[Fulfill Order]
    G --> H[Ship Package]
    H --> I([Delivery Complete])"#,
    ),
    (
        "sequence",
        r#"sequenceDiagram
    actor User
    participant Browser
    participant API
    participant DB

    User->>Browser: Enter credentials
    Browser->>API: POST /login
    API->>DB: Lookup user
    DB-->>API: User record
    API->>API: Verify password hash
    alt Valid credentials
        API-->>Browser: 200 OK + JWT
        Browser-->>User: Redirect to dashboard
    else Invalid credentials
        API-->>Browser: 401 Unauthorized
        Browser-->>User: Show error message
    end"#,
    ),
    (
        "er",
        r#"erDiagram
    CUSTOMER {
        int id PK
        string name
        string email
    }
    ORDER {
        int id PK
        int customer_id FK
        date placed_at
    }
    PRODUCT {
        int id PK
        string name
        float price
    }

    CUSTOMER ||--o{ ORDER : places
    ORDER ||--|{ PRODUCT : contains"#,
    ),
    (
        "class",
        r#"classDiagram
    class User {
        +int id
        +string email
        +login() bool
    }
    class Order {
        +int id
        +string status
        +submit() void
    }

    User "1" --> "many" Order : places"#,
    ),
    (
        "pie",
        r#"pie title Website Traffic Sources
    "Organic Search" : 42
    "Direct" : 25
    "Social Media" : 18
    "Referral" : 10
    "Email" : 5"#,
    ),
    (
        "state",
        r#"stateDiagram-v2
    [*] --> Pending
    Pending --> Confirmed : Payment received
    Confirmed --> Shipped : Dispatched
    Shipped --> Delivered : Package received
    Delivered --> [*]
    Pending --> Cancelled : User cancels
    Cancelled --> [*]"#,
    ),
    (
        "gantt",
        r#"gantt
    title Project Timeline
    dateFormat  YYYY-MM-DD
    section Planning
        Requirements   :a1, 2026-05-01, 7d
        Design         :a2, after a1, 5d
    section Development
        Backend        :b1, after a2, 14d
        Frontend       :b2, after a2, 14d"#,
    ),
    (
        "mindmap",
        r#"mindmap
  root((Travel App))
    Search
      Flights
      Hotels
    Bookings
      Manage
      Cancel
    Profile
      Details
      Payments"#,
    ),
];

fn one_dark_theme() -> MermaidTheme {
    let editor_background = hex(0x282c33);
    let surface_background = hex(0x2f343e);
    let text = hex(0xdce0e5);
    let border = hex(0x464b57);
    let border_variant = hex(0x363c46);
    let element_background = hex(0x2e343e);
    let ghost_element_hover = hex(0x363c46);
    let panel_background = hex(0x2f343e);

    let player_cursors = [
        hex(0x74ade8), hex(0xbe5046), hex(0xbf956a), hex(0xb477cf),
        hex(0x6eb4bf), hex(0xd07277), hex(0xdec184), hex(0xa1c181),
    ];

    let git_branch_colors = std::array::from_fn(|i| player_cursors[i % player_cursors.len()]);

    MermaidTheme {
        dark_mode: true,
        font_family: "Zed Plex Sans, system-ui".to_string(),
        background: editor_background,
        primary_color: surface_background,
        primary_text_color: text,
        primary_border_color: border,
        secondary_color: element_background,
        tertiary_color: ghost_element_hover,
        line_color: border,
        text_color: text,
        edge_label_background: editor_background,
        cluster_background: panel_background,
        cluster_border: border_variant,
        note_background: surface_background,
        note_border: border_variant,
        actor_background: element_background,
        actor_border: border,
        activation_background: ghost_element_hover,
        activation_border: border,
        git_branch_colors,
        git_branch_label_colors: git_branch_colors.map(mermaid_render::text_color_for_background),
        er_attr_bg_odd: surface_background,
        er_attr_bg_even: element_background,
        accent_colors: Vec::new(),
    }
}

#[test]
fn generate_individual_svgs() {
    let theme = one_dark_theme();
    let out_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/mermaid_individual");
    std::fs::create_dir_all(&out_dir).expect("failed to create output directory");

    for (name, source) in DIAGRAMS {
        let filename = format!("{name}.svg");
        let path = out_dir.join(&filename);
        match mermaid_render::render_to_svg(source, &theme) {
            Ok(svg) => {
                std::fs::write(&path, &svg).expect("failed to write SVG");
                println!("OK   {filename}");
            }
            Err(err) => {
                println!("FAIL {filename}: {err}");
            }
        }
    }

    let canonical = out_dir.canonicalize().unwrap_or(out_dir);
    println!("\nFiles written to: {}", canonical.display());
}
