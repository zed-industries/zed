import * as fs from 'fs/promises';
import * as path from 'path';
import * as url from 'url';
import { compile } from 'json-schema-to-typescript';

// or just __dirname if running non-ES-Module Node
const dirname = path.dirname(url.fileURLToPath(import.meta.url));

async function main() {
  let schemasPath = path.join(dirname, '..', 'crates/theme/schemas');
  // let schemasPath = path.join(dirname, '..', 'schemas');
  let schemaFiles = (await fs.readdir(schemasPath)).filter((x) => x.endsWith('.json'));

  let compiledTypes = new Set();

  for (let filename of schemaFiles) {
    let filePath = path.join(schemasPath, filename);
    let schema = JSON.parse(await fs.readFile(filePath));
    let compiled = await compile(schema, schema.title, { bannerComment: '' });
    let eachType = compiled.split('export');
    for (let type of eachType) {
      if (!type) {
        continue;
      }
      compiledTypes.add('export ' + type.trim());
    }
  }

  let output = Array.from(compiledTypes).join('\n\n');
  let outputPath = path.join(dirname, 'types', 'theme.ts');

  try {
    let existing = await fs.readFile(outputPath);
    if (existing == output) {
      // Skip writing if it hasn't changed, so that we don't confuse any sort
      // of incremental builds. This check isn't ideal but the script runs
      // quickly enough and rarely enough that it doesn't matter.
      console.log('Schemas are up to date');
      return;
    }
  } catch (e) {
    // It's fine if there's no output from a previous run.
    if (e.code !== 'ENOENT') {
      throw e;
    }
  }

  await fs.writeFile(outputPath, output);
  console.log(`Wrote Typescript types to ${outputPath}`);
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
