import { For, Show, createMemo, createSignal } from "solid-js";
import { SAMPLE_DOCUMENTS, type SampleDocumentKey } from "../constants";
import {
  getFallbackContent,
  getSampleFileContent,
} from "../utils/sampleLoader";

interface SampleDocumentSelectorProps {
  onSampleSelect: (content: string) => void;
  class?: string;
}

export function SampleDocumentSelector(props: SampleDocumentSelectorProps) {
  const [selectedSample, setSelectedSample] = createSignal<
    SampleDocumentKey | ""
  >("");
  const [error, setError] = createSignal<string | null>(null);

  const sampleTooltip = createMemo(() => {
    const sample = selectedSample();
    return sample && sample in SAMPLE_DOCUMENTS
      ? SAMPLE_DOCUMENTS[sample as SampleDocumentKey].description
      : "📄 Choose a sample document to load";
  });

  async function loadSampleDocument(key: SampleDocumentKey) {
    setError(null);
    try {
      const content = await getSampleFileContent(key);
      props.onSampleSelect(content);
      setSelectedSample(key);
    } catch (err) {
      const msg = err instanceof Error ? err.message : "Unknown error";
      console.error("Error loading sample document:", err);
      setError(msg);
      props.onSampleSelect(getFallbackContent(key, msg));
    }
  }

  function handleSampleChange(e: Event) {
    const value = (e.target as HTMLSelectElement).value as
      | SampleDocumentKey
      | "";
    if (value && value in SAMPLE_DOCUMENTS) {
      loadSampleDocument(value);
    } else {
      setSelectedSample("");
      setError(null);
      props.onSampleSelect("");
    }
  }

  return (
    <div class={props.class || ""}>
      <div class="flex items-center gap-2">
        <select
          value={selectedSample()}
          onChange={handleSampleChange}
          class="w-48"
          title={sampleTooltip()}
        >
          <option value="" disabled>
            Select a sample...
          </option>
          <For each={Object.entries(SAMPLE_DOCUMENTS)}>
            {([key, sample]) => (
              <option value={key} title={sample.description}>
                {sample.name}
              </option>
            )}
          </For>
        </select>

        <button
          type="button"
          onClick={() => {
            setSelectedSample("");
            setError(null);
            props.onSampleSelect("");
          }}
          class="btn w-8 h-8 p-0"
          title="Clear document and start fresh"
        >
          🗑️
        </button>

        <Show when={error()}>
          <div class="rounded border border-red-200 bg-red-50 px-2 py-1 text-xs text-red-500 dark:border-red-800 dark:bg-red-950/20">
            ⚠️ {error()}
          </div>
        </Show>
      </div>
    </div>
  );
}
