import { FC } from "react";

export const StatusBar: FC = () => {
  type ConnectionStatus = "Connected" | "Disconnected";
  type LiveUpdates = "ON" | "OFF";

  const STATUS: ConnectionStatus = "Connected";
  const LIVE_UPDATES: LiveUpdates = "ON";
  const LAST_BLOCK = 18392010;

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
