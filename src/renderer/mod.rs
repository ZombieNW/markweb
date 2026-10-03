use std::{error::Error, path::Path};

use egui::Ui;
use markdown::{ParseOptions, mdast::Node, to_mdast};

/// Return Markdown AST
pub fn parse(path: &Path) -> Result<Node, Box<dyn Error>> {
    let raw_text = std::fs::read_to_string(path)?;
    return to_mdast(&raw_text, &ParseOptions::default()).map_err(|e| e.to_string().into());
}

/// Recursively crawl Markdown AST and render to egui
pub fn draw(ui: &mut Ui, node: &Node) {
    return match node {
        Node::Root(root) => {
            for child in &root.children {
                draw(ui, child);
            }
        }

        _ => {}
    };
}
