export type SafetyClass = "SAFE" | "REVIEW" | "PROTECTED" | "UNKNOWN";

export type ActivityStatus =
  | { inactive: { days_since: number } }
  | { active: { days_since: number } }
  | "unknown";

export interface WasteScoreBreakdown {
  total: number;
  inactivity_score: number;
  reclaimable_ratio_score: number;
  reclaimable_size_score: number;
  artifact_score: number;
  reasons: string[];
}

export interface Artifact {
  id: string;
  name: string;
  path: string;
  size_bytes: number;
  safety: SafetyClass;
  kind: string;
  explanation: string;
  shared: boolean;
}

export interface Project {
  id: string;
  name: string;
  path: string;
  ecosystem: string;
  activity: ActivityStatus;
  total_size_bytes: number;
  reclaimable_size_bytes: number;
  waste_score: WasteScoreBreakdown;
  artifacts: Artifact[];
  is_inactive: boolean;
}

export interface ScanSummary {
  project_count: number;
  total_reclaimable_bytes: number;
  inactive_project_count: number;
  largest_waste: { name: string; reclaimable_bytes: number }[];
}

export interface ScanResult {
  scan_id: string;
  roots: string[];
  projects: Project[];
  summary: ScanSummary;
  scanned_at: string;
}

export interface ScanProgress {
  scan_id: string;
  phase: string;
  projects_found: number;
  artifacts_found: number;
  message: string;
}

export interface CleanupPlanItem {
  artifact_id: string;
  project_name: string;
  name: string;
  path: string;
  size_bytes: number;
  safety: SafetyClass;
}

export interface CleanupPlan {
  plan_id: string;
  items: CleanupPlanItem[];
  total_bytes: number;
  folder_count: number;
  approved_roots: string[];
}

export interface CleanupResult {
  plan_id: string;
  moved_bytes: number;
  moved_count: number;
  failed_count: number;
  completed_at: string;
}

export interface Settings {
  scan_locations: string[];
  exclusions: string[];
  inactivity_threshold: "days_30" | "days_90" | "days_180" | "days_365";
  appearance: "system" | "light" | "dark";
}

export interface LicenseStatus {
  tier: "free" | "pro";
  license_id: string | null;
  valid: boolean;
  message: string;
}

export interface HistoryEntry {
  timestamp: string;
  moved_bytes: number;
  item_count: number;
}

export interface BuildInfo {
  flavor: "personal" | "store";
  product_name: string;
  allow_local_license: boolean;
  purchase_url: string;
  pro_price_label: string;
}

export function formatBytes(bytes: number): string {
  const gb = bytes / (1024 ** 3);
  if (gb >= 1) return `${gb.toFixed(1)} GB`;
  const mb = bytes / (1024 ** 2);
  if (mb >= 1) return `${mb.toFixed(1)} MB`;
  const kb = bytes / 1024;
  if (kb >= 1) return `${kb.toFixed(1)} KB`;
  return `${bytes} B`;
}

export function formatActivity(activity: ActivityStatus): string {
  if (activity === "unknown") return "Unknown";
  if ("inactive" in activity) return `${activity.inactive.days_since} days inactive`;
  if ("active" in activity) return `Active (${activity.active.days_since}d ago)`;
  return "Unknown";
}

export function ecosystemLabel(eco: string): string {
  const map: Record<string, string> = {
    node_js: "Node.js",
    python: "Python",
    rust: "Rust",
    dot_net: ".NET",
    java: "Java",
    flutter: "Flutter",
    go: "Go",
    swift: "Swift",
    xcode: "Xcode",
    unknown: "Unknown",
  };
  return map[eco] ?? eco;
}
