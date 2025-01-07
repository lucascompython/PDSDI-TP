import { useTranslatedPath, getLangFromUrl } from "src/i18n/utils";
import { checkUser } from "src/api/utils";
import { isAdmin } from "@components/stores";

// Client side middleware
export default async function onRequest(
  url: string,
  pathname: string
): Promise<string | null> {
  const result = await checkUser();

  const translatePath = useTranslatedPath(getLangFromUrl(url));
  const isLoginPage = pathname === "/login" || pathname === "/pt/login";

  if (result.is_ok) {
    isAdmin.set(result.is_admin);
    if (isLoginPage) {
      return translatePath("/");
    }
  } else {
    if (!isLoginPage) {
      return translatePath("/login");
    }
  }

  return null;
}
