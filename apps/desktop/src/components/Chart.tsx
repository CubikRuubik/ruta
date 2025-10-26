import { FC } from "react";
import { tokenColors, transfers } from "../mocks/transfers";
import { Card, CardContent, CardHeader, CardTitle } from "./ui/Card";

export const Chart: FC = () => {
  return (
    <Card className="bg-card text-card-foreground rounded">
      <CardHeader>
        <CardTitle className="text-base font-semibold">
          Transfer Volume
        </CardTitle>
      </CardHeader>

      <CardContent className="space-y-2">
        {transfers.map((t, i) => (
          <div key={i} className="flex items-center gap-2">
            <span className="w-20 text-xs text-muted-foreground">{t.time}</span>

            <div className="flex-1 h-3 bg-muted rounded">
              <div
                className={`h-3 rounded ${tokenColors[t.token]}`}
                style={{ width: `${Math.floor(Math.random() * 100 + 40)}px` }}
              ></div>
            </div>

            <span
              className={`px-2 py-0.5 text-xs rounded min-w-[60px] text-center ${
                tokenColors[t.token]
              }`}
            >
              {t.token}
            </span>
          </div>
        ))}

        <div className="pt-2 text-xs text-muted-foreground flex flex-wrap items-center gap-2">
          <span>Legend:</span>
          <span className="bg-green-500/70 px-1 rounded text-white">USDT</span>
          <span className="bg-blue-500/70 px-1 rounded text-white">USDC</span>
          <span className="bg-yellow-500/70 px-1 rounded text-black">DAI</span>
        </div>
      </CardContent>
    </Card>
  );
};
