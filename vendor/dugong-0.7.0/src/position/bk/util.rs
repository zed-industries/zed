use crate::graphlib::Graph;
use crate::{EdgeLabel, GraphLabel, LabelPos, NodeLabel};

pub(super) fn sep(
    g: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
    v: &str,
    w: &str,
    reverse_sep: bool,
) -> f64 {
    let v_label = g.node(v).cloned().unwrap_or_default();
    let w_label = g.node(w).cloned().unwrap_or_default();

    let mut sum: f64 = 0.0;
    let mut delta: f64 = 0.0;

    sum += v_label.width / 2.0;
    if let Some(labelpos) = v_label.labelpos {
        delta = match labelpos {
            LabelPos::L => -v_label.width / 2.0,
            LabelPos::R => v_label.width / 2.0,
            LabelPos::C => 0.0,
        };
    }
    if delta != 0.0 {
        sum += if reverse_sep { delta } else { -delta };
    }
    delta = 0.0;

    let node_sep = g.graph().nodesep;
    let edge_sep = g.graph().edgesep;

    sum += if v_label.dummy.is_some() {
        edge_sep
    } else {
        node_sep
    } / 2.0;
    sum += if w_label.dummy.is_some() {
        edge_sep
    } else {
        node_sep
    } / 2.0;

    sum += w_label.width / 2.0;
    if let Some(labelpos) = w_label.labelpos {
        delta = match labelpos {
            LabelPos::L => w_label.width / 2.0,
            LabelPos::R => -w_label.width / 2.0,
            LabelPos::C => 0.0,
        };
    }
    if delta != 0.0 {
        sum += if reverse_sep { delta } else { -delta };
    }

    sum
}

pub(super) fn width(g: &Graph<NodeLabel, EdgeLabel, GraphLabel>, v: &str) -> f64 {
    g.node(v).map(|n| n.width).unwrap_or(0.0)
}
