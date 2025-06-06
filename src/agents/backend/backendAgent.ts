import { BaseAgent } from '../../core/Agent';
import { contextMemory } from '../../core/contextMemory';

/**
 * BackendAgent - Handles backend-related tasks like API development, database operations, and server logic.
 */
export class BackendAgent extends BaseAgent {
  constructor() {
    super(
      'backend',
      'Backend Agent',
      'Handles backend development tasks including APIs, database operations, and server logic',
      [
        'api', 'endpoint', 'server', 'database', 'crud', 'rest', 'graphql',
        'auth', 'authentication', 'authorization', 'middleware', 'validation'
      ]
    );
  }

  /**
   * Execute a backend-related task
   */
  public async execute(input: string, context: Record<string, any> = {}): Promise<AgentResult> {
    console.log(`[BackendAgent] Executing task: ${input.substring(0, 100)}...`);
    
    try {
      // Save the context for this agent
      contextMemory.save(this.id, 'lastTask', {
        input,
        timestamp: new Date().toISOString(),
        ...context
      });

      // Task classification
      if (this.isApiRelated(input)) {
        return this.handleApiTask(input, context);
      } else if (this.isDatabaseRelated(input)) {
        return this.handleDatabaseTask(input, context);
      } else if (this.isAuthRelated(input)) {
        return this.handleAuthTask(input, context);
      }

      // Default response for general backend tasks
      return this.createSuccessResult(
        `I'll help you with the backend task: "${input}"`,
        { task: 'backend', input, context },
        [
          // Example of a potential subtask
          {
            agentId: 'database',
            input: `Check database requirements for: ${input}`,
            priority: 2,
            context: { parentTask: input }
          }
        ]
      );
    } catch (error) {
      console.error(`[BackendAgent] Error:`, error);
      return this.createErrorResult(
        error as Error,
        `Failed to process backend task: ${input}`
      );
    }
  }

  /**
   * Handle API-related tasks
   */
  private async handleApiTask(
    input: string,
    context: Record<string, any>
  ): Promise<AgentResult> {
    const method = this.detectHttpMethod(input) || 'GET';
    const endpoint = this.detectEndpoint(input) || '/api/endpoint';
    const requiresAuth = this.requiresAuthentication(input);
    
    // Track API endpoints in context
    const endpoints = contextMemory.get<string[]>(this.id, 'endpoints') || [];
    endpoints.push(`${method} ${endpoint}`);
    contextMemory.save(this.id, 'endpoints', endpoints);
    
    return this.createSuccessResult(
      `I'll create a ${method} endpoint at ${endpoint}${requiresAuth ? ' with authentication' : ''}.`,
      {
        method,
        endpoint,
        requiresAuth,
        timestamp: new Date().toISOString(),
        ...context
      },
      [
        // Documentation subtask
        {
          agentId: 'docs',
          input: `Generate API documentation for ${method} ${endpoint}`,
          priority: 1,
          context: { method, endpoint, requiresAuth }
        },
        // Database subtask if needed
        ...(this.requiresDatabase(input) ? [{
          agentId: 'database',
          input: `Set up database schema for ${endpoint} endpoint`,
          priority: 2,
          context: { endpoint, method }
        }] : [])
      ]
    );
  }

  /**
   * Handle database-related tasks
   */
  private async handleDatabaseTask(
    input: string,
    context: Record<string, any>
  ): Promise<AgentResult> {
    const operation = this.detectDatabaseOperation(input) || 'query';
    const table = this.detectTableName(input) || 'items';
    
    return this.createSuccessResult(
      `I'll perform a database ${operation} on the ${table} table.`,
      {
        operation,
        table,
        timestamp: new Date().toISOString(),
        ...context
      }
    );
  }

