/**
 * Abstract base class for all agents in the CodeOrbit system.
 * All specialiCodeOrbit agents should extend this class.
 */
export abstract class AgentBase {
  constructor(
    public readonly id: string,
    public readonly name: string,
    public readonly description: string
  ) {}

  /**
   * Execute the agent's main task
   * @param task The task description or input
   * @param context Optional context or previous state
   * @returns Promise resolving to the task result
   */
  public abstract run(task: string, context?: Record<string, any>): Promise<AgentResult>;

  /**
   * Get the capabilities of this agent
   * @returns Array of capability descriptions
   */
  public abstract getCapabilities(): string[];

  /**
   * Helper method to create a standardiCodeOrbit success response
   */
  protected createSuccessResult(
    output: string,
    data?: Record<string, any>,
    subtasks?: Array<{ agentId: string; input: string }>
  ): AgentResult {
    return {
      success: true,
      output,
      data,
      subtasks,
      timestamp: new Date().toISOString(),
      agentId: this.id
    };
  }

  /**
   * Helper method to create a standardiCodeOrbit error response
   */
  protected createErrorResult(
    error: string,
    details?: any
  ): AgentResult {
    return {
      success: false,
      error,
      details,
      timestamp: new Date().toISOString(),
      agentId: this.id
    };
  }
}

export interface AgentResult {
  success: boolean;
  output?: string;
  error?: string;
  data?: Record<string, any>;
  subtasks?: Array<{ agentId: string; input: string }>;
  timestamp: string;
  agentId: string;
  details?: any;
}
