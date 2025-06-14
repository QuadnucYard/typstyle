import { type JSXElement, createEffect, createSignal } from "solid-js";
import type { ThemeType } from "../types";
import { ThemeContext, type ThemeContextType } from "./theme-context";

interface ThemeProviderProps {
  children: JSXElement;
}

export function ThemeProvider(props: ThemeProviderProps) {
  // Initialize theme with saved preference, defaulting to light
  const [theme, setTheme] = createSignal<ThemeType>(
    (() => {
      if (typeof window !== "undefined") {
        const savedTheme = localStorage.getItem("theme") as ThemeType | null;
        return savedTheme && (savedTheme === "light" || savedTheme === "dark")
          ? savedTheme
          : "light";
      }
      return "light"; // Default for SSR or if localStorage is not available
    })(),
  );

  const toggleTheme = () => {
    setTheme((prevTheme) => (prevTheme === "light" ? "dark" : "light"));
  };

  // Apply theme to document root and save to localStorage
  createEffect(() => {
    if (typeof window !== "undefined") {
      const currentTheme = theme(); // Get current value of the signal
      document.documentElement.setAttribute("data-theme", currentTheme);
      localStorage.setItem("theme", currentTheme);
    }
  });

  const contextValue: ThemeContextType = {
    theme, // Pass the signal accessor directly
    toggleTheme,
  };

  return (
    <ThemeContext.Provider value={contextValue}>
      {props.children}
    </ThemeContext.Provider>
  );
}
