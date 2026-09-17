type Props = {
  children: React.ReactNode;
  onClick?: () => void;
  type?: "button" | "submit";
  className?: string;
  title?: string;
  disabled?: boolean;
};

export function PrimaryButton({
  children,
  onClick,
  type = "button",
  className,
  title,
  disabled = false,
}: Props) {
  return (
    <button
      type={type}
      onClick={onClick}
      className={`w-full relative bg-gray-200 px-4 py-2 rounded-xl hover:bg-gray-300 active:bg-gray-400 transition-colors duration-200 disabled:cursor-not-allowed cursor-pointer ${className}`}
      title={title}
      disabled={disabled}
    >
      {children}
    </button>
  );
}
