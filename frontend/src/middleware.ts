import type { MiddlewareHandler } from "astro";
import { checkUser } from "./api/utils";
import { useTranslatedPath, getLangFromUrl } from "./i18n/utils";

export const onRequest: MiddlewareHandler = async (context, next) => {
  const url = new URL(context.request.url);

  const lang = getLangFromUrl(url);
  const translatePath = useTranslatedPath(lang);

  const cookies = context.request.headers.get("Cookie");
  const sessionId = cookies?.split(";").find((c) => c.trim().startsWith("id="));

  const isLoginPage = url.pathname === "/login" || url.pathname === "/pt/login";

  if (!sessionId) {
    if (!isLoginPage) {
      return Response.redirect(translatePath("/login"), 302);
    }
  } else {
    const result = await checkUser(sessionId);

    if (result.is_ok) {
      context.locals.isAdmin = result.is_admin;
      if (isLoginPage) {
        return Response.redirect(translatePath("/"), 302);
      }
    } else {
      if (!isLoginPage) {
        return Response.redirect(translatePath("/login"), 302);
      }
    }
  }
  return next();
};
