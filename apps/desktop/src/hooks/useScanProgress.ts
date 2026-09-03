import { useEffect } from "react";
import { onScanComplete, onScanError, onScanProgress } from "../lib/tauri";
import { useScanStore } from "../stores/scanStore";

export function useScanProgress() {
  const setProgress = useScanStore((s) => s.setProgress);
  const setScanResult = useScanStore((s) => s.setScanResult);
  const setScanError = useScanStore((s) => s.setScanError);

  useEffect(() => {
    const unsubs: (() => void)[] = [];
    onScanProgress((p) => {
      setScanError(null);
      setProgress(p);
    }).then((u) => unsubs.push(u));
    onScanComplete((r) => {
      setScanResult(r);
      setScanError(null);
      setProgress(null);
    }).then((u) => unsubs.push(u));
    onScanError((msg) => {
      setScanError(msg);
      setProgress(null);
    }).then((u) => unsubs.push(u));
    return () => unsubs.forEach((u) => u());
  }, [setProgress, setScanResult, setScanError]);
}
