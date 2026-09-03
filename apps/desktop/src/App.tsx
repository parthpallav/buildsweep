import { Routes, Route, Navigate } from "react-router-dom";
import Home from "./routes/Home";
import Results from "./routes/Results";
import ProjectDetail from "./routes/ProjectDetail";
import CleanupReview from "./routes/CleanupReview";
import Settings from "./routes/Settings";
import { useScanProgress } from "./hooks/useScanProgress";

export default function App() {
  useScanProgress();

  return (
    <div className="min-h-screen font-sans">
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/results" element={<Results />} />
        <Route path="/project/:id" element={<ProjectDetail />} />
        <Route path="/cleanup" element={<CleanupReview />} />
        <Route path="/settings" element={<Settings />} />
        <Route path="*" element={<Navigate to="/" replace />} />
      </Routes>
    </div>
  );
}
