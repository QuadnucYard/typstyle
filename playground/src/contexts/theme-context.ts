import { type Accessor, createContext } from "solid-js";
import type { ThemeType } from "../types";

export interface ThemeContextType {
  theme: Accessor<ThemeType>;
  toggleTheme: () => void;
}

export const ThemeContext = createContext<ThemeContextType | undefined>(
  undefined,
);
