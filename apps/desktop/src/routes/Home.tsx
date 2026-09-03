import { Link, useNavigate } from "react-router-dom";
import { useEffect, useState } from "react";
import Layout from "../components/Layout";
import Button from "../components/Button";
import { useScanStore } from "../stores/scanStore";
import { getScanSnapshot, pickScanFolders, startScan } from "../lib/tauri";

function sameScanId(a: string | null | undefined, b: string | null | undefined): boolean {
  if (!a || !b) return false;
  return a.toLowerCase() === b.toLowerCase();
}

export default function Home() {
  const {
    selectedFolders,
    setSelectedFolders,
    progress,
    scanId,
    setScanId,
    scanResult,
    setScanResult,
    scanError,
    setScanError,
    clearArtifactSelections,
  } = useScanStore();
  const [scanning, setScanning] = useState(false);
  const navigate = useNavigate();

  useEffect(() => {
    if (!scanning || !scanId) return;

    const interval = window.setInterval(() => {
      getScanSnapshot().then((snapshot) => {
        if (snapshot && sameScanId(snapshot.scan_id, scanId)) {
          setScanResult(snapshot);
        }
      });
    }, 800);

    return () => window.clearInterval(interval);
  }, [scanning, scanId, setScanResult]);

  useEffect(() => {
    if (!scanning || !scanId || !scanResult) return;
    if (!sameScanId(scanResult.scan_id, scanId)) return;

    if (scanResult.summary.project_count > 0 || !progress) {
      setScanning(false);
      navigate("/results");
    }
  }, [scanning, scanId, scanResult, progress, navigate]);

  useEffect(() => {
    if (!scanning || !scanError) return;
    setScanning(false);
  }, [scanning, scanError]);

  async function handleSelectFolders() {
    try {
      setScanError(null);
      const folders = await pickScanFolders();
      if (folders.length) setSelectedFolders(folders);
    } catch (e) {
      setScanError(String(e));
    }
  }

  async function handleScan() {
    if (!selectedFolders.length) return;

    setScanError(null);
    setScanResult(null);
    clearArtifactSelections();
    setScanning(true);

    try {
      const id = await startScan(selectedFolders);
      setScanId(id);
    } catch (e) {
      setScanError(String(e));
      setScanning(false);
    }
  }

  return (
    <Layout
      title="BuildSweep"
      action={
        <Link to="/settings" className="text-sm text-gray-500 hover:text-gray-700">
          Settings
        </Link>
      }
    >
      <p className="mb-8 text-gray-600 dark:text-gray-400">
        Find wasted space inside your development projects.
      </p>

      {scanError && (
        <div className="mb-6 rounded-md border border-red-200 bg-red-50 p-4 text-sm text-red-800 dark:border-red-900 dark:bg-red-950 dark:text-red-200">
          {scanError}
        </div>
      )}

      <div className="space-y-4">
        <Button variant="secondary" onClick={handleSelectFolders} disabled={scanning}>
          Select Folders
        </Button>

        {selectedFolders.length > 0 && (
          <div className="text-sm text-gray-600">
            <p>
              {selectedFolders.length} folder{selectedFolders.length !== 1 ? "s" : ""} selected:
            </p>
            <ul className="mt-2 list-inside list-disc break-all text-gray-500">
              {selectedFolders.map((folder) => (
                <li key={folder}>{folder}</li>
              ))}
            </ul>
          </div>
        )}

        {progress && scanning && (
          <div className="rounded-md border border-gray-200 p-4 text-sm dark:border-gray-800">
            <p>{progress.message}</p>
            <p className="mt-1 text-gray-500">
              {progress.projects_found} projects · {progress.artifacts_found} artifacts
            </p>
            <p className="mt-2 text-xs text-gray-400">
              Large folders like Downloads can take several minutes while sizes are calculated.
            </p>
          </div>
        )}

        <Button onClick={handleScan} disabled={!selectedFolders.length || scanning}>
          {scanning ? "Scanning..." : "Scan"}
        </Button>
      </div>
    </Layout>
  );
}
