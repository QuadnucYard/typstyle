import type { Monaco } from "@monaco-editor/loader";
import type { editor } from "monaco-editor";
import { MonacoEditor } from "solid-monaco";
import { useTheme } from "../contexts";
import { getEditorTheme } from "../utils/monacoThemes";

/**
 * CodeEditor - A configurable Monaco Editor wrapper for the Typstyle Playground
 *
 * This component serves as the base editor implementation across the application.
 * The `indentSize` prop now controls indentation: a positive value sets a fixed indent
 * size (using spaces), while 0 or a negative value enables auto-detection of indentation.
 */

export interface CodeEditorProps {
  value: string;
  onChange?: (value: string | undefined) => void;
  onMount?: (monaco: Monaco, editor: editor.IStandaloneCodeEditor) => void;
  indentSize: number; // Positive for fixed indent, 0 or negative for auto-detect
  language?: string;
  readOnly?: boolean;
  showLineNumbers?: boolean;
  enableFolding?: boolean;
  enableWordWrap?: boolean;
  enableMinimap?: boolean;
  rulers?: number[];
}

export function CodeEditor(_props: CodeEditorProps) {
  const props = mergeProps(
    {
      language: "typst",
      readOnly: false,
      showLineNumbers: true,
      enableFolding: true,
      enableWordWrap: true,
      enableMinimap: false,
    },
    _props,
  );
  const { theme } = useTheme();
  let editorRef: editor.IStandaloneCodeEditor;

  const editorTheme = () => getEditorTheme(theme());

  // const applyIndentationSettings = useCallback(() => {
  //   if (editorRef.current) {
  //     const editor = editorRef.current;
  //     const model = editor.getModel();

  //     if (model) {
  //       if (indentSize > 0) {
  //         // Positive indentSize: use fixed indentation
  //         editor.updateOptions({ detectIndentation: false });
  //         model.updateOptions({
  //           tabSize: indentSize,
  //           insertSpaces: true, // Typically use spaces for fixed indentation
  //         });
  //       } else {
  //         // indentSize is 0 or negative: use auto-detection
  //         editor.updateOptions({ detectIndentation: true });
  //         // When detectIndentation is true, Monaco handles tabSize and insertSpaces.
  //       }
  //     }
  //   }
  // }, [indentSize]); // Dependency is now only indentSize

  const updateEditorOptions = (
    editor: editor.IStandaloneCodeEditor,
    monaco: Monaco,
  ) => {
    // applyIndentationSettings(); // Apply initial settings
  };

  const handleEditorDidMount = (
    monaco: Monaco,
    editor: editor.IStandaloneCodeEditor,
  ) => {
    updateEditorOptions(editor, monaco);
    console.log("Editor mounted", editor);
  };
  const editorOptions: editor.IStandaloneEditorConstructionOptions = {
    readOnly: props.readOnly,
    minimap: { enabled: props.enableMinimap },
    scrollBeyondLastLine: false,
    fontSize: 14,
    fontFamily: "Monaco, Menlo, Ubuntu Mono, monospace",
    automaticLayout: true,
    padding: { top: 8, bottom: 8 },
    // tabSize, detectIndentation, and insertSpaces are now handled by applyIndentationSettings
    wordWrap: props.enableWordWrap ? "on" : "off",
    lineNumbers: props.showLineNumbers ? "on" : "off",
    folding: props.enableFolding,
    renderLineHighlight: props.readOnly ? "none" : "gutter",
    smoothScrolling: true,
    autoIndent: props.readOnly ? "none" : "full",
    scrollbar: {
      vertical: "auto",
      horizontal: "auto",
    },
    rulers: props.rulers && props.rulers.length > 0 ? props.rulers : undefined,
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
