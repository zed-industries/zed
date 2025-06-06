/**
 * ContextMemory - A simple in-memory key-value store for managing agent context and state.
 * Implements a singleton pattern to ensure consistent state across the application.
 */
export class ContextMemory {
  private static instance: ContextMemory;
  private memory: Map<string, Map<string, any>>;
  private maxEntriesPerAgent: number;

  /**
   * Private constructor to enforce singleton pattern
   * @param maxEntriesPerAgent Maximum number of entries to keep per agent (for memory management)
   */
  private constructor(maxEntriesPerAgent: number = 100) {
    this.memory = new Map();
    this.maxEntriesPerAgent = maxEntriesPerAgent;
  }

  /**
   * Get the singleton instance of ContextMemory
   * @param maxEntriesPerAgent Optional: Set the max entries per agent (only used on first call)
   */
  public static getInstance(maxEntriesPerAgent?: number): ContextMemory {
    if (!ContextMemory.instance) {
      ContextMemory.instance = new ContextMemory(maxEntriesPerAgent);
    }
    return ContextMemory.instance;
  }

  /**
   * Save context data for a specific agent
   * @param agentId The ID of the agent
   * @param key The key to store the data under
   * @param value The value to store
   */
  public save(agentId: string, key: string, value: any): void {
    if (!this.memory.has(agentId)) {
      this.memory.set(agentId, new Map());
    }

    const agentMemory = this.memory.get(agentId)!;
    agentMemory.set(key, value);
    
    // Enforce maximum entries
    if (agentMemory.size > this.maxEntriesPerAgent) {
      const firstKey = agentMemory.keys().next().value;
      agentMemory.delete(firstKey);
    }
    
    console.log(`[ContextMemory] Saved context for ${agentId}.${key}`);
  }

  /**
   * Get context data for a specific agent
   * @param agentId The ID of the agent
   * @param key The key to retrieve
   * @returns The stored value or undefined if not found
   */
  public get<T = any>(agentId: string, key: string): T | undefined {
    return this.memory.get(agentId)?.get(key);
  }

  /**
   * Get all context data for a specific agent
   * @param agentId The ID of the agent
   * @returns An object containing all context data for the agent
   */
  public getAll(agentId: string): Record<string, any> {
    const agentMemory = this.memory.get(agentId);
    if (!agentMemory) return {};
    
    const result: Record<string, any> = {};
    for (const [key, value] of agentMemory.entries()) {
      result[key] = value;
    }
    return result;
  }

  /**
   * Delete a specific context entry for an agent
   * @param agentId The ID of the agent
   * @param key The key to delete
   * @returns true if the key existed and was deleted, false otherwise
   */
  public delete(agentId: string, key: string): boolean {
    return this.memory.get(agentId)?.delete(key) ?? false;
  }

  /**
   * Clear all context data for a specific agent
   * @param agentId The ID of the agent
   */
  public clearAgent(agentId: string): void {
    this.memory.delete(agentId);
    console.log(`[ContextMemory] Cleared all context for agent: ${agentId}`);
  }

  /**
   * Clear all context data for all agents
   */
  public clearAll(): void {
    this.memory.clear();
    console.log('[ContextMemory] Cleared all context data');
  }

  /**
   * Get the number of entries for a specific agent
   * @param agentId The ID of the agent
   * @returns The number of context entries for the agent, or 0 if none
   */
  public size(agentId: string): number {
    return this.memory.get(agentId)?.size ?? 0;
  }

  /**
   * Check if a specific key exists for an agent
   * @param agentId The ID of the agent
   * @param key The key to check
   * @returns true if the key exists, false otherwise
   */
  public has(agentId: string, key: string): boolean {
    return this.memory.get(agentId)?.has(key) ?? false;
  }

  /**
   * Get all agent IDs that have context stored
   * @returns Array of agent IDs
   */
  public getAgentIds(): string[] {
    return Array.from(this.memory.keys());
  }

  /**
   * Merge new context data with existing data for an agent
   * @param agentId The ID of the agent
   * @param key The key to merge data into
   * @param value The value to merge (must be an object)
   * @param deep Whether to perform a deep merge (default: true)
   */
  public merge(
    agentId: string,
    key: string,
    value: Record<string, any>,
    deep: boolean = true
  ): void {
    if (!this.memory.has(agentId)) {
      this.memory.set(agentId, new Map());
    }

    const agentMemory = this.memory.get(agentId)!;
    const existing = agentMemory.get(key) || {};
    
    const merged = deep ? this.deepMerge(existing, value) : { ...existing, ...value };
    agentMemory.set(key, merged);
    
    console.log(`[ContextMemory] Merged context for ${agentId}.${key}`);
  }

  /**
   * Helper method for deep merging objects
   * @private
   */
  private deepMerge(target: any, source: any): any {
    if (typeof target !== 'object' || target === null) {
      return source;
    }

    if (Array.isArray(target) && Array.isArray(source)) {
      return [...new Set([...target, ...source])];
    }

    const result = { ...target };
    
    for (const key in source) {
      if (source.hasOwnProperty(key)) {
        if (source[key] && typeof source[key] === 'object' && !Array.isArray(source[key])) {
          result[key] = this.deepMerge(
            target[key] || {},
            source[key]
          );
        } else {
          result[key] = source[key];
        }
      }
    }
    
    return result;
  }
}

// Export a singleton instance
export const contextMemory = ContextMemory.getInstance();
