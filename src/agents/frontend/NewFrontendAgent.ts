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
  private readonly capabilities: string[] = [
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
    return [...this.capabilities];
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
    return 'react';
  }

  private async handleComponentTask(input: string, context: Record<string, any>): Promise<AgentResult> {
    const componentType = this.detectComponentType(input);
    const framework = this.detectFramework(input);
    const componentName = this.generateComponentName(input);
    
    return this.createSuccessResult(
      `Created ${componentType} component named ${componentName} using ${framework}`,
      { 
        componentType, 
        framework, 
        componentName,
        ...context 
      }
    );
  }

  private generateComponentName(input: string): string {
    // Simple name generation - in a real implementation, this would be more sophisticated
    const words = input
      .split(/\s+/)
      .filter(word => word.length > 3)
      .map(word => word.replace(/[^\w]/g, ''))
      .filter(word => word.length > 0);
    
    if (words.length === 0) return 'MyComponent';
    
    return words
      .map(word => word.charAt(0).toUpperCase() + word.slice(1).toLowerCase())
      .join('');
  }

  private async handleLayoutTask(input: string, context: Record<string, any>): Promise<AgentResult> {
    const layoutType = input.toLowerCase().includes('sidebar') ? 'sidebar' : 'standard';
    return this.createSuccessResult(
      `Created ${layoutType} layout`,
      { 
        layoutType, 
        ...context 
      }
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
      { 
        formFields, 
        ...context 
      }
    );
  }

  private async handleStylingTask(input: string, context: Record<string, any>): Promise<AgentResult> {
    const styleType = input.toLowerCase().includes('responsive') ? 'responsive' : 'standard';
    return this.createSuccessResult(
      `Applied ${styleType} styling`,
      { 
        styleType, 
        ...context 
      }
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
      
      if (lowerInput.includes('component') || 
          lowerInput.includes('button') || 
          lowerInput.includes('modal') ||
          lowerInput.includes('card')) {
        return this.handleComponentTask(input, context);
      } 
      
      if (lowerInput.includes('layout') || 
          lowerInput.includes('page') || 
          lowerInput.includes('template')) {
        return this.handleLayoutTask(input, context);
      } 
      
      if (lowerInput.includes('form') || 
          lowerInput.includes('input') || 
          lowerInput.includes('field')) {
        return this.handleFormTask(input, context);
      } 
      
      if (lowerInput.includes('style') || 
          lowerInput.includes('css') || 
          lowerInput.includes('design')) {
        return this.handleStylingTask(input, context);
      }

      // Default response for general frontend tasks
      return this.createSuccessResult(
        `I'll help you with the frontend task: "${input}"`,
        { 
          task: 'frontend', 
          input, 
          timestamp: new Date().toISOString(),
          ...context 
        }
      );
      
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Unknown error occurred';
      console.error(`[FrontendAgent] Error: ${errorMessage}`, error);
      return this.createErrorResult(`Failed to process frontend task: ${errorMessage}`);
    }
  }
}

// Self-registering the agent
import { agentRegistry } from '../../core/agentRegistry.js';
const frontendAgent = new FrontendAgent();
agentRegistry.registerAgent(frontendAgent);
