use gpui::Context;
use uuid::Uuid;
use std::collections::HashMap;

use crate::workflow::types::*;

pub struct WorkflowExecutor {
    pub nodes: HashMap<NodeId, WorkflowNode>,
    pub connections: Vec<NodeConnection>,
    pub is_running: bool,
    pub execution_state: ExecutionState,
}

impl WorkflowExecutor {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            connections: Vec::new(),
            is_running: false,
            execution_state: ExecutionState::Stopped,
        }
    }

    pub fn add_node(&mut self, node_type: NodeType, position: gpui::Point<f32>) -> NodeId {
        let id = NodeId::new();
        let node = WorkflowNode::create(id, node_type, position);
        self.nodes.insert(id, node);
        id
    }

    pub fn connect_nodes(
        &mut self,
        from_node: NodeId,
        from_port: String,
        to_node: NodeId,
        to_port: String,
    ) {
        let connection = NodeConnection {
            id: Uuid::new_v4(),
            from_node,
            from_port,
            to_node,
            to_port,
        };
        self.connections.push(connection);
    }

    pub fn delete_node(&mut self, node_id: NodeId) {
        self.nodes.remove(&node_id);
        self.connections.retain(|conn| conn.from_node != node_id && conn.to_node != node_id);
    }

    pub fn run_workflow<T>(&mut self, cx: &mut Context<T>) 
    where 
        T: 'static,
    {
        if self.is_running {
            return;
        }

        self.is_running = true;
        self.execution_state = ExecutionState::Running;

        // Reset all node states
        for node in self.nodes.values_mut() {
            node.state = NodeState::Idle;
        }

        // TODO: Implement actual workflow execution
        // For now, just simulate completion
        cx.spawn(async move |_this: gpui::WeakEntity<T>, _cx| {
            // Simulate some work
            std::thread::sleep(std::time::Duration::from_millis(1000));
            
            // Note: In a real implementation, we would need to update the executor
            // through the parent canvas. This is a simplified version.
        }).detach();
    }

    pub fn stop_workflow(&mut self) {
        self.is_running = false;
        self.execution_state = ExecutionState::Stopped;
        
        // Reset all node states
        for node in self.nodes.values_mut() {
            node.state = NodeState::Idle;
        }
    }

    pub fn get_node(&self, node_id: NodeId) -> Option<&WorkflowNode> {
        self.nodes.get(&node_id)
    }

    pub fn get_node_mut(&mut self, node_id: NodeId) -> Option<&mut WorkflowNode> {
        self.nodes.get_mut(&node_id)
    }

    pub fn get_connections_for_node(&self, node_id: NodeId) -> Vec<&NodeConnection> {
        self.connections
            .iter()
            .filter(|conn| conn.from_node == node_id || conn.to_node == node_id)
            .collect()
    }

    pub fn validate_workflow(&self) -> Result<(), String> {
        // Check for cycles
        if self.has_cycles() {
            return Err("Workflow contains cycles".to_string());
        }

        // Check for disconnected required inputs
        for node in self.nodes.values() {
            for input_port in &node.inputs {
                let has_connection = self.connections.iter().any(|conn| {
                    conn.to_node == node.id && conn.to_port == input_port.id
                });
                
                // For now, we'll just warn about disconnected inputs
                // In a real implementation, you might want to check if the input is required
                if !has_connection {
                    eprintln!("Warning: Node {} has disconnected input port {}", node.title, input_port.name);
                }
            }
        }

        Ok(())
    }

    fn has_cycles(&self) -> bool {
        // Simple cycle detection using DFS
        let mut visited = std::collections::HashSet::new();
        let mut rec_stack = std::collections::HashSet::new();

        for node_id in self.nodes.keys() {
            if !visited.contains(node_id) {
                if self.has_cycle_util(*node_id, &mut visited, &mut rec_stack) {
                    return true;
                }
            }
        }

        false
    }

    fn has_cycle_util(
        &self,
        node_id: NodeId,
        visited: &mut std::collections::HashSet<NodeId>,
        rec_stack: &mut std::collections::HashSet<NodeId>,
    ) -> bool {
        visited.insert(node_id);
        rec_stack.insert(node_id);

        // Get all nodes that this node connects to
        for connection in &self.connections {
            if connection.from_node == node_id {
                let next_node = connection.to_node;
                
                if !visited.contains(&next_node) {
                    if self.has_cycle_util(next_node, visited, rec_stack) {
                        return true;
                    }
                } else if rec_stack.contains(&next_node) {
                    return true;
                }
            }
        }

        rec_stack.remove(&node_id);
        false
    }
} 