//! Graphlib-compatible JSON read/write helpers.
//!
//! Upstream `graphlib.json.write/read` distinguishes omitted `value` fields (`undefined`) from an
//! explicit JSON `null`. The primary Rust seam preserves that shape exactly by operating on
//! `Graph<Option<N>, Option<E>, Option<G>>`.
//!
//! For callers that intentionally want to collapse missing values onto Rust defaults, the explicit
//! `*_with_defaults` helpers provide that fallback behavior.

use crate::{EdgeKey, Graph, GraphError, GraphOptions};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Value as JsonValue;
use std::io;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphJson {
    pub options: GraphOptions,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<JsonValue>,
    pub nodes: Vec<GraphJsonNode>,
    pub edges: Vec<GraphJsonEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphJsonNode {
    pub v: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphJsonEdge {
    pub v: String,
    pub w: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<JsonValue>,
}

pub fn write<N, E, G>(
    graph: &Graph<Option<N>, Option<E>, Option<G>>,
) -> Result<GraphJson, serde_json::Error>
where
    N: Default + Serialize + 'static,
    E: Default + Serialize + 'static,
    G: Default + Serialize,
{
    let nodes = graph
        .node_ids()
        .into_iter()
        .map(|id| {
            let value = graph
                .node(&id)
                .ok_or_else(|| missing_node_label_error(&id))?;
            Ok(GraphJsonNode {
                v: id.clone(),
                value: option_label_to_json(value)?,
                parent: graph.parent(&id).map(str::to_string),
            })
        })
        .collect::<Result<Vec<_>, serde_json::Error>>()?;

    let edges = graph
        .edge_keys()
        .into_iter()
        .map(|key| {
            let value = graph
                .edge_by_key(&key)
                .ok_or_else(|| missing_edge_label_error(&key))?;
            Ok(GraphJsonEdge {
                v: key.v,
                w: key.w,
                name: key.name,
                value: option_label_to_json(value)?,
            })
        })
        .collect::<Result<Vec<_>, serde_json::Error>>()?;

    Ok(GraphJson {
        options: graph.options(),
        value: option_label_to_json(graph.graph())?,
        nodes,
        edges,
    })
}

pub fn read<N, E, G>(
    json: &GraphJson,
) -> Result<Graph<Option<N>, Option<E>, Option<G>>, serde_json::Error>
where
    N: Default + DeserializeOwned + 'static,
    E: Default + DeserializeOwned + 'static,
    G: Default + DeserializeOwned,
{
    let mut graph = Graph::new(json.options);
    graph.set_graph(option_label_from_json(json.value.clone())?);

    for node in &json.nodes {
        graph.set_node(node.v.clone(), option_label_from_json(node.value.clone())?);
        if let Some(parent) = node.parent.as_deref().filter(|parent| !parent.is_empty()) {
            graph.set_parent_ref(&node.v, parent);
        }
    }

    for edge in &json.edges {
        graph
            .try_set_edge_named(
                edge.v.clone(),
                edge.w.clone(),
                edge.name.clone(),
                Some(option_label_from_json(edge.value.clone())?),
            )
            .map_err(graph_mutation_error)?;
    }

    Ok(graph)
}

pub fn write_with_defaults<N, E, G>(graph: &Graph<N, E, G>) -> Result<GraphJson, serde_json::Error>
where
    N: Default + Serialize + 'static,
    E: Default + Serialize + 'static,
    G: Default + Serialize,
{
    let nodes = graph
        .node_ids()
        .into_iter()
        .map(|id| {
            let value = graph
                .node(&id)
                .ok_or_else(|| missing_node_label_error(&id))?;
            Ok(GraphJsonNode {
                v: id.clone(),
                value: default_label_to_json(value)?,
                parent: graph.parent(&id).map(str::to_string),
            })
        })
        .collect::<Result<Vec<_>, serde_json::Error>>()?;

    let edges = graph
        .edge_keys()
        .into_iter()
        .map(|key| {
            let value = graph
                .edge_by_key(&key)
                .ok_or_else(|| missing_edge_label_error(&key))?;
            Ok(GraphJsonEdge {
                v: key.v,
                w: key.w,
                name: key.name,
                value: default_label_to_json(value)?,
            })
        })
        .collect::<Result<Vec<_>, serde_json::Error>>()?;

    Ok(GraphJson {
        options: graph.options(),
        value: default_label_to_json(graph.graph())?,
        nodes,
        edges,
    })
}

pub fn read_with_defaults<N, E, G>(json: &GraphJson) -> Result<Graph<N, E, G>, serde_json::Error>
where
    N: Default + DeserializeOwned + 'static,
    E: Default + DeserializeOwned + 'static,
    G: Default + DeserializeOwned,
{
    let mut graph = Graph::new(json.options);
    graph.set_graph(default_label_from_json(json.value.clone())?);

    for node in &json.nodes {
        graph.set_node(node.v.clone(), default_label_from_json(node.value.clone())?);
        if let Some(parent) = node.parent.as_deref().filter(|parent| !parent.is_empty()) {
            graph.set_parent_ref(&node.v, parent);
        }
    }

    for edge in &json.edges {
        graph
            .try_set_edge_named(
                edge.v.clone(),
                edge.w.clone(),
                edge.name.clone(),
                Some(default_label_from_json(edge.value.clone())?),
            )
            .map_err(graph_mutation_error)?;
    }

    Ok(graph)
}

fn graph_json_invariant_error(message: String) -> serde_json::Error {
    serde_json::Error::io(io::Error::new(io::ErrorKind::InvalidData, message))
}

fn missing_node_label_error(id: &str) -> serde_json::Error {
    graph_json_invariant_error(format!(
        "Graph JSON write saw node id without live label: {id}"
    ))
}

fn missing_edge_label_error(key: &EdgeKey) -> serde_json::Error {
    graph_json_invariant_error(format!(
        "Graph JSON write saw edge key without live label: {} -> {} ({})",
        key.v,
        key.w,
        key.name.as_deref().unwrap_or("<unnamed>")
    ))
}

fn graph_mutation_error(err: GraphError) -> serde_json::Error {
    graph_json_invariant_error(format!("Graph JSON read rejected edge mutation: {err}"))
}

fn option_label_to_json<T>(value: &Option<T>) -> Result<Option<JsonValue>, serde_json::Error>
where
    T: Serialize,
{
    match value {
        Some(value) => Ok(Some(serde_json::to_value(value)?)),
        None => Ok(None),
    }
}

fn option_label_from_json<T>(value: Option<JsonValue>) -> Result<Option<T>, serde_json::Error>
where
    T: DeserializeOwned,
{
    match value {
        Some(value) => serde_json::from_value(value).map(Some),
        None => Ok(None),
    }
}

fn default_label_to_json<T>(value: &T) -> Result<Option<JsonValue>, serde_json::Error>
where
    T: Serialize,
{
    let value = serde_json::to_value(value)?;
    Ok((value != JsonValue::Null).then_some(value))
}

fn default_label_from_json<T>(value: Option<JsonValue>) -> Result<T, serde_json::Error>
where
    T: Default + DeserializeOwned,
{
    match value {
        Some(value) => serde_json::from_value(value),
        None => Ok(T::default()),
    }
}
