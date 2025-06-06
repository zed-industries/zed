import { BaseAgent } from '../../core/Agent';
import { contextMemory } from '../../core/contextMemory';

/**
 * DevOpsAgent - Handles deployment, infrastructure, and CI/CD related tasks.
 */
export class DevOpsAgent extends BaseAgent {
  private supportedPlatforms = ['aws', 'azure', 'gcp', 'docker', 'kubernetes', 'github-actions', 'gitlab-ci'];
  
  constructor() {
    super(
      'devops',
      'DevOps Agent',
      'Manages deployment, infrastructure, and CI/CD pipelines',
      [
        'deploy', 'deployment', 'ci/cd', 'pipeline', 'docker', 'kubernetes',
        'aws', 'azure', 'gcp', 'cloud', 'infrastructure', 'terraform', 'ansible'
      ]
    );
  }

  /**
   * Execute a DevOps-related task
   */
  public async execute(input: string, context: Record<string, any> = {}): Promise<AgentResult> {
    console.log(`[DevOpsAgent] Executing task: ${input.substring(0, 100)}...`);
    
    try {
      // Save the context for this agent
      contextMemory.save(this.id, 'lastTask', {
        input,
        timestamp: new Date().toISOString(),
        ...context
      });

      // Task classification
      if (this.isDeploymentTask(input)) {
        return this.handleDeploymentTask(input, context);
      } else if (this.isInfrastructureTask(input)) {
        return this.handleInfrastructureTask(input, context);
      } else if (this.isCICDTask(input)) {
        return this.handleCICDTask(input, context);
      }

      // Default response for general DevOps tasks
      return this.createSuccessResult(
        `I'll help you with the DevOps task: "${input}"`,
        { task: 'devops', input, context },
        [
          // Example of a potential subtask
          {
            agentId: 'backend',
            input: `Check system requirements for: ${input}`,
            priority: 2,
            context: { parentTask: input }
          }
        ]
      );
    } catch (error) {
      console.error(`[DevOpsAgent] Error:`, error);
      return this.createErrorResult(
        error as Error,
        `Failed to process DevOps task: ${input}`
      );
    }
  }

  /**
   * Handle deployment-related tasks
   */
  private async handleDeploymentTask(
    input: string,
    context: Record<string, any>
  ): Promise<AgentResult> {
    const environment = this.detectEnvironment(input) || 'staging';
    const target = this.detectDeploymentTarget(input) || 'web server';
    const platform = this.detectPlatform(input) || 'docker';
    
    // Track deployments in context
    const deployments = contextMemory.get<Array<{env: string, target: string, platform: string, timestamp: string}>>(this.id, 'deployments') || [];
    deployments.push({
      env: environment,
      target,
      platform,
      timestamp: new Date().toISOString()
    });
    contextMemory.save(this.id, 'deployments', deployments);
    
    return this.createSuccessResult(
      `I'll deploy to ${environment} environment on ${target} using ${platform}.`,
      {
        environment,
        target,
        platform,
        timestamp: new Date().toISOString(),
        ...context
      },
      [
        // Pre-deployment checks
        {
          agentId: 'backend',
          input: 'Run pre-deployment tests',
          priority: 1,
          context: { environment, target, platform }
        },
        // Database migration if needed
        ...(this.needsDatabaseMigration(input) ? [{
          agentId: 'database',
          input: 'Run database migrations before deployment',
          priority: 2,
          context: { environment }
        }] : [])
      ]
    );
  }

  /**
   * Handle infrastructure-related tasks
   */
  private async handleInfrastructureTask(
    input: string,
    context: Record<string, any>
  ): Promise<AgentResult> {
    const resource = this.detectInfrastructureResource(input) || 'server';
    const action = this.detectInfrastructureAction(input) || 'create';
    const platform = this.detectPlatform(input) || 'aws';
    
    // Track infrastructure changes in context
    const changes = contextMemory.get<Array<{resource: string, action: string, platform: string, timestamp: string}>>(this.id, 'infraChanges') || [];
    changes.push({
      resource,
      action,
      platform,
      timestamp: new Date().toISOString()
    });
    contextMemory.save(this.id, 'infraChanges', changes);
    
    return this.createSuccessResult(
      `I'll ${action} ${resource} on ${platform}.`,
      {
        resource,
        action,
        platform,
        timestamp: new Date().toISOString(),
        ...context
      },
      [
        // Infrastructure validation
        {
          agentId: 'devops',
          input: `Validate ${resource} configuration for ${platform}`,
          priority: 1,
          context: { resource, action, platform }
        }
      ]
    );
  }

