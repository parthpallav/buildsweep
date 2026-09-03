import { Link } from "react-router-dom";
import type { Project } from "../lib/types";
import { formatActivity, formatBytes, ecosystemLabel } from "../lib/types";
import { reclaimableArtifacts } from "../lib/reclaim";

interface ProjectRowProps {
  project: Project;
  selectedCount?: number;
}

export default function ProjectRow({ project, selectedCount = 0 }: ProjectRowProps) {
  const reclaimable = reclaimableArtifacts(project);

  return (
    <Link
      to={`/project/${project.id}`}
      className="block border-b border-gray-100 py-4 hover:bg-gray-50 dark:border-gray-800 dark:hover:bg-gray-900"
    >
      <div className="flex items-start justify-between gap-4">
        <div className="min-w-0 flex-1">
          <div className="flex items-center gap-2">
            <div className="truncate font-medium">{project.name}</div>
            {selectedCount > 0 && (
              <span className="shrink-0 rounded-full bg-emerald-100 px-2 py-0.5 text-xs text-emerald-800 dark:bg-emerald-900 dark:text-emerald-100">
                {selectedCount} selected
              </span>
            )}
          </div>
          <div className="mt-1 text-sm text-gray-500">
            {ecosystemLabel(project.ecosystem)} · {formatActivity(project.activity)}
          </div>
          <div className="mt-1 text-sm text-gray-500">
            {formatBytes(project.total_size_bytes)} total ·{" "}
            <span className={project.reclaimable_size_bytes > 0 ? "text-emerald-700 dark:text-emerald-300" : ""}>
              {formatBytes(project.reclaimable_size_bytes)} reclaimable
            </span>
          </div>
          {reclaimable.length > 0 ? (
            <div className="mt-2 flex flex-wrap gap-1.5">
              {reclaimable.map((a) => (
                <span
                  key={a.id}
                  className="rounded bg-gray-100 px-2 py-0.5 text-xs text-gray-700 dark:bg-gray-800 dark:text-gray-300"
                  title={a.explanation}
                >
                  {a.name} · {formatBytes(a.size_bytes)}
                </span>
              ))}
            </div>
          ) : (
            <p className="mt-2 text-xs text-gray-400">No regenerable artifacts found on disk</p>
          )}
        </div>
        <div className="shrink-0 text-right text-sm">
          <div className="font-medium">Waste Score: {project.waste_score.total}</div>
        </div>
      </div>
    </Link>
  );
}
