import { create } from "zustand";
import { invoke } from "@tauri-apps/api/core";
import { listen, UnlistenFn } from "@tauri-apps/api/event";

export interface Transfer {
  id: number;
  block_number: number;
  transaction_hash: string;
  log_index: number;
  from_address: string;
  to_address: string;
  amount: string;
  contract_address: string;
  created_at?: string | null;
}

interface TransferStore {
  transfers: Transfer[];
  unlisten: UnlistenFn | null;
  setTransfers: (data: Transfer[]) => void;
  fetchInitialData: () => Promise<void>;
  startListening: () => Promise<void>;
  stopListening: () => void;
}

export const useTransferStore = create<TransferStore>((set, get) => ({
  transfers: [],
  unlisten: null,
  setTransfers: (data) => set({ transfers: data }),

  fetchInitialData: async () => {
    try {
      const data = await invoke<Transfer[]>("get_initial_data");
      set({ transfers: data });
    } catch (err) {
      console.error("Failed to fetch initial data:", err);
    }
  },

  startListening: async () => {
    if (typeof get().unlisten === "function") return;

    try {
      await invoke("start_listening_sse");

      const unlistenFn = await listen<Transfer>("sse-update", (event) => {
        console.log("Received SSE event:", event.payload);
        set({ transfers: [...get().transfers, event.payload] });
      });

      set({ unlisten: unlistenFn });
    } catch (err) {
      console.error("Failed to start SSE listener:", err);
    }
  },

  stopListening: () => {
    get().unlisten?.();
    set({ unlisten: null });
  },
}));
