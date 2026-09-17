import {
  QueryClient,
  useQueryClient,
  type InfiniteData,
} from "@tanstack/react-query";
import { useNavigate, type NavigateFn } from "@tanstack/react-router";
import { useEffect, useRef } from "react";
import { config } from "../shared/config";
import { delete_name_by_room } from "../shared/names";
import {
  ZWsServerMessage,
  type Message,
  type WsClientMessage,
  type WsServerMessage,
} from "../shared/types_and_schemas";

export function useChatWs(
  room_id: string,
  new_message: () => void,
  set_connected: React.Dispatch<React.SetStateAction<number>>,
  set_typing_count: React.Dispatch<React.SetStateAction<number>>,
) {
  const wsRef = useRef<WebSocket | null>(null);
  const qc = useQueryClient();
  const navigate = useNavigate();

  useEffect(() => {
    const ws = new WebSocket(`${config.api_url}/ws/chat`);
    wsRef.current = ws;

    ws.onopen = () => {
      const joinEvent: WsClientMessage = {
        action: "join-room",
        data: { id: room_id },
      };
      ws.send(JSON.stringify(joinEvent));
    };

    ws.onmessage = (event) => {
      const parsed = JSON.parse(event.data);
      const server_message = ZWsServerMessage.parse(parsed);
      handle_server_message(
        qc,
        navigate,
        server_message,
        room_id,
        new_message,
        set_connected,
        set_typing_count,
      );
    };

    return () => {
      ws.close();
    };
  }, [room_id, qc]);

  function send(message: WsClientMessage) {
    if (wsRef.current && wsRef.current.readyState === WebSocket.OPEN) {
      wsRef.current.send(JSON.stringify(message));
    }
  }

  return { send };
}

function handle_server_message(
  qc: QueryClient,
  navigate: NavigateFn,
  server_message: WsServerMessage,
  room_id: string,
  new_message: () => void,
  set_connected: React.Dispatch<React.SetStateAction<number>>,
  set_typing_count: React.Dispatch<React.SetStateAction<number>>,
) {
  switch (server_message.type) {
    case "message":
      new_message();
      const message = server_message.data.message;
      qc.setQueryData<InfiniteData<Message[]>>(
        ["rooms", room_id, "messages"],
        (data) => {
          if (!data) return data;
          return {
            ...data,
            pages: data.pages.map((page, index) =>
              index === data.pages.length - 1 ? [...page, message] : page,
            ),
          };
        },
      );
      break;
    case "error":
      switch (server_message.data.code) {
        case "room_not_found":
          delete_name_by_room(room_id);
          qc.invalidateQueries({ queryKey: ["rooms", room_id] });
          navigate({ to: "/" });
          break;
      }
      break;
    case "connected_users":
      set_connected(server_message.data.count);
      break;
    case "connected":
      set_connected((prev) => prev + 1);
      break;
    case "disconnected":
      set_connected((prev) => prev - 1);
      break;
    case "typing":
      if (server_message.data.status) {
        set_typing_count((prev) => prev + 1);
      } else {
        set_typing_count((prev) => prev - 1);
      }
      break;
  }
}
