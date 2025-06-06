import { agentRegistry } from './agentRegistry';
import { contextMemory } from './contextMemory';
import { Agent } from '../agents/base';

/** Central orchestrator that routes tasks between agents */
export class Orchestrator {
  async receivePrompt(prompt: string): Promise<string> {
    console.log(`[Orchestrator] received prompt: ${prompt}`);

    const tasks = this.breakIntoSubtasks(prompt);
    const results: string[] = [];

    for (const { agentName, task } of tasks) {
      const agent = agentRegistry.getAgent(agentName);
      if (!agent) {
        console.warn(`No agent registered for ${agentName}`);
        continue;
      }
      const result = await this.runAgentTask(agent, task);
      results.push(`${agentName}: ${result}`);
      contextMemory.saveContext(agentName, { lastTask: task, lastResult: result });
    }

    return results.join('\n');
  }

  private breakIntoSubtasks(prompt: string): Array<{ agentName: string; task: string }> {
    const lower = prompt.toLowerCase();
    const tasks: Array<{ agentName: string; task: string }> = [];

    if (/(ui|frontend|react|component)/.test(lower)) {
      tasks.push({ agentName: 'frontend', task: prompt });
    }
    if (/(api|server|backend)/.test(lower)) {
      tasks.push({ agentName: 'backend', task: prompt });
    }
    if (/(db|database|schema)/.test(lower)) {
      tasks.push({ agentName: 'database', task: prompt });
    }
    if (/(deploy|ci|docker|infrastructure)/.test(lower)) {
      tasks.push({ agentName: 'devops', task: prompt });
    }
    if (/(doc|readme)/.test(lower)) {
      tasks.push({ agentName: 'docs', task: prompt });
    }

    // Default to frontend if no keywords matched
    if (tasks.length === 0) {
      tasks.push({ agentName: 'frontend', task: prompt });
    }

    return tasks;
  }

  private async runAgentTask(agent: Agent, task: string): Promise<string> {
    try {
      return await agent.run(task);
    } catch (err: any) {
      console.error(`Agent ${agent.name} failed:`, err);
      return `error from ${agent.name}`;
    }
  }
}
