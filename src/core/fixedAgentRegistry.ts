import { AgentBase, type AgentResult, type Subtask } from '../agents/AgentBase.js';

/**
 * Registry for managing all agents in the system.
 * Implements the singleton pattern to ensure a single source of truth.
 */
export class AgentRegistry {
  private static instance: AgentRegistry;
  private agents: Map<string, AgentBase> = new Map();

  /**
   * Private constructor to enforce singleton pattern
   */
  private constructor() {}

  /**
   * Get the singleton instance of the AgentRegistry
   */
  public static getInstance(): AgentRegistry {
    if (!AgentRegistry.instance) {
      AgentRegistry.instance = new AgentRegistry();
    }
    return AgentRegistry.instance;
  }

  /**
   * Register a new agent with the registry
   * @param agent The agent to register
   * @throws Error if an agent with the same ID is already registered
   */
  public registerAgent(agent: AgentBase): void {
    if (this.agents.has(agent.id)) {
      throw new Error(`Agent with ID '${agent.id}' is already registered`);
    }
    
    console.log(`[AgentRegistry] Registering agent: ${agent.name} (${agent.id})`);
    this.agents.set(agent.id, agent);
  }

  /**
   * Get an agent by its ID
   * @param agentId The ID of the agent to retrieve
   * @returns The agent if found, undefined otherwise
   */
  public getAgent(agentId: string): AgentBase | undefined {
    return this.agents.get(agentId);
  }

  /**
   * Get all registered agents
   * @returns Array of all registered agents
   */
  public getAgents(): AgentBase[] {
    return Array.from(this.agents.values());
  }

  /**
   * Find agents that have all the specified capabilities
   * @param capabilities Array of capability strings to match
   * @returns Array of agents that have all specified capabilities
   */
  public findAgentsByCapabilities(capabilities: string[]): AgentBase[] {
    return this.getAgents().filter(agent =>
      capabilities.every(capability => 
        agent.getCapabilities().includes(capability)
      )
    );
  }

  /**
   * Execute a task with the specified agent
   * @param agentId The ID of the agent to execute the task
   * @param input The input for the task
   * @param context Optional context data
   * @returns Promise that resolves to the task result
   */
  public async executeWithAgent(
    agentId: string,
    input: string,
    context: Record<string, any> = {}
  ): Promise<AgentResult> {
    const agent = this.getAgent(agentId);
    if (!agent) {
      return {
        success: false,
        output: `Agent with ID '${agentId}' not found`,
        error: `Agent not found: ${agentId}`,
        agentId,
        input,
        timestamp: new Date().toISOString()
      };
    }

    console.log(`[AgentRegistry] Executing with agent: ${agent.name} (${agentId})`);
    console.log(`[AgentRegistry] Input: ${input.substring(0, 100)}${input.length > 100 ? '...' : ''}`);
    
    try {
      const result = await agent.run(input, context);
      console.log(`[AgentRegistry] Agent ${agent.name} execution ${result.success ? 'succeeded' : 'failed'}`);
      return result;
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : String(error);
      console.error(`[AgentRegistry] Error executing agent ${agentId}:`, errorMessage);
      return {
        success: false,
        output: `Error executing agent: ${errorMessage}`,
        error: errorMessage,
        agentId,
        input,
        timestamp: new Date().toISOString()
      };
    }
  }

  /**
   * Route a task to the most appropriate agent based on content
   * @param input The task input
   * @param context Optional context data
   * @returns Promise that resolves to the task result
   */
  public async routeTask(input: string, context: Record<string, any> = {}): Promise<AgentResult> {
    // Simple keyword-based routing - can be enhanced with NLP
    const lowerInput = input.toLowerCase();
    
    let agentType: string;
    if (lowerInput.includes('frontend') || lowerInput.includes('react') || lowerInput.includes('ui')) {
      agentType = 'frontend';
    } else if (lowerInput.includes('backend') || lowerInput.includes('api') || lowerInput.includes('server')) {
      agentType = 'backend';
    } else if (lowerInput.includes('database') || lowerInput.includes('schema') || lowerInput.includes('sql')) {
      agentType = 'database';
    } else if (lowerInput.includes('deploy') || lowerInput.includes('devops') || lowerInput.includes('aws')) {
      agentType = 'devops';
    } else if (lowerInput.includes('doc') || lowerInput.includes('help') || lowerInput.includes('readme')) {
      agentType = 'docs';
    } else {
      agentType = 'general';
    }

    const agent = this.getAgent(agentType);
    if (!agent) {
      return {
        success: false,
        output: `No agent found to handle this task: ${input}`,
        error: `No agent found for type: ${agentType}`,
        agentId: 'system',
        input,
        timestamp: new Date().toISOString()
      };
    }

    return this.executeWithAgent(agentType, input, context);
  }
}

// Export a singleton instance
export const agentRegistry = AgentRegistry.getInstance();
