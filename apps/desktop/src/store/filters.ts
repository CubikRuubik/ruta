import { create } from "zustand";

interface FilterState {
  token: string;
  address: string;
  dateFrom: string;
  dateTo: string;
  setToken: (token: string) => void;
  setAddress: (address: string) => void;
  setDateFrom: (date: string) => void;
  setDateTo: (date: string) => void;
}

export const useFilterStore = create<FilterState>((set) => ({
  token: "All",
  address: "",
  dateFrom: "",
  dateTo: "",
  setToken: (token) => set({ token }),
  setAddress: (address) => set({ address }),
  setDateFrom: (date) => set({ dateFrom: date }),
  setDateTo: (date) => set({ dateTo: date }),
}));
