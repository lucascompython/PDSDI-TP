import { writable } from "svelte/store";

export const isErrorVisible = writable(false);
export const isAdmin = writable(false);
export const fileName = writable("");
