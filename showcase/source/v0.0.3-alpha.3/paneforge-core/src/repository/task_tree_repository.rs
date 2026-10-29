use crate::models::{
    AgentTaskNode, AgentTaskNodeState, BlueprintTaskPlanning, CreateBlueprintTaskRequest,
    RuntimeHealth,
};
use crate::utils::error::{AppError, AppResult};
use rusqlite::{params, Connection, OptionalExtension, TransactionBehavior};
use std::path::Path;
use std::time::Duration;

const NODE_COLUMNS: &str = "node_id,parent_id,session_id,attempt_id,title,state,runtime_health,revision,updated_at,planning_json";

pub struct TaskTreeRepository {
    connection: Connection,
}

impl TaskTreeRepository {
    pub fn in_memory() -> AppResult<Self> {
        Self::from_connection(Connection::open_in_memory()?)
    }
    pub fn open(path: impl AsRef<Path>) -> AppResult<Self> {
        Self::from_connection(Connection::open(path)?)
    }
    fn from_connection(mut connection: Connection) -> AppResult<Self> {
        connection.busy_timeout(Duration::from_secs(5))?;
        let transaction = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        transaction.execute_batch("CREATE TABLE IF NOT EXISTS paneforge_agent_task_nodes (node_id TEXT PRIMARY KEY,parent_id TEXT,session_id TEXT NOT NULL,attempt_id TEXT NOT NULL,title TEXT NOT NULL,state TEXT NOT NULL,runtime_health TEXT,revision INTEGER NOT NULL,updated_at INTEGER NOT NULL,planning_json TEXT); CREATE INDEX IF NOT EXISTS idx_paneforge_agent_task_nodes_parent ON paneforge_agent_task_nodes(parent_id); CREATE INDEX IF NOT EXISTS idx_paneforge_agent_task_nodes_session ON paneforge_agent_task_nodes(session_id);")?;
        let has_planning = transaction
            .prepare("PRAGMA table_info(paneforge_agent_task_nodes)")?
            .query_map([], |row| row.get::<_, String>(1))?
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .any(|name| name == "planning_json");
        if !has_planning {
            transaction.execute_batch(
                "ALTER TABLE paneforge_agent_task_nodes ADD COLUMN planning_json TEXT",
            )?;
        }
        transaction.commit()?;
        Ok(Self { connection })
    }

    pub fn get(&self, node_id: &str) -> AppResult<Option<AgentTaskNode>> {
        get_node(&self.connection, node_id)
    }

    pub fn upsert(&self, node: &AgentTaskNode) -> AppResult<AgentTaskNode> {
        if node.planning.is_some() || node.node_id.starts_with("blueprint-") {
            return Err(AppError::coded(
                "TASK_TREE_PLAN_IMMUTABLE",
                "Runtime projection cannot replace a manual blueprint plan",
            ));
        }
        node.validate().map_err(|field| {
            AppError::coded(
                "TASK_TREE_INVALID",
                format!("Field {field} is required or invalid"),
            )
        })?;
        self.connection.execute("INSERT INTO paneforge_agent_task_nodes(node_id,parent_id,session_id,attempt_id,title,state,runtime_health,revision,updated_at) VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9) ON CONFLICT(node_id) DO UPDATE SET parent_id=excluded.parent_id,session_id=excluded.session_id,attempt_id=excluded.attempt_id,title=excluded.title,state=excluded.state,runtime_health=excluded.runtime_health,revision=excluded.revision,updated_at=excluded.updated_at WHERE excluded.revision > paneforge_agent_task_nodes.revision", params![node.node_id,node.parent_id,node.session_id,node.attempt_id,node.title,node.state.as_db_str(),node.runtime_health.map(|h| format!("{h:?}").to_lowercase()),node.revision,node.updated_at])?;
        self.get(&node.node_id)?
            .ok_or_else(|| AppError::coded("TASK_TREE_MISSING", "Node was not persisted"))
    }

    pub fn list(&self, session_id: Option<&str>) -> AppResult<Vec<AgentTaskNode>> {
        let mut statement = self.connection.prepare(&format!("SELECT {NODE_COLUMNS} FROM paneforge_agent_task_nodes WHERE (?1 IS NULL OR session_id=?1) ORDER BY COALESCE(parent_id,''),updated_at,node_id"))?;
        let rows = statement.query_map(params![session_id], row_node)?;
        rows.collect::<Result<Vec<_>, _>>().map_err(AppError::from)
    }

