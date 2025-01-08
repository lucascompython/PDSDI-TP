import { writable } from "svelte/store";
import type { Clothe } from "src/api/utils";
import { useTranslations } from "src/i18n/utils";

export const isErrorVisible = writable(false);
export const isAdmin = writable(false);
export const fileName = writable("");

export const clothes = writable<Clothe[]>([]);

export const t = writable<ReturnType<typeof useTranslations>>();
