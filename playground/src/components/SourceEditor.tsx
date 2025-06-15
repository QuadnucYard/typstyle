import { CodeEditor } from "./CodeEditor";

export interface SourceEditorProps {
  value: string;
  onChange: (value: string | undefined) => void;
  lineLengthGuide?: number;
}

export function SourceEditor(props: SourceEditorProps) {
  return (
    <CodeEditor
      value={props.value}
      onChange={props.onChange}
      indentSize={0}
      language="typst"
      readOnly={false}
      showLineNumbers={true}
      enableWordWrap={true}
      enableMinimap={false}
      rulers={props.lineLengthGuide ? [props.lineLengthGuide] : []}
    />
  );
}
