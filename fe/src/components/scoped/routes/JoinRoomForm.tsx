import { useForm } from "@tanstack/react-form";
import { useNavigate } from "@tanstack/react-router";
import slugify from "slugify";
import { useJoinRoom } from "../../../hooks/rooms";
import { ZRoom } from "../../../shared/types_and_schemas";
import { FetchErrorDisplay } from "../../shared/ui/FetchErrorDisplay";
import { PrimaryButton } from "../../shared/ui/PrimaryButton";
import { TextField } from "../../shared/ui/TextField";

export function JoinRoomForm() {
  const room = useJoinRoom(useNavigate());

  const form = useForm({
    defaultValues: {
      id: "",
    },
    validators: {
      onBlur: ZRoom.pick({ id: true }),
      onSubmit: ZRoom.pick({ id: true }),
    },
    onSubmit: ({ value }) => {
      room.mutate(value);
    },
  });

  return (
    <section className="mb-10 pb-10 border-b border-black/50">
      {room.isError && <FetchErrorDisplay error={room.error} />}
      <h1 className="font-bold mb-4">Join a room</h1>
      <form
        onSubmit={(e) => {
          e.preventDefault();
          form.handleSubmit();
        }}
      >
        <fieldset disabled={room.isPending}>
          <form.Field name="id">
            {(field) => (
              <TextField
                field={field}
                label="Room name"
                id="id"
                max_length={ZRoom.shape.id.maxLength}
                change_on_blur={(value) =>
                  slugify(value, { trim: true, strict: true })
                }
              />
            )}
          </form.Field>
          <PrimaryButton type="submit">Join</PrimaryButton>
        </fieldset>
      </form>
    </section>
  );
}
