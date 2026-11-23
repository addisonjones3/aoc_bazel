use anyhow::{Ok, anyhow};
use iddqd::{IdOrdItem, IdOrdMap, id_upcast};

#[derive(Debug, PartialEq, Eq)]
pub enum Step {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub name: String,
    pub left_name: String,
    pub right_name: String,
}

impl IdOrdItem for Node {
    type Key<'a> = &'a str;

    fn key(&self) -> Self::Key<'_> {
        self.name.as_str()
    }

    id_upcast!();
}

pub type NodeMap = IdOrdMap<Node>;

pub enum EndType {
    Exact(&'static str),
    EndsWith(&'static str),
}

pub fn node_map_steps_to_final(
    steps: &Vec<Step>,
    nm: &NodeMap,
    origin_node_name: &str,
    end_type: EndType,
) -> anyhow::Result<usize> {
    let mut curr_node = nm
        .get(origin_node_name)
        .ok_or(anyhow!("couldn't get {origin_node_name}"))?;

    for (i, step) in steps.iter().cycle().enumerate() {
        match end_type {
            EndType::Exact(name) => {
                if curr_node.name == name {
                    return Ok(i);
                }
            }
            EndType::EndsWith(ending) => {
                if curr_node.name.ends_with(ending) {
                    return Ok(i);
                }
            }
        }
        match step {
            Step::Left => {
                curr_node = nm
                    .get(curr_node.left_name.as_str())
                    .ok_or(anyhow!("couldn't get left name from node {curr_node:?}"))?
            }
            Step::Right => {
                curr_node = nm
                    .get(curr_node.right_name.as_str())
                    .ok_or(anyhow!("couldn't get right name from node {curr_node:?}"))?
            }
        }
    }
    unreachable!();
}
