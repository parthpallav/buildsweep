import { create } from "zustand";
import type { CleanupPlan, Project, ScanProgress, ScanResult } from "../lib/types";
import { isReclaimableArtifact } from "../lib/reclaim";

interface ScanStore {
  selectedFolders: string[];
  setSelectedFolders: (folders: string[]) => void;
  scanId: string | null;
  setScanId: (id: string | null) => void;
  progress: ScanProgress | null;
  setProgress: (p: ScanProgress | null) => void;
  scanResult: ScanResult | null;
  setScanResult: (r: ScanResult | null) => void;
  scanError: string | null;
  setScanError: (e: string | null) => void;
  selectedArtifacts: Map<string, Set<string>>;
  toggleArtifact: (projectId: string, artifactId: string) => void;
  setArtifactSelected: (projectId: string, artifactId: string, selected: boolean) => void;
  selectAllReclaimable: (includeReview?: boolean) => void;
  clearArtifactSelections: () => void;
  cleanupPlan: CleanupPlan | null;
  setCleanupPlan: (plan: CleanupPlan | null) => void;
  getProject: (id: string) => Project | undefined;
}

export const useScanStore = create<ScanStore>((set, get) => ({
  selectedFolders: [],
  setSelectedFolders: (folders) => set({ selectedFolders: folders }),
  scanId: null,
  setScanId: (id) => set({ scanId: id }),
  progress: null,
  setProgress: (p) => set({ progress: p }),
  scanResult: null,
  setScanResult: (r) => set({ scanResult: r }),
  scanError: null,
  setScanError: (e) => set({ scanError: e }),
  selectedArtifacts: new Map(),
  toggleArtifact: (projectId, artifactId) => {
    const map = new Map(get().selectedArtifacts);
    const set_ = new Set(map.get(projectId) ?? []);
    if (set_.has(artifactId)) set_.delete(artifactId);
    else set_.add(artifactId);
    map.set(projectId, set_);
    set({ selectedArtifacts: map, cleanupPlan: null });
  },
  setArtifactSelected: (projectId, artifactId, selected) => {
    const map = new Map(get().selectedArtifacts);
    const set_ = new Set(map.get(projectId) ?? []);
    if (selected) set_.add(artifactId);
    else set_.delete(artifactId);
    map.set(projectId, set_);
    set({ selectedArtifacts: map, cleanupPlan: null });
  },
  selectAllReclaimable: (includeReview = true) => {
    const result = get().scanResult;
    if (!result) return;

    const map = new Map<string, Set<string>>();
    for (const project of result.projects) {
      const ids = new Set<string>();
      for (const artifact of project.artifacts) {
        if (isReclaimableArtifact(artifact.safety, includeReview)) {
          ids.add(artifact.id);
        }
      }
      if (ids.size > 0) {
        map.set(project.id, ids);
      }
    }
    set({ selectedArtifacts: map, cleanupPlan: null });
  },
  clearArtifactSelections: () => set({ selectedArtifacts: new Map(), cleanupPlan: null }),
  cleanupPlan: null,
  setCleanupPlan: (plan) => set({ cleanupPlan: plan }),
  getProject: (id) => get().scanResult?.projects.find((p) => p.id === id),
}));
