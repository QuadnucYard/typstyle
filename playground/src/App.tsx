import { createSignal } from "solid-js";
import {
  FormatOptionsContent,
  Header,
  MainLayout,
  OutputEditor,
  SourceEditor,
} from "./components";
import { DEFAULT_FORMAT_OPTIONS } from "./constants";
import { useInitialSample, useScreenSize, useTypstFormatter } from "./hooks";

function App() {
  const [sourceCode, setSourceCode] = createSignal("");
  useInitialSample({ setSourceCode });
  const [formatOptions, setFormatOptions] = createSignal(
    DEFAULT_FORMAT_OPTIONS,
  );

  const screenSize = useScreenSize();
  const { formattedCode, astOutput, irOutput } = useTypstFormatter(
    sourceCode,
    formatOptions,
  );
  const handleEditorChange = (value: string | undefined) => {
    if (value !== undefined) {
      setSourceCode(value);
    }
  };

  const handleSampleSelect = (content: string) => {
    setSourceCode(content);
  };

  return (
    <div
      class="
        h-screen flex flex-col
       bg-gradient-to-br from-koishi-green-50 via-koishi-green-100 to-koishi-green-200
       dark:from-koishi-purple-900 dark:via-koishi-purple-800 dark:to-koishi-purple-700
      "
    >
      <Header onSampleSelect={handleSampleSelect} />

      <MainLayout
        screenSize={screenSize()}
        optionsPanel={
          <FormatOptionsContent
            formatOptions={formatOptions()}
            setFormatOptions={setFormatOptions}
          />
        }
        sourcePanel={
          <SourceEditor
            value={sourceCode()}
            onChange={handleEditorChange}
            lineLengthGuide={formatOptions().maxLineLength}
          />
        }
        formattedPanel={
          <OutputEditor
            content={formattedCode()}
            language="typst"
            indentSize={formatOptions().indentSize}
            lineLengthGuide={formatOptions().maxLineLength}
          />
        }
        astPanel={
          <OutputEditor content={astOutput()} language="json" indentSize={4} />
        }
        irPanel={
          <OutputEditor content={irOutput()} language="python" indentSize={4} />
        }
      />
    </div>
  );
}

export default App;
