import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export function useSSE() {
  const [messages, setMessages] = useState<string[]>([]);
  const [unlisten, setUnlisten] = useState<(() => void) | null>(null);

  useEffect(() => {
    const setupListener = async () => {
      const unlistenFn = await listen<string>("sse-update", (event) => {
        if (event.payload === "stop") {
          console.log("SSE stop");
        } else {
          setMessages((prev) => [...prev, event.payload]);
        }
      });
      setUnlisten(() => unlistenFn);
    };

    setupListener();

    return () => {
      unlisten?.();
    };
  }, []);

  const stopSSE = async () => {
    await invoke("stop_listening_sse");

    unlisten?.();
    setUnlisten(null);
  };

  return { messages, stopSSE };
}
