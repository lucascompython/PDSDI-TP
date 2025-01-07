const API_BASE_URL = "http://localhost:1234";

export async function registerUser(
  email: string,
  password: string
): Promise<boolean> {
  const response = await fetch(`${API_BASE_URL}/register`, {
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
  const response = await fetch(`${API_BASE_URL}/login`, {
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
  const response = await fetch(`${API_BASE_URL}/logout`, {
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
  const response = await fetch(`${API_BASE_URL}/check`, {
    method: "GET",
    credentials: "include",
  });

  if (!response.ok) {
    return { is_ok: false, is_admin: false };
  }

  const json = await response.json();
  return { is_ok: true, is_admin: json.is_admin };
}
