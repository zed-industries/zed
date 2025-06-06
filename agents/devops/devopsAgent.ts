import { Agent } from '../base';

/** A dummy DevOps agent managing deployment and infrastructure tasks */
export class DevOpsAgent implements Agent {
  readonly name = 'devops';

  async run(task: string): Promise<string> {
    console.log(`[DevOpsAgent] processing: ${task}`);
    await delay(100);
    return `DevOps response for: ${task}`;
  }

  getCapabilities(): string[] {
    return ['deployment', 'infrastructure'];
  }
}

async function delay(ms: number) {
  return new Promise(resolve => setTimeout(resolve, ms));
}
