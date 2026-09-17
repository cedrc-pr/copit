import { createFileRoute } from "@tanstack/react-router";
import { ZRoom } from "../shared/types_and_schemas";

export const Route = createFileRoute("/$id")({
  parseParams: (params) => ZRoom.pick({ id: true }).parse(params),
});
