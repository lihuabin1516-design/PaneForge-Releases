import { beforeEach, describe, expect, it, vi } from "vitest";
import { act, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { invoke } from "@tauri-apps/api/core";
import { createInstance } from "i18next";
import { I18nextProvider } from "react-i18next";
import en from "@/i18n/locales/en/sidebar.json";
import type { AgentTaskNode } from "@/types/task-tree";
import BlueprintView from "./BlueprintView";

const i18n = createInstance();
const root: AgentTaskNode = { nodeId: "root", sessionId: "s", attemptId: "a", title: "Ship preview", state: "planned", revision: 3, updatedAt: 1 };
const created = { nodeId: "blueprint-new", sessionId: "", attemptId: "", title: "Create a useful blueprint", state: "planned", revision: 1, updatedAt: 2,
  planning: { description: "Keep the user in control", acceptanceCriteria: ["Survives restart", "Visible in the map"] } };
const show = () => render(<I18nextProvider i18n={i18n}><BlueprintView /></I18nextProvider>);
const fill = () => {
  fireEvent.change(screen.getByRole("textbox", { name: "Task title" }), { target: { value: `  ${created.title}  ` } });
  fireEvent.change(screen.getByRole("textbox", { name: "Description" }), { target: { value: created.planning.description } });
  fireEvent.change(screen.getByRole("textbox", { name: "Acceptance criteria (one per line)" }), { target: { value: "Survives restart\n\n Visible in the map " } });
};

beforeEach(async () => {
  vi.mocked(invoke).mockReset().mockImplementation(async (command) => command === "create_blueprint_task" ? created : [root]);
  window.__TAURI_INTERNALS__ = {};
  await i18n.init({ lng: "en", resources: { en: { sidebar: en } }, interpolation: { escapeValue: false } });
});

describe("create a blueprint task", () => {
  it("saves a root with its brief and criteria, then selects it even under an old filter", async () => {
    show();
    await screen.findByRole("list", { name: "Blueprint nodes" });
    fireEvent.change(screen.getByRole("textbox", { name: "Search blueprint" }), { target: { value: "no match" } });
    fireEvent.click(screen.getByRole("button", { name: "New task" }));
    expect(screen.getByRole("button", { name: "Save task" })).toBeDisabled();
    fill();
    fireEvent.click(screen.getByRole("button", { name: "Save task" }));
    const details = await screen.findByRole("region", { name: "Blueprint node details" });
    expect(within(details).getByRole("heading", { name: created.title })).toBeInTheDocument();
    expect(within(details).getByText("Survives restart")).toBeInTheDocument();
    expect(within(details).getByRole("definition", { name: "Session ID" })).toHaveTextContent("Unassigned");
    expect(screen.getByRole("textbox", { name: "Search blueprint" })).toHaveValue("");
    expect(invoke).toHaveBeenCalledWith("create_blueprint_task", { request: {
      requestId: expect.any(String), title: created.title, description: created.planning.description,
      acceptanceCriteria: created.planning.acceptanceCriteria,
    } });
    expect(vi.mocked(invoke).mock.calls.every(([command]) => ["get_agent_task_tree", "create_blueprint_task"].includes(command))).toBe(true);
  });

  it("creates a child against the selected parent's observed revision and expands it", async () => {
    vi.mocked(invoke).mockImplementation(async (command) => command === "create_blueprint_task" ? { ...created, parentId: root.nodeId } : [root]);
    show();
    fireEvent.click(await screen.findByRole("button", { name: /Ship preview/ }));
    fireEvent.click(screen.getByRole("button", { name: "Add subtask" }));
    expect(screen.getByText("Parent: Ship preview")).toBeInTheDocument();
    fill();
    fireEvent.click(screen.getByRole("button", { name: "Save task" }));
    await screen.findByRole("region", { name: "Blueprint node details" });
    expect(invoke).toHaveBeenCalledWith("create_blueprint_task", { request: expect.objectContaining({ parentId: "root", expectedParentRevision: 3 }) });
    expect(screen.getByRole("button", { name: new RegExp(created.title) })).toBeInTheDocument();
  });

  it("preserves the form and request ID after failure and blocks duplicate submissions", async () => {
    let rejectSave!: (error: Error) => void;
    vi.mocked(invoke).mockImplementation(async (command) => command === "create_blueprint_task"
      ? new Promise((_, reject) => { rejectSave = reject; }) : [root]);
    show();
    await screen.findByRole("list", { name: "Blueprint nodes" });
    fireEvent.click(screen.getByRole("button", { name: "New task" }));
    fill();
    fireEvent.click(screen.getByRole("button", { name: "Save task" }));
    expect(screen.getByRole("button", { name: "Saving task…" })).toBeDisabled();
    expect(screen.getByRole("button", { name: "Cancel" })).toBeDisabled();
    fireEvent.submit(screen.getByRole("form", { name: "New blueprint task" }));
    expect(vi.mocked(invoke).mock.calls.filter(([command]) => command === "create_blueprint_task")).toHaveLength(1);
    await act(async () => rejectSave(new Error("Database is busy")));
    expect(await screen.findByRole("alert")).toHaveTextContent("Database is busy");
    expect(screen.getByRole("textbox", { name: "Task title" })).toHaveValue(`  ${created.title}  `);
    const first = vi.mocked(invoke).mock.calls.find(([command]) => command === "create_blueprint_task")![1];
    vi.mocked(invoke).mockResolvedValueOnce(created);
    fireEvent.click(screen.getByRole("button", { name: "Save task" }));
    await waitFor(() => expect(screen.queryByRole("form", { name: "New blueprint task" })).not.toBeInTheDocument());
    const writes = vi.mocked(invoke).mock.calls.filter(([command]) => command === "create_blueprint_task");
    expect(writes).toHaveLength(2);
    expect(writes[1][1]).toEqual(first);
  });

  it("keeps the new selection and existing nodes when an earlier tree request completes late", async () => {
    let finishList!: (nodes: AgentTaskNode[]) => void;
    vi.mocked(invoke).mockImplementation(async (command) => command === "create_blueprint_task" ? created
      : new Promise<AgentTaskNode[]>((resolve) => { finishList = resolve; }));
    show();
    fireEvent.click(screen.getByRole("button", { name: "New task" }));
    fill();
    fireEvent.click(screen.getByRole("button", { name: "Save task" }));
    await screen.findByRole("region", { name: "Blueprint node details" });
    await act(async () => finishList([root]));
    expect(screen.getByRole("button", { name: new RegExp(created.title) })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /Ship preview/ })).toBeInTheDocument();
    expect(within(screen.getByRole("region", { name: "Blueprint node details" })).getByRole("heading", { name: created.title })).toBeInTheDocument();
  });

  it("cancels without saving", async () => {
    show();
    await screen.findByRole("list", { name: "Blueprint nodes" });
    fireEvent.click(screen.getByRole("button", { name: "New task" }));
    fill();
    fireEvent.click(screen.getByRole("button", { name: "Cancel" }));
    expect(screen.queryByRole("form", { name: "New blueprint task" })).not.toBeInTheDocument();
    expect(vi.mocked(invoke).mock.calls.some(([command]) => command === "create_blueprint_task")).toBe(false);
  });

  it("keeps blank titles and oversized criteria from being submitted", async () => {
    show();
    await screen.findByRole("list", { name: "Blueprint nodes" });
    fireEvent.click(screen.getByRole("button", { name: "New task" }));
    fill();
    fireEvent.change(screen.getByRole("textbox", { name: "Task title" }), { target: { value: " \n " } });
    expect(screen.getByRole("button", { name: "Save task" })).toBeDisabled();
    fill();
    fireEvent.change(screen.getByRole("textbox", { name: "Acceptance criteria (one per line)" }), { target: { value: Array(21).fill("Criterion").join("\n") } });
    expect(screen.getByRole("alert")).toHaveTextContent("at most 20 criteria");
    expect(screen.getByRole("button", { name: "Save task" })).toBeDisabled();
  });

  it("disables creation in a browser-only preview", async () => {
    delete window.__TAURI_INTERNALS__;
    show();
    expect(await screen.findByRole("alert")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "New task" })).toBeDisabled();
    expect(invoke).not.toHaveBeenCalled();
  });
});
