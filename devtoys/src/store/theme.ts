import { defineStore } from "pinia";
import { ref, computed, watch } from "vue";
import { darkTheme } from "naive-ui";
import { getCurrentWindow } from "@tauri-apps/api/window";

const STORAGE_KEY = "devtoys-theme-dark";

function getInitialDark(): boolean {
  const saved = localStorage.getItem(STORAGE_KEY);
  if (saved !== null) {
    return saved === "true";
  }
  return window.matchMedia("(prefers-color-scheme: dark)").matches;
}

async function syncWindowTheme(isDark: boolean) {
  try {
    await getCurrentWindow().setTheme(isDark ? "dark" : "light");
  } catch {
    // 非 Tauri 环境（如 vite dev）下忽略
  }
}

export const useThemeStore = defineStore("theme", () => {
  const dark = ref(getInitialDark());

  const theme = computed(() => {
    return dark.value ? darkTheme : null;
  });

  function setDark(value: boolean) {
    dark.value = value;
  }

  function toggleTheme() {
    dark.value = !dark.value;
  }

  watch(
    dark,
    (value) => {
      localStorage.setItem(STORAGE_KEY, String(value));
      document.documentElement.dataset.theme = value ? "dark" : "light";
      syncWindowTheme(value);
    },
    { immediate: true },
  );

  return {
    dark,
    theme,
    setDark,
    toggleTheme,
  };
});
