use workflow_core::*;
use gpui::*;
use serde::{Deserialize, Serialize};

/// Visual representation of a module on the canvas
#[derive(Debug, Clone)]
pub struct CanvasNode {
    pub id: ModuleId,
    pub template: ModuleTemplate,
    pub position: Point<f32>,
    pub size: Size<f32>,
    pub input_ports: Vec<CanvasPort>,
    pub output_ports: Vec<CanvasPort>,
    pub selected: bool,
    pub enabled: bool,
}

impl CanvasNode {
    pub fn from_module(module: &ModuleInstance) -> Self {
        let input_ports: Vec<CanvasPort> = module.template.inputs.iter()
            .map(|port| CanvasPort::from_port(port, PortDirection::Input))
            .collect();
        
        let output_ports: Vec<CanvasPort> = module.template.outputs.iter()
            .map(|port| CanvasPort::from_port(port, PortDirection::Output))
            .collect();

        // Calculate size based on content
        let port_count = input_ports.len().max(output_ports.len());
        let height = 60.0 + (port_count as f32 * 20.0);
        let width = 200.0;

        Self {
            id: module.id,
            template: module.template.clone(),
            position: Point::new(module.position.0, module.position.1),
            size: Size::new(width, height),
            input_ports,
            output_ports,
            selected: false,
            enabled: module.enabled,
        }
    }

    pub fn bounds(&self) -> Bounds<f32> {
        Bounds::new(self.position, self.size)
    }

    pub fn header_bounds(&self) -> Bounds<f32> {
        Bounds::new(self.position, Size::new(self.size.width, 40.0))
    }

    pub fn get_input_port_position(&self, port_id: &str) -> Option<Point<f32>> {
        for (i, port) in self.input_ports.iter().enumerate() {
            if port.id == port_id {
                return Some(Point::new(
                    self.position.x,
                    self.position.y + 50.0 + (i as f32 * 20.0)
                ));
            }
        }
        None
    }

    pub fn get_output_port_position(&self, port_id: &str) -> Option<Point<f32>> {
        for (i, port) in self.output_ports.iter().enumerate() {
            if port.id == port_id {
                return Some(Point::new(
                    self.position.x + self.size.width,
                    self.position.y + 50.0 + (i as f32 * 20.0)
                ));
            }
        }
        None
    }

    pub fn get_port_at_position(&self, position: Point<f32>) -> Option<(String, PortDirection)> {
        let relative_pos = position - self.position;
        
        // Check input ports
        for (i, port) in self.input_ports.iter().enumerate() {
            let port_y = 50.0 + (i as f32 * 20.0);
            if relative_pos.x >= -10.0 && relative_pos.x <= 10.0 && 
               relative_pos.y >= port_y - 5.0 && relative_pos.y <= port_y + 15.0 {
                return Some((port.id.clone(), PortDirection::Input));
            }
        }

        // Check output ports
        for (i, port) in self.output_ports.iter().enumerate() {
            let port_y = 50.0 + (i as f32 * 20.0);
            if relative_pos.x >= self.size.width - 10.0 && relative_pos.x <= self.size.width + 10.0 && 
               relative_pos.y >= port_y - 5.0 && relative_pos.y <= port_y + 15.0 {
                return Some((port.id.clone(), PortDirection::Output));
            }
        }

        None
    }
}

/// Visual representation of a port on a canvas node
#[derive(Debug, Clone)]
pub struct CanvasPort {
    pub id: String,
    pub name: String,
    pub bit_type: BitType,
    pub direction: PortDirection,
    pub connected: bool,
    pub required: bool,
}

impl CanvasPort {
    pub fn from_port(port: &Port, direction: PortDirection) -> Self {
        Self {
            id: port.id.clone(),
            name: port.name.clone(),
            bit_type: port.bit_type.clone(),
            direction,
            connected: false,
            required: port.required,
        }
    }

    pub fn type_color(&self) -> Hsla {
        match self.bit_type {
            BitType::Raw => hsla(0.0, 0.0, 0.5, 1.0),        // Gray
            BitType::Byte => hsla(0.6, 0.8, 0.5, 1.0),       // Blue
            BitType::Word => hsla(0.3, 0.8, 0.5, 1.0),       // Green
            BitType::DWord => hsla(0.1, 0.8, 0.5, 1.0),      // Orange
            BitType::QWord => hsla(0.0, 0.8, 0.5, 1.0),      // Red
            BitType::Text => hsla(0.8, 0.8, 0.5, 1.0),       // Purple
            BitType::Float32 => hsla(0.5, 0.8, 0.5, 1.0),    // Cyan
            BitType::Float64 => hsla(0.7, 0.8, 0.5, 1.0),    // Pink
            BitType::Boolean => hsla(0.15, 0.8, 0.5, 1.0),   // Yellow
            BitType::Array(_) => hsla(0.4, 0.8, 0.5, 1.0),   // Teal
            BitType::Struct(_) => hsla(0.9, 0.8, 0.5, 1.0),  // Magenta
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PortDirection {
    Input,
    Output,
} 