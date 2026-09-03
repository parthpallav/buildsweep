import { Link, useParams } from "react-router-dom";
import { useEffect } from "react";
import Layout from "../components/Layout";
import Button from "../components/Button";
import { useScanStore } from "../stores/scanStore";
import { getScanSnapshot } from "../lib/tauri";
import {
  formatActivity,
  formatBytes,
  ecosystemLabel,
  type SafetyClass,
} from "../lib/types";
import { isReclaimableArtifact } from "../lib/reclaim";

function safetyLabel(s: SafetyClass): string {
  return s;
}

export default function ProjectDetail() {
  const { id } = useParams<{ id: string }>();
  const { scanResult, setScanResult, selectedArtifacts, toggleArtifact, setArtifactSelected } =
    useScanStore();

  useEffect(() => {
    if (!scanResult) getScanSnapshot().then((r) => r && setScanResult(r));
  }, [scanResult, setScanResult]);

  const project = scanResult?.projects.find((p) => p.id === id);

  if (!project) {
    return (
      <Layout title="Project">
        <p className="text-gray-500">Project not found.</p>
        <Link to="/results" className="mt-4 inline-block text-sm underline">
          Back to results
        </Link>
      </Layout>
    );
  }

  const selected = selectedArtifacts.get(project.id) ?? new Set();
  const projectId = project.id;
  const artifacts = project.artifacts;

  function selectAllReclaimable() {
    for (const artifact of artifacts) {
      if (isReclaimableArtifact(artifact.safety)) {
        setArtifactSelected(projectId, artifact.id, true);
      }
    }
  }

  function clearSelection() {
    for (const artifact of artifacts) {
      setArtifactSelected(projectId, artifact.id, false);
    }
  }

  return (
    <Layout
      title={project.name}
      action={
        <Link to="/results" className="text-sm text-gray-500 hover:text-gray-700">
          Back
        </Link>
      }
    >
      <div className="mb-6 space-y-1 text-sm text-gray-600">
        <p>{project.path}</p>
        <p>{ecosystemLabel(project.ecosystem)}</p>
        <p>{formatActivity(project.activity)}</p>
        <p>{formatBytes(project.total_size_bytes)} total</p>
        <p>{formatBytes(project.reclaimable_size_bytes)} reclaimable</p>
        <p className="font-medium text-gray-900 dark:text-gray-100">
          Waste Score: {project.waste_score.total}
        </p>
        <ul className="mt-2 list-inside list-disc text-gray-500">
          {project.waste_score.reasons.map((r) => (
            <li key={r}>{r}</li>
          ))}
        </ul>
      </div>

      <div className="mb-3 flex flex-wrap items-center justify-between gap-2">
        <h2 className="text-sm font-medium">Regenerable artifacts</h2>
        <div className="flex gap-2">
          <Button variant="secondary" className="px-3 py-1 text-xs" onClick={selectAllReclaimable}>
            Select all
          </Button>
          <Button variant="secondary" className="px-3 py-1 text-xs" onClick={clearSelection}>
            Clear
          </Button>
        </div>
      </div>

      <ul className="divide-y divide-gray-100 dark:divide-gray-800">
        {project.artifacts.length === 0 ? (
          <li className="py-3 text-sm text-gray-500">No regenerable artifacts found on disk.</li>
        ) : (
          project.artifacts.map((a) => {
            const eligible = isReclaimableArtifact(a.safety);
            return (
              <li key={a.id} className="flex items-start gap-3 py-3">
                <input
                  type="checkbox"
                  className="mt-1"
                  checked={selected.has(a.id)}
                  disabled={!eligible}
                  onChange={() => toggleArtifact(project.id, a.id)}
                />
                <div className="flex-1">
                  <div className="flex justify-between gap-4">
                    <span className="font-medium">{a.name}</span>
                    <span className="text-sm text-gray-500">{formatBytes(a.size_bytes)}</span>
                  </div>
                  <div className="mt-1 text-sm text-gray-500">
                    {safetyLabel(a.safety)} — {a.explanation}
                  </div>
                </div>
              </li>
            );
          })
        )}
      </ul>

      <Link
        to="/cleanup"
        className="mt-6 inline-block rounded-md bg-gray-900 px-4 py-2 text-sm text-white dark:bg-gray-100 dark:text-gray-900"
      >
        Review Cleanup
      </Link>
    </Layout>
  );
}
