const API_BASE_URL = "https://127.0.0.1:1234";

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

export interface ClotheResponse {
  id: number;
  name: string;
  color: string;
  category: string;
  user_id: number;
  is_hot_weather: boolean;
  image?: File;
}

export async function getClothes(): Promise<{
  clothes: ClotheResponse[];
  images: { [key: string]: Blob };
}> {
  console.log("ola");
  const response = await fetch(`${API_BASE_URL}/clothes/get`, {
    method: "GET",
    credentials: "include",
  });

  if (!response.ok) {
    throw new Error("Failed to fetch clothes");
  }

  const contentType = response.headers.get("Content-Type");
  if (!contentType || !contentType.includes("multipart/form-data")) {
    throw new Error("Invalid response format");
  }

  const boundary = contentType.split("boundary=")[1].trim();
  const arrayBuffer = await response.arrayBuffer();
  // const text = await response.text();
  const text = new TextDecoder().decode(arrayBuffer);
  const parts = text.split(`--${boundary}`);

  const clothes: ClotheResponse[] = [];
  const images: { [key: string]: Blob } = {};

  let offset = 0;

  for (const part of parts) {
    // if (part === "") {
    //   continue;
    // }

    if (part === "") {
      continue;
    }
    // console.log(part);

    if (part.includes('Content-Disposition: form-data; name="clothe"')) {
      const json = part.split("\r\n\r\n")[1].split("\r\n")[0];
      const clothe = JSON.parse(json);
      clothes.push(clothe);
    } else if (part.includes('Content-Disposition: form-data; name="file"')) {
      const headers = part.split("\r\n\r\n")[0];
      const match = headers.match(/filename="(.+?)"/);
      if (!match) {
        throw new Error("Filename not found in headers");
      }
      const filename = match[1];
      const fileContentIndex = part.indexOf("\r\n\r\n") + 4;
      const fileContentStart = offset + fileContentIndex + 2;

      const fileContentEnd =
        fileContentStart + part.split("\r\n\r\n")[1].length;
      const fileContent = arrayBuffer.slice(fileContentStart, fileContentEnd);
      console.log(fileContent);
      const blob = new Blob([fileContent], {
        type: "application/octet-stream",
      });
      images[filename] = blob;
    }
    offset += part.length + boundary.length + 4; // 4 for the "--" and "\r\n"
  }

  return { clothes, images };
}
