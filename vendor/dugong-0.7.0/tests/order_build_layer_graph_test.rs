use dugong::graphlib::{Graph, GraphOptions};
use dugong::order::{self, LayerGraphLabel, OrderNodeRange, Relationship, WeightLabel};
use serde_json::{Value, json};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
struct SharedJson(Rc<RefCell<Value>>);

impl Default for SharedJson {
    fn default() -> Self {
        Self(Rc::new(RefCell::new(Value::Null)))
    }
}

impl SharedJson {
    fn new(v: Value) -> Self {
        Self(Rc::new(RefCell::new(v)))
    }

    fn borrow(&self) -> std::cell::Ref<'_, Value> {
        self.0.borrow()
    }

    fn borrow_mut(&self) -> std::cell::RefMut<'_, Value> {
        self.0.borrow_mut()
    }
}

fn get_i32(v: &Value, key: &str) -> Option<i32> {
    v.get(key)
        .and_then(|v| v.as_i64())
        .and_then(|n| i32::try_from(n).ok())
}

fn get_string_at(v: &Value, key: &str, idx: usize) -> Option<String> {
    v.get(key)
        .and_then(|v| v.as_array())
        .and_then(|a| a.get(idx))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

impl OrderNodeRange for SharedJson {
    fn rank(&self) -> Option<i32> {
        get_i32(&self.borrow(), "rank")
    }

    fn min_rank(&self) -> Option<i32> {
        get_i32(&self.borrow(), "minRank")
    }

    fn max_rank(&self) -> Option<i32> {
        get_i32(&self.borrow(), "maxRank")
    }

    fn has_min_rank(&self) -> bool {
        self.borrow().get("minRank").is_some()
    }

    fn border_left_at(&self, rank: i32) -> Option<String> {
        get_string_at(&self.borrow(), "borderLeft", rank as usize)
    }

    fn border_right_at(&self, rank: i32) -> Option<String> {
        get_string_at(&self.borrow(), "borderRight", rank as usize)
    }

    fn subgraph_layer_label(&self, rank: i32) -> Self {
        let border_left = self.border_left_at(rank);
        let border_right = self.border_right_at(rank);
        SharedJson::new(json!({
            "borderLeft": border_left,
            "borderRight": border_right,
        }))
    }
}

fn new_graph() -> Graph<SharedJson, WeightLabel, Value> {
    Graph::new(GraphOptions {
        compound: true,
        multigraph: true,
        ..Default::default()
    })
}

#[derive(Debug, Clone, Default, PartialEq)]
struct DefaultSubgraphLabel {
    rank: Option<i32>,
    min_rank: Option<i32>,
    max_rank: Option<i32>,
    marker: &'static str,
}

impl OrderNodeRange for DefaultSubgraphLabel {
    fn rank(&self) -> Option<i32> {
        self.rank
    }

    fn min_rank(&self) -> Option<i32> {
        self.min_rank
    }

    fn max_rank(&self) -> Option<i32> {
        self.max_rank
    }
}

fn children_vec(
    g: &Graph<SharedJson, WeightLabel, LayerGraphLabel>,
    parent: Option<&str>,
) -> Vec<String> {
    let mut out: Vec<String> = match parent {
        Some(p) => g.children(p).into_iter().map(|s| s.to_string()).collect(),
        None => g
            .children_root()
            .into_iter()
            .map(|s| s.to_string())
            .collect(),
    };
    out.sort();
    out
}

#[test]
fn build_layer_graph_places_movable_nodes_with_no_parents_under_the_root_node() {
    let mut g = new_graph();
    g.set_node("a", SharedJson::new(json!({ "rank": 1 })));
    g.set_node("b", SharedJson::new(json!({ "rank": 1 })));
    g.set_node("c", SharedJson::new(json!({ "rank": 2 })));
    g.set_node("d", SharedJson::new(json!({ "rank": 3 })));

    let lg = order::build_layer_graph(&g, 1, Relationship::InEdges, None);
    let root = lg.graph().root.clone();
    assert!(lg.has_node(&root));
    assert_eq!(children_vec(&lg, None), vec![root.clone()]);
    assert_eq!(
        children_vec(&lg, Some(&root)),
        vec!["a".to_string(), "b".to_string()]
    );
}

#[test]
fn build_layer_graph_copies_flat_nodes_from_the_layer_to_the_graph() {
    let mut g = new_graph();
    g.set_node("a", SharedJson::new(json!({ "rank": 1 })));
    g.set_node("b", SharedJson::new(json!({ "rank": 1 })));
    g.set_node("c", SharedJson::new(json!({ "rank": 2 })));
    g.set_node("d", SharedJson::new(json!({ "rank": 3 })));

    let lg1 = order::build_layer_graph(&g, 1, Relationship::InEdges, None);
    assert!(lg1.has_node("a"));
    assert!(lg1.has_node("b"));

    let lg2 = order::build_layer_graph(&g, 2, Relationship::InEdges, None);
    assert!(lg2.has_node("c"));

    let lg3 = order::build_layer_graph(&g, 3, Relationship::InEdges, None);
    assert!(lg3.has_node("d"));
}

#[test]
fn build_layer_graph_uses_the_original_node_label_for_copied_nodes() {
    let mut g = new_graph();
    g.set_node("a", SharedJson::new(json!({ "foo": 1, "rank": 1 })));
    g.set_node("b", SharedJson::new(json!({ "foo": 2, "rank": 2 })));
    g.set_edge_with_label("a", "b", WeightLabel { weight: 1.0 });

    let lg = order::build_layer_graph(&g, 2, Relationship::InEdges, None);

    assert_eq!(
        lg.node("a").unwrap().borrow().get("foo").unwrap(),
        &json!(1)
    );
    g.node("a")
        .unwrap()
        .borrow_mut()
        .as_object_mut()
        .unwrap()
        .insert("foo".to_string(), json!("updated"));
    assert_eq!(
        lg.node("a").unwrap().borrow().get("foo").unwrap(),
        &json!("updated")
    );

    assert_eq!(
        lg.node("b").unwrap().borrow().get("foo").unwrap(),
        &json!(2)
    );
    g.node("b")
        .unwrap()
        .borrow_mut()
        .as_object_mut()
        .unwrap()
        .insert("foo".to_string(), json!("updated"));
    assert_eq!(
        lg.node("b").unwrap().borrow().get("foo").unwrap(),
        &json!("updated")
    );
}

#[test]
fn build_layer_graph_copies_edges_incident_on_rank_nodes_to_the_graph_in_edges() {
    let mut g = new_graph();
    g.set_node("a", SharedJson::new(json!({ "rank": 1 })));
    g.set_node("b", SharedJson::new(json!({ "rank": 1 })));
    g.set_node("c", SharedJson::new(json!({ "rank": 2 })));
    g.set_node("d", SharedJson::new(json!({ "rank": 3 })));
    g.set_edge_with_label("a", "c", WeightLabel { weight: 2.0 });
    g.set_edge_with_label("b", "c", WeightLabel { weight: 3.0 });
    g.set_edge_with_label("c", "d", WeightLabel { weight: 4.0 });

    assert_eq!(
        order::build_layer_graph(&g, 1, Relationship::InEdges, None).edge_count(),
        0
    );
    let lg2 = order::build_layer_graph(&g, 2, Relationship::InEdges, None);
    assert_eq!(lg2.edge_count(), 2);
    assert_eq!(lg2.edge("a", "c", None), Some(&WeightLabel { weight: 2.0 }));
    assert_eq!(lg2.edge("b", "c", None), Some(&WeightLabel { weight: 3.0 }));

    let lg3 = order::build_layer_graph(&g, 3, Relationship::InEdges, None);
    assert_eq!(lg3.edge_count(), 1);
    assert_eq!(lg3.edge("c", "d", None), Some(&WeightLabel { weight: 4.0 }));
}

#[test]
fn build_layer_graph_copies_edges_incident_on_rank_nodes_to_the_graph_out_edges() {
    let mut g = new_graph();
    g.set_node("a", SharedJson::new(json!({ "rank": 1 })));
    g.set_node("b", SharedJson::new(json!({ "rank": 1 })));
    g.set_node("c", SharedJson::new(json!({ "rank": 2 })));
    g.set_node("d", SharedJson::new(json!({ "rank": 3 })));
    g.set_edge_with_label("a", "c", WeightLabel { weight: 2.0 });
    g.set_edge_with_label("b", "c", WeightLabel { weight: 3.0 });
    g.set_edge_with_label("c", "d", WeightLabel { weight: 4.0 });

    let lg1 = order::build_layer_graph(&g, 1, Relationship::OutEdges, None);
    assert_eq!(lg1.edge_count(), 2);
    assert_eq!(lg1.edge("c", "a", None), Some(&WeightLabel { weight: 2.0 }));
    assert_eq!(lg1.edge("c", "b", None), Some(&WeightLabel { weight: 3.0 }));

    let lg2 = order::build_layer_graph(&g, 2, Relationship::OutEdges, None);
    assert_eq!(lg2.edge_count(), 1);
    assert_eq!(lg2.edge("d", "c", None), Some(&WeightLabel { weight: 4.0 }));

    assert_eq!(
        order::build_layer_graph(&g, 3, Relationship::OutEdges, None).edge_count(),
        0
    );
}

#[test]
fn build_layer_graph_collapses_multi_edges() {
    let mut g = new_graph();
    g.set_node("a", SharedJson::new(json!({ "rank": 1 })));
    g.set_node("b", SharedJson::new(json!({ "rank": 2 })));
    g.set_edge_with_label("a", "b", WeightLabel { weight: 2.0 });
    g.set_edge_named("a", "b", Some("multi"), Some(WeightLabel { weight: 3.0 }));

    let lg = order::build_layer_graph(&g, 2, Relationship::InEdges, None);
    assert_eq!(lg.edge("a", "b", None), Some(&WeightLabel { weight: 5.0 }));
}

#[test]
fn build_layer_graph_preserves_hierarchy_for_the_movable_layer() {
    let mut g = new_graph();
    g.set_node("a", SharedJson::new(json!({ "rank": 0 })));
    g.set_node("b", SharedJson::new(json!({ "rank": 0 })));
    g.set_node("c", SharedJson::new(json!({ "rank": 0 })));
    g.set_node(
        "sg",
        SharedJson::new(json!({
            "minRank": 0,
            "maxRank": 0,
            "borderLeft": ["bl"],
            "borderRight": ["br"]
        })),
    );
    g.set_parent("a", "sg");
    g.set_parent("b", "sg");

    let lg = order::build_layer_graph(&g, 0, Relationship::InEdges, None);
    let root = lg.graph().root.clone();

    assert_eq!(
        children_vec(&lg, Some(&root)),
        vec!["c".to_string(), "sg".to_string()]
    );
    assert_eq!(lg.parent("a"), Some("sg"));
    assert_eq!(lg.parent("b"), Some("sg"));
}

#[test]
fn build_layer_graph_uses_default_label_for_unimplemented_subgraph_layer_label() {
    let mut g: Graph<DefaultSubgraphLabel, WeightLabel, ()> = Graph::new(GraphOptions {
        compound: true,
        multigraph: true,
        ..Default::default()
    });
    g.set_node(
        "sg",
        DefaultSubgraphLabel {
            min_rank: Some(0),
            max_rank: Some(0),
            marker: "source",
            ..Default::default()
        },
    );

    let lg = order::build_layer_graph(&g, 0, Relationship::InEdges, None);

    assert_eq!(lg.node("sg"), Some(&DefaultSubgraphLabel::default()));
}
