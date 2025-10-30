import { FC } from "react";
import { useFilterStore } from "../store/filters";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "./ui/Table";
import { useTransferStore } from "../store/transfers";

export const TransfersTable: FC = () => {
  const { sort: selectedSort, block: selectedBlock } = useFilterStore();
  const { transfers } = useTransferStore();

  const filteredTransfers = transfers.filter((t) => {
    if (!selectedBlock) return true;
    const blockStr = t.block_number.toString();
    const searchStr = selectedBlock.toString();
    return blockStr.startsWith(searchStr);
  });

  const sortedTransfers = filteredTransfers.sort((a, b) => {
    if (selectedSort === "asc") {
      return a.block_number - b.block_number;
    } else {
      return b.block_number - a.block_number;
    }
  });

  console.log(sortedTransfers);

  return (
    <div className="border rounded p-2 bg-card text-card-foreground max-h-[600px] overflow-y-scroll">
      <h2 className="font-bold mb-2">Transfers Table</h2>
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead>ID</TableHead>
            <TableHead>Block</TableHead>
            <TableHead>Tx Hash</TableHead>
            <TableHead>From</TableHead>
            <TableHead>To</TableHead>
            <TableHead>Amount</TableHead>
            <TableHead>Contract</TableHead>
            <TableHead>Created At</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {sortedTransfers.map((t) => (
            <TableRow key={t.id}>
              <TableCell>{t.id}</TableCell>
              <TableCell>{t.block_number}</TableCell>
              <TableCell className="font-mono text-xs truncate max-w-[140px]">
                {t.transaction_hash}
              </TableCell>
              <TableCell className="font-mono text-xs truncate max-w-[120px]">
                {t.from_address}
              </TableCell>
              <TableCell className="font-mono text-xs truncate max-w-[120px]">
                {t.to_address}
              </TableCell>
              <TableCell>{t.amount}</TableCell>
              <TableCell className="font-mono text-xs truncate max-w-[120px]">
                {t.contract_address}
              </TableCell>
              <TableCell>{t.created_at ?? "—"}</TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </div>
  );
};
