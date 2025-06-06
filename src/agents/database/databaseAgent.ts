import { BaseAgent } from '../../core/Agent';
import { contextMemory } from '../../core/contextMemory';

/**
 * DatabaseAgent - Handles database-related tasks including schema design, queries, and migrations.
 */
export class DatabaseAgent extends BaseAgent {
  constructor() {
    super(
      'database',
      'Database Agent',
      'Manages database operations including schema design, queries, and migrations',
      [
        'sql', 'nosql', 'schema', 'query', 'migration', 'index', 'join',
        'transaction', 'orm', 'model', 'table', 'collection'
      ]
    );
  }

  /**
   * Execute a database-related task
   */
  public async execute(input: string, context: Record<string, any> = {}): Promise<AgentResult> {
    console.log(`[DatabaseAgent] Executing task: ${input.substring(0, 100)}...`);
    
    try {
      // Save the context for this agent
      contextMemory.save(this.id, 'lastTask', {
        input,
        timestamp: new Date().toISOString(),
        ...context
      });

      // Task classification
      if (this.isSchemaRelated(input)) {
        return this.handleSchemaTask(input, context);
      } else if (this.isQueryRelated(input)) {
        return this.handleQueryTask(input, context);
      } else if (this.isMigrationRelated(input)) {
        return this.handleMigrationTask(input, context);
      }

      // Default response for general database tasks
      return this.createSuccessResult(
        `I'll help you with the database task: "${input}"`,
        { task: 'database', input, context },
        [
          // Example of a potential subtask
          {
            agentId: 'backend',
            input: `Verify database requirements for: ${input}`,
            priority: 2,
            context: { parentTask: input }
          }
        ]
      );
    } catch (error) {
      console.error(`[DatabaseAgent] Error:`, error);
      return this.createErrorResult(
        error as Error,
        `Failed to process database task: ${input}`
      );
    }
  }

  /**
   * Handle schema-related tasks
   */
  private async handleSchemaTask(
    input: string,
    context: Record<string, any>
  ): Promise<AgentResult> {
    const tableName = this.detectTableName(input) || 'table';
    const columns = this.detectColumns(input) || ['id', 'created_at'];
    const dbType = this.detectDatabaseType(input) || 'PostgreSQL';
    
    // Track created tables in context
    const tables = contextMemory.get<string[]>(this.id, 'tables') || [];
    if (!tables.includes(tableName)) {
      tables.push(tableName);
      contextMemory.save(this.id, 'tables', tables);
    }
    
    return this.createSuccessResult(
      `I'll create a ${dbType} table named '${tableName}' with columns: ${columns.join(', ')}.`,
      {
        tableName,
        columns,
        dbType,
        timestamp: new Date().toISOString(),
        ...context
      },
      [
        // Documentation subtask
        {
          agentId: 'docs',
          input: `Generate schema documentation for table '${tableName}'`,
          priority: 1,
          context: { tableName, columns, dbType }
        }
      ]
    );
  }

  /**
   * Handle query-related tasks
   */
  private async handleQueryTask(
    input: string,
    context: Record<string, any>
  ): Promise<AgentResult> {
    const operation = this.detectQueryOperation(input) || 'SELECT';
    const table = this.detectTableName(input) || 'table';
    const conditions = this.detectQueryConditions(input);
    
    // Track query history in context
    const queryHistory = contextMemory.get<Array<{query: string, timestamp: string}>>(this.id, 'queryHistory') || [];
    queryHistory.push({
      query: `${operation} FROM ${table}${conditions ? ' WHERE ' + conditions : ''}`,
      timestamp: new Date().toISOString()
    });
    contextMemory.save(this.id, 'queryHistory', queryHistory);
    
    return this.createSuccessResult(
      `I'll execute a ${operation} query on the '${table}' table${conditions ? ` with conditions: ${conditions}` : ''}.`,
      {
        operation,
        table,
        conditions,
        timestamp: new Date().toISOString(),
        ...context
      }
    );
  }

