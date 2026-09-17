import z from "zod/v4";
import { ZName, type Name } from "./types_and_schemas";

export function get_name_for_room(room_id: string) {
  const names = get_names();

  const name = names.find((name) => name.room_id === room_id);
  return name;
}

export function delete_name_by_room(room_id: string) {
  let names = get_names();
  names = names.filter((name) => name.room_id !== room_id);
  save_names(names);
}

export function set_name_for_room(to_set: Name) {
  const names = get_names();
  const index = names.findIndex((name) => name.room_id === to_set.room_id);
  if (index === -1) {
    names.push(to_set);
  } else {
    names[index].name = to_set.name;
  }
  save_names(names);
}

export function get_names() {
  const raw = localStorage.getItem("names") ?? "[]";
  let names: Name[] = [];
  const parsed = z.array(ZName).safeParse(JSON.parse(raw));
  if (parsed.success) {
    names = parsed.data;
  }
  const now = new Date();
  const filtered = names.filter((name) => name.expires_at > now);
  save_names(names);
  return filtered;
}

function save_names(names: Name[]) {
  localStorage.setItem("names", JSON.stringify(names));
}
