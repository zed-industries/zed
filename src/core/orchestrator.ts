import { agentRegistry, AgentResult } from './agentRegistry';
import { contextMemory } from './contextMemory';
import { IAgent, Subtask } from './Agent';

/**
 * Orchestrator - Manages the flow of tasks between different agents.
 * Responsible for parsing prompts, routing tasks, and managing the execution flow.
 */
export class Orchestrator {
  private static instance: Orchestrator;
  private taskQueue: Array<{
    task: string;
    context: Record<string, any>;
    resolve: (value: AgentResult) => void;
    reject: (reason?: any) => void;
  }> = [];
  private isProcessing: boolean = false;

  /**
   * Private constructor to enforce singleton pattern
   */
  private constructor() {}

  /**
   * Get the singleton instance of Orchestrator
   */
  public static getInstance(): Orchestrator {
    if (!Orchestrator.instance) {
      Orchestrator.instance = new Orchestrator();
    }
    return Orchestrator.instance;
  }

  /**
   * Receive a prompt and process it through the appropriate agents
   * @param prompt The user's input prompt
   * @param context Optional context data
   * @returns Promise that resolves with the final result
   */
  public async receivePrompt(
    prompt: string,
    context: Record<string, any> = {}
  ): Promise<AgentResult> {
    console.log(`[Orchestrator] Received prompt: ${prompt.substring(0, 100)}${prompt.length > 100 ? '...' : ''}`);
    
    return new Promise((resolve, reject) => {
      // Add to queue
      this.taskQueue.push({ task: prompt, context, resolve, reject });
      
      // Start processing if not already
      if (!this.isProcessing) {
        this.processQueue();
      }
    });
  }

  /**
   * Process the next task in the queue
   * @private
   */
  private async processQueue(): Promise<void> {
    if (this.taskQueue.length === 0) {
      this.isProcessing = false;
      return;
    }

    this.isProcessing = true;
    const { task, context, resolve, reject } = this.taskQueue.shift()!;

    try {
      // Parse the prompt to determine the appropriate agent(s)
      const { agentId, subtasks } = this.parsePrompt(task, context);
      
      // Execute with the primary agent
      let result = await agentRegistry.executeWithAgent(agentId, task, context);
      
      // Process any subtasks if the agent returned them
      if (result.subtasks && result.subtasks.length > 0) {
        console.log(`[Orchestrator] Processing ${result.subtasks.length} subtasks`);
        const subtaskResults = await agentRegistry.executeSubtasks(result.subtasks);
        
        // Merge subtask results
        result = this.mergeResults(result, subtaskResults);
      }
      
      // Save context if provided
      if (result.context) {
        contextMemory.merge('orchestrator', 'lastContext', result.context);
      }
      
      resolve(result);
    } catch (error) {
      console.error('[Orchestrator] Error processing task:', error);
      reject({
        success: false,
        output: `Error processing task: ${error instanceof Error ? error.message : String(error)}`,
        error: error instanceof Error ? error : new Error(String(error))
      });
    } finally {
      // Process next task in queue
      this.processQueue();
    }
  }

  /**
   * Parse the prompt to determine which agent(s) should handle it
   * @private
   */
  private parsePrompt(
    prompt: string,
    context: Record<string, any>
  ): { agentId: string; subtasks?: Subtask[] } {
    // Simple keyword-based routing - can be enhanced with NLP later
    const lowerPrompt = prompt.toLowerCase();
    
    // Check for specific agent mentions
    const agentMentionMatch = prompt.match(/@(\w+)/);
    if (agentMentionMatch) {
      const agentId = agentMentionMatch[1].toLowerCase();
      if (agentRegistry.getAgent(agentId)) {
        return { agentId };
      }
    }
    
    // Route based on keywords
    const frontendKeywords = ['ui', 'frontend', 'react', 'vue', 'component', 'button', 'form'];
    const backendKeywords = ['api', 'server', 'database', 'endpoint', 'route'];
    const devopsKeywords = ['deploy', 'build', 'ci/cd', 'pipeline', 'docker', 'kubernetes'];
    const docsKeywords = ['document', 'readme', 'comment', 'explain', 'help'];
    
    if (frontendKeywords.some(kw => lowerPrompt.includes(kw))) {
      return { agentId: 'frontend' };
    } else if (backendKeywords.some(kw => lowerPrompt.includes(kw))) {
      return { agentId: 'backend' };
    } else if (devopsKeywords.some(kw => lowerPrompt.includes(kw))) {
      return { agentId: 'devops' };
    } else if (docsKeywords.some(kw => lowerPrompt.includes(kw))) {
      return { agentId: 'docs' };
    }
    
    // Default to frontend agent
    return { agentId: 'frontend' };
  }

  /**
   * Merge results from multiple agents
   * @private
   */
  private mergeResults(
    mainResult: AgentResult,
    subtaskResults: AgentResult[]
  ): AgentResult {
    // Simple merge strategy - can be enhanced based on requirements
    const allSuccess = subtaskResults.every(r => r.success);
    
    // Combine outputs with newlines
    const combinedOutput = [
      mainResult.output,
      ...subtaskResults.map(r => r.output)
    ].filter(Boolean).join('\n\n');
    
    // Combine data objects
    const combinedData = {
      ...mainResult.data,
      subtaskResults: subtaskResults.map(r => r.data)
    };
    
    // Get the first error if any subtask failed
    const error = subtaskResults.find(r => r.error)?.error;
    
    return {
      success: mainResult.success && allSuccess,
      output: combinedOutput,
      data: combinedData,
      error: mainResult.error || error
    };
  }

  /**
   * Get the status of the orchestrator
   */
  public getStatus() {
    return {
      isProcessing: this.isProcessing,
      queueLength: this.taskQueue.length,
      nextTask: this.taskQueue[0]?.task.substring(0, 50) + (this.taskQueue[0]?.task.length > 50 ? '...' : '')
    };
  }

  /**
   * Clear the task queue
   */
  public clearQueue(): void {
    this.taskQueue = [];
    console.log('[Orchestrator] Task queue cleared');
  }
}

// Export a singleton instance
export const orchestrator = Orchestrator.getInstance();
