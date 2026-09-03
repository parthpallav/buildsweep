import { useEffect, useState } from "react";
import { Link } from "react-router-dom";
import { open } from "@tauri-apps/plugin-shell";
import Layout from "../components/Layout";
import Button from "../components/Button";
import {
  getSettings,
  saveSettings,
  getLicenseStatus,
  activateLicense,
  generateLocalLicense,
  getCleanupHistory,
  getBuildInfo,
} from "../lib/tauri";
import type { BuildInfo, HistoryEntry, LicenseStatus, Settings } from "../lib/types";
import { formatBytes } from "../lib/types";

export default function SettingsPage() {
  const [settings, setSettings] = useState<Settings | null>(null);
  const [license, setLicense] = useState<LicenseStatus | null>(null);
  const [buildInfo, setBuildInfo] = useState<BuildInfo | null>(null);
  const [licenseKey, setLicenseKey] = useState("");
  const [history, setHistory] = useState<HistoryEntry[]>([]);
  const [saved, setSaved] = useState(false);
  const [licenseError, setLicenseError] = useState<string | null>(null);
  const [generating, setGenerating] = useState(false);

  const isPro = license?.tier === "pro" && license.valid;
  const isStore = buildInfo?.flavor === "store";

  useEffect(() => {
    getSettings().then(setSettings);
    getLicenseStatus().then(setLicense);
    getBuildInfo().then(setBuildInfo);
    getCleanupHistory()
      .then(setHistory)
      .catch(() => setHistory([]));
  }, []);

  async function handleSave() {
    if (!settings) return;
    try {
      await saveSettings(settings);
      setSaved(true);
      setTimeout(() => setSaved(false), 2000);
    } catch (e) {
      setLicenseError(String(e));
    }
  }

  async function handleActivate() {
    setLicenseError(null);
    try {
      const status = await activateLicense(licenseKey);
      setLicense(status);
      setLicenseKey("");
      getCleanupHistory()
        .then(setHistory)
        .catch(() => setHistory([]));
    } catch (e) {
      setLicenseError(String(e));
    }
  }

  async function handleGenerateLocal() {
    setLicenseError(null);
    setGenerating(true);
    try {
      const status = await generateLocalLicense();
      setLicense(status);
      getCleanupHistory()
        .then(setHistory)
        .catch(() => setHistory([]));
    } catch (e) {
      setLicenseError(String(e));
    } finally {
      setGenerating(false);
    }
  }

  async function handleBuyPro() {
    if (!buildInfo?.purchase_url) return;
    await open(buildInfo.purchase_url);
  }

  if (!settings) {
    return <Layout title="Settings">Loading...</Layout>;
  }

  return (
    <Layout
      title="Settings"
      productName={buildInfo?.product_name}
      action={
        <Link to="/" className="text-sm text-gray-500 hover:text-gray-700">
          Home
        </Link>
      }
    >
      <section className="mb-8">
        <h2 className="mb-3 text-sm font-medium">Inactivity threshold</h2>
        <select
          className="rounded border border-gray-300 px-3 py-2 text-sm dark:border-gray-700 dark:bg-gray-900"
          value={settings.inactivity_threshold}
          onChange={(e) =>
            setSettings({
              ...settings,
              inactivity_threshold: e.target.value as Settings["inactivity_threshold"],
            })
          }
        >
          <option value="days_30">30 days</option>
          <option value="days_90">90 days</option>
          <option value="days_180">180 days</option>
          <option value="days_365">365 days</option>
        </select>
      </section>

      <section className="mb-8">
        <h2 className="mb-3 text-sm font-medium">Exclusions (Pro)</h2>
        {!isPro && (
          <p className="mb-2 text-xs text-amber-700 dark:text-amber-300">
            Pro license required to save scan exclusions.
          </p>
        )}
        <textarea
          className="w-full rounded border border-gray-300 p-2 text-sm disabled:cursor-not-allowed disabled:opacity-60 dark:border-gray-700 dark:bg-gray-900"
          rows={3}
          placeholder="One path per line"
          disabled={!isPro}
          value={settings.exclusions.join("\n")}
          onChange={(e) =>
            setSettings({
              ...settings,
              exclusions: e.target.value.split("\n").filter(Boolean),
            })
          }
        />
      </section>

      <section className="mb-8">
        <h2 className="mb-3 text-sm font-medium">License</h2>
        <p className="text-sm text-gray-600">
          {license?.message ?? "Free tier"} ({license?.tier ?? "free"})
        </p>
        {buildInfo && (
          <p className="mt-1 text-xs text-gray-500">
            Build: {buildInfo.flavor === "personal" ? "Personal (your device)" : "Store (distribution)"}
          </p>
        )}
        {licenseError && <p className="mt-2 text-sm text-red-600">{licenseError}</p>}

        {!isPro && isStore && buildInfo?.purchase_url && (
          <div className="mt-4 rounded-lg border border-emerald-200 bg-emerald-50 p-4 dark:border-emerald-900 dark:bg-emerald-950">
            <p className="text-sm font-medium text-emerald-900 dark:text-emerald-100">
              Unlock batch cleanup, exclusions, and history
            </p>
            <p className="mt-1 text-sm text-emerald-800 dark:text-emerald-200">
              {buildInfo.pro_price_label || "Pro lifetime license"}
            </p>
            <Button className="mt-3" onClick={handleBuyPro}>
              Buy Pro
            </Button>
          </div>
        )}

        {buildInfo?.allow_local_license && (
          <>
            <div className="mt-3 flex flex-wrap gap-2">
              <Button variant="secondary" onClick={handleGenerateLocal} disabled={generating}>
                {generating ? "Generating..." : "Generate local Pro license"}
              </Button>
            </div>
            <p className="mt-2 text-xs text-gray-500">
              Personal build only. Run{" "}
              <code className="rounded bg-gray-100 px-1 dark:bg-gray-800">
                cargo run -p license-signer -- install-dev
              </code>{" "}
              once, then click above.
            </p>
          </>
        )}

        <div className="mt-4 flex gap-2">
          <input
            type="text"
            className="flex-1 rounded border border-gray-300 px-3 py-2 text-sm dark:border-gray-700 dark:bg-gray-900"
            placeholder={isStore ? "Paste license from your purchase email" : "Or paste license JSON"}
            value={licenseKey}
            onChange={(e) => setLicenseKey(e.target.value)}
          />
          <Button variant="secondary" onClick={handleActivate}>
            Activate
          </Button>
        </div>
      </section>

      {history.length > 0 && (
        <section className="mb-8">
          <h2 className="mb-3 text-sm font-medium">Cleanup history (Pro)</h2>
          <ul className="space-y-2 text-sm">
            {history.map((h, i) => (
              <li key={i}>
                {new Date(h.timestamp).toLocaleDateString()} — {formatBytes(h.moved_bytes)} moved
              </li>
            ))}
          </ul>
        </section>
      )}

      <section className="mb-8">
        <h2 className="mb-3 text-sm font-medium">About</h2>
        <p className="text-sm text-gray-600">
          {buildInfo?.product_name ?? "BuildSweep"} v1.0.0 — Find wasted space inside your
          development projects.
        </p>
        <p className="mt-2 text-sm text-gray-500">
          No telemetry. No cloud. Files are moved to Trash / Recycle Bin only.
        </p>
      </section>

      <Button onClick={handleSave}>{saved ? "Saved" : "Save Settings"}</Button>
    </Layout>
  );
}
