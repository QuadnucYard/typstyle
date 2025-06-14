import type { Accessor } from "solid-js";
import { CodeEditor } from "./CodeEditor";

export interface OutputEditorProps {
  content: Accessor<string>;
  language?: string;
  indentSize?: number;
  lineLengthGuide?: number;
}

export function OutputEditor(props: OutputEditorProps) {
  return (
    <div class="h-full">
      <CodeEditor
        value={props.content()}
        indentSize={props.indentSize ?? 2}
        language={props.language}
        readOnly={true}
        showLineNumbers={false}
        enableFolding={props.language === "json"}
        enableWordWrap={false}
        enableMinimap={false}
        rulers={props.lineLengthGuide ? [props.lineLengthGuide] : []}
      />
    </div>
  );
}
