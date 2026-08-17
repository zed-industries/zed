use crate::*;
use futures::executor::block_on;
use serde_json::json;

#[test]
fn parse_diagram_sequence_basic_messages_and_notes() {
    let engine = Engine::new();
    let text = r#"sequenceDiagram
Alice->Bob:Hello Bob, how are you?
Note right of Bob: Bob thinks
Bob-->Alice: I am good thanks!"#;

    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.meta.diagram_type, "sequence");

    let msgs = res.model["messages"].as_array().unwrap();
    assert_eq!(msgs.len(), 3);
    assert_eq!(msgs[0]["from"], json!("Alice"));
    assert_eq!(msgs[0]["to"], json!("Bob"));
    assert_eq!(msgs[0]["message"], json!("Hello Bob, how are you?"));
    assert_eq!(msgs[0]["type"], json!(5));
    assert_eq!(msgs[0]["wrap"], json!(false));

    assert_eq!(msgs[1]["type"], json!(2));
    assert_eq!(msgs[1]["placement"], json!(1));
    assert_eq!(msgs[1]["from"], json!("Bob"));
    assert_eq!(msgs[1]["to"], json!("Bob"));
    assert_eq!(msgs[1]["message"], json!("Bob thinks"));

    assert_eq!(msgs[2]["from"], json!("Bob"));
    assert_eq!(msgs[2]["to"], json!("Alice"));
    assert_eq!(msgs[2]["message"], json!("I am good thanks!"));
    assert_eq!(msgs[2]["type"], json!(6));
}

#[test]
fn parse_diagram_sequence_central_connections_use_upstream_message_model() {
    let engine = Engine::new();
    let text = r#"sequenceDiagram
participant Alice
participant John
Alice->>()John: Hello John
Alice()->>John: How are you?
John()->>()Alice: Great!"#;

    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();

    assert_eq!(res.model["actorOrder"], json!(["Alice", "John"]));
    let actors = res.model["actors"].as_object().unwrap();
    assert!(actors.get("Alice").is_some());
    assert!(actors.get("John").is_some());
    assert!(actors.get("Alice()").is_none());
    assert!(actors.get("()John").is_none());

    let msgs = res.model["messages"].as_array().unwrap();
    assert_eq!(msgs.len(), 7);
    assert_eq!(msgs[0]["id"], json!("0"));
    assert_eq!(msgs[0]["from"], json!("Alice"));
    assert_eq!(msgs[0]["to"], json!("John"));
    assert_eq!(msgs[0]["centralConnection"], json!(59));
    assert_eq!(msgs[0]["activate"], json!(true));
    assert_eq!(msgs[1]["type"], json!(59));
    assert_eq!(msgs[2]["id"], json!("2"));
    assert_eq!(msgs[2]["from"], json!("Alice"));
    assert_eq!(msgs[2]["to"], json!("John"));
    assert_eq!(msgs[2]["centralConnection"], json!(60));
    assert_eq!(msgs[3]["type"], json!(60));
    assert_eq!(msgs[4]["id"], json!("4"));
    assert_eq!(msgs[4]["from"], json!("John"));
    assert_eq!(msgs[4]["to"], json!("Alice"));
    assert_eq!(msgs[4]["centralConnection"], json!(61));
    assert_eq!(msgs[4]["activate"], json!(true));
    assert_eq!(msgs[5]["type"], json!(59));
    assert_eq!(msgs[6]["type"], json!(60));
}

#[test]
fn parse_diagram_sequence_autonumber_allows_decimal_start_and_step() {
    let engine = Engine::new();
    let text = r#"sequenceDiagram
autonumber 10.1 .01
Alice->>Bob:Hello
Bob-->>Alice:Back"#;

    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();

    let msgs = res.model["messages"].as_array().unwrap();
    assert_eq!(msgs[0]["type"], json!(26));
    assert_eq!(msgs[0]["message"]["start"].as_f64(), Some(10.1));
    assert_eq!(msgs[0]["message"]["step"].as_f64(), Some(0.01));
    assert_eq!(msgs[0]["message"]["visible"], json!(true));
    assert_eq!(msgs[1]["message"], json!("Hello"));
    assert_eq!(msgs[2]["message"], json!("Back"));
}

#[test]
fn parse_diagram_sequence_autonumber_rejects_thousandths() {
    let engine = Engine::new();
    let text = r#"sequenceDiagram
autonumber 10.001
Alice->>Bob:Hello"#;

    let res = block_on(engine.parse_diagram(text, ParseOptions::default()));
    assert!(
        res.is_err(),
        "expected Mermaid 11.15-compatible parse failure for thousandths"
    );
}

