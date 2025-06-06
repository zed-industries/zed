import { Agent } from '../base';

/** A dummy database agent managing data tasks */
export class DatabaseAgent implements Agent {
  readonly name = 'database';

  async run(task: string): Promise<string> {
    console.log(`[DatabaseAgent] processing: ${task}`);
    await delay(100);
    return `Database response for: ${task}`;
  }

  getCapabilities(): string[] {
    return ['schema-design', 'query-optimization'];
  }
}

async function delay(ms: number) {
  return new Promise(resolve => setTimeout(resolve, ms));
}