  /**
   * Handle CI/CD pipeline tasks
   */
  private async handleCICDTask(
    input: string,
    context: Record<string, any>
  ): Promise<AgentResult> {
    const pipeline = this.detectPipelineType(input) || 'CI/CD';
    const action = this.detectPipelineAction(input) || 'configure';
    
    // Track pipeline changes in context
    const pipelineChanges = contextMemory.get<Array<{pipeline: string, action: string, timestamp: string}>>(this.id, 'pipelineChanges') || [];
    pipelineChanges.push({
      pipeline,
      action,
      timestamp: new Date().toISOString()
    });
    contextMemory.save(this.id, 'pipelineChanges', pipelineChanges);
    
    return this.createSuccessResult(
      `I'll ${action} the ${pipeline} pipeline.`,
      {
        pipeline,
        action,
        timestamp: new Date().toISOString(),
        ...context
      },
      [
        // Pipeline validation
        {
          agentId: 'devops',
          input: `Validate ${pipeline} pipeline configuration`,
          priority: 1,
          context: { pipeline, action }
        }
      ]
    );
  }

  // --- Helper Methods ---
  
  private isDeploymentTask(input: string): boolean {
    const deployKeywords = ['deploy', 'release', 'publish', 'ship'];
    return deployKeywords.some(keyword => input.toLowerCase().includes(keyword));
  }
  
  private isInfrastructureTask(input: string): boolean {
    const infraKeywords = ['infrastructure', 'provision', 'server', 'cloud', 'instance', 'terraform', 'ansible'];
    return infraKeywords.some(keyword => input.toLowerCase().includes(keyword));
  }
  
  private isCICDTask(input: string): boolean {
    const ciKeywords = ['ci/cd', 'pipeline', 'github actions', 'gitlab ci', 'jenkins', 'circleci'];
    return ciKeywords.some(keyword => input.toLowerCase().includes(keyword));
  }
  
  private detectEnvironment(input: string): string | null {
    const envMatch = input.match(/(staging|production|development|prod|dev|test)(?:\s+environment)?/i);
    if (envMatch) {
      const env = envMatch[1].toLowerCase();
      return env === 'prod' ? 'production' : env === 'dev' ? 'development' : env;
    }
    return null;
  }
  
  private detectDeploymentTarget(input: string): string | null {
    const targetMatch = input.match(/to\s+(server|container|lambda|function|static\s+site|kubernetes|k8s)/i);
    return targetMatch ? targetMatch[1].toLowerCase() : null;
  }
  
  private detectPlatform(input: string): string | null {
    const lowerInput = input.toLowerCase();
    const platform = this.supportedPlatforms.find(p => lowerInput.includes(p));
    return platform || null;
  }
  
  private needsDatabaseMigration(input: string): boolean {
    const migrationKeywords = ['database', 'db', 'migration', 'schema change'];
    return migrationKeywords.some(keyword => input.toLowerCase().includes(keyword));
  }
  
  private detectInfrastructureResource(input: string): string | null {
    const resourceMatch = input.match(/(server|database|load balancer|vpc|network|cluster|kubernetes|k8s)/i);
    return resourceMatch ? resourceMatch[1].toLowerCase() : null;
  }
  
  private detectInfrastructureAction(input: string): string | null {
    const lowerInput = input.toLowerCase();
    if (lowerInput.includes('create') || lowerInput.includes('provision')) return 'create';
    if (lowerInput.includes('update') || lowerInput.includes('modify')) return 'update';
    if (lowerInput.includes('delete') || lowerInput.includes('remove')) return 'delete';
    if (lowerInput.includes('scale')) return 'scale';
    return null;
  }
  
  private detectPipelineType(input: string): string | null {
    const lowerInput = input.toLowerCase();
    if (lowerInput.includes('github')) return 'GitHub Actions';
    if (lowerInput.includes('gitlab')) return 'GitLab CI';
    if (lowerInput.includes('jenkins')) return 'Jenkins';
    if (lowerInput.includes('circleci')) return 'CircleCI';
    return null;
  }
  
  private detectPipelineAction(input: string): string | null {
    const lowerInput = input.toLowerCase();
    if (lowerInput.includes('create') || lowerInput.includes('set up')) return 'create';
    if (lowerInput.includes('update') || lowerInput.includes('modify')) return 'update';
    if (lowerInput.includes('fix') || lowerInput.includes('debug')) return 'fix';
    if (lowerInput.includes('run') || lowerInput.includes('execute')) return 'run';
    return null;
  }
}

// Self-registering the agent
import { agentRegistry } from '../../core/agentRegistry';
const devopsAgent = new DevOpsAgent();
agentRegistry.registerAgent(devopsAgent);

export default devopsAgent;
