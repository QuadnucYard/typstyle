import type { Monaco } from "@monaco-editor/react";

// Typst language definition for Monaco Editor
export const registerTypstLanguage = (monaco: Monaco) => {
  // Register the Typst language
  monaco.languages.register({ id: "typst" });

  // Define the language configuration
  monaco.languages.setLanguageConfiguration("typst", {
    comments: {
      lineComment: "//",
      blockComment: ["/*", "*/"],
    },
    brackets: [
      ["[", "]"],
      ["{", "}"],
      ["(", ")"],
    ],
    autoClosingPairs: [
      { open: "[", close: "]" },
      { open: "{", close: "}" },
      { open: "(", close: ")" },
      { open: '"', close: '"', notIn: ["string"] },
      { open: "$", close: "$", notIn: ["string"] },
    ],
    autoCloseBefore: ";:.,=}])>$ \n\t",
    surroundingPairs: [
      { open: "[", close: "]" },
      { open: "{", close: "}" },
      { open: "(", close: ")" },
      { open: '"', close: '"' },
      { open: "*", close: "*" },
      { open: "_", close: "_" },
      { open: "`", close: "`" },
      { open: "$", close: "$" },
    ],
  });

};
