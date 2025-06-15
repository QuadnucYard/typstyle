import type { Monaco } from "@monaco-editor/loader";
import type { editor } from "monaco-editor";
import { createEffect, mergeProps } from "solid-js";
import { MonacoEditor } from "solid-monaco";
import { useTheme } from "../contexts";
import { getEditorTheme } from "../utils/monacoThemes";

export interface CodeEditorProps {
  value: string;
  language: string;
  indentSize: number;
  readOnly?: boolean;
  showLineNumbers?: boolean;
  enableWordWrap?: boolean;
  enableMinimap?: boolean;
  rulers?: number[];
  onChange?: (value: string | undefined) => void;
  onMount?: (monaco: Monaco, editor: editor.IStandaloneCodeEditor) => void;
}

export function CodeEditor(_props: CodeEditorProps) {
  const props = mergeProps(
    {
      readOnly: false,
      showLineNumbers: true,
      enableWordWrap: true,
      enableMinimap: false,
    },
    _props,
  );
  const { theme } = useTheme();
  const editorTheme = () => getEditorTheme(theme());

  let editorRef!: editor.IStandaloneCodeEditor;

  const handleEditorDidMount = (
    monaco: Monaco,
    editor: editor.IStandaloneCodeEditor,
  ) => {
    editorRef = editor;
    props.onMount?.(monaco, editor);
  };

  // Update indentation reactively
  createEffect(() => {
    const indentSize = props.indentSize;
    if (editorRef?.getModel()) {
      const model = editorRef.getModel()!;
      model.updateOptions({
        tabSize: indentSize > 0 ? indentSize : undefined,
      });
      editorRef.updateOptions({
        detectIndentation: indentSize <= 0,
      });
    }
  });

  // Update rulers reactively
  createEffect(() => {
    const rulers = props.rulers;
    if (editorRef) {
      editorRef.updateOptions({
        rulers: rulers && rulers.length > 0 ? rulers : undefined,
      });
    }
  });

  const editorOptions: editor.IStandaloneEditorConstructionOptions = {
    readOnly: props.readOnly,
    minimap: { enabled: props.enableMinimap },
    scrollBeyondLastLine: false,
    fontSize: 14,
    fontFamily: "Monaco, Menlo, Ubuntu Mono, monospace",
    automaticLayout: true,
    padding: { top: 8, bottom: 8 },
    wordWrap: props.enableWordWrap ? "on" : "off",
    lineNumbers: props.showLineNumbers ? "on" : "off",
    folding: true,
    renderLineHighlight: props.readOnly ? "none" : "gutter",
    smoothScrolling: true,
    autoIndent: props.readOnly ? "none" : "full",
    scrollbar: {
      vertical: "auto",
      horizontal: "auto",
    },
    // Initial values - updated reactively by effects
    rulers: undefined,
    tabSize: props.indentSize,
    insertSpaces: true,
    detectIndentation: false,
  };

  return (
    <div
      class={`
        h-full flex-1 overflow-hidden flex flex-col relative
        bg-[rgba(232,245,232,0.6)] dark:bg-[rgba(42,31,74,0.6)]
        transition-all duration-300 ease-in-out
    `}
    >
      <MonacoEditor
        ref={editorRef}
        language={props.language}
        value={props.value}
        theme={editorTheme()}
        onChange={props.onChange}
        onMount={handleEditorDidMount}
        options={editorOptions}
      />
    </div>
  );
}
