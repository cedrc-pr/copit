type Props = {
  message?: string;
  className?: string;
};

export function Spinner({ message, className }: Props) {
  if (!message) {
    return (
      <div
        className={`absolute right-4 top-3 w-4.5 h-4.5 border-3 border-transparent border-t-black border-l-black rounded-full animate-spin ${className}`}
      />
    );
  }

  return (
    <div className={`flex gap-4 ${className}`}>
      <p>{message}</p>
      <div className="w-4.5 h-4.5 border-3 border-transparent border-t-black border-l-black rounded-full animate-spin" />
    </div>
  );
}