#[test]
fn parse_diagram_sequence_is_stateless_across_multiple_parses() {
    let engine = Engine::new();
    let first = r#"sequenceDiagram
Alice->Bob:Hello Bob, how are you?
Bob-->Alice:I am good thanks!"#;
    let second = r#"sequenceDiagram
Alice->John:Hello John, how are you?
John-->Alice:I am good thanks!"#;

    let a = block_on(engine.parse_diagram(first, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let b = block_on(engine.parse_diagram(second, ParseOptions::default()))
        .unwrap()
        .unwrap();

    let a_msgs = a.model["messages"].as_array().unwrap();
    let b_msgs = b.model["messages"].as_array().unwrap();

    assert_eq!(a_msgs.len(), 2);
    assert_eq!(a_msgs[0]["id"], json!("0"));
    assert_eq!(a_msgs[1]["id"], json!("1"));
    assert_eq!(a_msgs[0]["from"], json!("Alice"));
    assert_eq!(a_msgs[0]["to"], json!("Bob"));
    assert_eq!(a_msgs[1]["from"], json!("Bob"));
    assert_eq!(a_msgs[1]["to"], json!("Alice"));

    assert_eq!(b_msgs.len(), 2);
    assert_eq!(b_msgs[0]["id"], json!("0"));
    assert_eq!(b_msgs[1]["id"], json!("1"));
    assert_eq!(b_msgs[0]["from"], json!("Alice"));
    assert_eq!(b_msgs[0]["to"], json!("John"));
    assert_eq!(b_msgs[1]["from"], json!("John"));
    assert_eq!(b_msgs[1]["to"], json!("Alice"));
}

#[test]
fn parse_diagram_sequence_title_and_accessibility_fields() {
    let engine = Engine::new();
    let text = r#"sequenceDiagram
title: Diagram Title
accTitle: Accessible Title
accDescr: Accessible Description
Alice->Bob:Hello"#;

    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();

    assert_eq!(res.model["title"], json!("Diagram Title"));
    assert_eq!(res.model["accTitle"], json!("Accessible Title"));
    assert_eq!(res.model["accDescr"], json!("Accessible Description"));
}

#[test]
fn parse_diagram_sequence_wrap_directive_controls_default_wrap() {
    let engine = Engine::new();
    let text = r#"%%{wrap}%%
sequenceDiagram
Alice->Bob:Hello
Alice->Bob:nowrap:Hello again"#;

    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let msgs = res.model["messages"].as_array().unwrap();

    assert_eq!(msgs[0]["wrap"], json!(true));
    assert_eq!(msgs[1]["wrap"], json!(false));
    assert_eq!(msgs[1]["message"], json!("Hello again"));
}

#[test]
fn parse_diagram_sequence_links() {
    let engine = Engine::new();
    let text = r#"sequenceDiagram
participant a as Alice
participant b as Bob
participant c as Charlie
links a: { "Repo": "https://repo.contoso.com/", "Dashboard": "https://dashboard.contoso.com/" }
links b: { "Dashboard": "https://dashboard.contoso.com/" }
links a: { "On-Call": "https://oncall.contoso.com/?svc=alice" }
link a: Endpoint @ https://alice.contoso.com
link a: Swagger @ https://swagger.contoso.com
link a: Tests @ https://tests.contoso.com/?svc=alice@contoso.com
"#;

    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let actors = res.model["actors"].as_object().unwrap();
    assert_eq!(
        actors["a"]["links"]["Repo"],
        json!("https://repo.contoso.com/")
    );
    assert_eq!(actors["b"]["links"].get("Repo"), None);
    assert_eq!(
        actors["a"]["links"]["Dashboard"],
        json!("https://dashboard.contoso.com/")
    );
    assert_eq!(
        actors["b"]["links"]["Dashboard"],
        json!("https://dashboard.contoso.com/")
    );
    assert_eq!(
        actors["a"]["links"]["On-Call"],
        json!("https://oncall.contoso.com/?svc=alice")
    );
    assert_eq!(actors["c"]["links"].get("Dashboard"), None);
    assert_eq!(
        actors["a"]["links"]["Endpoint"],
        json!("https://alice.contoso.com")
    );
    assert_eq!(
        actors["a"]["links"]["Swagger"],
        json!("https://swagger.contoso.com")
    );
    assert_eq!(
        actors["a"]["links"]["Tests"],
        json!("https://tests.contoso.com/?svc=alice@contoso.com")
    );
}

#[test]
fn parse_diagram_sequence_allows_keyword_like_actor_ids() {
    let engine = Engine::new();
    let text = r#"sequenceDiagram
participant AS as AppService
participant DB as Store
participant END as End Service
participant loop as Loop Service
participant RECT as Rectangle Worker
AS->>DB: get recorded file timestamps
END->>RECT: uppercase keyword id can send
loop->>AS: lowercase keyword id can send when followed by a signal"#;

    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();

    let actors = res.model["actors"].as_object().unwrap();
    assert_eq!(actors["AS"]["description"], json!("AppService"));
    assert_eq!(actors["DB"]["description"], json!("Store"));
    assert_eq!(actors["END"]["description"], json!("End Service"));
    assert_eq!(actors["loop"]["description"], json!("Loop Service"));
    assert_eq!(actors["RECT"]["description"], json!("Rectangle Worker"));

    let msgs = res.model["messages"].as_array().unwrap();
    assert_eq!(msgs[0]["from"], json!("AS"));
    assert_eq!(msgs[0]["to"], json!("DB"));
    assert_eq!(msgs[1]["from"], json!("END"));
    assert_eq!(msgs[1]["to"], json!("RECT"));
    assert_eq!(msgs[2]["from"], json!("loop"));
    assert_eq!(msgs[2]["to"], json!("AS"));
}

#[test]
fn parse_diagram_sequence_properties() {
    let engine = Engine::new();
    let text = r#"sequenceDiagram
participant a as Alice
participant b as Bob
participant c as Charlie
properties a: {"class": "internal-service-actor", "icon": "@clock"}
properties b: {"class": "external-service-actor", "icon": "@computer"}
"#;

    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let actors = res.model["actors"].as_object().unwrap();
    assert_eq!(
        actors["a"]["properties"]["class"],
        json!("internal-service-actor")
    );
    assert_eq!(
        actors["b"]["properties"]["class"],
        json!("external-service-actor")
    );
    assert_eq!(actors["a"]["properties"]["icon"], json!("@clock"));
    assert_eq!(actors["b"]["properties"]["icon"], json!("@computer"));
    assert_eq!(actors["c"]["properties"].get("class"), None);
}

#[test]
fn parse_diagram_sequence_box_color_and_membership() {
    let engine = Engine::new();
    let text = r#"sequenceDiagram
box green Group 1
participant a as Alice
participant b as Bob
end
participant c as Charlie
links a: { "Repo": "https://repo.contoso.com/", "Dashboard": "https://dashboard.contoso.com/" }
links b: { "Dashboard": "https://dashboard.contoso.com/" }
links a: { "On-Call": "https://oncall.contoso.com/?svc=alice" }
link a: Endpoint @ https://alice.contoso.com
link a: Swagger @ https://swagger.contoso.com
link a: Tests @ https://tests.contoso.com/?svc=alice@contoso.com
"#;

    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let boxes = res.model["boxes"].as_array().unwrap();
    assert_eq!(boxes[0]["name"], json!("Group 1"));
    assert_eq!(boxes[0]["actorKeys"], json!(["a", "b"]));
    assert_eq!(boxes[0]["fill"], json!("green"));
}

#[test]
fn parse_diagram_sequence_box_without_color_defaults_to_transparent() {
    let engine = Engine::new();
    let text = r#"sequenceDiagram
box Group 1
participant a as Alice
participant b as Bob
end
"#;

    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let boxes = res.model["boxes"].as_array().unwrap();
    assert_eq!(boxes[0]["name"], json!("Group 1"));
    assert_eq!(boxes[0]["actorKeys"], json!(["a", "b"]));
    assert_eq!(boxes[0]["fill"], json!("transparent"));
}

#[test]
fn parse_diagram_sequence_box_without_description_has_falsy_name() {
    let engine = Engine::new();
    let text = r#"sequenceDiagram
box aqua
participant a as Alice
participant b as Bob
end
"#;

    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let boxes = res.model["boxes"].as_array().unwrap();
    assert!(boxes[0]["name"].is_null());
    assert_eq!(boxes[0]["actorKeys"], json!(["a", "b"]));
    assert_eq!(boxes[0]["fill"], json!("aqua"));
}

#[test]
fn parse_diagram_sequence_box_rgb_color() {
    let engine = Engine::new();
    let text = r#"sequenceDiagram
box rgb(34, 56, 0) Group1
participant a as Alice
participant b as Bob
end
"#;

    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let boxes = res.model["boxes"].as_array().unwrap();
    assert_eq!(boxes[0]["name"], json!("Group1"));
    assert_eq!(boxes[0]["fill"], json!("rgb(34, 56, 0)"));
    assert_eq!(boxes[0]["actorKeys"], json!(["a", "b"]));
}

#[test]
fn parse_diagram_sequence_create_participant_and_actor() {
    let engine = Engine::new();
    let text = r#"sequenceDiagram
participant a as Alice
a ->>b: Hello Bob?
create participant c
b-->>c: Hello c!
c ->> b: Hello b?
create actor d as Donald
a ->> d: Hello Donald?
"#;

    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let actors = res.model["actors"].as_object().unwrap();
    let created = res.model["createdActors"].as_object().unwrap();

    assert_eq!(actors["c"]["name"], json!("c"));
    assert_eq!(actors["c"]["description"], json!("c"));
    assert_eq!(actors["c"]["type"], json!("participant"));
    assert_eq!(created["c"], json!(1));

    assert_eq!(actors["d"]["name"], json!("d"));
    assert_eq!(actors["d"]["description"], json!("Donald"));
    assert_eq!(actors["d"]["type"], json!("actor"));
    assert_eq!(created["d"], json!(3));
}

#[test]
fn parse_diagram_sequence_destroy_participant_marks_destroyed_actor_index() {
    let engine = Engine::new();
    let text = r#"sequenceDiagram
participant a as Alice
a ->>b: Hello Bob?
destroy a
b-->>a: Hello Alice!
b ->> c: Where is Alice?
destroy c
b ->> c: Where are you?
"#;

    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let destroyed = res.model["destroyedActors"].as_object().unwrap();
    assert_eq!(destroyed["a"], json!(1));
    assert_eq!(destroyed["c"], json!(3));
}

#[test]
fn parse_diagram_sequence_create_and_destroy_same_actor() {
    let engine = Engine::new();
    let text = r#"sequenceDiagram
a ->>b: Hello Bob?
create participant c
b ->>c: Hello c!
c ->> b: Hello b?
destroy c
b ->> c : Bye c !
"#;

    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let created = res.model["createdActors"].as_object().unwrap();
    let destroyed = res.model["destroyedActors"].as_object().unwrap();
    assert_eq!(created["c"], json!(1));
    assert_eq!(destroyed["c"], json!(3));
}

#[test]
fn parse_diagram_sequence_extended_participant_syntax_parses_type_override() {
    let engine = Engine::new();
    let text = r#"sequenceDiagram
participant Alice@{ "type" : "database" }
participant Bob@{ "type" : "database" }
participant Carl@{ type: "database" }
participant David@{ "type" : 'database' }
participant Eve@{ type: 'database' }
participant Favela@{ "type" : "database"    }
Bob->>+Alice: Hi Alice
Alice->>+Bob: Hi Bob
"#;

    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let actors = res.model["actors"].as_object().unwrap();

    for id in ["Alice", "Bob", "Carl", "David", "Eve", "Favela"] {
        assert_eq!(actors[id]["type"], json!("database"));
        assert_eq!(actors[id]["description"], json!(id));
    }
}

#[test]
fn parse_diagram_sequence_extended_participant_syntax_mixed_types_and_implicit_participants() {
    let engine = Engine::new();
    let text = r#"sequenceDiagram
participant lead
participant dsa@{ "type" : "queue" }
API->>+Database: getUserb
Database-->>-API: userb
dsa --> Database: hello
"#;

    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let actors = res.model["actors"].as_object().unwrap();

    assert_eq!(actors["lead"]["type"], json!("participant"));
    assert_eq!(actors["lead"]["description"], json!("lead"));
    assert_eq!(actors["dsa"]["type"], json!("queue"));
    assert_eq!(actors["dsa"]["description"], json!("dsa"));

    assert_eq!(actors["API"]["type"], json!("participant"));
    assert_eq!(actors["Database"]["type"], json!("participant"));
}

#[test]
fn parse_diagram_sequence_extended_participant_syntax_invalid_config_fails() {
    let engine = Engine::new();
    let bad_json = r#"sequenceDiagram
participant D@{ "type: "entity" }
participant E@{ "type": "dat
abase }
"#;
    assert!(block_on(engine.parse_diagram(bad_json, ParseOptions::default())).is_err());

    let missing_colon = r#"sequenceDiagram
participant C@{ "type" "control" }
C ->> C: action
"#;
    assert!(block_on(engine.parse_diagram(missing_colon, ParseOptions::default())).is_err());

    let missing_brace = r#"sequenceDiagram
participant E@{ "type": "entity"
E ->> E: process
"#;
    assert!(block_on(engine.parse_diagram(missing_brace, ParseOptions::default())).is_err());
}

#[test]
fn parse_diagram_sequence_deactivate_inactive_participant_fails_like_upstream() {
    let engine = Engine::new();
    let text = r#"sequenceDiagram
participant user as End User
participant Server as Server
participant System as System
participant System2 as System2

user->>+Server: Test
user->>+Server: Test2
user->>System: Test
Server->>-user: Test
Server->>-user: Test2

%% The following deactivation of Server will fail
Server->>-user: Test3"#;

    let err = block_on(engine.parse_diagram(text, ParseOptions::default())).unwrap_err();
    assert!(
        err.to_string()
            .contains("Trying to inactivate an inactive participant (Server)"),
        "unexpected error: {err}"
    );
}

#[test]
fn parse_diagram_sequence_alt_multiple_elses_inserts_control_messages() {
    let engine = Engine::new();
    let text = r#"sequenceDiagram
Alice->Bob: Hello Bob, how are you?

%% Comment
Note right of Bob: Bob thinks
alt isWell

Bob-->Alice: I am good thanks!
else isSick
Bob-->Alice: Feel sick...
else default
Bob-->Alice: :-)
end"#;

    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let messages = res.model["messages"].as_array().unwrap();

    assert_eq!(messages.len(), 9);
    assert_eq!(messages[1]["from"], json!("Bob"));
    assert_eq!(messages[2]["type"], json!(12));
    assert_eq!(messages[3]["from"], json!("Bob"));
    assert_eq!(messages[4]["type"], json!(13));
    assert_eq!(messages[5]["from"], json!("Bob"));
    assert_eq!(messages[6]["type"], json!(13));
    assert_eq!(messages[7]["from"], json!("Bob"));
    assert_eq!(messages[8]["type"], json!(14));
}

#[test]
fn parse_diagram_sequence_critical_without_options() {
    let engine = Engine::new();
    let text = r#"sequenceDiagram
critical Establish a connection to the DB
Service-->DB: connect
end"#;

    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let messages = res.model["messages"].as_array().unwrap();

    assert_eq!(messages.len(), 3);
    assert_eq!(messages[0]["type"], json!(27));
    assert_eq!(messages[1]["from"], json!("Service"));
    assert_eq!(messages[2]["type"], json!(29));
}

#[test]
fn parse_diagram_sequence_critical_with_options() {
    let engine = Engine::new();
    let text = r#"sequenceDiagram
critical Establish a connection to the DB
Service-->DB: connect
option Network timeout
Service-->Service: Log error
option Credentials rejected
Service-->Service: Log different error
end"#;

    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let messages = res.model["messages"].as_array().unwrap();

    assert_eq!(messages.len(), 7);
    assert_eq!(messages[0]["type"], json!(27));
    assert_eq!(messages[1]["from"], json!("Service"));
    assert_eq!(messages[2]["type"], json!(28));
    assert_eq!(messages[3]["from"], json!("Service"));
    assert_eq!(messages[4]["type"], json!(28));
    assert_eq!(messages[5]["from"], json!("Service"));
    assert_eq!(messages[6]["type"], json!(29));
}

#[test]
fn parse_diagram_sequence_break_block_inserts_control_messages() {
    let engine = Engine::new();
    let text = r#"sequenceDiagram
Consumer-->API: Book something
API-->BookingService: Start booking process
break when the booking process fails
API-->Consumer: show failure
end
API-->BillingService: Start billing process"#;

    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let messages = res.model["messages"].as_array().unwrap();

    assert_eq!(messages.len(), 6);
    assert_eq!(messages[0]["from"], json!("Consumer"));
    assert_eq!(messages[1]["from"], json!("API"));
    assert_eq!(messages[2]["type"], json!(30));
    assert_eq!(messages[3]["from"], json!("API"));
    assert_eq!(messages[4]["type"], json!(31));
    assert_eq!(messages[5]["from"], json!("API"));
}

#[test]
fn parse_diagram_sequence_par_over_block() {
    let engine = Engine::new();
    let text = r#"sequenceDiagram
par_over Parallel overlap
Alice ->> Bob: Message
Note left of Alice: Alice note
Note right of Bob: Bob note
end"#;

    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let messages = res.model["messages"].as_array().unwrap();

    assert_eq!(messages.len(), 5);
    assert_eq!(messages[0]["type"], json!(32));
    assert_eq!(messages[0]["message"], json!("Parallel overlap"));
    assert_eq!(messages[1]["from"], json!("Alice"));
    assert_eq!(messages[2]["from"], json!("Alice"));
    assert_eq!(messages[3]["from"], json!("Bob"));
    assert_eq!(messages[4]["type"], json!(21));
}

#[test]
fn parse_diagram_sequence_special_characters_in_loop_opt_alt_par() {
    let engine = Engine::new();

    let res = block_on(engine.parse_diagram(
        r#"sequenceDiagram
Alice->Bob: Hello Bob, how are you?
loop -:<>,;# comment
Bob-->Alice: I am good thanks!
end"#,
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();
    let messages = res.model["messages"].as_array().unwrap();
    assert_eq!(messages[1]["message"], json!("-:<>,"));

    let res = block_on(engine.parse_diagram(
        r#"sequenceDiagram
Alice->Bob: Hello Bob, how are you?
opt -:<>,;# comment
Bob-->Alice: I am good thanks!
end"#,
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();
    let messages = res.model["messages"].as_array().unwrap();
    assert_eq!(messages[1]["message"], json!("-:<>,"));

    let res = block_on(engine.parse_diagram(
        r#"sequenceDiagram
Alice->Bob: Hello Bob, how are you?
alt -:<>,;# comment
Bob-->Alice: I am good thanks!
else ,<>:-#; comment
Bob-->Alice: I am good thanks!
end"#,
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();
    let messages = res.model["messages"].as_array().unwrap();
    assert_eq!(messages[1]["message"], json!("-:<>,"));
    assert_eq!(messages[3]["message"], json!(",<>:-"));

    let res = block_on(engine.parse_diagram(
        r#"sequenceDiagram
Alice->Bob: Hello Bob, how are you?
par -:<>,;# comment
Bob-->Alice: I am good thanks!
and ,<>:-#; comment
Bob-->Alice: I am good thanks!
end"#,
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();
    let messages = res.model["messages"].as_array().unwrap();
    assert_eq!(messages[1]["message"], json!("-:<>,"));
    assert_eq!(messages[3]["message"], json!(",<>:-"));
}

#[test]
fn parse_diagram_sequence_no_label_loop_opt_alt_par() {
    let engine = Engine::new();

    let res = block_on(engine.parse_diagram(
        r#"sequenceDiagram
Alice->Bob: Hello Bob, how are you?
loop
Bob-->Alice: I am good thanks!
end"#,
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();
    let messages = res.model["messages"].as_array().unwrap();
    assert_eq!(messages[1]["message"], json!(""));
    assert_eq!(messages[2]["message"], json!("I am good thanks!"));

    let res = block_on(engine.parse_diagram(
        r#"sequenceDiagram
Alice->Bob: Hello Bob, how are you?
opt # comment
Bob-->Alice: I am good thanks!
end"#,
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();
    let messages = res.model["messages"].as_array().unwrap();
    assert_eq!(messages[1]["message"], json!(""));
    assert_eq!(messages[2]["message"], json!("I am good thanks!"));

    let res = block_on(engine.parse_diagram(
        r#"sequenceDiagram
Alice->Bob: Hello Bob, how are you?
alt;Bob-->Alice: I am good thanks!
else # comment
Bob-->Alice: I am good thanks!
end"#,
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();
    let messages = res.model["messages"].as_array().unwrap();
    assert_eq!(messages[1]["message"], json!(""));
    assert_eq!(messages[2]["message"], json!("I am good thanks!"));
    assert_eq!(messages[3]["message"], json!(""));
    assert_eq!(messages[4]["message"], json!("I am good thanks!"));

    let res = block_on(engine.parse_diagram(
        r#"sequenceDiagram
Alice->Bob: Hello Bob, how are you?
par;Bob-->Alice: I am good thanks!
and # comment
Bob-->Alice: I am good thanks!
end"#,
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();
    let messages = res.model["messages"].as_array().unwrap();
    assert_eq!(messages[1]["message"], json!(""));
    assert_eq!(messages[2]["message"], json!("I am good thanks!"));
    assert_eq!(messages[3]["message"], json!(""));
    assert_eq!(messages[4]["message"], json!("I am good thanks!"));
}
