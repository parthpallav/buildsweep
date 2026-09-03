import { Link, useNavigate } from "react-router-dom";
import { useEffect, useMemo, useState } from "react";
import Layout from "../components/Layout";
import Button from "../components/Button";
import ProjectRow from "../components/ProjectRow";
import { useScanStore } from "../stores/scanStore";
import { formatBytes } from "../lib/types";
import { getScanSnapshot } from "../lib/tauri";
import {
  countReclaimableArtifacts,
  selectedTotals,
  sortProjects,
  type ProjectSortKey,
} from "../lib/reclaim";

export default function Results() {
  const {
    scanResult,
    setScanResult,
    selectedArtifacts,
    selectAllReclaimable,
    clearArtifactSelections,
  } = useScanStore();
  const navigate = useNavigate();
  const [sortKey, setSortKey] = useState<ProjectSortKey>("reclaimable");
  const [onlyReclaimable, setOnlyReclaimable] = useState(false);

  useEffect(() => {
    if (!scanResult) {
      getScanSnapshot().then((r) => r && setScanResult(r));
    }
  }, [scanResult, setScanResult]);

  const visibleProjects = useMemo(() => {
    if (!scanResult) return [];
    let list = scanResult.projects;
    if (onlyReclaimable) {
      list = list.filter((p) => p.reclaimable_size_bytes > 0);
    }
    return sortProjects(list, sortKey);
  }, [scanResult, sortKey, onlyReclaimable]);

  const reclaimableSummary = useMemo(
    () => (scanResult ? countReclaimableArtifacts(scanResult.projects) : null),
    [scanResult]
  );

  const selectionSummary = useMemo(
    () =>
      scanResult ? selectedTotals(scanResult.projects, selectedArtifacts) : null,
    [scanResult, selectedArtifacts]
  );

  if (!scanResult) {
    return (
      <Layout title="BuildSweep">
        <p className="text-gray-500">No scan results yet.</p>
        <Link to="/" className="mt-4 inline-block text-sm underline">
          Go to Home
        </Link>
      </Layout>
    );
  }

  const { summary } = scanResult;

  if (summary.project_count === 0) {
    return (
      <Layout
        title="BuildSweep"
        action={
          <Link to="/" className="text-sm text-gray-500 hover:text-gray-700">
            Home
          </Link>
        }
      >
        <p className="mb-4 text-lg">No development projects found.</p>
        <p className="mb-6 text-sm text-gray-600">
          BuildSweep looks for project markers like package.json, Cargo.toml, pyproject.toml,
          and .git repositories. A folder of documents or installers alone will not produce results.
        </p>
        <Button variant="secondary" onClick={() => navigate("/")}>
          Scan another folder
        </Button>
      </Layout>
    );
  }

  const hasSelection = (selectionSummary?.artifactCount ?? 0) > 0;

  return (
    <Layout
      title="BuildSweep"
      action={
        <Link to="/" className="text-sm text-gray-500 hover:text-gray-700">
          Home
        </Link>
      }
    >
      <div className="mb-6 space-y-1">
        <p className="text-2xl font-semibold">{summary.project_count} projects</p>
        <p className="text-lg text-gray-600">
          {formatBytes(summary.total_reclaimable_bytes)} reclaimable
        </p>
        <p className="text-sm text-gray-500">
          {reclaimableSummary?.artifactCount ?? 0} regenerable folders across{" "}
          {reclaimableSummary?.projectCount ?? 0} projects (node_modules, target, .venv, build
          caches, etc.)
        </p>
      </div>

      <div className="mb-6 flex flex-wrap items-end gap-3 rounded-lg border border-gray-200 p-4 dark:border-gray-800">
        <div>
          <label htmlFor="sort" className="mb-1 block text-xs font-medium text-gray-500">
            Sort by
          </label>
          <select
            id="sort"
            value={sortKey}
            onChange={(e) => setSortKey(e.target.value as ProjectSortKey)}
            className="rounded-md border border-gray-300 bg-white px-3 py-2 text-sm dark:border-gray-700 dark:bg-gray-900"
          >
            <option value="reclaimable">Reclaimable size</option>
            <option value="waste_score">Waste score</option>
            <option value="inactive">Inactive first</option>
            <option value="total_size">Total project size</option>
            <option value="name">Name (A–Z)</option>
          </select>
        </div>

        <label className="flex items-center gap-2 pb-2 text-sm text-gray-600">
          <input
            type="checkbox"
            checked={onlyReclaimable}
            onChange={(e) => setOnlyReclaimable(e.target.checked)}
          />
          Only projects with reclaimable space
        </label>

        <div className="ml-auto flex flex-wrap gap-2">
          <Button variant="secondary" onClick={() => selectAllReclaimable(true)}>
            Select all reclaimable
          </Button>
          {hasSelection && (
            <Button variant="secondary" onClick={clearArtifactSelections}>
              Clear selection
            </Button>
          )}
        </div>
      </div>

      {hasSelection && selectionSummary && (
        <div className="mb-6 rounded-md border border-emerald-200 bg-emerald-50 p-4 text-sm text-emerald-900 dark:border-emerald-900 dark:bg-emerald-950 dark:text-emerald-100">
          <p className="font-medium">
            {selectionSummary.artifactCount} folders selected ·{" "}
            {formatBytes(selectionSummary.totalBytes)} to reclaim across{" "}
            {selectionSummary.projectCount} projects
          </p>
          <p className="mt-1 text-emerald-800/80 dark:text-emerald-200/80">
            Selected items are regenerable artifacts only (dependencies, build output, caches).
          </p>
        </div>
      )}

      {summary.largest_waste.length > 0 && (
        <div className="mb-6">
          <h2 className="mb-2 text-sm font-medium text-gray-500">Largest waste</h2>
          <ul className="space-y-1 text-sm">
            {summary.largest_waste.map((e) => (
              <li key={e.name}>
                {e.name} — {formatBytes(e.reclaimable_bytes)}
              </li>
            ))}
          </ul>
        </div>
      )}

      <div className="mb-6">
        {visibleProjects.length === 0 ? (
          <p className="text-sm text-gray-500">No projects match this filter.</p>
        ) : (
          visibleProjects.map((p) => (
            <ProjectRow
              key={p.id}
              project={p}
              selectedCount={selectedArtifacts.get(p.id)?.size ?? 0}
            />
          ))
        )}
      </div>

      <div className="flex flex-wrap gap-3">
        <Button
          variant="danger"
          disabled={!hasSelection}
          onClick={() => navigate("/cleanup")}
        >
          Reclaim selected ({formatBytes(selectionSummary?.totalBytes ?? 0)})
        </Button>
        {!hasSelection && (reclaimableSummary?.artifactCount ?? 0) > 0 && (
          <Button
            onClick={() => {
              selectAllReclaimable(true);
              navigate("/cleanup");
            }}
          >
            Reclaim all ({formatBytes(reclaimableSummary?.totalBytes ?? 0)})
          </Button>
        )}
      </div>
    </Layout>
  );
}
