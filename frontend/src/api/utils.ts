const API_BASE_URL = "http://pauloministro.com:6969";

// Max index -> 11
export enum Color {
  Red = "gray",
  Orange = "black",
  Yellow = "white",
  Green = "brown",
  Blue = "pink",
  Purple = "purple",
  Pink = "blue",
  Brown = "green",
  White = "yellow",
  Black = "orange",
  Gray = "red",
  Gold = "gold",
}
// Max index -> 10
export enum ClotheCategory {
  Shirt = "shirt",
  Pants = "pants",
  Shorts = "shorts",
  Dress = "dress",
  Skirt = "skirt",
  Jacket = "jacket",
  Sweater = "sweater",
  Shoes = "shoes",
  Hat = "hat",
  Gloves = "gloves",
  Scarf = "scarf",
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
