import type { JSXElement } from "solid-js";
import type { ScreenSizeType } from "../types";
import { Panel, Tabs } from "./base";

interface MainLayoutProps {
  screenSize: ScreenSizeType;
  optionsPanel: JSXElement;
  sourcePanel: JSXElement;
  formattedPanel: JSXElement;
  astPanel: JSXElement;
  irPanel: JSXElement;
}

export function MainLayout({
  screenSize,
  optionsPanel,
  sourcePanel,
  formattedPanel,
  astPanel,
  irPanel,
}: MainLayoutProps) {
  const outputTabs = [
    { id: "formatted", label: "Formatted", content: formattedPanel },
    { id: "ast", label: "AST", content: astPanel },
    { id: "ir", label: "Pretty IR", content: irPanel },
  ];

  const sourceTabs = [
    { id: "options", label: "Options", content: optionsPanel },
    { id: "source", label: "Source", content: sourcePanel },
  ];

  const allTabs = [
    { id: "options", label: "Options", content: optionsPanel },
    { id: "source", label: "Source", content: sourcePanel },
    { id: "formatted", label: "Formatted", content: formattedPanel },
    { id: "ast", label: "AST", content: astPanel },
    { id: "ir", label: "Pretty IR", content: irPanel },
  ];

  return (
    <div class="flex overflow-hidden min-h-0 h-full p-4 gap-2">
      {/* Wide Layout: 3 Columns */}
      {screenSize === "wide" && (
        <>
          <Panel header="Format Options" class="w-[240px] flex-none">
            {optionsPanel}
          </Panel>
          <Panel header="Source Code" class="flex-1">
            {sourcePanel}
          </Panel>
          <Tabs defaultActiveTab="formatted" class="flex-1" tabs={outputTabs} />
        </>
      )}

      {/* Medium Layout: 2 Columns (Equal 1:1) */}
      {screenSize === "medium" && (
        <>
          <Panel class="flex-1">
            <Tabs defaultActiveTab="source" tabs={sourceTabs} />
          </Panel>
          <Tabs defaultActiveTab="formatted" class="flex-1" tabs={outputTabs} />
        </>
      )}

      {/* Thin Layout: 1 Column (Full Width) */}
      {screenSize === "thin" && (
        <Tabs defaultActiveTab="source" class="flex-1" tabs={allTabs} />
      )}
    </div>
  );
}
