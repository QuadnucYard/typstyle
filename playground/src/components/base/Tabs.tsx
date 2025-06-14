import {
  createSignal,
  createEffect,
  createMemo,
  For,
  Show,
  type JSXElement,
} from "solid-js";

/**
 * Represents a single tab item with its metadata and content
 */
export interface TabItem {
  id: string;
  label: string;
  content: JSXElement;
}

/**
 * Props for the Tab component
 */
export interface TabProps {
  id: string;
  label: string;
  children: JSXElement;
}

/**
 * Props for the Tabs container component
 */
export interface TabsProps {
  children?: JSXElement;
  /** Callback when tab changes */
  onTabChange?: (tabId: string) => void;
  /** Default active tab for uncontrolled mode */
  defaultActiveTab?: string;
  /** CSS class for the main container */
  class?: string;
  /** CSS class for individual tab buttons */
  tabClassName?: string;
  /** CSS class for the content panel */
  contentClassName?: string;
  /** Tab definitions for simpler usage */
  tabs?: TabItem[];
}

/**
 * Utility to combine CSS classes
 */
function combineClasses(...classes: (string | undefined)[]): string {
  return classes.filter(Boolean).join(" ");
}

/**
 * Tabs component - manages a collection of tabs with controlled/uncontrolled state
 */
export function Tabs(props: TabsProps) {
  /**
   * Extract tab items from provided tabs prop
   */
  const tabItems = () => {
    return props.tabs || [];
  };

  /**
   * Get the default active tab ID
   */
  const getDefaultActiveTab = () => {
    return props.defaultActiveTab || tabItems()[0]?.id || "";
  };

  // Internal state for uncontrolled mode
  const [internalActiveTab, setInternalActiveTab] = createSignal<string>(
    getDefaultActiveTab(),
  );

  /**
   * Determine the currently active tab (controlled or uncontrolled)
   */
  const activeTabId = () => internalActiveTab();
  /**
   * Handle tab change events
   */
  const handleTabChange = (tabId: string) => {
    props.onTabChange?.(tabId);
    setInternalActiveTab(tabId);
  };

  createEffect(() => {
    const currentTabs = tabItems();
    const currentDefault = getDefaultActiveTab();
    const currentActive = internalActiveTab();

    // Reset to default if current active tab is invalid
    const isCurrentTabValid = currentTabs.some(
      (tab) => tab.id === currentActive,
    );

    if (!isCurrentTabValid && currentDefault) {
      setInternalActiveTab(currentDefault);
    }
  });

  /**
   * Get the content of the currently active tab
   */
  const activeTabContent = () => {
    const currentTabId = activeTabId();
    return tabItems().find((tab) => tab.id === currentTabId)?.content;
  };

  return (
    <div class={combineClasses("tabs", props.class)}>
      {/* Tab Navigation */}
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
      <div>{activeTabId()}</div>
      {/* Tab Content Panel */}
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
