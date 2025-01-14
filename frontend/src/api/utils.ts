const API_BASE_URL = "http://localhost:1234";

// Max index -> 11
export enum Color {
  Red = "Gray",
  Orange = "Black",
  Yellow = "White",
  Green = "Brown",
  Blue = "Pink",
  Purple = "Purple",
  Pink = "Blue",
  Brown = "Green",
  White = "Yellow",
  Black = "Orange",
  Gray = "Red",
  Gold = "Gold",
}
// Max index -> 10
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
  const formData = new FormData();

  for (let i = 0; i < clothes.length; i++) {
    formData.append("clothe", JSON.stringify(clothes[i]));
    formData.append("image", clothes[i].image);
  }

  const response = await fetch(`${API_BASE_URL}/clothes/upload`, {
    method: "POST",
    credentials: "include",
    body: formData,
  });

  return response.ok;
}
