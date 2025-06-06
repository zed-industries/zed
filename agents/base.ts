export interface Agent {
  /** Unique name of the agent */
  readonly name: string;
  /** Execute a task and return a result */
  run(task: string): Promise<string>;
  /** List capabilities of this agent */
  getCapabilities(): string[];
}
