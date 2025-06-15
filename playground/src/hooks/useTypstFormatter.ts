import { type Accessor, createResource } from "solid-js";
import * as typstyle from "typstyle-wasm";
import type { FormatOptions } from "../types";

export function useTypstFormatter(
  sourceCode: Accessor<string>,
  formatOptions: Accessor<FormatOptions>,
) {
  const [formatterOutput] = createResource(
    () => ({
      source: sourceCode(),
      options: formatOptions(),
    }),
    async ({ source, options }) => {
      const config: typstyle.Config = {
        max_width: options.maxLineLength,
        tab_spaces: options.indentSize,
        blank_lines_upper_bound: 2, // Default value, not exposed in UI
        collapse_markup_spaces: options.collapseMarkupSpaces,
        reorder_import_items: options.reorderImportItems,
        wrap_text: options.wrapText,
      };

      try {
        const ast = typstyle.parse(source);
        const ir = typstyle.format_ir(source, config);
        const formatted = typstyle.format(source, config);

        return { formatted, ast, ir };
      } catch (error) {
        return {
          formatted: source,
          ast: `Error parsing AST: ${
            error instanceof Error ? error.message : String(error)
          }`,
          ir: `Error generating IR: ${
            error instanceof Error ? error.message : String(error)
          }`,
        };
      }
    },
  );

  const formattedCode = () => formatterOutput()?.formatted ?? sourceCode();
  const astOutput = () => formatterOutput()?.ast ?? "";
  const irOutput = () => formatterOutput()?.ir ?? "";

  return {
    formattedCode,
    astOutput,
    irOutput,
  };
}
