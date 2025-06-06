import { Agent } from '../base';

/** A dummy backend agent handling server-side tasks */
export class BackendAgent implements Agent {
  readonly name = 'backend';

  async run(task: string): Promise<string> {
    console.log(`[BackendAgent] processing: ${task}`);
    await delay(100);
    return `Backend response for: ${task}`;
  }

  getCapabilities(): string[] {
    return ['api-design', 'business-logic'];
  }
}

async function delay(ms: number) {
  return new Promise(resolve => setTimeout(resolve, ms));
}
