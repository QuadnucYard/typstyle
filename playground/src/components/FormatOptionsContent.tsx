import type { Setter } from "solid-js";
import { DEFAULT_FORMAT_OPTIONS } from "../constants";
import type { FormatOptions } from "../types";

interface FormatOptionsContentProps {
  formatOptions: FormatOptions;
  setFormatOptions: Setter<FormatOptions>;
}

export function FormatOptionsContent(props: FormatOptionsContentProps) {
  const handleReset = () => {
    props.setFormatOptions(DEFAULT_FORMAT_OPTIONS);
  };
  return (
    <div class="p-2 overflow-y-auto flex-1">
      {/* Reset Button */}
      <div class="mb-3 pb-3 border-b border-[rgba(200, 230, 201, 0.9)] dark:border-[rgba(74, 63, 106, 0.9)]">
        <button type="button" onClick={handleReset} class="btn w-full">
          🔄 Reset to Defaults
        </button>
      </div>

      <div class="flex flex-wrap gap-3 items-center">
        <div class="flex items-center justify-between w-full min-w-[200px] gap-2">
          <label for="lineLengthSelect">Line Length:</label>
          <div class="flex gap-1 flex-shrink-0">
            <select
              id="lineLengthSelect"
              name="lineWidth"
              value={
                [40, 60, 80, 100, 120].includes(
                  props.formatOptions.maxLineLength,
                )
                  ? props.formatOptions.maxLineLength
                  : "custom"
              }
              onChange={(e) => {
                if (e.target.value !== "custom") {
                  props.setFormatOptions((prev) => ({
                    ...prev,
                    maxLineLength: Number.parseInt(e.target.value),
                  }));
                }
              }}
              class="w-14"
            >
              <option value={40}>40</option>
              <option value={60}>60</option>
              <option value={80}>80</option>
              <option value={100}>100</option>
              <option value={120}>120</option>
              <option value="custom">Custom</option>
            </select>
            <input
              id="lineLengthInput"
              type="number"
              min="40"
              max="200"
              aria-label="Custom Line Length"
              value={props.formatOptions.maxLineLength}
              onChange={(e) =>
                props.setFormatOptions((prev) => ({
                  ...prev,
                  maxLineLength: Number.parseInt(e.target.value),
                }))
              }
              class="w-14"
            />
          </div>
        </div>{" "}
        <div class="flex items-center justify-between w-full min-w-[200px] gap-2">
          <label for="indentSizeSelect">Indent:</label>
          <div class="flex gap-1 flex-shrink-0">
            <select
              id="indentSizeSelect"
              name="indentSize"
              value={
                [2, 4, 8].includes(props.formatOptions.indentSize)
                  ? props.formatOptions.indentSize
                  : "custom"
              }
              onChange={(e) => {
                if (e.target.value !== "custom") {
                  props.setFormatOptions((prev) => ({
                    ...prev,
                    indentSize: Number.parseInt(e.target.value),
                  }));
                }
              }}
              class="w-14"
            >
              <option value={2}>2</option>
              <option value={4}>4</option>
              <option value={8}>8</option>
              <option value="custom">Custom</option>
            </select>
            <input
              id="indentSizeInput"
              type="number"
              min="1"
              max="16"
              aria-label="Custom Indent Size"
              value={props.formatOptions.indentSize}
              onChange={(e) =>
                props.setFormatOptions((prev) => ({
                  ...prev,
                  indentSize: Number.parseInt(e.target.value),
                }))
              }
              class="w-14"
            />
          </div>
        </div>{" "}
        <div class="flex items-center justify-between w-full min-w-[200px] gap-2">
          <label for="collapseMarkupSpaces">Collapse Markup Spaces:</label>
          <input
            id="collapseMarkupSpaces"
            type="checkbox"
            checked={props.formatOptions.collapseMarkupSpaces}
            onChange={(e) =>
              props.setFormatOptions((prev) => ({
                ...prev,
                collapseMarkupSpaces: e.target.checked,
              }))
            }
          />
        </div>
        <div class="flex items-center justify-between w-full min-w-[200px] gap-2">
          <label for="reorderImportItems">Reorder Import Items:</label>
          <input
            id="reorderImportItems"
            type="checkbox"
            checked={props.formatOptions.reorderImportItems}
            onChange={(e) =>
              props.setFormatOptions((prev) => ({
                ...prev,
                reorderImportItems: e.target.checked,
              }))
            }
          />
        </div>
        <div class="flex items-center justify-between w-full min-w-[200px] gap-2">
          <label for="wrapText">Wrap Text:</label>
          <input
            id="wrapText"
            type="checkbox"
            checked={props.formatOptions.wrapText}
            onChange={(e) =>
              props.setFormatOptions((prev) => ({
                ...prev,
                wrapText: e.target.checked,
              }))
            }
          />
        </div>
      </div>
    </div>
  );
}
