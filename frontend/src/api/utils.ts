const API_BASE_URL = "https://pauloministro.com:6969";
export type { ClotheResponse }; // re-export Clothe from cbf.d.ts
import init, { Clothe as ClotheResponse } from "../../../cbf/pkg/cbf.js";
import type { BoolPack, OutfitType } from "./bool_pack";

export enum Color {
  Red = "Red",
  Orange = "Orange",
  Yellow = "Yellow",
  Green = "Green",
  Blue = "Blue",
  Purple = "Purple",
  Pink = "Pink",
  Brown = "Brown",
  Black = "Black",
  White = "White",
  Gold = "Gold",
  Gray = "Gray",
}

function colorToNumber(color: Color): number {
  switch (color) {
    case Color.Red:
      return 0;
    case Color.Orange:
      return 1;
    case Color.Yellow:
      return 2;
    case Color.Green:
      return 3;
    case Color.Blue:
      return 4;
    case Color.Purple:
      return 5;
    case Color.Pink:
      return 6;
    case Color.Brown:
      return 7;
    case Color.Black:
      return 8;
    case Color.White:
      return 9;
    case Color.Gold:
      return 10;
    case Color.Gray:
      return 11;
  }
}

export function numberToColor(number: number): Color {
  switch (number) {
    case 0:
      return Color.Red;
    case 1:
      return Color.Orange;
    case 2:
      return Color.Yellow;
    case 3:
      return Color.Green;
    case 4:
      return Color.Blue;
    case 5:
      return Color.Purple;
    case 6:
      return Color.Pink;
    case 7:
      return Color.Brown;
    case 8:
      return Color.Black;
    case 9:
      return Color.White;
    case 10:
      return Color.Gold;
    case 11:
      return Color.Gray;
    default:
      throw new Error(`Invalid color number: ${number}`);
  }
}

export enum ClotheCategory {
  Shirt = "Shirt",
  Pants = "Pants",
  Shorts = "Shorts",
  Dress = "Dress",
  Skirt = "Skirt",
  Jacket = "Jacket",
  Sweater = "Sweater",
  Shoes = "Shoes",
  Hat = "Hat",
  Gloves = "Gloves",
  Scarf = "Scarf",
}

function categoryToNumber(category: ClotheCategory): number {
  switch (category) {
    case ClotheCategory.Shirt:
      return 0;
    case ClotheCategory.Pants:
      return 1;
    case ClotheCategory.Shorts:
      return 2;
    case ClotheCategory.Dress:
      return 3;
    case ClotheCategory.Skirt:
      return 4;
    case ClotheCategory.Jacket:
      return 5;
    case ClotheCategory.Sweater:
      return 6;
    case ClotheCategory.Shoes:
      return 7;
    case ClotheCategory.Hat:
      return 8;
    case ClotheCategory.Gloves:
      return 9;
    case ClotheCategory.Scarf:
      return 10;
  }
}

export function numberToCategory(number: number): ClotheCategory {
  switch (number) {
    case 0:
      return ClotheCategory.Shirt;
    case 1:
      return ClotheCategory.Pants;
    case 2:
      return ClotheCategory.Shorts;
    case 3:
      return ClotheCategory.Dress;
    case 4:
      return ClotheCategory.Skirt;
    case 5:
      return ClotheCategory.Jacket;
    case 6:
      return ClotheCategory.Sweater;
    case 7:
      return ClotheCategory.Shoes;
    case 8:
      return ClotheCategory.Hat;
    case 9:
      return ClotheCategory.Gloves;
    case 10:
      return ClotheCategory.Scarf;
    default:
      throw new Error(`Invalid category number: ${number}`);
  }
}

export interface Clothe {
  name: string;
  color: Color;
  category: ClotheCategory;
  isForHotWeather: boolean;
  image: File;
}

export async function registerUser(
  email: string,
  password: string
): Promise<boolean> {
  const response = await fetch(`${API_BASE_URL}/user/register`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ email, password }),
  });

  return response.ok;
}

export async function loginUser(
  email: string,
  password: string
): Promise<boolean> {
  const response = await fetch(`${API_BASE_URL}/user/login`, {
    method: "POST",
    credentials: "include",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ email, password }),
  });

  return response.ok;
}

export async function logoutUser(): Promise<boolean> {
  const response = await fetch(`${API_BASE_URL}/user/logout`, {
    method: "POST",
    credentials: "include",
  });
  return response.ok;
}

interface checkUserResponse {
  is_ok: boolean;
  is_admin: boolean;
}

export async function checkUser(): Promise<checkUserResponse> {
  const response = await fetch(`${API_BASE_URL}/user/check`, {
    method: "GET",
    credentials: "include",
  });

  if (!response.ok) {
    return { is_ok: false, is_admin: false };
  }

  const json = await response.json();
  return { is_ok: true, is_admin: json.is_admin };
}

export async function uploadClothes(clothes: Clothe[]): Promise<boolean> {
  const { serializeClothesToBytes } = await import("../../../cbf/pkg");
  await init();

  const clotheResponses = [];

  for (const clothe of clothes) {
    const { name, color, category, isForHotWeather, image } = clothe;
    clotheResponses.push(
      new ClotheResponse(
        0, // dummy value, the server will assign an id
        name,
        colorToNumber(color),
        categoryToNumber(category),
        0, // dummy value, the server will use the cookie to get the user id
        isForHotWeather,
        image.name,
        new Uint8Array(await image.arrayBuffer())
      )
    );
  }
  const serialized = serializeClothesToBytes(clotheResponses);

  const response = await fetch(`${API_BASE_URL}/clothes/upload`, {
    method: "POST",
    headers: {
      "Content-Type": "application/octet-stream",
    },
    credentials: "include",
    body: serialized,
  });

  return response.ok;
}

export async function getClothes(): Promise<ClotheResponse[]> {
  const { deserializeClothesFromBytes } = await import(
    "../../../cbf/pkg/cbf.js"
  );

  await init();

  const response = await fetch(`${API_BASE_URL}/clothes/get`, {
    method: "GET",
    credentials: "include",
  });

  if (!response.ok) {
    return []; // No clothes
  }

  const buffer = await response.arrayBuffer();
  const clothes = deserializeClothesFromBytes(new Uint8Array(buffer));

  return clothes;
}

export async function generateOutfit(
  boolPack: BoolPack
): Promise<ClotheResponse[]> {
  const { deserializeClothesFromBytes } = await import(
    "../../../cbf/pkg/cbf.js"
  );
  await init();

  const response = await fetch(`${API_BASE_URL}/outfits/generate`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    credentials: "include",
    body: JSON.stringify({ bool_pack: boolPack.toNumber() }),
  });

  if (!response.ok) {
    return [];
  }

  const buffer = await response.arrayBuffer();
  const clothes = deserializeClothesFromBytes(new Uint8Array(buffer));

  return clothes;
}

export async function saveOutfit(
  name: string,
  outfitType: OutfitType,
  clothes: number[]
): Promise<boolean> {
  const response = await fetch(`${API_BASE_URL}/outfits/save`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    credentials: "include",
    body: JSON.stringify({
      name,
      outfit_type: outfitType,
      clothes: clothes,
    }),
  });

  return response.ok;
}