  /**
   * Handle authentication/authorization tasks
   */
  private async handleAuthTask(
    input: string,
    context: Record<string, any>
  ): Promise<AgentResult> {
    const authType = this.detectAuthType(input) || 'JWT';
    const requiresRoles = this.detectRequiredRoles(input);
    
    return this.createSuccessResult(
      `I'll implement ${authType} authentication${requiresRoles.length > 0 ? ` with roles: ${requiresRoles.join(', ')}` : ''}.`,
      {
        authType,
        requiresRoles,
        timestamp: new Date().toISOString(),
        ...context
      }
    );
  }

  // --- Helper Methods ---

  
  private isApiRelated(input: string): boolean {
    const apiKeywords = ['api', 'endpoint', 'route', 'rest', 'graphql'];
    return apiKeywords.some(keyword => input.toLowerCase().includes(keyword));
  }
  
  private isDatabaseRelated(input: string): boolean {
    const dbKeywords = ['database', 'table', 'query', 'schema', 'migration'];
    return dbKeywords.some(keyword => input.toLowerCase().includes(keyword));
  }
  
  private isAuthRelated(input: string): boolean {
    const authKeywords = ['auth', 'login', 'signin', 'signup', 'register', 'jwt', 'oauth'];
    return authKeywords.some(keyword => input.toLowerCase().includes(keyword));
  }
  
  private detectHttpMethod(input: string): string | null {
    const lowerInput = input.toLowerCase();
    if (lowerInput.includes('get ')) return 'GET';
    if (lowerInput.includes('post ') || lowerInput.includes('create')) return 'POST';
    if (lowerInput.includes('put ') || lowerInput.includes('update')) return 'PUT';
    if (lowerInput.includes('delete ') || lowerInput.includes('remove')) return 'DELETE';
    if (lowerInput.includes('patch ')) return 'PATCH';
    return null;
  }
  
  private detectEndpoint(input: string): string | null {
    // Simple regex to find URL patterns
    const endpointMatch = input.match(/\/([a-zA-Z0-9\/\-_{}]+)/);
    return endpointMatch ? endpointMatch[0] : null;
  }
  
  private requiresAuthentication(input: string): boolean {
    const authKeywords = ['auth', 'protected', 'secure', 'login required'];
    return authKeywords.some(keyword => input.toLowerCase().includes(keyword));
  }
  
  private requiresDatabase(input: string): boolean {
    const dbKeywords = ['save', 'store', 'retrieve', 'fetch', 'get data'];
    return dbKeywords.some(keyword => input.toLowerCase().includes(keyword));
  }
  
  private detectDatabaseOperation(input: string): string | null {
    const lowerInput = input.toLowerCase();
    if (lowerInput.includes('select') || lowerInput.includes('read')) return 'select';
    if (lowerInput.includes('insert') || lowerInput.includes('create')) return 'insert';
    if (lowerInput.includes('update')) return 'update';
    if (lowerInput.includes('delete') || lowerInput.includes('remove')) return 'delete';
    if (lowerInput.includes('migrate')) return 'migration';
    return null;
  }
  
  private detectTableName(input: string): string | null {
    // Look for patterns like "users table" or "in the posts"
    const tableMatch = input.match(/(?:table|in the|from)\s+([a-zA-Z0-9_]+)/i);
    return tableMatch ? tableMatch[1] : null;
  }
  
  private detectAuthType(input: string): string | null {
    const lowerInput = input.toLowerCase();
    if (lowerInput.includes('jwt')) return 'JWT';
    if (lowerInput.includes('oauth')) return 'OAuth';
    if (lowerInput.includes('saml')) return 'SAML';
    if (lowerInput.includes('basic auth')) return 'Basic Auth';
    return null;
  }
  
  private detectRequiredRoles(input: string): string[] {
    const roles: string[] = [];
    const roleKeywords = ['admin', 'user', 'manager', 'editor', 'viewer'];
    
    for (const role of roleKeywords) {
      if (input.toLowerCase().includes(role)) {
        roles.push(role);
      }
    }
    
    return roles;
  }
}

// Self-registering the agent
import { agentRegistry } from '../../core/agentRegistry';
const backendAgent = new BackendAgent();
agentRegistry.registerAgent(backendAgent);

export default backendAgent;
