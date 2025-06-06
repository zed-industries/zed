/**
 * Base interface that all agents must implement.
 * Defines the common contract for all agents in the system.
 */
export interface IAgent {
  /**
   * Unique identifier for the agent
   */
  readonly id: string;
  
  /**
   * Human-readable name of the agent
   */
  readonly name: string;
  
  /**
   * Description of what the agent does
   */
  readonly description: string;
  
  /**
   * Array of capabilities this agent provides
   */
  readonly capabilities: string[];
  
  /**
   * Execute a task with the given input and return a result
   * @param input The input for the task
   * @param context Optional context data
   * @returns Promise that resolves to the task result
   */
  execute(input: string, context?: Record<string, any>): Promise<AgentResult>;
  
  /**
   * Handle a subtask result from another agent
   * @param result The result from the subtask
   * @param originalContext The original context from the parent task
   */
  handleSubtaskResult?(result: AgentResult, originalContext: Record<string, any>): Promise<void>;
}

/**
 * Result returned by an agent after executing a task
 */
export interface AgentResult {
  /**
   * Whether the task was completed successfully
   */
  success: boolean;
  
  /**
   * Output from the task execution
   */
  output: string;
  
  /**
   * Optional data related to the task result
   */
  data?: Record<string, any>;
  
  /**
   * Optional error if the task failed
   */
  error?: Error;
  
  /**
   * Optional subtasks that need to be executed
   */
  subtasks?: Subtask[];
  
  /**
   * Optional context to be passed to the next agent or subtask
   */
  context?: Record<string, any>;
}

/**
 * Represents a subtask that needs to be executed by another agent
 */
export interface Subtask {
  /**
   * The agent ID that should handle this subtask
   */
  agentId: string;
  
  /**
   * The input for the subtask
   */
  input: string;
  
  /**
   * Optional priority of the subtask (higher = more important)
   */
  priority?: number;
  
  /**
   * Optional context specific to this subtask
   */
  context?: Record<string, any>;
}

/**
 * Base class for all agents that provides common functionality
 */
export abstract class BaseAgent implements IAgent {
  constructor(
    public readonly id: string,
    public readonly name: string,
    public readonly description: string,
    public readonly capabilities: string[] = []
  ) {}

  /**
   * Execute the agent's main task
   */
  public abstract execute(input: string, context?: Record<string, any>): Promise<AgentResult>;
  
  /**
   * Handle a subtask result if the agent needs to process it
   */
  public async handleSubtaskResult(
    result: AgentResult,
    originalContext: Record<string, any>
  ): Promise<void> {
    // Default implementation does nothing
    // Override in subclasses if needed
  }
  
  /**
   * Helper method to create a successful result
   */
  protected createSuccessResult(
    output: string,
    data?: Record<string, any>,
    subtasks?: Subtask[],
    context?: Record<string, any>
  ): AgentResult {
    return {
      success: true,
      output,
      data,
      subtasks,
      context
    };
  }
  
  /**
   * Helper method to create an error result
   */
  protected createErrorResult(
    error: Error | string,
    output: string = 'An error occurred',
    data?: Record<string, any>
  ): AgentResult {
    return {
      success: false,
      output,
      error: error instanceof Error ? error : new Error(error),
      data
    };
  }
}
