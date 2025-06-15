import {
  For,
  type JSXElement,
  Show,
  createEffect,
  createMemo,
  createSignal,
} from "solid-js";

export interface TabItem {
  id: string;
  label: string;
  content: JSXElement;
}

export interface TabProps {
  id: string;
  label: string;
  children: JSXElement;
}

export interface TabsProps {
  children?: JSXElement;
  onTabChange?: (tabId: string) => void;
  defaultActiveTab?: string;
  class?: string;
  tabClassName?: string;
  contentClassName?: string;
  tabs?: TabItem[];
}

function combineClasses(...classes: (string | undefined)[]): string {
  return classes.filter(Boolean).join(" ");
}

export function Tabs(props: TabsProps) {
  const tabItems = () => props.tabs || [];

  const getDefaultActiveTab = () => {
    return props.defaultActiveTab || tabItems()[0]?.id || "";
  };

  const [internalActiveTab, setInternalActiveTab] = createSignal<string>(
    getDefaultActiveTab(),
  );

  const activeTabId = () => internalActiveTab();

  const handleTabChange = (tabId: string) => {
    props.onTabChange?.(tabId);
    setInternalActiveTab(tabId);
  };

  createEffect(() => {
    const currentTabs = tabItems();
    const currentDefault = getDefaultActiveTab();
    const currentActive = internalActiveTab();

    const isCurrentTabValid = currentTabs.some(
      (tab) => tab.id === currentActive,
    );

    if (!isCurrentTabValid && currentDefault) {
      setInternalActiveTab(currentDefault);
    }
  });

  const activeTabContent = createMemo(() => {
    const currentTabId = activeTabId();
    return tabItems().find((tab) => tab.id === currentTabId)?.content;
  });

  return (
    <div class={combineClasses("tabs", props.class)}>
      <div class="tabs-nav">
        <For each={tabItems()}>
          {(tab) => {
            const isActive = () => activeTabId() === tab.id;
            return (
              <button
                type="button"
                class={combineClasses(
                  "tab",
                  isActive() ? "active" : "",
                  props.tabClassName,
                )}
                onClick={() => handleTabChange(tab.id)}
                aria-selected={isActive()}
                role="tab"
                tabindex={isActive() ? 0 : -1}
              >
                {tab.label}
              </button>
            );
          }}
        </For>
      </div>

      <Show when={activeTabContent()} keyed>
        {(content) => (
          <div
            class={combineClasses("tabs-panel", props.contentClassName)}
            role="tabpanel"
          >
            {content}
          </div>
        )}
      </Show>
    </div>
  );
}
