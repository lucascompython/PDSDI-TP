import { type AstroIntegration, type RouteData } from "astro";
import { type AddressInfo } from "node:net";
import { getLangFromUrl, useTranslations } from "src/i18n/utils";
import { languages, defaultLang, translations } from "src/i18n/translations";

const translate = async () => {
  console.log("translate");
  const regex = /t\("([^"]+)"\)/g;

  //   const t = useTranslations(lang);
  const file = Bun.file("src/pages/index.astro");
  const text = await file.text();

  const t = useTranslations("en");

  const matches = text.match(regex);
  console.log(`matches: ${matches}`);
};

export default function I18nIntegration(): AstroIntegration {
  return {
    name: "I18nIntegration",
    hooks: {
      // ver astro:build:setup change page
      "astro:build:done": async (options: {
        dir: URL;
        routes: RouteData[];
        pages: { pathname: string }[];
      }) => {
        console.log("release");
        await translate();
      },
      "astro:server:start": async (options: { address: AddressInfo }) => {
        console.log("dev");
        await translate();
      },
    },
  };
}
