import type { ClotheResponse } from "src/api/utils";
import { writable } from "svelte/store";

export const isErrorVisible = writable(false);
export const isAdmin = writable(false);
export const fileName = writable("");

export const currentIndex = writable(0);

export const clothes = writable<ClotheResponse[]>([]);
export const loading = writable(false);
