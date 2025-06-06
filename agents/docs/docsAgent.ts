import { Agent } from '../base';

/** A dummy documentation agent */
export class DocsAgent implements Agent {
  readonly name = 'docs';

  async run(task: string): Promise<string> {
    console.log(`[DocsAgent] processing: ${task}`);
    await delay(100);
    return `Docs response for: ${task}`;
  }

  getCapabilities(): string[] {
    return ['documentation'];
  }
}

async function delay(ms: number) {
  return new Promise(resolve => setTimeout(resolve, ms));
}
