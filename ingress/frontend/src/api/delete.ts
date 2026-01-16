import { ToDoItems } from "../interfaces/toDoItems";
import { deleteCall } from "./utils";
import { Url } from "./urls";

export async function deleteToDoItemCall(name: string) {
  return deleteCall<ToDoItems>(new Url().deleteUrl(name), 200);
}
