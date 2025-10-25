import { FC } from "react";
import { tokenColors, transfers } from "../mocks/transfers";
import { useFilterStore } from "../store/filters";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "./ui/Table";

export const TransfersTable: FC = () => {
  const { token: selectedToken } = useFilterStore();

  const filteredTransfers = transfers.filter(
    (t) => selectedToken === "All" || t.token === selectedToken
  );

  return (
    <div className="border rounded p-2 bg-card text-card-foreground">
      <h2 className="font-bold mb-2">Transfers Table</h2>
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead>Time</TableHead>
            <TableHead>From</TableHead>
            <TableHead>To</TableHead>
            <TableHead>Token</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {filteredTransfers.map((t, i) => (
            <TableRow key={i}>
              <TableCell>{t.time}</TableCell>
              <TableCell>{t.from}</TableCell>
              <TableCell>{t.to}</TableCell>
              <TableCell className="px-4  py-4">
                <span
                  className={`inline-flex items-center justify-center w-20 h-7 rounded font-medium text-sm ${
                    tokenColors[t.token]
                  }`}
                >
                  {t.token}
                </span>
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </div>
  );
};
