import type { Accessor } from "solid-js";
import type { ScreenSizeType } from "../types";

export function useScreenSize(): Accessor<ScreenSizeType> {
  const [screenSize, setScreenSize] = createSignal<ScreenSizeType>(
    (() => {
      const width = window.innerWidth;
      if (width >= 1200) return "wide";
      if (width >= 768) return "medium";
      return "thin";
    })(),
  );

  createEffect(() => {
    const updateScreenSize = () => {
      const width = window.innerWidth;
      if (width >= 1200) setScreenSize("wide");
      else if (width >= 768) setScreenSize("medium");
      else setScreenSize("thin");
    };

    updateScreenSize(); // initial check
    window.addEventListener("resize", updateScreenSize);

    return () => window.removeEventListener("resize", updateScreenSize);
  });

  return screenSize;
}
