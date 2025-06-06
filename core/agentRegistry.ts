import { Agent } from '../agents/base';

/** Global registry holding all available agents */
class AgentRegistry {
  private agents = new Map<string, Agent>();

  registerAgent(name: string, agent: Agent) {
    this.agents.set(name, agent);
  }

  getAgent(name: string): Agent | undefined {
    return this.agents.get(name);
  }

  listAgents(): string[] {
    return Array.from(this.agents.keys());
  }
}

export const agentRegistry = new AgentRegistry();
