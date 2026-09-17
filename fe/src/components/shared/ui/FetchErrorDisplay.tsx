import { AxiosError } from "axios";
import type { FetchError } from "../../../shared/types_and_schemas";

type Props = {
  error: FetchError;
  pretty_error_message?: (error: AxiosError<{ message: string }>) => string;
  className?: string;
};

export function default_pretty_error_display(
  error: AxiosError<{ message: string }>,
) {
  if (
    error.config?.url === "/rooms" &&
    error.config.method === "post" &&
    error.status === 409
  ) {
    return "a room with this name already exists, please choose another one";
  }
  if (
    error.config?.url?.startsWith("/rooms/") &&
    error.config.method === "get" &&
    error.status === 404
  ) {
    return "no room with this name";
  }
  return "";
}

export function FetchErrorDisplay({
  error,
  pretty_error_message,
  className,
}: Props) {
  if (error instanceof AxiosError && error.response) {
    return (
      <h2
        className={`text-red-700 bg-red-200 py-2 mt-1 font-semibold rounded text-center ${className}`}
      >
        {pretty_error_message
          ? pretty_error_message(error)
          : default_pretty_error_display(error)}
      </h2>
    );
  }

  return (
    <h1
      className={`text-red-700 beg-red-200 py-2 font-semibold rounded ${className}`}
    >
      An error occuried
    </h1>
  );
}
