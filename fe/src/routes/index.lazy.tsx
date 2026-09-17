import { createLazyFileRoute } from "@tanstack/react-router";
import { CreateRoomForm } from "../components/scoped/routes/CreateRoomForm";
import { JoinRoomForm } from "../components/scoped/routes/JoinRoomForm";
import { RoomList } from "../components/scoped/routes/RoomList";
import { get_names } from "../shared/names";

export const Route = createLazyFileRoute("/")({
  component: RouteComponent,
});

function RouteComponent() {
  return (
    <div className="h-full grid grid-cols-[1fr_50%_1fr] gap-10">
      <RoomList names={get_names()} />
      <div className="space-y-10 border-l border-black/50 px-10 h-full">
        <h1 className="text-xl font-bold">What is Copit?</h1>
        <ol className="text-lg space-y-2 list-decimal ml-5">
          <li>create a room</li>
          <li>anyone with its name can join it</li>
          <li>choose your name</li>
          <li>chat in it</li>
        </ol>
        <h1 className="text-xl font-bold">Try it!</h1>
        <JoinRoomForm />
        <CreateRoomForm />
      </div>
    </div>
  );
}
