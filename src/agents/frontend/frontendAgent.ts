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
  private createSuccessResult(
    message: string,
    data: Record<string, any> = {},
    subtasks: Array<{agentId: string; input: string; context?: Record<string, any>}> = []
  ): AgentResult {
    return {
      success: true,
      output: message,
      data,
      subtasks,
      timestamp: new Date().toISOString(),
      duration: 0 // Will be set by the registry
    };
  }

  private createErrorResult(error: string): AgentResult {
    return {
      success: false,
      error,
      output: `Error: ${error}`,
      data: {},
      subtasks: [],
      timestamp: new Date().toISOString(),
      duration: 0
    };
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
      };
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Unknown error occurred';
      return {
        success: false,
        error: errorMessage,
        output: `Error: ${errorMessage}`,
        agentId: this.id,
        input,
        timestamp: new Date().toISOString()
      };
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
  /**
   * Generate component code based on type and framework
   * @private
   */
  private generateComponentCode(
    componentType: string,
    framework: string,
    isTypeScript: boolean,
    context: Record<string, any>
  ): string {
    const ext = isTypeScript ? 'tsx' : 'jsx';
    const typeAnnotation = isTypeScript ? ': React.FC' : '';
    const propsType = isTypeScript ? `interface ${componentType}Props {
  // Add your prop types here
  title?: string;
  children?: React.ReactNode;
}` : '';

    switch (framework.toLowerCase()) {
      case 'react':
        return `import React${isTypeScript ? ', { FC }' : ''} from 'react';
${isTypeScript ? propsType : ''}
const ${componentType}${isTypeScript ? `: React.FC<${componentType}Props>` : ''} = (${isTypeScript ? '{
  title = "${componentType}",
  children
}' : 'props'}) => {
  return (
    <div className="${componentType.toLowerCase()}">
      <h2>{title}</h2>
      {children}
    </div>
  );
};

export default ${componentType};`;

      case 'vue':
        const scriptTag = isTypeScript ? '<script setup lang="ts">' : '<script setup>';
        return `${scriptTag}
// Component logic here
${isTypeScript ? 'interface Props {\n  title?: string\n}\n\nconst props = withDefaults(defineProps<Props>(), {\n  title: "${componentType}"\n});' : 'const props = defineProps({\n  title: {\n    type: String,\n    default: "${componentType}"\n  }\n});'}
</script>

<template>
  <div class="${componentType.toLowerCase()}">
    <h2>{{ title }}</h2>
    <slot></slot>
  </div>
</template>

<style scoped>
.${componentType.toLowerCase()} {
  /* Your styles here */
}
</style>`;

      default:
        return `// ${componentType} component for ${framework}
// Implementation depends on the framework`;
    }
  }

  /**
   * Detect component type from input
   * @private
   */
  private detectComponentType(input: string): string {
    const lowerInput = input.toLowerCase();
    
    // Map of component types and their keywords
    const componentTypes = [
      { type: 'Button', keywords: ['button', 'btn'] },
      { type: 'Card', keywords: ['card', 'panel'] },
      { type: 'Modal', keywords: ['modal', 'dialog', 'popup'] },
      { type: 'DataTable', keywords: ['table', 'datatable', 'grid'] },
      { type: 'Navigation', keywords: ['nav', 'menu', 'sidebar'] },
      { type: 'Form', keywords: ['form', 'input', 'field'] },
      { type: 'List', keywords: ['list', 'itemlist'] },
      { type: 'Carousel', keywords: ['carousel', 'slider'] },
      { type: 'Accordion', keywords: ['accordion', 'collapse'] },
      { type: 'Tabs', keywords: ['tabs', 'tabview'] },
    ];
    
    // Find the most specific match
    for (const { type, keywords } of componentTypes) {
      if (keywords.some(kw => lowerInput.includes(kw))) {
        return type;
      }
    }
    
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
