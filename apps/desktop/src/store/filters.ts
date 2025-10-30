import { create } from "zustand";

export type Sort = "asc" | "desc";

interface FilterState {
  sort: Sort;
  block: number;
  dateFrom: string;
  dateTo: string;
  setSort: (sort: Sort) => void;
  setBlock: (block: number) => void;
  setDateFrom: (date: string) => void;
  setDateTo: (date: string) => void;
}

export const useFilterStore = create<FilterState>((set) => ({
  sort: "asc",
  block: 0,
  dateFrom: "",
  dateTo: "",
  setSort: (sort) => set({ sort }),
  setBlock: (block) => set({ block }),
  setDateFrom: (date) => set({ dateFrom: date }),
  setDateTo: (date) => set({ dateTo: date }),
}));
