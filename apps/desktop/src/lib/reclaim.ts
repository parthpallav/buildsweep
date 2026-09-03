import type { Artifact, Project, SafetyClass } from "./types";

/** Artifacts that can be removed and regenerated (SAFE, optionally REVIEW). */
export function isReclaimableArtifact(
  safety: SafetyClass,
  includeReview = true
): boolean {
  if (safety === "SAFE") return true;
  if (includeReview && safety === "REVIEW") return true;
  return false;
}

export function reclaimableArtifacts(
  project: Project,
  includeReview = true
): Artifact[] {
  return project.artifacts.filter((a) => isReclaimableArtifact(a.safety, includeReview));
}

export type ProjectSortKey =
  | "reclaimable"
  | "waste_score"
  | "name"
  | "inactive"
  | "total_size";

export function sortProjects(projects: Project[], key: ProjectSortKey): Project[] {
  const sorted = [...projects];
  switch (key) {
    case "reclaimable":
      return sorted.sort(
        (a, b) =>
          b.reclaimable_size_bytes - a.reclaimable_size_bytes ||
          b.waste_score.total - a.waste_score.total
      );
    case "waste_score":
      return sorted.sort(
        (a, b) =>
          b.waste_score.total - a.waste_score.total ||
          b.reclaimable_size_bytes - a.reclaimable_size_bytes
      );
    case "name":
      return sorted.sort((a, b) => a.name.localeCompare(b.name));
    case "inactive":
      return sorted.sort(
        (a, b) =>
          Number(b.is_inactive) - Number(a.is_inactive) ||
          b.reclaimable_size_bytes - a.reclaimable_size_bytes
      );
    case "total_size":
      return sorted.sort(
        (a, b) =>
          b.total_size_bytes - a.total_size_bytes ||
          b.reclaimable_size_bytes - a.reclaimable_size_bytes
      );
    default:
      return sorted;
  }
}

export function countReclaimableArtifacts(
  projects: Project[],
  includeReview = true
): { artifactCount: number; projectCount: number; totalBytes: number } {
  let artifactCount = 0;
  let projectCount = 0;
  let totalBytes = 0;

  for (const project of projects) {
    const arts = reclaimableArtifacts(project, includeReview);
    if (arts.length === 0) continue;
    projectCount += 1;
    artifactCount += arts.length;
    totalBytes += arts.reduce((n, a) => n + a.size_bytes, 0);
  }

  return { artifactCount, projectCount, totalBytes };
}

export function selectedTotals(
  projects: Project[],
  selectedArtifacts: Map<string, Set<string>>
): { artifactCount: number; projectCount: number; totalBytes: number } {
  let artifactCount = 0;
  let projectCount = 0;
  let totalBytes = 0;

  for (const project of projects) {
    const ids = selectedArtifacts.get(project.id);
    if (!ids?.size) continue;
    projectCount += 1;
    for (const artifact of project.artifacts) {
      if (!ids.has(artifact.id)) continue;
      artifactCount += 1;
      totalBytes += artifact.size_bytes;
    }
  }

  return { artifactCount, projectCount, totalBytes };
}
