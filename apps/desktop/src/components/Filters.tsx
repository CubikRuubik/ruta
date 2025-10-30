import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "./ui/Select";
import { useFilterStore } from "../store/filters";
import { Input } from "./ui/Input";
// import { Popover, PopoverContent, PopoverTrigger } from "./ui/Popover";
// import { CalendarIcon } from "lucide-react";
// import { useState } from "react";
// import { Calendar } from "./ui/Calendar";
// import { format } from "date-fns";

export const Filters = () => {
  const {
    sort,
    block,
    setSort,
    setBlock,
    // setDateFrom,
    // setDateTo,
  } = useFilterStore();

  // const [openFrom, setOpenFrom] = useState(false);
  // const [openTo, setOpenTo] = useState(false);

  return (
    <div className="flex gap-4 mb-4 flex-wrap">
      <Select value={sort} onValueChange={setSort}>
        <SelectTrigger className="w-32">
          <SelectValue placeholder="Select Token" />
        </SelectTrigger>
        <SelectContent className="bg-(--card) text-(--card-foreground) dark:bg-(--card) dark:text-(--card-foreground)">
          <SelectItem value="asc">ASC</SelectItem>
          <SelectItem value="desc">DESC</SelectItem>
        </SelectContent>
      </Select>

      <Input
        type="text"
        placeholder="Block"
        value={block || ""}
        onChange={(e) => setBlock(Number(e.target.value))}
        className="w-64"
      />

      {/* <Popover open={openFrom} onOpenChange={setOpenFrom}>
        <PopoverTrigger asChild>
          <button
            className="flex items-center justify-between w-40 px-3 py-2 text-sm border rounded bg-input text-foreground"
            onClick={() => setOpenFrom(true)}
          >
            {dateFrom ? format(new Date(dateFrom), "yyyy-MM-dd") : "From"}
            <CalendarIcon className="ml-2 h-4 w-4 opacity-50" />
          </button>
        </PopoverTrigger>
        <PopoverContent align="start" className="p-0 bg-(--background) w-fit">
          <Calendar
            mode="single"
            selected={dateFrom ? new Date(dateFrom) : undefined}
            onSelect={(date) => {
              if (date) {
                setDateFrom(date.toISOString());
                setOpenFrom(false);
              }
            }}
          />
        </PopoverContent>
      </Popover> */}

      {/* <Popover open={openTo} onOpenChange={setOpenTo}>
        <PopoverTrigger asChild>
          <button
            className="flex items-center justify-between w-40 px-3 py-2 text-sm border rounded bg-input text-foreground"
            onClick={() => setOpenTo(true)}
          >
            {dateTo ? format(new Date(dateTo), "yyyy-MM-dd") : "To"}
            <CalendarIcon className="ml-2 h-4 w-4 opacity-50" />
          </button>
        </PopoverTrigger>
        <PopoverContent align="start" className="p-0 bg-(--background) w-fit">
          <Calendar
            mode="single"
            selected={dateTo ? new Date(dateTo) : undefined}
            onSelect={(date) => {
              if (date) {
                setDateTo(date.toISOString());
                setOpenTo(false);
              }
            }}
          />
        </PopoverContent>
      </Popover> */}
    </div>
  );
};
