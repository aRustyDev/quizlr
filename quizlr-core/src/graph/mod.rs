use petgraph::graph::DiGraph;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicNode {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicEdge {
    pub relationship: RelationshipType,
    pub weight: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationshipType {
    Prerequisite,
    Related,
    Subtopic,
}

pub struct KnowledgeGraph {
    #[allow(dead_code)] // Will be used in future implementations
    graph: DiGraph<TopicNode, TopicEdge>,
}

impl KnowledgeGraph {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
        }
    }
}

impl Default for KnowledgeGraph {
    fn default() -> Self {
        Self::new()
    }
}