import { useState } from "react";
import { Link, useNavigate } from "react-router-dom";
import Layout from "../components/Layout";
import Button from "../components/Button";
import { useScanStore } from "../stores/scanStore";
import { buildCleanupPlan, executeCleanup } from "../lib/tauri";
import { formatBytes } from "../lib/types";

export default function CleanupReview() {
  const { scanResult, selectedArtifacts, cleanupPlan, setCleanupPlan } = useScanStore();
  const [loading, setLoading] = useState(false);
  const [done, setDone] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const navigate = useNavigate();

  const selections = scanResult
    ? Array.from(selectedArtifacts.entries())
        .filter(([, ids]) => ids.size > 0)
        .map(([project_id, artifact_ids]) => ({
          project_id,
          artifact_ids: Array.from(artifact_ids),
        }))
    : [];

  const totalSelected = selections.reduce((n, s) => n + s.artifact_ids.length, 0);

  async function handleBuildPlan() {
    if (!scanResult || !selections.length) return;
    setLoading(true);
    setError(null);
    try {
      const plan = await buildCleanupPlan(selections, scanResult.roots);
      setCleanupPlan(plan);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }

  async function handleConfirm() {
    if (!cleanupPlan) {
      await handleBuildPlan();
      return;
    }
    setLoading(true);
    setError(null);
    try {
      await executeCleanup(cleanupPlan.plan_id);
      setDone(true);
      setCleanupPlan(null);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }

  if (done) {
    return (
      <Layout title="Review Cleanup">
        <p className="mb-4">Selected items were moved to Trash / Recycle Bin.</p>
        <Button onClick={() => navigate("/")}>Done</Button>
      </Layout>
    );
  }

  return (
    <Layout
      title="Review Cleanup"
      action={
        <Link to="/results" className="text-sm text-gray-500 hover:text-gray-700">
          Cancel
        </Link>
      }
    >
      {cleanupPlan ? (
        <>
          <p className="mb-2 text-lg">You are about to move regenerable artifacts to Trash:</p>
          <p className="mb-1 text-2xl font-semibold">{formatBytes(cleanupPlan.total_bytes)}</p>
          <p className="mb-6 text-gray-600">
            {cleanupPlan.folder_count} folder{cleanupPlan.folder_count !== 1 ? "s" : ""} (e.g.
            node_modules, target, build caches) across your selected projects.
          </p>
          <h2 className="mb-2 text-sm font-medium text-gray-500">Selected:</h2>
          <ul className="mb-6 space-y-1 text-sm">
            {cleanupPlan.items.map((i) => (
              <li key={i.artifact_id}>
                {i.project_name}/{i.name}
              </li>
            ))}
          </ul>
          <p className="mb-6 text-sm text-gray-500">Nothing else will be changed.</p>
        </>
      ) : (
        <>
          <p className="mb-4 text-gray-600">
            {totalSelected} regenerable folder{totalSelected !== 1 ? "s" : ""} selected across{" "}
            {selections.length} project{selections.length !== 1 ? "s" : ""}.
          </p>
          {!selections.length && (
            <p className="text-sm text-gray-500">
              Use &quot;Select all reclaimable&quot; on Results, or pick artifacts per project.
            </p>
          )}
        </>
      )}

      {error && <p className="mb-4 text-sm text-red-600">{error}</p>}

      <div className="flex gap-3">
        <Button variant="secondary" onClick={() => navigate("/results")}>
          Cancel
        </Button>
        <Button
          variant="danger"
          disabled={!selections.length || loading}
          onClick={cleanupPlan ? handleConfirm : handleBuildPlan}
        >
          {loading ? "Working..." : cleanupPlan ? "Move to Trash" : "Preview Cleanup"}
        </Button>
      </div>
    </Layout>
  );
}
