import { Link } from "@tanstack/react-router";
import { formatDuration, intervalToDuration } from "date-fns";
import type { Name } from "../../../shared/types_and_schemas";

type Props = {
  names: Name[];
};

export function RoomList({ names }: Props) {
  return (
    <section>
      <h1 className="font-bold text-xl mb-6">Rooms</h1>
      {!names.length && <p className="text-neutral-600">no rooms yet ...</p>}
      <ul className="space-y-6">
        {names.map((name) => {
          const splited = window.location.href.split("/");
          const is_current = splited[splited.length - 1] === name.room_id;
          return (
            <li key={name.room_id}>
              <Link
                to="/$id"
                params={{ id: name.room_id }}
                disabled={is_current}
                className={`block px-6 py-4 rounded-xl transition-colors duration-200 ${is_current ? "" : " hover:bg-neutral-200 bg-neutral-100"}`}
              >
                <div className="flex justify-between flex-wrap">
                  <p className="line-clamp-1 break-all min-w-0">
                    {name.room_id}
                  </p>
                  <p>
                    {formatDuration(
                      intervalToDuration({
                        start: new Date(),
                        end: name.expires_at,
                      }),
                      { format: ["days", "hours", "minutes"] },
                    )}
                  </p>
                </div>
                <p className="text-neutral-600">as {name.name}</p>
              </Link>
            </li>
          );
        })}
      </ul>
    </section>
  );
}
