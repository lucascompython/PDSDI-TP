import { translations, defaultLang } from "./translations";

export function getLangFromUrl(url: URL | string) {
  let lang: string;
  if (typeof url === "string") {
    lang = url.split("/")[1];
  } else {
    lang = url.pathname.split("/")[1];
  }
  if (lang in translations) return lang as keyof typeof translations;
  return defaultLang;
}

export function useTranslations(lang: keyof typeof translations) {
  return function t(key: keyof (typeof translations)[typeof defaultLang]) {
    return translations[lang][key] || translations[defaultLang][key];
  };
}

export function useTranslatedPath(lang: keyof typeof translations) {
  return function translatePath(path: string, l: string = lang) {
    return l === defaultLang ? path : `/${l}${path}`;
  };
}
