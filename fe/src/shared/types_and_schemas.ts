import type { AxiosError } from "axios";
import type { ZodError } from "zod/v4";
import z from "zod/v4";

export type FetchError = ZodError | AxiosError<{ message: string }>;

export const ZRoom = z.object({
  id: z
    .string()
    .min(1, "should have at least one character")
    .max(50, "should have a max of 50 charaters")
    .regex(/[A-Za-z0-9-]/),
  created_at: z.coerce.date(),
  expires_at: z.coerce.date(),
});
export type Room = z.infer<typeof ZRoom>;

export const ZMessage = z.object({
  id: z.uuidv7(),
  content: z
    .string()
    .min(1, "should have at least one character")
    .max(2048, "should have a maximum of 2048 characters"),
  author: z
    .string()
    .min(1, "should have at least one caracter")
    .max(50, "should have a maximum of 50 characters"),
  created_at: z.coerce.date(),
  updated_at: z.coerce.date(),
});
export const ZMessageDto = ZMessage.omit({
  id: true,
  created_at: true,
  updated_at: true,
});
export type Message = z.infer<typeof ZMessage>;
export type MessageDto = z.infer<typeof ZMessageDto>;

export const ZName = z.object({
  name: ZMessage.shape.author,
  room_id: ZRoom.shape.id,
  expires_at: ZRoom.shape.expires_at,
});
export type Name = z.infer<typeof ZName>;

export type WsClientMessage =
  | { action: "join-room"; data: Pick<Room, "id"> }
  | {
      action: "send-message";
      data: MessageDto;
    }
  | { action: "typing"; data: { status: boolean } };

export const ZWsServerMessage = z
  .object({
    type: z.literal("message"),
    data: z.object({ message: ZMessage }),
  })
  .or(
    z.object({
      type: z.literal("error"),
      data: z.object({
        code: z.enum([
          "room_not_found",
          "not_in_a_room",
          "invalid_message",
          "unexpected",
        ]),
      }),
    }),
  )
  .or(
    z.object({
      type: z.literal("connected_users"),
      data: z.object({ count: z.coerce.number().int().positive() }),
    }),
  )
  .or(z.object({ type: z.literal("connected") }))
  .or(z.object({ type: z.literal("disconnected") }))
  .or(
    z.object({
      type: z.literal("typing"),
      data: z.object({
        status: z.boolean(),
      }),
    }),
  );

export type WsServerMessage = z.infer<typeof ZWsServerMessage>;
