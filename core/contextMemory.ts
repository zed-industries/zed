/** Simple in-memory store for agent context */
export class ContextMemory {
  private store = new Map<string, any>();

  saveContext(agentName: string, data: any) {
    this.store.set(agentName, data);
  }

  getContext<T>(agentName: string): T | undefined {
    return this.store.get(agentName);
  }
}

export const contextMemory = new ContextMemory();
