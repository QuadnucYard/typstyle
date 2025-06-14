import {
  createSignal,
  createEffect,
  createMemo,
  For,
  Show,
  type JSXElement,
  children as childrenHelper,
} from "solid-js";

// Interface for the data structure a Tab component will produce
export interface TabItem {
  id: string;
  label: string;
  content: JSXElement; // This is the original children of the Tab component
}

// Props for the Tab "data constructor" component
export interface TabProps {
  id: string;
  label: string;
  children: JSXElement; // This will become the 'content' of the TabItem
}

export interface TabsProps {
  children?: JSXElement | JSXElement[]; // Expects results of Tab(...) calls
  activeTab?: string; // Controlled active tab
  onTabChange?: (tabId: string) => void;
  defaultActiveTab?: string;
  class?: string; // Main container class
  tabClassName?: string; // Class for each tab button
  contentClassName?: string; // Class for the content panel
}

/**
 * Tab component - acts as a data descriptor.
 * It doesn't render directly but provides its props as an object
 * for the parent Tabs component to consume.
 */
export function Tab(props: TabProps): TabItem {
  return {
    id: props.id,
    label: props.label,
    content: props.children,
  };
}

export function Tabs(props: TabsProps) {
  // Extract TabItem objects from children
  const resolvedChildren = childrenHelper(() => props.children);

  const items = createMemo<TabItem[]>(() => {
    const kids = resolvedChildren();
    const collectedTabs: TabItem[] = [];
    if (Array.isArray(kids)) {
      kids.forEach((child) => {
        // Check if the child is a TabItem-like object
        if (
          child &&
          typeof child === "object" &&
          "id" in child &&
          "label" in child &&
          "content" in child
        ) {
          collectedTabs.push(child as TabItem);
        }
      });
    } else if (
      kids &&
      typeof kids === "object" &&
      "id" in kids &&
      "label" in kids &&
      "content" in kids
    ) {
      // Handle single child case
      collectedTabs.push(kids as TabItem);
    }
    return collectedTabs;
  });

  // Internal state for uncontrolled mode
  const [internalActiveTab, setInternalActiveTab] = createSignal<string>(
    props.defaultActiveTab || items()[0]?.id || "",
  );

  // Determine active tab (controlled or uncontrolled)
  const activeTabId = createMemo<string>(() => {
    if (props.activeTab !== undefined) {
      return props.activeTab; // Controlled
    }
    return internalActiveTab(); // Uncontrolled
  });

  // Handle tab click
  const handleTabChange = (tabId: string) => {
    if (props.onTabChange) {
      props.onTabChange(tabId);
    }
    if (props.activeTab === undefined) {
      // Only set internal state if uncontrolled
      setInternalActiveTab(tabId);
    }
  };

  // Effect to update internal state if defaultActiveTab or items change (for uncontrolled mode)
  createEffect(() => {
    if (props.activeTab === undefined) { // Uncontrolled
      const currentItems = items();
      const currentDefault = props.defaultActiveTab || currentItems[0]?.id || "";
      if (internalActiveTab() !== currentDefault && (!currentItems.find(t => t.id === internalActiveTab()) || props.defaultActiveTab)) {
         // If current active tab is not in items, or defaultActiveTab is explicitly set, reset
        setInternalActiveTab(currentDefault);
      } else if (!internalActiveTab() && currentDefault) {
        // If no active tab is set, initialize with default
        setInternalActiveTab(currentDefault);
      }
    }
  });

  // Effect to sync internal active tab when external activeTab prop changes (for controlled mode)
  // This might not be strictly necessary if activeTabId() correctly reflects external prop.
  // Solid's derived signals usually handle this.
  // createEffect(() => {
  //   if (props.activeTab !== undefined) {
  //     setInternalActiveTab(props.activeTab); // This could cause a loop if onTabChange also sets externalActiveTab
  //   }
  // });

  const activeTabContent = createMemo(() => {
    const currentTabId = activeTabId();
    return items().find((tab) => tab.id === currentTabId)?.content;
  });

  return (
    <div class={`tabs ${props.class || ""}`}>
      {/* Tab Headers */}
      <div class="tabs-nav">
        <For each={items()}>
          {(tab) => {
            const isActive = () => activeTabId() === tab.id;
            return (
              <button
                type="button"
                class={`tab ${isActive() ? "active" : ""} ${
                  props.tabClassName || ""
                }`.trim()}
                onClick={() => handleTabChange(tab.id)}
                aria-selected={isActive()}
                role="tab"
              >
                {tab.label}
              </button>
            );
          }}
        </For>
      </div>

      {/* Tab Content */}
      <Show when={activeTabContent()} keyed>
        {(content) => (
          <div class={`tabs-panel ${props.contentClassName || ""}`.trim()}>
            {content}
          </div>
        )}
      </Show>
    </div>
  );
}
