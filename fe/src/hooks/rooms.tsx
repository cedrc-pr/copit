import { useInfiniteQuery, useMutation, useQuery } from "@tanstack/react-query";
import { type NavigateFn } from "@tanstack/react-router";
import z from "zod/v4";
import { api } from "../shared/api";
import {
  ZMessage,
  ZRoom,
  type FetchError,
  type Message,
  type Room,
} from "../shared/types_and_schemas";

export function useCreateRoom(navigate: NavigateFn) {
  return useMutation<Room, FetchError, Omit<Room, "created_at" | "expires_at">>(
    {
      mutationFn: async (room_dto) => {
        const res = await api.post("/rooms", room_dto);
        return ZRoom.parse(res.data);
      },
      onSuccess: (room) => {
        navigate({ to: `/${room.id}` });
      },
    },
  );
}

export function useGetRoom(room_id: Room["id"]) {
  return useQuery<Room, FetchError>({
    queryKey: ["rooms", room_id],
    queryFn: async () => {
      const res = await api.get(`/rooms/${room_id}`);
      return ZRoom.parse(res.data);
    },
  });
}

export function useJoinRoom(navigate: NavigateFn) {
  return useMutation<Room, FetchError, Pick<Room, "id">>({
    mutationFn: async ({ id }) => {
      const res = await api.get(`/rooms/${id}`);
      return ZRoom.parse(res.data);
    },
    onSuccess: (room) => {
      navigate({ to: `/${room.id}` });
    },
  });
}

// export function useGetRoomMessages(id: string) {
//   return useQuery<Message[], FetchError>({
//     queryKey: ["rooms", id, "messages"],
//     queryFn: async () => {
//       const res = await api.get(`/rooms/${id}/messages`);
//       return z.array(ZMessage).parse(res.data);
//     },
//   });
// }

export function useGetRoomMessages(id: string) {
  return useInfiniteQuery<Message[], FetchError>({
    queryKey: ["rooms", id, "messages"],
    initialPageParam: undefined,
    queryFn: async ({ pageParam }) => {
      const res = await api.get(`/rooms/${id}/messages`, {
        params: pageParam ? { before: pageParam } : undefined,
      });
      return z.array(ZMessage).parse(res.data);
    },
    getPreviousPageParam: (messages) => messages[0]?.id,
    getNextPageParam: () => {},
  });
}
