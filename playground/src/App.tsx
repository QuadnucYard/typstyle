import { MonacoProvider } from "@react-monaco/core";
import Playground from "./Playground";
import { ThemeProvider } from "./contexts";
import {
  TextmateInjection,
  tmConfig,
  type TextmateCodeSet,
  type TextmateFilterCodeSetCallback,
  type TextmateProviderCallback,
} from "@react-monaco/plugin-textmate";
import { createThemesPlugin } from "@react-monaco/plugin-themes";
import Xcode_default from "monaco-themes/themes/Xcode_default.json";

const tmProvider: TextmateProviderCallback = ({ language, extname }) => {
  console.log("provide", language, extname);
  if (language?.id === "typst" || extname === ".typ") {
    return {
      url: new URL("typst.tmLanguage.json", tmConfig("baseUrl")),
      format: "json",
      languageId: "typst",
      scopeName: "source.typst",
    };
  }
};

const tmFilter: TextmateFilterCodeSetCallback = (code: TextmateCodeSet) => {
  return code;
};

// const [themes, ThemesInjection] = createThemesPlugin({
//   themes: [
//     { key: "github-light", name: "GitHub Light" },
//     { key: "webstorm-dark", name: "Webstorm Dark" },
//   ],
// });

function App() {
  return (
    <ThemeProvider>
      <MonacoProvider>
        {/* <ThemesInjection debug theme={"webstorm-dark"} /> */}
        <TextmateInjection debug provider={tmProvider} filter={tmFilter} />
        <Playground />
      </MonacoProvider>
    </ThemeProvider>
  );
}

export default App;
