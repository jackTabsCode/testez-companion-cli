use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum PlanNodeType {
    Describe,
    It,
    BeforeAll,
    AfterAll,
    BeforeEach,
    AfterEach,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ModifierType {
    None,
    Skip,
    Focus,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlanNode {
    pub phrase: String,
    #[serde(rename = "type")]
    pub node_type: PlanNodeType,
    pub modifier: ModifierType,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ReporterStatus {
    Success,
    Failure,
    Skipped,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReporterChildNode {
    pub children: Vec<ReporterChildNode>,
    pub errors: Vec<String>,
    pub plan_node: PlanNode,
    pub status: ReporterStatus,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReporterOutput {
    pub children: Vec<ReporterChildNode>,
    pub errors: Vec<String>,
    pub failure_count: u32,
    pub skipped_count: u32,
    pub success_count: u32,
}
