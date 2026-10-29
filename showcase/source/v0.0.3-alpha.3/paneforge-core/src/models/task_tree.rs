use crate::models::RuntimeHealth;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentTaskNodeState {
    Planned,
    Active,
    Waiting,
    Completed,
    Failed,
    Unknown,
}

impl AgentTaskNodeState {
    pub fn as_db_str(self) -> &'static str {
        match self {
            Self::Planned => "planned",
            Self::Active => "active",
            Self::Waiting => "waiting",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Unknown => "unknown",
        }
    }

    pub fn from_db_str(value: &str) -> Option<Self> {
        Some(match value {
            "planned" => Self::Planned,
            "active" => Self::Active,
            "waiting" => Self::Waiting,
            "completed" => Self::Completed,
            "failed" => Self::Failed,
            "unknown" => Self::Unknown,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentTaskNode {
    pub node_id: String,
    pub parent_id: Option<String>,
    pub session_id: String,
    pub attempt_id: String,
    pub title: String,
    pub state: AgentTaskNodeState,
    pub runtime_health: Option<RuntimeHealth>,
    pub revision: i64,
    pub updated_at: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub planning: Option<BlueprintTaskPlanning>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BlueprintTaskPlanning {
    pub description: String,
    pub acceptance_criteria: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_revision: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CreateBlueprintTaskRequest {
    pub request_id: String,
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub acceptance_criteria: Vec<String>,
    pub parent_id: Option<String>,
    pub expected_parent_revision: Option<i64>,
}

impl AgentTaskNode {
    pub fn validate(&self) -> Result<(), &'static str> {
        for (field, value) in [
            ("node_id", self.node_id.as_str()),
            ("title", self.title.as_str()),
        ] {
            if value.trim().is_empty() {
                return Err(field);
            }
        }
        let unassigned_plan = self.planning.is_some()
            && self.state == AgentTaskNodeState::Planned
            && self.session_id.is_empty()
            && self.attempt_id.is_empty()
            && self.runtime_health.is_none();
        if !unassigned_plan
            && (self.session_id.trim().is_empty() || self.attempt_id.trim().is_empty())
        {
            return Err("runtime_assignment");
        }
        if self.revision < 0 {
            return Err("revision");
        }
        if self.updated_at < 0 {
            return Err("updated_at");
        }
        Ok(())
    }
}
