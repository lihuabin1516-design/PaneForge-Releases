use crate::models::{AgentTaskNode, CreateBlueprintTaskRequest};
use crate::repository::TaskTreeRepository;
use crate::utils::error::{AppError, AppResult};
use std::collections::HashSet;

pub struct AgentTaskTreeService {
    repository: TaskTreeRepository,
}

impl AgentTaskTreeService {
    pub fn new(repository: TaskTreeRepository) -> Self {
        Self { repository }
    }
    pub fn repository(&self) -> &TaskTreeRepository {
        &self.repository
    }
    pub fn create_blueprint_task(
        &self,
        mut request: CreateBlueprintTaskRequest,
        now: i64,
    ) -> AppResult<AgentTaskNode> {
        request.request_id = uuid::Uuid::parse_str(&request.request_id)
            .map_err(|_| AppError::coded("BLUEPRINT_TASK_INVALID", "Request ID must be a UUID"))?
            .to_string();
        request.title = request.title.trim().to_owned();
        request.description = request.description.trim().to_owned();
        if request.acceptance_criteria.len() > 20 {
            return Err(AppError::coded(
                "BLUEPRINT_TASK_INVALID",
                "At most 20 criteria are allowed",
            ));
        }
        request.acceptance_criteria = request
            .acceptance_criteria
            .into_iter()
            .map(|value| value.trim().to_owned())
            .filter(|value| !value.is_empty())
            .collect();
        if request.title.is_empty()
            || request.title.chars().count() > 200
            || request.description.chars().count() > 10_000
            || request
                .acceptance_criteria
                .iter()
                .any(|value| value.chars().count() > 500)
            || now < 0
            || request
                .parent_id
                .as_ref()
                .is_some_and(|id| id.trim().is_empty())
            || request.parent_id.is_some() != request.expected_parent_revision.is_some()
            || request
                .expected_parent_revision
                .is_some_and(|revision| revision < 0)
        {
            return Err(AppError::coded(
                "BLUEPRINT_TASK_INVALID",
                "Invalid title, brief, criteria or parent revision",
            ));
        }
        self.repository.create_blueprint_task(&request, now)
    }
    pub fn project(&self, node: AgentTaskNode) -> AppResult<AgentTaskNode> {
        if let Some(parent_id) = &node.parent_id {
            if parent_id == &node.node_id {
                return Err(AppError::coded(
                    "TASK_TREE_CYCLE",
                    "A node cannot parent itself",
                ));
            }
            let mut cursor = Some(parent_id.clone());
            let mut visited = HashSet::new();
            while let Some(id) = cursor {
                if !visited.insert(id.clone()) {
                    return Err(AppError::coded(
                        "TASK_TREE_CYCLE",
                        "Existing task tree cycle detected",
                    ));
                }
                let parent = self.repository.get(&id)?.ok_or_else(|| {
                    AppError::coded("TASK_TREE_PARENT_MISSING", "Parent node is not durable yet")
                })?;
                if parent.node_id == node.node_id {
                    return Err(AppError::coded(
                        "TASK_TREE_CYCLE",
                        "Task tree cycle detected",
                    ));
                }
                cursor = parent.parent_id;
            }
        }
        self.repository.upsert(&node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{AgentTaskNodeState, RuntimeHealth};
    fn node(id: &str, parent: Option<&str>) -> AgentTaskNode {
        AgentTaskNode {
            node_id: id.into(),
            parent_id: parent.map(str::to_owned),
            session_id: "s".into(),
            attempt_id: "a".into(),
            title: id.into(),
            state: AgentTaskNodeState::Planned,
            runtime_health: Some(RuntimeHealth::Ready),
            revision: 1,
            updated_at: 1,
            planning: None,
        }
    }
    #[test]
    fn projection_rejects_cycles_and_missing_parents() {
        let service = AgentTaskTreeService::new(TaskTreeRepository::in_memory().unwrap());
        assert_eq!(
            service
                .project(node("child", Some("root")))
                .unwrap_err()
                .code(),
            Some("TASK_TREE_PARENT_MISSING")
        );
        service.project(node("root", None)).unwrap();
        assert_eq!(
            service
                .project(node("root", Some("root")))
                .unwrap_err()
                .code(),
            Some("TASK_TREE_CYCLE")
        );
    }
}

#[cfg(test)]
#[path = "blueprint_task_creation_tests.rs"]
mod blueprint_tests;
