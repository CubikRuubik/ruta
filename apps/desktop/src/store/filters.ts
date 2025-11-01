import { create } from "zustand";

export type Sort = "asc" | "desc";

interface FilterState {
  sort: Sort;
  block: number;
  addressFrom: string;
  addressTo: string;
  dateFrom: string;
  dateTo: string;
  setSort: (sort: Sort) => void;
  setBlock: (block: number) => void;
  setAddressFrom: (address: string) => void;
  setAddressTo: (address: string) => void;
  setDateFrom: (date: string) => void;
  setDateTo: (date: string) => void;
}

export const useFilterStore = create<FilterState>((set) => ({
  sort: "asc",
  block: 0,
  addressFrom: "",
  addressTo: "",
  dateFrom: "",
  dateTo: "",
  setSort: (sort) => set({ sort }),
  setBlock: (block) => set({ block }),
  setAddressFrom: (address) => set({ addressFrom: address }),
  setAddressTo: (address) => set({ addressTo: address }),
  setDateFrom: (date) => set({ dateFrom: date }),
  setDateTo: (date) => set({ dateTo: date }),
}));