  /**
   * Handle database migration tasks
   */
  private async handleMigrationTask(
    input: string,
    context: Record<string, any>
  ): Promise<AgentResult> {
    const migrationType = this.detectMigrationType(input) || 'schema';
    const description = this.detectMigrationDescription(input) || 'Update database';
    
    // Track migrations in context
    const migrations = contextMemory.get<Array<{type: string, description: string, timestamp: string}>>(this.id, 'migrations') || [];
    migrations.push({
      type: migrationType,
      description,
      timestamp: new Date().toISOString()
    });
    contextMemory.save(this.id, 'migrations', migrations);
    
    return this.createSuccessResult(
      `I'll create a ${migrationType} migration: ${description}.`,
      {
        migrationType,
        description,
        timestamp: new Date().toISOString(),
        ...context
      },
      [
        // Backup database before migration
        {
          agentId: 'devops',
          input: 'Create a database backup before applying migration',
          priority: 1,
          context: { migrationType, description }
        }
      ]
    );
  }

  // --- Helper Methods ---
  
  private isSchemaRelated(input: string): boolean {
    const schemaKeywords = ['schema', 'table', 'create table', 'alter table', 'column'];
    return schemaKeywords.some(keyword => input.toLowerCase().includes(keyword));
  }
  
  private isQueryRelated(input: string): boolean {
    const queryKeywords = ['select', 'insert', 'update', 'delete', 'query', 'find', 'get data'];
    return queryKeywords.some(keyword => input.toLowerCase().includes(keyword));
  }
  
  private isMigrationRelated(input: string): boolean {
    const migrationKeywords = ['migrate', 'migration', 'update schema', 'database version'];
    return migrationKeywords.some(keyword => input.toLowerCase().includes(keyword));
  }
  
  private detectTableName(input: string): string | null {
    // Look for patterns like "users table" or "in the posts"
    const tableMatch = input.match(/(?:table|in the|from)\s+([a-zA-Z0-9_]+)/i);
    return tableMatch ? tableMatch[1] : null;
  }
  
  private detectColumns(input: string): string[] | null {
    // Simple detection of column names
    const columnMatches = input.match(/columns?\s*[:=]?\s*\[([^\]]+)\]/i);
    if (columnMatches && columnMatches[1]) {
      return columnMatches[1].split(',').map(col => col.trim());
    }
    
    // Try to detect individual column mentions
    const columnKeywords = ['id', 'name', 'email', 'created_at', 'updated_at'];
    return columnKeywords.filter(keyword => input.toLowerCase().includes(keyword)) || null;
  }
  
  private detectDatabaseType(input: string): string | null {
    const lowerInput = input.toLowerCase();
    if (lowerInput.includes('postgres') || lowerInput.includes('postgre') || lowerInput.includes('postgresql')) return 'PostgreSQL';
    if (lowerInput.includes('mysql')) return 'MySQL';
    if (lowerInput.includes('sqlite')) return 'SQLite';
    if (lowerInput.includes('mongo')) return 'MongoDB';
    if (lowerInput.includes('redis')) return 'Redis';
    return null;
  }
  
  private detectQueryOperation(input: string): string | null {
    const lowerInput = input.toLowerCase();
    if (lowerInput.includes('select') || lowerInput.includes('read') || lowerInput.includes('fetch')) return 'SELECT';
    if (lowerInput.includes('insert') || lowerInput.includes('create') || lowerInput.includes('add')) return 'INSERT';
    if (lowerInput.includes('update') || lowerInput.includes('modify')) return 'UPDATE';
    if (lowerInput.includes('delete') || lowerInput.includes('remove')) return 'DELETE';
    return null;
  }
  
  private detectQueryConditions(input: string): string | null {
    // Simple condition detection
    const conditionMatch = input.match(/(?:where|with|having|when)\s+([^;]+)/i);
    return conditionMatch ? conditionMatch[1].trim() : null;
  }
  
  private detectMigrationType(input: string): string | null {
    const lowerInput = input.toLowerCase();
    if (lowerInput.includes('schema')) return 'schema';
    if (lowerInput.includes('data')) return 'data';
    if (lowerInput.includes('seed')) return 'seed';
    return null;
  }
  
  private detectMigrationDescription(input: string): string | null {
    // Look for a description after keywords like "create migration for"
    const descMatch = input.match(/(?:create|add|new|for|:)\s*(migration\s+)?(?:for\s+)?([^.]+)/i);
    return descMatch ? descMatch[2].trim() : null;
  }
}

// Self-registering the agent
import { agentRegistry } from '../../core/agentRegistry';
const databaseAgent = new DatabaseAgent();
agentRegistry.registerAgent(databaseAgent);

export default databaseAgent;
