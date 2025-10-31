import fs from "fs";
import path from "path";

const appName = "RUTA";
const targetDir = "./target/release";
const dmgDir = `${targetDir}/bundle/macos/${appName}.app/Contents/Resources`;
const indexerPath = path.resolve(targetDir, "indexer");

if (fs.existsSync(indexerPath)) {
  const destPath = path.join(dmgDir, "indexer");
  fs.copyFileSync(indexerPath, destPath);
  console.log(`indexer found in ${destPath}`);
} else {
  console.error("indexer not found in:", indexerPath);
  process.exit(1);
}
