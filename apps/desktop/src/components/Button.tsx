interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: "primary" | "secondary" | "danger";
}

export default function Button({
  variant = "primary",
  className = "",
  children,
  ...props
}: ButtonProps) {
  const base =
    "inline-flex items-center justify-center rounded-md px-4 py-2 text-sm font-medium transition-colors disabled:opacity-50";
  const variants = {
    primary: "bg-gray-900 text-white hover:bg-gray-800 dark:bg-gray-100 dark:text-gray-900",
    secondary: "border border-gray-300 hover:bg-gray-50 dark:border-gray-700 dark:hover:bg-gray-900",
    danger: "bg-red-600 text-white hover:bg-red-700",
  };
  return (
    <button className={`${base} ${variants[variant]} ${className}`} {...props}>
      {children}
    </button>
  );
}
