import { Chart } from "./Chart";
import { Filters } from "./Filters";
import { StatusBar } from "./StatusBar";
import { TransfersTable } from "./TransfersTable";

export default function Dashboard() {
  return (
    <div className="p-4 h-full bg-background text-foreground font-mono">
      <h1 className="text-2xl font-bold mb-4 text-center">RUTA Dashboard</h1>
      <Filters />
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        <TransfersTable />
        <Chart />
      </div>

      <StatusBar />
    </div>
  );
}
