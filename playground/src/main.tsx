import { render } from "solid-js/web";
import { ThemeProvider } from "./contexts"; // This now needs to be a SolidJS Context Provider
import "./styles/index.css";
import App from "./App"; // This now needs to be a SolidJS component
import { initMonaco } from "./utils/monacoThemes";

await initMonaco();

render(
  () => (
    <ThemeProvider>
      <App />
    </ThemeProvider>
  ),
  document.getElementById("root")!
);
