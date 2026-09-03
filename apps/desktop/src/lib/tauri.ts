import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type {
  CleanupPlan,
  CleanupResult,
  HistoryEntry,
  LicenseStatus,
  ScanProgress,
  ScanResult,
  Settings,
  BuildInfo,
} from "./types";

export async function pickScanFolders(): Promise<string[]> {
  return invoke<string[]>("pick_scan_folders");
}

export async function startScan(roots: string[]): Promise<string> {
  return invoke<string>("start_scan", { roots });
}

export async function cancelScan(scanId: string): Promise<void> {
  return invoke("cancel_scan", { scanId });
}

export async function getScanSnapshot(): Promise<ScanResult | null> {
  return invoke<ScanResult | null>("get_scan_snapshot");
}

export async function buildCleanupPlan(
  selections: { project_id: string; artifact_ids: string[] }[],
  approvedRoots: string[]
): Promise<CleanupPlan> {
  return invoke<CleanupPlan>("build_cleanup_plan", {
    selections,
    approved_roots: approvedRoots,
  });
}

export async function executeCleanup(planId: string): Promise<CleanupResult> {
  return invoke<CleanupResult>("execute_cleanup", { plan_id: planId });
}

export async function getSettings(): Promise<Settings> {
  return invoke<Settings>("get_settings");
}

export async function saveSettings(settings: Settings): Promise<void> {
  return invoke("save_settings", { settings });
}

export async function getLicenseStatus(): Promise<LicenseStatus> {
  return invoke<LicenseStatus>("get_license_status");
}

export async function activateLicense(licenseKey: string): Promise<LicenseStatus> {
  return invoke<LicenseStatus>("activate_license", { license_key: licenseKey });
}

export async function generateLocalLicense(): Promise<LicenseStatus> {
  return invoke<LicenseStatus>("generate_local_license");
}

export async function getBuildInfo(): Promise<BuildInfo> {
  return invoke<BuildInfo>("get_build_info_cmd");
}

export async function getCleanupHistory(): Promise<HistoryEntry[]> {
  return invoke<HistoryEntry[]>("get_cleanup_history");
}

export function onScanProgress(cb: (p: ScanProgress) => void) {
  return listen<ScanProgress>("scan://progress", (e) => cb(e.payload));
}

export function onScanComplete(cb: (r: ScanResult) => void) {
  return listen<ScanResult>("scan://complete", (e) => cb(e.payload));
}

export function onScanError(cb: (msg: string) => void) {
  return listen<string>("scan://error", (e) => cb(e.payload));
}
