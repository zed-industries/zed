use std::collections::HashMap;

use petgraph::algo::has_path_connecting;
use petgraph::graph::NodeIndex;
use petgraph::Graph;

use prost_types::{field_descriptor_proto, DescriptorProto, FileDescriptorProto};

/// `MessageGraph` builds a graph of messages whose edges correspond to nesting.
/// The goal is to recognize when message types are recursively nested, so
/// that fields can be boxed when necessary.
pub struct MessageGraph {
    index: HashMap<String, NodeIndex>,
    graph: Graph<String, ()>,
}

impl MessageGraph {
    pub fn new<'a>(
        files: impl Iterator<Item = &'a FileDescriptorProto>,
    ) -> Result<MessageGraph, String> {
        let mut msg_graph = MessageGraph {
            index: HashMap::new(),
            graph: Graph::new(),
        };

        for file in files {
            let package = format!(
                "{}{}",
                if file.package.is_some() { "." } else { "" },
                file.package.as_ref().map(String::as_str).unwrap_or("")
            );
            for msg in &file.message_type {
                msg_graph.add_message(&package, msg);
            }
        }

        Ok(msg_graph)
    }

    fn get_or_insert_index(&mut self, msg_name: String) -> NodeIndex {
        let MessageGraph {
            ref mut index,
            ref mut graph,
        } = *self;
        assert_eq!(b'.', msg_name.as_bytes()[0]);
        *index
            .entry(msg_name.clone())
            .or_insert_with(|| graph.add_node(msg_name))
    }

    /// Adds message to graph IFF it contains a non-repeated field containing another message.
    /// The purpose of the message graph is detecting recursively nested messages and co-recursively nested messages.
    /// Because prost does not box message fields, recursively nested messages would not compile in Rust.
    /// To allow recursive messages, the message graph is used to detect recursion and automatically box the recursive field.
    /// Since repeated messages are already put in a Vec, boxing them isn’t necessary even if the reference is recursive.
    fn add_message(&mut self, package: &str, msg: &DescriptorProto) {
        let msg_name = format!("{}.{}", package, msg.name.as_ref().unwrap());
        let msg_index = self.get_or_insert_index(msg_name.clone());

        for field in &msg.field {
            if field.r#type() == field_descriptor_proto::Type::Message
                && field.label() != field_descriptor_proto::Label::Repeated
            {
                let field_index = self.get_or_insert_index(field.type_name.clone().unwrap());
                self.graph.add_edge(msg_index, field_index, ());
            }
        }

        for msg in &msg.nested_type {
            self.add_message(&msg_name, msg);
        }
    }

    /// Returns true if message type `inner` is nested in message type `outer`.
    pub fn is_nested(&self, outer: &str, inner: &str) -> bool {
        let outer = match self.index.get(outer) {
            Some(outer) => *outer,
            None => return false,
        };
        let inner = match self.index.get(inner) {
            Some(inner) => *inner,
            None => return false,
        };

        has_path_connecting(&self.graph, outer, inner, None)
    }
}
