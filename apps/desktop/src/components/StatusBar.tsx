import { FC } from "react";
import { useTransferStore } from "../store/transfers";

export const StatusBar: FC = () => {
  const { unlisten, transfers } = useTransferStore();

  const STATUS: "Connected" | "Disconnected" = unlisten
    ? "Connected"
    : "Disconnected";
  const LIVE_UPDATES: "ON" | "OFF" = unlisten ? "ON" : "OFF";

  const LAST_BLOCK = transfers.length
    ? transfers.reduce((max, t) => {
        return t.block_number > max ? t.block_number : max;
      }, 0)
    : "—";

  const statusColor = STATUS === "Connected" ? "bg-green-500" : "bg-red-500";

  return (
    <div className="mt-2 p-2 border rounded bg-input text-sm text-center flex items-center justify-center gap-2">
      <span className={`h-3 w-3 rounded-full ${statusColor}`}></span>
      <span>
        Status: {STATUS} | Live Updates {LIVE_UPDATES} | Last Block:{" "}
        {LAST_BLOCK}
      </span>
    </div>
  );
};
