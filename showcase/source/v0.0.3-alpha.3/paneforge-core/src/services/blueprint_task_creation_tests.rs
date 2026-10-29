use super::*;
use crate::models::AgentTaskNodeState;

#[test]
fn blueprint_simultaneous_retry_from_two_connections_saves_only_one_node() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("tree.db");
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(2));
    let workers: Vec<_> = (0..2)
        .map(|index| {
            let path = path.clone();
            let barrier = barrier.clone();
            std::thread::spawn(move || {
                let service = AgentTaskTreeService::new(TaskTreeRepository::open(path).unwrap());
                barrier.wait();
                service
                    .create_blueprint_task(request(), 100 + index)
                    .unwrap()
            })
        })
        .collect();
    let nodes: Vec<_> = workers
        .into_iter()
        .map(|worker| worker.join().unwrap())
        .collect();
    assert_eq!(nodes[0], nodes[1]);
    assert_eq!(
        TaskTreeRepository::open(path)
            .unwrap()
            .list(None)
            .unwrap()
            .len(),
        1
    );
}

fn request() -> CreateBlueprintTaskRequest {
    CreateBlueprintTaskRequest {
        request_id: "5200a011-5a1c-4a69-a8d0-185b62c75a01".into(),
        title: "  Build a useful blueprint  ".into(),
        description: "  Keep the user in control  ".into(),
        acceptance_criteria: vec![
            " Survives restart ".into(),
            "".into(),
            "Visible in the map".into(),
        ],
        parent_id: None,
        expected_parent_revision: None,
    }
}

#[test]
fn blueprint_plan_survives_reopening_without_runtime_assignment() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("tree.db");
    let service = AgentTaskTreeService::new(TaskTreeRepository::open(&path).unwrap());
    let node = service.create_blueprint_task(request(), 100).unwrap();
    assert_eq!(node.title, "Build a useful blueprint");
    assert_eq!(node.state, AgentTaskNodeState::Planned);
    assert_eq!(node.revision, 1);
    assert!(node.session_id.is_empty() && node.attempt_id.is_empty());
    assert!(node.runtime_health.is_none());
    assert!(node.validate().is_ok());
    assert_eq!(
        node.planning.as_ref().unwrap().acceptance_criteria,
        ["Survives restart", "Visible in the map"]
    );
    drop(service);
    let reopened = TaskTreeRepository::open(&path).unwrap();
    assert_eq!(reopened.list(None).unwrap(), vec![node.clone()]);
    assert!(reopened.list(Some("real-session")).unwrap().is_empty());
    assert_eq!(reopened.get(&node.node_id).unwrap(), Some(node));
}

#[test]
fn blueprint_legacy_schema_is_extended_without_replacing_runtime_nodes() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("tree.db");
    let connection = rusqlite::Connection::open(&path).unwrap();
    connection.execute_batch("CREATE TABLE paneforge_agent_task_nodes (node_id TEXT PRIMARY KEY,parent_id TEXT,session_id TEXT NOT NULL,attempt_id TEXT NOT NULL,title TEXT NOT NULL,state TEXT NOT NULL,runtime_health TEXT,revision INTEGER NOT NULL,updated_at INTEGER NOT NULL); INSERT INTO paneforge_agent_task_nodes VALUES('existing',NULL,'session','attempt','Existing task','active','ready',4,10);").unwrap();
    drop(connection);
    let service = AgentTaskTreeService::new(TaskTreeRepository::open(&path).unwrap());
    let old = service.repository().get("existing").unwrap().unwrap();
    assert_eq!(old.session_id, "session");
    assert_eq!(old.revision, 4);
    assert!(old.planning.is_none());
    service.create_blueprint_task(request(), 100).unwrap();
    drop(service);
    let reopened = TaskTreeRepository::open(&path).unwrap();
    assert_eq!(reopened.get("existing").unwrap(), Some(old));
    assert_eq!(reopened.list(None).unwrap().len(), 2);
}

#[test]
fn blueprint_retries_are_idempotent_and_never_overwrite_saved_content() {
    let service = AgentTaskTreeService::new(TaskTreeRepository::in_memory().unwrap());
    let first = service.create_blueprint_task(request(), 100).unwrap();
    assert_eq!(
        service.create_blueprint_task(request(), 200).unwrap(),
        first
    );
    let mut changed = request();
    changed.title = "A different task".into();
    assert_eq!(
        service
            .create_blueprint_task(changed, 300)
            .unwrap_err()
            .code(),
        Some("BLUEPRINT_TASK_REQUEST_CONFLICT")
    );
    assert_eq!(service.repository().list(None).unwrap(), vec![first]);
}

