import { AgentBase, type AgentResult } from '../AgentBase.js';
import { contextMemory } from '../../core/contextMemory.js';

interface ComponentInfo {
  type: string;
  framework: string;
  name: string;
  props: Record<string, any>;
  children?: string[];
}

interface FormField {
  name: string;
  type: string;
  required: boolean;
  label: string;
  placeholder?: string;
  options?: string[];
}

export class FrontendAgent extends AgentBase {
  private capabilities: string[] = [
    'react', 'vue', 'svelte', 'angular', 'component', 'ui', 'layout',
    'button', 'form', 'input', 'modal', 'table', 'grid', 'typescript',
    'javascript', 'css', 'html', 'responsive', 'accessibility'
  ];

  constructor() {
    super(
      'frontend',
      'Frontend Agent',
      'Handles frontend development tasks including UI components, layouts, and interactions'
    );
  }

  public getCapabilities(): string[] {
    return this.capabilities;
  }

  private detectComponentType(input: string): string {
    const lowerInput = input.toLowerCase();
    if (lowerInput.includes('button')) return 'button';
    if (lowerInput.includes('modal') || lowerInput.includes('dialog')) return 'modal';
    if (lowerInput.includes('card')) return 'card';
    if (lowerInput.includes('form')) return 'form';
    if (lowerInput.includes('input') || lowerInput.includes('field')) return 'input';
    if (lowerInput.includes('table') || lowerInput.includes('grid')) return 'data-grid';
    if (lowerInput.includes('nav') || lowerInput.includes('menu')) return 'navigation';
    return 'generic';
  }

  private detectFramework(input: string): string {
    const lowerInput = input.toLowerCase();
    if (lowerInput.includes('react')) return 'react';
    if (lowerInput.includes('vue')) return 'vue';
    if (lowerInput.includes('svelte')) return 'svelte';
    if (lowerInput.includes('angular')) return 'angular';
    return 'react'; // Default to React
  }

  private async handleComponentTask(input: string, context: Record<string, any>): Promise<AgentResult> {
    const componentType = this.detectComponentType(input);
    const framework = this.detectFramework(input);
    
    return this.createSuccessResult(
      `Created ${componentType} component using ${framework}`,
      { componentType, framework, input, ...context }
    );
  }

  private async handleLayoutTask(input: string, context: Record<string, any>): Promise<AgentResult> {
    const layoutType = input.toLowerCase().includes('sidebar') ? 'sidebar' : 'standard';
    return this.createSuccessResult(
      `Created ${layoutType} layout`,
      { layoutType, input, ...context }
    );
  }

  private async handleFormTask(input: string, context: Record<string, any>): Promise<AgentResult> {
    const formFields: FormField[] = [
      { name: 'username', type: 'text', required: true, label: 'Username' },
      { name: 'email', type: 'email', required: true, label: 'Email' },
      { name: 'password', type: 'password', required: true, label: 'Password' }
    ];
    
    return this.createSuccessResult(
      'Created form with standard fields',
      { formFields, input, ...context }
    );
  }

  private async handleStylingTask(input: string, context: Record<string, any>): Promise<AgentResult> {
    const styleType = input.toLowerCase().includes('responsive') ? 'responsive' : 'standard';
    return this.createSuccessResult(
      `Applied ${styleType} styling`,
      { styleType, input, ...context }
    );
  }

  public async run(input: string, context: Record<string, any> = {}): Promise<AgentResult> {
    console.log(`[FrontendAgent] Executing task: ${input.substring(0, 100)}...`);
    
    try {
      // Save the context for this agent
      contextMemory.save(this.id, 'lastTask', {
        input,
        timestamp: new Date().toISOString(),
        ...context
      });

      // Simple task classification
      const lowerInput = input.toLowerCase();
      let result: AgentResult;
      
      if (lowerInput.includes('component') || 
          lowerInput.includes('button') || 
          lowerInput.includes('modal') ||
          lowerInput.includes('card')) {
        result = await this.handleComponentTask(input, context);
      } else if (lowerInput.includes('layout') || 
                lowerInput.includes('page') || 
                lowerInput.includes('template')) {
        result = await this.handleLayoutTask(input, context);
      } else if (lowerInput.includes('form') || 
                lowerInput.includes('input') || 
                lowerInput.includes('field')) {
        result = await this.handleFormTask(input, context);
      } else if (lowerInput.includes('style') || 
                lowerInput.includes('css') || 
                lowerInput.includes('design')) {
        result = await this.handleStylingTask(input, context);
      } else {
        // Default response for general frontend tasks
        result = this.createSuccessResult(
          `I'll help you with the frontend task: "${input}"`,
          { 
            task: 'frontend', 
            input, 
            timestamp: new Date().toISOString(),
            ...context 
          },
          [
            // Example of a subtask that might be needed
            {
              agentId: 'backend',
              input: `Check if we need any backend APIs for: ${input}`
            }
          ]
        );
      }

      // Ensure the result has the required fields
      return {
        ...result,
        agentId: this.id,
        input,
        timestamp: new Date().toISOString()
      } as AgentResult;
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Unknown error occurred';
      return {
        success: false,
        error: errorMessage,
        output: `Error: ${errorMessage}`,
        agentId: this.id,
        input,
        timestamp: new Date().toISOString()
      } as AgentResult;
    }
  }
}

// Self-registering the agent
import { agentRegistry } from '../../core/fixedAgentRegistry.js';
const frontendAgent = new FrontendAgent();
agentRegistry.registerAgent(frontendAgent);
