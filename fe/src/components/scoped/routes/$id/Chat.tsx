import { useForm } from "@tanstack/react-form";
import { useLayoutEffect } from "@tanstack/react-router";
import React, { useEffect, useRef, useState } from "react";
import { useChatWs } from "../../../../hooks/messages";
import { useGetRoomMessages } from "../../../../hooks/rooms";
import arrow_down_svg from "../../../../img/svg/arrow_down.svg";
import { get_name_for_room, get_names } from "../../../../shared/names";
import {
  ZMessage,
  type Name,
  type Room,
} from "../../../../shared/types_and_schemas";
import { SetNameFormModal } from "../../../modals/SetNameFormModal";
import { FetchErrorDisplay } from "../../../shared/ui/FetchErrorDisplay";
import { Spinner } from "../../../shared/ui/Spinner";

type Props = {
  id: Room["id"];
  set_names: React.Dispatch<
    React.SetStateAction<
      {
        name: string;
        room_id: string;
        expires_at: Date;
      }[]
    >
  >;
  set_connected: React.Dispatch<React.SetStateAction<number>>;
};

export function Chat({ id, set_names, set_connected }: Props) {
  const messages = useGetRoomMessages(id);
  const message_list = messages.data?.pages.flatMap((page) => page) ?? [];
  const [name, set_name] = useState<Name | undefined>(get_name_for_room(id));
  const [unread, set_unread] = useState(0);
  const [is_scrolling, set_is_scrolling] = useState(false);
  const list = useRef<HTMLUListElement>(null);
  const textarea = useRef<HTMLTextAreaElement>(null);
  const previous_scroll_height = useRef<number | null>(null);
  const is_initial_mount = useRef(true);
  const new_message = useRef(false);
  const [is_typing, set_is_typing] = useState(false);
  const typing_timeout = useRef<ReturnType<typeof setTimeout> | null>(null);
  const [typing_count, set_typing_count] = useState(0);
  const chat = useChatWs(
    id,
    () => (new_message.current = true),
    set_connected,
    set_typing_count,
  );

  const form = useForm({
    defaultValues: {
      content: "",
    },
    validators: {
      onSubmit: ZMessage.pick({ content: true }),
    },
    onSubmit: ({ value }) => {
      if (!name) return;
      chat.send({
        action: "send-message",
        data: { ...value, author: name.name },
      });
      form.reset();
      requestAnimationFrame(() => textarea.current?.focus());
      scroll_to_bottom();
    },
  });

  const last_message = message_list.at(-1);

  useLayoutEffect(() => {
    if (!messages.isSuccess || !list.current) return;
    requestAnimationFrame(() => {
      scroll_to_bottom();
      set_unread(0);
    });
  }, [messages.isSuccess, name]);

  useLayoutEffect(() => {
    if (is_initial_mount.current) {
      is_initial_mount.current = false;
      return;
    }
    if (near_last_message()) {
      scroll_to_bottom();
    } else {
      set_unread((prev) => prev + 1);
    }
  }, [last_message?.id]);

  useLayoutEffect(() => {
    if (!list.current || !previous_scroll_height.current) return;
    if (new_message.current) {
      new_message.current = false;
      return;
    }
    list.current.scrollTop +=
      list.current.scrollHeight - previous_scroll_height.current;
    previous_scroll_height.current = null;
  }, [message_list.length]);

  useEffect(() => {
    if (is_scrolling) return;
    const timer = setTimeout(() => {
      set_unread(0);
    }, 2000);
    return () => clearTimeout(timer);
  }, [is_scrolling]);

  useEffect(() => {
    return () => {
      if (typing_timeout.current) clearTimeout(typing_timeout.current);
    };
  }, []);

  function set_name_and_update_rooms(name: Name) {
    set_name(name);
    set_names(get_names());
  }

  function near_last_message() {
    if (!list.current) return;
    const height = list.current.scrollHeight;
    const client_height = list.current.clientHeight;
    const from_top = list.current.scrollTop;
    const pixels_from_end = height - client_height - from_top;
    if (list.current) {
      return pixels_from_end < 100;
    }
    return false;
  }

  function update_textarea_height(textarea: EventTarget & HTMLTextAreaElement) {
    requestAnimationFrame(() => {
      textarea.style.height = "auto";
      textarea.style.height = `${textarea.scrollHeight}px`;
    });
  }

  function scroll_to_bottom() {
    if (!list.current) return;
    list.current.scrollTop = list.current.scrollHeight;
  }

  if (messages.isPending)
    return (
      <Spinner
        message={"loading messages"}
        className="text-lg items-center justify-center px-10 border-x border-black/50"
      />
    );
  if (messages.isError) return <FetchErrorDisplay error={messages.error} />;

  const real_typing_count = is_typing ? typing_count - 1 : typing_count;

  return (
    <>
      {!name && (
        <SetNameFormModal
          room_id={id}
          set_name_and_update_rooms={set_name_and_update_rooms}
        />
      )}
      <main className="relative flex flex-col h-full min-h-0 px-10 border-x border-black/50">
        <h1 className="text-xl font-bold mb-6">Chat</h1>
        {name && (
          <>
            <ul
              ref={list}
              onScroll={async (e) => {
                set_is_scrolling(!near_last_message());
                previous_scroll_height.current = e.currentTarget.scrollHeight;
                if (
                  e.currentTarget.scrollTop <= 50 &&
                  messages.hasPreviousPage &&
                  !messages.isFetchingPreviousPage
                ) {
                  await messages.fetchPreviousPage();
                }
              }}
              className="flex flex-1 flex-col space-y-4 min-h-0 overflow-y-auto scrollbar-none"
            >
              {message_list.map((message, index) => {
                const is_me = message.author === name?.name;
                return (
                  <React.Fragment key={message.id}>
                    {unread === message_list.length - index && (
                      <li>
                        <p className="text-neutral-600 text-sm border-b border-neutral-600/50 pl-4">
                          unread messages
                        </p>
                      </li>
                    )}
                    <li className={`flex flex-col ${is_me ? "items-end" : ""}`}>
                      <div className="w-2/3 border px-4 py-1 rounded-xl">
                        {!is_me && (
                          <p className="text-neutral-600">~ {message.author}</p>
                        )}
                        <p className="wrap-break-word">{message.content}</p>
                      </div>
                    </li>
                  </React.Fragment>
                );
              })}
            </ul>
            {is_scrolling && (
              <div className="flex justify-end">
                {unread !== 0 && (
                  <div className="absolute bottom-25.5 right-15 text-xs bg-neutral-400 rounded-full w-5 text-center pt-0.75 z-10 text-white">
                    <span>{unread}</span>
                  </div>
                )}
                <button
                  className="absolute bottom-20 rounded-full opacity-75 bg-neutral-200 cursor-pointer"
                  onClick={() => scroll_to_bottom()}
                >
                  <img
                    src={arrow_down_svg}
                    title="scroll to bottom"
                    className="w-8"
                  ></img>
                </button>
              </div>
            )}
            {real_typing_count > 0 && (
              <p className="text-[13px] text-neutral-600 absolute bottom-11">
                {real_typing_count} {real_typing_count === 1 ? "is" : "are"}{" "}
                typing
              </p>
            )}
            <form className="pt-4">
              <form.Field name="content">
                {(field) => (
                  <textarea
                    ref={textarea}
                    value={field.state.value}
                    placeholder="your message ..."
                    onKeyDown={(e) => {
                      if (e.key === "Enter" && !e.shiftKey) {
                        e.preventDefault();
                        form.handleSubmit();
                        update_textarea_height(e.currentTarget);
                      }
                    }}
                    onChange={(e) => {
                      field.handleChange(e.target.value);
                      update_textarea_height(e.currentTarget);
                      if (typing_timeout.current) {
                        clearTimeout(typing_timeout.current);
                      } else {
                        set_is_typing(true);
                        chat.send({
                          action: "typing",
                          data: { status: true },
                        });
                      }
                      typing_timeout.current = setTimeout(() => {
                        set_is_typing(false);
                        chat.send({
                          action: "typing",
                          data: { status: false },
                        });
                        typing_timeout.current = null;
                      }, 5000);
                    }}
                    onBlur={(e) => {
                      field.handleBlur();
                      field.handleChange(e.target.value.trim());
                      update_textarea_height(e.currentTarget);
                    }}
                    rows={1}
                    className="resize-none border px-4 py-2 w-full rounded-lg mt-4"
                  />
                )}
              </form.Field>
            </form>
          </>
        )}
      </main>
    </>
  );
}
