import { useForm } from "@tanstack/react-form";
import { useGetRoom } from "../../hooks/rooms";
import { set_name_for_room } from "../../shared/names";
import { ZMessage, type Name, type Room } from "../../shared/types_and_schemas";
import { PrimaryButton } from "../shared/ui/PrimaryButton";
import { TextField } from "../shared/ui/TextField";

type Props = {
  room_id: Room["id"];
  set_name_and_update_rooms: (name: Name) => void;
};

export function SetNameFormModal({
  room_id,
  set_name_and_update_rooms,
}: Props) {
  const room = useGetRoom(room_id);

  const form = useForm({
    defaultValues: {
      author: "",
    },
    validators: {
      onSubmit: ZMessage.pick({ author: true }),
      onBlur: ZMessage.pick({ author: true }),
    },
    onSubmit: ({ value }) => {
      if (!room.data) return;
      const name = {
        name: value.author,
        room_id: room.data.id,
        expires_at: room.data.expires_at,
      };
      set_name_for_room(name);
      set_name_and_update_rooms(name);
    },
  });

  return (
    <section className="flex justify-center items-center top-0 left-0 z-10 fixed w-screen h-screen bg-black/80">
      <div className="p-10 m-4 rounded-2xl space-y-4 bg-white">
        <h1 className="font-bold text-2xl">Your name</h1>
        <form
          onSubmit={(e) => {
            e.preventDefault();
            form.handleSubmit();
          }}
        >
          <form.Field name="author">
            {(field) => (
              <TextField
                field={field}
                label="Your name for this room"
                id="author"
                max_length={ZMessage.shape.author.maxLength}
              />
            )}
          </form.Field>
          <PrimaryButton type="submit">Save</PrimaryButton>
        </form>
      </div>
    </section>
  );
}
