import { useForm } from "@tanstack/react-form";
import { useNavigate } from "@tanstack/react-router";
import { useState } from "react";
import slugify from "slugify";
import {
  adjectives,
  animals,
  uniqueNamesGenerator,
} from "unique-names-generator";
import { useCreateRoom } from "../../../hooks/rooms";
import stars_svg from "../../../img/svg/stars.svg";
import { ZRoom } from "../../../shared/types_and_schemas";
import { FetchErrorDisplay } from "../../shared/ui/FetchErrorDisplay";
import { PrimaryButton } from "../../shared/ui/PrimaryButton";
import { Spinner } from "../../shared/ui/Spinner";

export function CreateRoomForm() {
  const create_room = useCreateRoom(useNavigate());
  const [generate, set_generate] = useState(false);

  const form = useForm({
    defaultValues: {
      id: "",
    },
    validators: {
      onSubmit: ZRoom.pick({ id: true }),
      onBlur: ZRoom.pick({ id: true }),
    },
    onSubmit: ({ value }) => {
      create_room.mutate(value);
    },
  });

  function generate_alphanum_code(length = 6, with_lower_case = false) {
    const chars = `${with_lower_case ? "abcdefghijklmnopqrstuvwxyz" : ""}${"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"}`;
    return Array.from(
      { length },
      () => chars[Math.floor(Math.random() * chars.length)],
    ).join("");
  }

  function generate_code(type: "memorable" | "short" | "strong" | "maximum") {
    let res = "";
    switch (type) {
      case "memorable":
        res = uniqueNamesGenerator({
          dictionaries: [adjectives, animals],
          separator: "-",
          style: "lowerCase",
        });
        break;
      case "short":
        res = generate_alphanum_code();
        break;
      case "strong":
        res = `${generate_alphanum_code()}-${generate_alphanum_code()}-${generate_alphanum_code()}`;
        break;
      case "maximum":
        res = generate_alphanum_code(50, true);
    }
    form.reset({ id: res });
  }

  return (
    <form
      onSubmit={(e) => {
        e.preventDefault();
        form.handleSubmit();
      }}
    >
      <h1 className="font-bold mb-4">Create a new room</h1>
      {create_room.isError && (
        <FetchErrorDisplay error={create_room.error} className="mb-6" />
      )}
      <fieldset disabled={create_room.isPending}>
        <form.Field name="id">
          {(field) => {
            const { errors, isTouched } = field.state.meta;

            return (
              <div>
                <label htmlFor="create_room">Room name</label>
                <div className="border rounded">
                  <div className="flex">
                    <input
                      className="flex-1 pl-2 pb-1 pt-2 rounded-l"
                      id="create_room"
                      onChange={(e) => field.handleChange(e.target.value)}
                      value={field.state.value}
                      onBlur={(e) => {
                        const slugified = slugify(e.target.value, {
                          trim: true,
                          strict: true,
                        });
                        field.handleChange(slugified);
                        field.handleBlur();
                      }}
                    ></input>
                    <button
                      type="button"
                      className="rounded-r cursor-pointer"
                      onClick={() => set_generate((prev) => !prev)}
                    >
                      <img
                        src={stars_svg}
                        alt="stars"
                        title="generate"
                        className="px-4 w-13"
                      />
                    </button>
                  </div>
                  {generate && (
                    <div className="flex flex-col">
                      <button
                        type="button"
                        className="flex justify-between items-center pl-6 pr-14 py-1 hover:bg-neutral-100 cursor-pointer transition-colors duration-200 border-t"
                        onClick={() => generate_code("memorable")}
                      >
                        <span>memorable</span>
                        <code className="text-neutral-800 text-sm">
                          clever-tiger
                        </code>
                      </button>
                      <button
                        type="button"
                        className="flex justify-between items-center pl-6 pr-14 py-1 hover:bg-neutral-100 cursor-pointer transition-colors duration-200 border-t"
                        onClick={() => generate_code("short")}
                      >
                        <span>short</span>
                        <code className="text-neutral-800 text-sm">AFJ52K</code>
                      </button>
                      <button
                        type="button"
                        className="flex justify-between items-center pl-6 pr-14 py-1 hover:bg-neutral-100 cursor-pointer transition-colors duration-200 border-t"
                        onClick={() => generate_code("strong")}
                      >
                        <span>strong</span>
                        <code className="text-neutral-800 text-sm">
                          AFJ52K-Q9T2LX-7MNP4R
                        </code>
                      </button>
                      <button
                        type="button"
                        className="flex justify-between items-center pl-6 pr-14 py-1 hover:bg-neutral-100 cursor-pointer transition-colors duration-200 border-t"
                        onClick={() => generate_code("maximum")}
                      >
                        <span>maximum</span>
                        <code className="text-neutral-800 text-sm">
                          xK7mP2Qa9Lr4Tn8Vz3Yw6Bc1Hd5Fg0Jp8Rs4Nu2Wx7Ke9
                        </code>
                      </button>
                    </div>
                  )}
                </div>
                <div className="flex justify-between">
                  <p
                    className={`text-red-500 text-sm line-clamp-1 mt-1 font-semibold h-6 ${
                      errors[0] && isTouched ? "visible" : "invisible"
                    }`}
                  >
                    {errors[0]?.message}
                  </p>
                  <p>
                    {field.state.value.length}/{50}
                  </p>
                </div>
              </div>
            );
          }}
        </form.Field>
        <PrimaryButton type="submit">
          Create {create_room.isPending && <Spinner />}
        </PrimaryButton>
      </fieldset>
    </form>
  );
}
