import type { Accessor } from "solid-js";
import type { ScreenSizeType } from "../types";

export function useScreenSize(): Accessor<ScreenSizeType> {
  const getScreenSize = (width: number): ScreenSizeType => {
    if (width >= 1200) return "wide";
    if (width >= 768) return "medium";
    return "thin";
  };

  const [screenSize, setScreenSize] = createSignal<ScreenSizeType>(
    getScreenSize(window.innerWidth),
  );

  createEffect(() => {
    const updateScreenSize = () => {
      setScreenSize(getScreenSize(window.innerWidth));
    };

    window.addEventListener("resize", updateScreenSize);
    onCleanup(() => window.removeEventListener("resize", updateScreenSize));
  });

  return screenSize;
}
