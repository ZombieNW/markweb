use std::{error::Error, path::Path};

use markdown::{ParseOptions, mdast::Node, to_mdast};

pub fn parse(path: &Path) -> Result<Node, Box<dyn Error>> {
    let raw_text = std::fs::read_to_string(path)?;
    return to_mdast(&raw_text, &ParseOptions::default()).map_err(|e| e.to_string().into());
}