    pub(crate) fn create_blueprint_task(
        &self,
        request: &CreateBlueprintTaskRequest,
        now: i64,
    ) -> AppResult<AgentTaskNode> {
        let transaction =
            rusqlite::Transaction::new_unchecked(&self.connection, TransactionBehavior::Immediate)?;
        let node_id = format!("blueprint-{}", request.request_id);
        let planning = BlueprintTaskPlanning {
            description: request.description.clone(),
            acceptance_criteria: request.acceptance_criteria.clone(),
            parent_revision: request.expected_parent_revision,
        };
        if let Some(existing) = get_node(&transaction, &node_id)? {
            if existing.title != request.title
                || existing.parent_id != request.parent_id
                || existing.planning.as_ref() != Some(&planning)
            {
                return Err(AppError::coded("BLUEPRINT_TASK_REQUEST_CONFLICT", "This request ID already saved different content; refresh the blueprint before creating another task"));
            }
            return Ok(existing);
        }
        validate_blueprint_parent(&transaction, request)?;
        let payload =
            serde_json::to_string(&planning).map_err(|error| AppError::from(error.to_string()))?;
        transaction.execute("INSERT INTO paneforge_agent_task_nodes(node_id,parent_id,session_id,attempt_id,title,state,runtime_health,revision,updated_at,planning_json) VALUES(?1,?2,'','',?3,'planned',NULL,1,?4,?5)", params![node_id,request.parent_id,request.title,now,payload])?;
        let node = get_node(&transaction, &node_id)?
            .ok_or_else(|| AppError::coded("TASK_TREE_MISSING", "Created node is missing"))?;
        transaction.commit()?;
        Ok(node)
    }
}

fn get_node(connection: &Connection, node_id: &str) -> AppResult<Option<AgentTaskNode>> {
    connection
        .query_row(
            &format!("SELECT {NODE_COLUMNS} FROM paneforge_agent_task_nodes WHERE node_id=?1"),
            params![node_id],
            row_node,
        )
        .optional()
        .map_err(AppError::from)
}

fn validate_blueprint_parent(
    connection: &Connection,
    request: &CreateBlueprintTaskRequest,
) -> AppResult<()> {
    if let Some(parent_id) = &request.parent_id {
        let parent = get_node(connection, parent_id)?.ok_or_else(|| {
            AppError::coded(
                "TASK_TREE_PARENT_MISSING",
                "Parent task no longer exists; refresh the blueprint",
            )
        })?;
        if Some(parent.revision) != request.expected_parent_revision {
            return Err(AppError::coded(
                "TASK_TREE_PARENT_STALE",
                "Parent task changed; refresh and select it again",
            ));
        }
        if matches!(
            parent.state,
            AgentTaskNodeState::Completed
                | AgentTaskNodeState::Failed
                | AgentTaskNodeState::Unknown
        ) {
            return Err(AppError::coded(
                "TASK_TREE_PARENT_CLOSED",
                "Choose an open parent task",
            ));
        }
    }
    Ok(())
}

fn row_node(row: &rusqlite::Row<'_>) -> rusqlite::Result<AgentTaskNode> {
    let state: String = row.get(5)?;
    let health: Option<String> = row.get(6)?;
    Ok(AgentTaskNode {
        node_id: row.get(0)?,
        parent_id: row.get(1)?,
        session_id: row.get(2)?,
        attempt_id: row.get(3)?,
        title: row.get(4)?,
        state: AgentTaskNodeState::from_db_str(&state).ok_or_else(|| {
            rusqlite::Error::InvalidColumnType(5, "state".into(), rusqlite::types::Type::Text)
        })?,
        runtime_health: health
            .map(|value| match value.as_str() {
                "starting" => Ok(RuntimeHealth::Starting),
                "ready" => Ok(RuntimeHealth::Ready),
                "busy" => Ok(RuntimeHealth::Busy),
                "degraded" => Ok(RuntimeHealth::Degraded),
                "disconnected" => Ok(RuntimeHealth::Disconnected),
                _ => Err(rusqlite::Error::InvalidColumnType(
                    6,
                    "runtime_health".into(),
                    rusqlite::types::Type::Text,
                )),
            })
            .transpose()?,
        revision: row.get(7)?,
        updated_at: row.get(8)?,
        planning: row
            .get::<_, Option<String>>(9)?
            .map(|json| {
                serde_json::from_str(&json).map_err(|error| {
                    rusqlite::Error::FromSqlConversionFailure(
                        9,
                        rusqlite::types::Type::Text,
                        Box::new(error),
                    )
                })
            })
            .transpose()?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn upsert_is_revision_monotonic_and_replayable() {
        let repo = TaskTreeRepository::in_memory().unwrap();
        let node = AgentTaskNode {
            node_id: "root".into(),
            parent_id: None,
            session_id: "s".into(),
            attempt_id: "a1".into(),
            title: "Root".into(),
            state: AgentTaskNodeState::Active,
            runtime_health: Some(RuntimeHealth::Ready),
            revision: 2,
            updated_at: 20,
            planning: None,
        };
        repo.upsert(&node).unwrap();
        let mut stale = node.clone();
        stale.title = "stale".into();
        stale.revision = 1;
        assert_eq!(repo.upsert(&stale).unwrap().title, "Root");
        assert_eq!(repo.list(Some("s")).unwrap().len(), 1);
    }

    #[test]
    fn unknown_runtime_health_fails_closed() {
        let repo = TaskTreeRepository::in_memory().unwrap();
        repo.connection
            .execute(
                "INSERT INTO paneforge_agent_task_nodes(node_id,parent_id,session_id,attempt_id,title,state,runtime_health,revision,updated_at) VALUES('n',NULL,'s','a','Node','planned','corrupt',1,1)",
                [],
            )
            .unwrap();
        assert!(repo.get("n").is_err());
    }
}
