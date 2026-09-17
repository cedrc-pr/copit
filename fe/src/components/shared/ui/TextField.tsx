type Props = {
  field: {
    state: {
      meta: { errors: ({ message: string } | undefined)[]; isTouched: boolean };
      value: string;
    };
    handleChange: (value: string) => void;
    handleBlur: () => void;
  };
  label: string;
  id: string;
  type?: string;
  handle_change?: (value: string) => string;
  change_on_blur?: (value: string) => string;
  max_length?: number | null;
  placeholder?: string;
  disabled?: boolean;
  trim?: boolean;
  className?: string;
};

export function TextField({
  field,
  label,
  id,
  type = "text",
  handle_change,
  change_on_blur,
  max_length,
  placeholder,
  disabled = false,
  trim = true,
  className,
}: Props) {
  const { errors, isTouched } = field.state.meta;

  return (
    <div className={`flex flex-col ${className}`}>
      <label htmlFor={id}>{label}</label>
      <input
        type={type}
        id={id}
        value={field.state.value}
        placeholder={placeholder}
        disabled={disabled}
        onChange={(e) =>
          field.handleChange(
            handle_change
              ? handle_change(e.currentTarget.value)
              : e.currentTarget.value,
          )
        }
        onBlur={() => {
          if (trim) field.handleChange(field.state.value.trim());
          if (change_on_blur)
            field.handleChange(change_on_blur(field.state.value));
          field.handleBlur();
        }}
        className="border border-neutral-800 rounded px-2 py-1 mt-2"
      />
      <div className="flex justify-between">
        <p
          className={`text-red-500 text-sm line-clamp-1 mt-1 font-semibold h-6 ${
            errors[0] && isTouched ? "visible" : "invisible"
          }`}
        >
          {errors[0]?.message}
        </p>
        {max_length && (
          <p>
            {field.state.value.length}/{max_length}
          </p>
        )}
      </div>
    </div>
  );
}
