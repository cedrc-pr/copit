import { createLazyFileRoute } from "@tanstack/react-router";
import { useEffect, useState } from "react";
import { Chat } from "../components/scoped/routes/$id/Chat";
import { RoomInfos } from "../components/scoped/routes/$id/JoinRoom";
import { RoomList } from "../components/scoped/routes/RoomList";
import { get_names } from "../shared/names";

export const Route = createLazyFileRoute("/$id")({
  component: RouteComponent,
});

function RouteComponent() {
  const { id } = Route.useParams();
  const [names, set_names] = useState(get_names());
  const [connected, set_connected] = useState(0);

  useEffect(() => {
    const interval = setInterval(() => {
      set_names(get_names);
    }, 60 * 1000);
    return () => clearInterval(interval);
  }, []);

  return (
    <section className="flex-1 min-h-0 grid grid-cols-[1fr_50%_1fr] gap-10">
      <RoomList names={names} />
      <Chat
        key={id}
        id={id}
        set_names={set_names}
        set_connected={set_connected}
      />
      <section>
        <RoomInfos id={id} />
        <div className="mt-10 pt-10 border-t border-black/50 space-y-6">
          <h1 className="text-xl font-bold">Connected members</h1>
          <p>{connected} currently connected</p>
        </div>
      </section>
    </section>
  );
}
