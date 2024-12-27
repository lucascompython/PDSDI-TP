import { translations, defaultLang } from "./translations";

export function getLangFromUrl(url: URL | string) {
  let lang: string;
  if (typeof url === "string") {
    const [, l] = url.split("/");
    lang = l;
  }
  else {
    const [, l] = url.pathname.split("/");
    lang = l;
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
