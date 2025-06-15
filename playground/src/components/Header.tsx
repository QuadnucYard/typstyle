import { useTheme } from "../contexts";
import { DarkModeIcon, GitHubIcon, LightModeIcon } from "./Icons";
import { SampleDocumentSelector } from "./SampleDocumentSelector";

interface HeaderProps {
  onSampleSelect: (content: string) => void;
}

export function Header({ onSampleSelect }: HeaderProps) {
  const { theme, toggleTheme } = useTheme();
  return (
    <div
      class="
      px-4 py-2 backdrop-blur-md flex items-center justify-between flex-shrink-0
      border-b border-[rgba(200, 230, 201, 0.9)] dark:border-[rgba(74, 63, 106, 0.9)] shadow-soft relative
    "
    >
      <div class="flex items-center gap-4">
        <h1 class="text-2xl font-bold text-[#2e7d32] dark:text-[#c5b8e3] m-0 drop-shadow-sm">
          Typstyle Playground
        </h1>
        <SampleDocumentSelector
          onSampleSelect={onSampleSelect}
          class="min-w-[200px] max-w-[300px]"
        />
      </div>

      <div class="flex items-center gap-2">
        {/* GitHub Repo Link */}
        <a
          href="https://github.com/enter-tainer/typstyle"
          target="_blank"
          rel="noopener noreferrer"
          class="btn-icon size-8 no-underline hover:no-underline"
          title="View Typstyle on GitHub"
        >
          <GitHubIcon />
        </a>

        {/* Theme Toggle */}
        <button
          type="button"
          onClick={toggleTheme}
          class="btn-icon size-8"
          title={`Switch to ${theme() === "light" ? "dark" : "light"} mode`}
        >
          {theme() === "light" ? <LightModeIcon /> : <DarkModeIcon />}
        </button>
      </div>
    </div>
  );
}
