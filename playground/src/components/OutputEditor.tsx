import { CodeEditor } from "./CodeEditor";

export interface OutputEditorProps {
  content: string;
  language: string;
  indentSize: number;
  lineLengthGuide?: number;
}

export function OutputEditor(props: OutputEditorProps) {
  return (
    <div class="h-full">
      <CodeEditor
        value={props.content}
        indentSize={props.indentSize}
        language={props.language}
        readOnly={true}
        showLineNumbers={false}
        enableWordWrap={false}
        enableMinimap={false}
        rulers={props.lineLengthGuide ? [props.lineLengthGuide] : []}
      />
    </div>
  );
}
