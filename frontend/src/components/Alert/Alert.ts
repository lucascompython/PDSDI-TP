import _ from "./Alert.svelte"; // Can't remove this form some reason
import { get } from "svelte/store";
import { isErrorVisible } from "@components/stores";

export const enum AlertType {
  SUCCESS = 0,
  ERROR = 1,
  WARNING = 2,
}

type AlertPopup = HTMLElement & {
  message: string;
  type: AlertType;
};

let currentAlert: AlertPopup | null = null;

/**
 * Displays an alert popup on the screen.
 * Already has a timeout of 5 seconds to remove the alert.
 * Already handles the case where an alert is already visible.
 *
 * @param {string} message - The message to display in the alert
 * @param {AlertType} type - The type of alert to display (SUCCESS, ERROR, WARNING)
 */
export function showAlert(
  message: string,
  type: AlertType,
  parent?: HTMLElement
) {
  if (get(isErrorVisible)) {
    if (currentAlert?.type !== type) {
      currentAlert?.remove();
      isErrorVisible.set(false);
    } else {
      return;
    }
  }

  const alert = document.createElement("alert-popup") as AlertPopup;
  alert.message = message;
  alert.type = type;
  isErrorVisible.set(true);
  currentAlert = alert;

  if (parent) {
    parent.appendChild(alert);
  } else {
    document.body.appendChild(alert);
  }

  setTimeout(() => {
    isErrorVisible.set(false);
    alert.remove();
  }, 5000);
}
