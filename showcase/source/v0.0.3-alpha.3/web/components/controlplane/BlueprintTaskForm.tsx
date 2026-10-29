import { useRef, useState, type FormEvent } from "react";
import { useTranslation } from "react-i18next";
import { taskTreeService } from "@/services/taskTreeService";
import { translateError } from "@/utils";
import type { AgentTaskNode } from "@/types/task-tree";

interface Props {
  parent: AgentTaskNode | null;
  onCreated: (node: AgentTaskNode) => void;
  onCancel: () => void;
}

const inputClass = "w-full border border-[var(--app-border)] bg-[var(--app-input-bg)] px-2 py-1.5 text-[var(--app-text-primary)] outline-none focus:border-[var(--app-accent)]";

export default function BlueprintTaskForm({ parent, onCreated, onCancel }: Props) {
  const { t } = useTranslation("sidebar");
  const [title, setTitle] = useState("");
  const [description, setDescription] = useState("");
  const [criteria, setCriteria] = useState("");
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const requestId = useRef<string | null>(null);
  const inFlight = useRef(false);
  const acceptanceCriteria = criteria.split(/\r?\n/).map((line) => line.trim()).filter(Boolean);
  const valid = Boolean(title.trim()) && acceptanceCriteria.length <= 20
    && acceptanceCriteria.every((line) => Array.from(line).length <= 500);

  const submit = async (event: FormEvent) => {
    event.preventDefault();
    if (!valid || inFlight.current) return;
    inFlight.current = true;
    setSaving(true);
    setError(null);
    try {
      requestId.current ??= crypto.randomUUID();
      const created = await taskTreeService.create({
        requestId: requestId.current, title: title.trim(), description: description.trim(), acceptanceCriteria,
        ...(parent ? { parentId: parent.nodeId, expectedParentRevision: parent.revision } : {}),
      });
      onCreated(created);
    } catch (cause) {
      setError(translateError(cause));
    } finally {
      inFlight.current = false;
      setSaving(false);
    }
  };

  return <form aria-label={t("blueprint.create.form")} onSubmit={(event) => void submit(event)} className="app-scrollbar max-h-[65%] shrink-0 overflow-y-auto border-b border-[var(--app-border)] bg-[var(--app-panel-bg)] p-3 text-xs">
    <fieldset disabled={saving} className="space-y-3">
      <legend className="mb-2 font-medium">{t(parent ? "blueprint.create.child" : "blueprint.create.new")}</legend>
      <p className="text-[var(--app-text-tertiary)]">{parent ? t("blueprint.create.parent", { title: parent.title }) : t("blueprint.create.root")}</p>
      <label className="block space-y-1"><span>{t("blueprint.create.title")}</span>
        <input autoFocus required maxLength={200} value={title} onChange={(event) => setTitle(event.target.value)} className={inputClass} />
      </label>
      <label className="block space-y-1"><span>{t("blueprint.create.description")}</span>
        <textarea rows={2} maxLength={10000} value={description} onChange={(event) => setDescription(event.target.value)} className={inputClass} />
      </label>
      <label className="block space-y-1"><span>{t("blueprint.create.criteria")}</span>
        <textarea rows={3} value={criteria} onChange={(event) => setCriteria(event.target.value)} className={inputClass} />
      </label>
      <p className="text-[var(--app-text-tertiary)]">{t("blueprint.create.criteriaHint")}</p>
      {acceptanceCriteria.length > 20 || acceptanceCriteria.some((line) => Array.from(line).length > 500)
        ? <p role="alert" className="text-[var(--app-status-danger)]">{t("blueprint.create.criteriaLimit")}</p> : null}
      <p className="text-[var(--app-text-tertiary)]">{t("blueprint.create.hint")}</p>
      {error && <p role="alert" className="break-words text-[var(--app-status-danger)]">{t("blueprint.create.failed")}: {error}</p>}
      <div className="flex justify-end gap-2">
        <button type="button" onClick={onCancel} className="border border-[var(--app-border)] px-3 py-1.5 disabled:opacity-40">{t("blueprint.create.cancel")}</button>
        <button type="submit" disabled={!valid || saving} className="bg-[var(--app-accent)] px-3 py-1.5 text-white disabled:opacity-40">{t(saving ? "blueprint.create.saving" : "blueprint.create.save")}</button>
      </div>
    </fieldset>
  </form>;
}
