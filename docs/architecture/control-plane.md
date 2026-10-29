# Control-plane lifecycle

The public preview exposes the first part of this lifecycle. The private development line is extending the same records for run profiles, team templates, human handoff, and delivery evidence.

```text
draft task
   │ save intent and acceptance criteria
   ▼
planned task
   │ select workspace/project/runtime/session profile
   ▼
dispatch request
   │ bind an explicit session and record the launch revision
   ▼
observed session ───────► waiting / failed / stopped
   │                               │
   │ collect outputs and checks     │ recovery or human review
   ▼                               ▼
evidence candidate ───────► reviewed delivery
```

## Invariants

1. Saving a task does not imply that a CLI was started.
2. A session identity is not reused for a different task binding.
3. A restart or reconnect must replay observations with a revision or timestamp guard.
4. A failed or interrupted process cannot produce a successful delivery state without fresh evidence.
5. A public release must identify the exact source archive and build inputs that produced it.

## Blueprint direction

The private development line adds four reviewable steps around the existing lifecycle:

- **Launch preview:** choose project, working directory, runtime, and launch profile; validate stale paths before dispatch.
- **Role and team templates:** reuse a template but freeze the resolved choices on the task so later template edits do not rewrite history.
- **Collaboration inbox:** route an explicit human question or handoff card while keeping the session's default isolation.
- **Delivery view:** combine task requirements, Git changes, artifacts, and acceptance evidence in one view.

These additions are intentionally documented as development work until the corresponding public installer and Windows acceptance run contain them.
