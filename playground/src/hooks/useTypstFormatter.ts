import { type Accessor, createResource } from "solid-js";
import * as typstyle from "typstyle-wasm";
import type { FormatOptions } from "../types";

export function useTypstFormatter(
  sourceCode: Accessor<string>,
  formatOptions: Accessor<FormatOptions>,
) {
  const [formatterOutput] = createResource(
    // Source function: This function's return value is tracked.
    // If it changes, the fetcher function is re-run.
    () => ({
      source: sourceCode(),
      options: formatOptions(),
    }),
    // Fetcher function: Performs the asynchronous formatting.
    async ({ source, options }) => {
      // It's good practice to handle cases like empty source if typstyle requires it,
      // though typstyle might handle it gracefully.
      // For this example, we assume typstyle functions can process empty strings.

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
        // Return a state that indicates an error, allowing UI to react.
        // The original source is returned as formatted text in case of error.
        return {
          formatted: source, // Fallback to original source on error
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

  // Derived accessors for the formatted outputs.
  // These will reactively update when the resource data changes.
  // Provide fallbacks for initial loading state or if properties are missing.
  const formattedCode = () => formatterOutput()?.formatted ?? sourceCode();
  const astOutput = () => formatterOutput()?.ast ?? "";
  const irOutput = () => formatterOutput()?.ir ?? "";

  return {
    formattedCode,
    astOutput,
    irOutput,
  };
}
