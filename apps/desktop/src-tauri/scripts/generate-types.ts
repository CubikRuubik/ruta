// import { compile } from "json-schema-to-typescript";
// import * as fs from "fs";
// import * as path from "path";

// const inputDir = path.resolve(__dirname, "../gen");
// const outputDir = path.resolve(__dirname, "../../src/types/generated");

// fs.mkdirSync(outputDir, { recursive: true });

// for (const file of fs.readdirSync(inputDir)) {
//   if (file.endsWith(".schema.json")) {
//     const schema = JSON.parse(
//       fs.readFileSync(path.join(inputDir, file), "utf-8")
//     );
//     const typeName = file.replace(".schema.json", "");
//     compile(schema, typeName)
//       .then((ts) =>
//         fs.writeFileSync(path.join(outputDir, `${typeName}.ts`), ts)
//       )
//       .catch(console.error);
//   }
// }
