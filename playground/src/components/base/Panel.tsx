import type { JSXElement } from "solid-js";

export interface PanelProps {
  children: JSXElement;
  header?: string;
  class?: string;
}

export function Panel(props: PanelProps) {
  return (
    <div class={`panel ${props.class}`}>
      {props.header && <div class="panel-header">{props.header}</div>}
      <div class="panel-content">{props.children}</div>
    </div>
  );
}
