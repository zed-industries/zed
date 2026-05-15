use mermaid_render::{MermaidBackend, MermaidTheme};
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
    let editor_background = "#282c33";
    let surface_background = "#2f343e";
    let text = "#dce0e5";
    let border = "#464b57";
    let border_variant = "#363c46";
    let element_background = "#2e343e";
    let ghost_element_hover = "#363c46";
    let panel_background = "#2f343e";

    let player_cursors = [
        "#74ade8", "#be5046", "#bf956a", "#b477cf",
        "#6eb4bf", "#d07277", "#dec184", "#a1c181",
    ];

    MermaidTheme {
        dark_mode: true,
        font_family: "Zed Plex Sans, system-ui".to_string(),
        background: editor_background.to_string(),
        primary_color: surface_background.to_string(),
        primary_text_color: text.to_string(),
        primary_border_color: border.to_string(),
        secondary_color: element_background.to_string(),
        tertiary_color: ghost_element_hover.to_string(),
        line_color: border.to_string(),
        text_color: text.to_string(),
        edge_label_background: editor_background.to_string(),
        cluster_background: panel_background.to_string(),
        cluster_border: border_variant.to_string(),
        note_background: surface_background.to_string(),
        note_border: border_variant.to_string(),
        actor_background: element_background.to_string(),
        actor_border: border.to_string(),
        activation_background: ghost_element_hover.to_string(),
        activation_border: border.to_string(),
        git_branch_colors: std::array::from_fn(|i| player_cursors[i % player_cursors.len()].to_string()),
        git_branch_label_colors: std::array::from_fn(|_| "#fff".to_string()),
        er_attr_bg_odd: surface_background.to_string(),
        er_attr_bg_even: element_background.to_string(),
        accent_colors: Vec::new(),
    }
}

#[test]
fn generate_individual_svgs() {
    let theme = one_dark_theme();
    let out_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/mermaid_individual");
    std::fs::create_dir_all(&out_dir).expect("failed to create output directory");

    let backends = [
        (MermaidBackend::MermaidRs, "mermaid_rs"),
        (MermaidBackend::Merman, "merman"),
    ];

    for (name, source) in DIAGRAMS {
        for (backend, backend_label) in &backends {
            let filename = format!("{name}_{backend_label}.svg");
            let path = out_dir.join(&filename);
            match mermaid_render::render_to_svg(source, &theme, *backend) {
                Ok(svg) => {
                    std::fs::write(&path, &svg).expect("failed to write SVG");
                    println!("OK   {filename}");
                }
                Err(err) => {
                    println!("FAIL {filename}: {err}");
                }
            }
        }
    }

    let canonical = out_dir.canonicalize().unwrap_or(out_dir);
    println!("\nFiles written to: {}", canonical.display());
}
