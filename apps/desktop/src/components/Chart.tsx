import { FC } from "react";
import { useTransferStore } from "../store/transfers";
import { Card, CardContent, CardHeader, CardTitle } from "./ui/Card";
import {
  BarChart,
  Bar,
  XAxis,
  YAxis,
  Tooltip,
  ResponsiveContainer,
  LabelList,
  Cell,
  PieChart,
  Pie,
} from "recharts";
import { useFilterStore } from "../store/filters";

export const Chart: FC = () => {
  const { transfers } = useTransferStore();
  const { block: selectedBlock } = useFilterStore();

  const amountByBlock = transfers.reduce<Record<number, number>>((acc, t) => {
    const block = t.block_number;
    const amount = parseFloat(t.amount);
    acc[block] = (acc[block] || 0) + amount;
    return acc;
  }, {});

  let barData = Object.entries(amountByBlock).map(([block, total]) => ({
    block,
    total,
  }));

  barData.sort((a, b) => Number(a.block) - Number(b.block));
  barData = barData.slice(-5);

  const countByBlock = transfers.reduce<Record<number, number>>((acc, t) => {
    const block = t.block_number;
    acc[block] = (acc[block] || 0) + 1;
    return acc;
  }, {});

  let pieData;

  if (selectedBlock) {
    const filtered = transfers.filter((t) => t.block_number === selectedBlock);

    const amountByAddress = filtered.reduce<Record<string, number>>(
      (acc, t) => {
        const addr = t.from_address || "Unknown";
        const amount = parseFloat(t.amount);
        acc[addr] = (acc[addr] || 0) + amount;
        return acc;
      },
      {}
    );

    pieData = Object.entries(amountByAddress).map(([addr, total]) => ({
      name:
        "From " +
        (addr.length > 12 ? `${addr.slice(0, 8)}...${addr.slice(-4)}` : addr),
      value: total,
    }));
  } else {
    pieData = Object.entries(countByBlock).map(([block, count]) => ({
      name: `Block ${block}`,
      value: count,
    }));

    pieData.sort(
      (a, b) =>
        Number(a.name.replace("Block ", "")) -
        Number(b.name.replace("Block ", ""))
    );
    pieData = pieData.slice(-5);
  }

  const chartColors = [
    "var(--chart-1)",
    "var(--chart-2)",
    "var(--chart-3)",
    "var(--chart-4)",
    "var(--chart-5)",
  ];

  return (
    <Card className="bg-card text-card-foreground rounded-lg space-y-4 max-h-[600px] p-0 pt-2">
      <CardHeader className="my-0 py-0">
        <CardTitle className="text-sm font-semibold">
          Transfer Amount per Block
        </CardTitle>
      </CardHeader>

      <CardContent className="p-2 py-0 h-[220px] my-0">
        <ResponsiveContainer width="100%" height="100%" maxHeight={220}>
          <BarChart
            data={barData}
            margin={{ top: 10, right: 10, left: 0, bottom: 20 }}
          >
            <XAxis
              dataKey="block"
              fontSize={9}
              tick={{ fill: "#888" }}
              tickLine={false}
            />
            <YAxis
              yAxisId="left"
              orientation="left"
              tick={{ fill: "#888", fontSize: 10 }}
              tickLine={false}
            />
            <YAxis
              yAxisId="right"
              orientation="left"
              axisLine={false}
              tick={false}
              tickLine={false}
              label={{
                value: "Total Amount",
                angle: -90,
                position: "insideRight",
                offset: 0,
                fontSize: 11,
                fill: "#888",
              }}
            />
            <Tooltip />
            <Bar dataKey="total" radius={[3, 3, 0, 0]}>
              <LabelList
                dataKey="total"
                position="top"
                fontSize={9}
                fill="#ccc"
                formatter={(val) =>
                  typeof val === "number"
                    ? val.toLocaleString("en-US", { maximumFractionDigits: 0 })
                    : val
                }
              />
              {barData.map((_, index) => (
                <Cell
                  key={`cell-${index}`}
                  fill={chartColors[index % chartColors.length]}
                />
              ))}
            </Bar>
          </BarChart>
        </ResponsiveContainer>
      </CardContent>

      <CardHeader className="py-0 my-0">
        <CardTitle className="text-sm font-semibold py-0">
          {selectedBlock
            ? `Transfers in Block ${selectedBlock}`
            : "Transactions per Block"}
        </CardTitle>
      </CardHeader>

      <CardContent className="p-0 h-[220px] my-0 text-xs">
        <ResponsiveContainer width="100%" height="100%" maxHeight={220}>
          <PieChart>
            <Pie
              data={pieData}
              dataKey="value"
              nameKey="name"
              cx="50%"
              cy="50%"
              outerRadius={80}
              label={(entry: { name?: string }) => entry.name ?? ""}
            >
              {pieData.map((_, index) => (
                <Cell
                  key={`cell-pie-${index}`}
                  fill={chartColors[index % chartColors.length]}
                />
              ))}
            </Pie>
            <Tooltip
              formatter={(val: number) =>
                selectedBlock
                  ? `${val.toLocaleString("en-US")} total amount`
                  : `${val} txs`
              }
              labelFormatter={(label) => `${label}`}
            />
          </PieChart>
        </ResponsiveContainer>
      </CardContent>
    </Card>
  );
};
