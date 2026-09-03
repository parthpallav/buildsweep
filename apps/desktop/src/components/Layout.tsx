interface LayoutProps {
  title: string;
  children: React.ReactNode;
  action?: React.ReactNode;
  productName?: string;
}

export default function Layout({ title, children, action, productName }: LayoutProps) {
  return (
    <div className="mx-auto max-w-3xl px-6 py-8">
      <header className="mb-8 flex items-center justify-between border-b border-gray-200 pb-4 dark:border-gray-800">
        <div className="flex items-center gap-3">
          <img
            src="/logo.png"
            alt=""
            className="h-9 w-9 rounded-lg shadow-sm"
          />
          <div>
            <p className="text-xs font-medium uppercase tracking-wide text-emerald-600 dark:text-emerald-400">
              {productName ?? "BuildSweep"}
            </p>
            <h1 className="text-xl font-semibold tracking-tight">{title}</h1>
          </div>
        </div>
        {action}
      </header>
      {children}
    </div>
  );
}
