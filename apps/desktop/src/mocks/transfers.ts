const transfers = [
  { time: "2025-10-02 10:15", from: "0xAlice", to: "0xBob", token: "USDT" },
  { time: "2025-10-02 10:17", from: "0xEve", to: "0xCarl", token: "USDC" },
  { time: "2025-10-02 10:19", from: "0xDan", to: "0xMike", token: "DAI" },
  { time: "2025-10-02 10:21", from: "0xTom", to: "0xSue", token: "USDT" },
];

const tokenColors: Record<string, string> = {
  USDT: "bg-green-500",
  USDC: "bg-blue-500",
  DAI: "bg-yellow-500",
};

export { transfers, tokenColors };