#[test]
fn blueprint_child_checks_the_saved_parent_revision() {
    let service = AgentTaskTreeService::new(TaskTreeRepository::in_memory().unwrap());
    let parent = service.create_blueprint_task(request(), 100).unwrap();
    let mut child = request();
    child.request_id = "5200a011-5a1c-4a69-a8d0-185b62c75a02".into();
    child.parent_id = Some(parent.node_id.clone());
    child.expected_parent_revision = Some(0);
    assert_eq!(
        service
            .create_blueprint_task(child.clone(), 101)
            .unwrap_err()
            .code(),
        Some("TASK_TREE_PARENT_STALE")
    );
    assert_eq!(service.repository().list(None).unwrap().len(), 1);
    child.expected_parent_revision = Some(1);
    let saved = service.create_blueprint_task(child.clone(), 102).unwrap();
    assert_eq!(saved.parent_id, Some(parent.node_id));
    assert_eq!(saved.planning.unwrap().parent_revision, Some(1));
    assert_eq!(
        service
            .create_blueprint_task(child, 103)
            .unwrap()
            .updated_at,
        102
    );
}

#[test]
fn blueprint_missing_or_closed_parent_never_inserts_a_child() {
    let service = AgentTaskTreeService::new(TaskTreeRepository::in_memory().unwrap());
    let mut child = request();
    child.parent_id = Some("parent".into());
    child.expected_parent_revision = Some(1);
    assert_eq!(
        service
            .create_blueprint_task(child.clone(), 100)
            .unwrap_err()
            .code(),
        Some("TASK_TREE_PARENT_MISSING")
    );
    let parent = AgentTaskNode {
        node_id: "parent".into(),
        parent_id: None,
        session_id: "s".into(),
        attempt_id: "a".into(),
        title: "Done".into(),
        state: AgentTaskNodeState::Completed,
        runtime_health: None,
        revision: 1,
        updated_at: 1,
        planning: None,
    };
    service.project(parent).unwrap();
    assert_eq!(
        service
            .create_blueprint_task(child, 100)
            .unwrap_err()
            .code(),
        Some("TASK_TREE_PARENT_CLOSED")
    );
    assert_eq!(service.repository().list(None).unwrap().len(), 1);
}

#[test]
fn blueprint_input_validation_rejects_invalid_and_oversized_plans() {
    let service = AgentTaskTreeService::new(TaskTreeRepository::in_memory().unwrap());
    let mut invalid = Vec::new();
    let mut value = request();
    value.title = " \n ".into();
    invalid.push(value);
    let mut value = request();
    value.title = "x".repeat(201);
    invalid.push(value);
    let mut value = request();
    value.description = "x".repeat(10_001);
    invalid.push(value);
    let mut value = request();
    value.acceptance_criteria = vec!["x".repeat(501)];
    invalid.push(value);
    let mut value = request();
    value.acceptance_criteria = vec!["x".into(); 21];
    invalid.push(value);
    let mut value = request();
    value.parent_id = Some("missing".into());
    invalid.push(value);
    let mut value = request();
    value.request_id = "arbitrary-path".into();
    invalid.push(value);
    for value in invalid {
        assert_eq!(
            service
                .create_blueprint_task(value, 100)
                .unwrap_err()
                .code(),
            Some("BLUEPRINT_TASK_INVALID")
        );
    }
    assert!(service.repository().list(None).unwrap().is_empty());
}

#[test]
fn blueprint_cannot_override_runtime_fields_or_be_replaced_by_a_projection() {
    let mut payload = serde_json::to_value(request()).unwrap();
    payload["state"] = serde_json::json!("active");
    assert!(serde_json::from_value::<CreateBlueprintTaskRequest>(payload).is_err());
    let service = AgentTaskTreeService::new(TaskTreeRepository::in_memory().unwrap());
    let mut node = service.create_blueprint_task(request(), 100).unwrap();
    node.title = "Forged projection".into();
    node.revision = 100;
    node.planning = None;
    assert_eq!(
        service.project(node).unwrap_err().code(),
        Some("TASK_TREE_PLAN_IMMUTABLE")
    );
    assert_eq!(
        service.repository().list(None).unwrap()[0].title,
        "Build a useful blueprint"
    );
}
