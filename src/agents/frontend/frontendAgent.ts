import { BaseAgent } from '../../core/Agent';
import { contextMemory } from '../../core/contextMemory';

/**
 * FrontendAgent - Handles frontend-related tasks like UI components and layouts.
 */
export class FrontendAgent extends BaseAgent {
  constructor() {
    super(
      'frontend',
      'Frontend Agent',
      'Handles frontend development tasks including UI components, layouts, and interactions',
      [
        'react', 'vue', 'svelte', 'component', 'ui', 'layout',
        'button', 'form', 'input', 'modal', 'table', 'grid'
      ]
    );
  }

  /**
   * Execute a frontend-related task
   */
  public async execute(input: string, context: Record<string, any> = {}): Promise<AgentResult> {
    console.log(`[FrontendAgent] Executing task: ${input.substring(0, 100)}...`);
    
    try {
      // Save the context for this agent
      contextMemory.save(this.id, 'lastTask', {
        input,
        timestamp: new Date().toISOString(),
        ...context
      });

      // Simple task classification
      if (input.toLowerCase().includes('component')) {
        return this.handleComponentTask(input, context);
      } else if (input.toLowerCase().includes('layout')) {
        return this.handleLayoutTask(input, context);
      } else if (input.toLowerCase().includes('form') || input.toLowerCase().includes('input')) {
        return this.handleFormTask(input, context);
      }

      // Default response for general frontend tasks
      return this.createSuccessResult(
        `I'll help you with the frontend task: "${input}"`,
        { task: 'frontend', input, context },
        [
          // Example of a subtask that might be needed
          {
            agentId: 'backend',
            input: `Check if we need any backend APIs for: ${input}`,
            priority: 2,
            context: { parentTask: input }
          }
        ]
      );
    } catch (error) {
      console.error(`[FrontendAgent] Error:`, error);
      return this.createErrorResult(
        error as Error,
        `Failed to process frontend task: ${input}`
      );
    }
  }

  /**
   * Handle component-related tasks
   */
  private async handleComponentTask(
    input: string,
    context: Record<string, any>
  ): Promise<AgentResult> {
    const componentType = this.detectComponentType(input);
    const framework = this.detectFramework(input) || 'react';
    
    // Save component creation in context
    const components = contextMemory.get<string[]>(this.id, 'components') || [];
    components.push(componentType);
    contextMemory.save(this.id, 'components', components);
    
    return this.createSuccessResult(
      `I'll create a ${componentType} component using ${framework}.`,
      {
        componentType,
        framework,
        timestamp: new Date().toISOString(),
        ...context
      },
      [
        // Sub-tasks for component creation
        {
          agentId: 'docs',
          input: `Generate documentation for a ${componentType} component in ${framework}`,
          priority: 1,
          context: { parentTask: input, componentType, framework }
        }
      ]
    );
  }

  /**
   * Handle layout-related tasks
   */
  private async handleLayoutTask(
    input: string,
    context: Record<string, any>
  ): Promise<AgentResult> {
    const layoutType = this.detectLayoutType(input);
    
    return this.createSuccessResult(
      `I'll create a ${layoutType} layout.`,
      {
        layoutType,
        timestamp: new Date().toISOString(),
        ...context
      }
    );
  }

  /**
   * Handle form-related tasks
   */
  private async handleFormTask(
    input: string,
    context: Record<string, any>
  ): Promise<AgentResult> {
    const formFields = this.detectFormFields(input);
    
    return this.createSuccessResult(
      `I'll create a form with these fields: ${formFields.join(', ')}`,
      {
        formFields,
        timestamp: new Date().toISOString(),
        ...context
      },
      [
        // Validate form fields with backend
        {
          agentId: 'backend',
          input: `Validate form fields: ${formFields.join(', ')}`,
          priority: 1,
          context: { parentTask: input, formFields }
        }
      ]
    );
  }

  /**
   * Detect component type from input
   */
  private detectComponentType(input: string): string {
    const lowerInput = input.toLowerCase();
    
    if (lowerInput.includes('button')) return 'Button';
    if (lowerInput.includes('card')) return 'Card';
    if (lowerInput.includes('modal') || lowerInput.includes('dialog')) return 'Modal';
    if (lowerInput.includes('table') || lowerInput.includes('grid')) return 'DataTable';
    if (lowerInput.includes('nav') || lowerInput.includes('menu')) return 'Navigation';
    
    return 'Component';
  }

  /**
   * Detect framework from input
   */
  private detectFramework(input: string): string | null {
    const lowerInput = input.toLowerCase();
    
    if (lowerInput.includes('react')) return 'React';
    if (lowerInput.includes('vue')) return 'Vue';
    if (lowerInput.includes('svelte')) return 'Svelte';
    if (lowerInput.includes('angular')) return 'Angular';
    
    return null;
  }

  /**
   * Detect layout type from input
   */
  private detectLayoutType(input: string): string {
    const lowerInput = input.toLowerCase();
    
    if (lowerInput.includes('dashboard')) return 'Dashboard';
    if (lowerInput.includes('auth') || lowerInput.includes('login')) return 'Authentication';
    if (lowerInput.includes('admin')) return 'Admin';
    if (lowerInput.includes('landing') || lowerInput.includes('home')) return 'Landing Page';
    
    return 'Default';
  }

  /**
   * Detect form fields from input
   */
  private detectFormFields(input: string): string[] {
    const fields: string[] = [];
    const fieldTypes = [
      'name', 'email', 'password', 'address', 'phone', 'date',
      'time', 'color', 'file', 'checkbox', 'radio', 'select'
    ];
    
    for (const field of fieldTypes) {
      if (input.toLowerCase().includes(field)) {
        fields.push(field);
      }
    }
    
    // If no specific fields detected, return some defaults
    if (fields.length === 0) {
      return ['name', 'email', 'message'];
    }
    
    return fields;
  }
}

// Self-registering the agent
import { agentRegistry } from '../../core/agentRegistry';
const frontendAgent = new FrontendAgent();
agentRegistry.registerAgent(frontendAgent);

export default frontendAgent;
