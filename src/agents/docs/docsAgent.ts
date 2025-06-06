import { BaseAgent } from '../../core/Agent';
import { contextMemory } from '../../core/contextMemory';

/**
 * DocsAgent - Handles documentation generation, updates, and management.
 */
export class DocsAgent extends BaseAgent {
  private supportedDocTypes = [
    'api', 'tutorial', 'guide', 'reference', 'changelog', 'readme', 'comments'
  ];
  
  constructor() {
    super(
      'docs',
      'Documentation Agent',
      'Manages project documentation including API docs, guides, and code comments',
      [
        'documentation', 'docs', 'readme', 'api docs', 'tutorial', 'guide',
        'comment', 'changelog', 'reference', 'help', 'explain'
      ]
    );
  }

  /**
   * Execute a documentation-related task
   */
  public async execute(input: string, context: Record<string, any> = {}): Promise<AgentResult> {
    console.log(`[DocsAgent] Executing task: ${input.substring(0, 100)}...`);
    
    try {
      // Save the context for this agent
      contextMemory.save(this.id, 'lastTask', {
        input,
        timestamp: new Date().toISOString(),
        ...context
      });

      // Task classification
      if (this.isApiDocTask(input)) {
        return this.handleApiDocTask(input, context);
      } else if (this.isGuideTask(input)) {
        return this.handleGuideTask(input, context);
      } else if (this.isCodeCommentTask(input)) {
        return this.handleCodeCommentTask(input, context);
      }

      // Default response for general documentation tasks
      return this.createSuccessResult(
        `I'll help you with the documentation task: "${input}"`,
        { task: 'documentation', input, context },
        [
          // Example of a potential subtask
          {
            agentId: 'backend',
            input: `Gather technical details for documentation: ${input}`,
            priority: 2,
            context: { parentTask: input }
          }
        ]
      );
    } catch (error) {
      console.error(`[DocsAgent] Error:`, error);
      return this.createErrorResult(
        error as Error,
        `Failed to process documentation task: ${input}`
      );
    }
  }

  /**
   * Handle API documentation tasks
   */
  private async handleApiDocTask(
    input: string,
    context: Record<string, any>
  ): Promise<AgentResult> {
    const apiName = this.detectApiName(input) || 'API';
    const format = this.detectDocFormat(input) || 'markdown';
    
    // Track documentation in context
    const docs = contextMemory.get<Array<{type: string, name: string, format: string, timestamp: string}>>(this.id, 'documentation') || [];
    docs.push({
      type: 'api',
      name: apiName,
      format,
      timestamp: new Date().toISOString()
    });
    contextMemory.save(this.id, 'documentation', docs);
    
    return this.createSuccessResult(
      `I'll generate ${format} documentation for the ${apiName} API.`,
      {
        docType: 'api',
        apiName,
        format,
        timestamp: new Date().toISOString(),
        ...context
      },
      [
        // Gather API details
        {
          agentId: 'backend',
          input: `Provide API specification for ${apiName}`,
          priority: 1,
          context: { forDocumentation: true, format }
        }
      ]
    );
  }

  /**
   * Handle guide/tutorial tasks
   */
  private async handleGuideTask(
    input: string,
    context: Record<string, any>
  ): Promise<AgentResult> {
    const guideType = this.detectGuideType(input) || 'tutorial';
    const topic = this.detectTopic(input) || 'feature';
    const level = this.detectSkillLevel(input) || 'beginner';
    
    // Track guide creation in context
    const guides = contextMemory.get<Array<{type: string, topic: string, level: string, timestamp: string}>>(this.id, 'guides') || [];
    guides.push({
      type: guideType,
      topic,
      level,
      timestamp: new Date().toISOString()
    });
    contextMemory.save(this.id, 'guides', guides);
    
    return this.createSuccessResult(
      `I'll create a ${level}-level ${guideType} about ${topic}.`,
      {
        docType: guideType,
        topic,
        level,
        timestamp: new Date().toISOString(),
        ...context
      },
      [
        // Gather technical details if needed
        ...(this.needsTechnicalDetails(input) ? [{
          agentId: 'backend',
          input: `Provide technical details about ${topic} for ${guideType}`,
          priority: 1,
          context: { forGuide: true, level }
        }] : [])
      ]
    );
  }

  /**
   * Handle code comment tasks
   */
  private async handleCodeCommentTask(
    input: string,
    context: Record<string, any>
  ): Promise<AgentResult> {
    const language = this.detectLanguage(input) || 'typescript';
    const commentType = this.detectCommentType(input) || 'inline';
    
    // Track comment generation in context
    const comments = contextMemory.get<Array<{language: string, type: string, timestamp: string}>>(this.id, 'comments') || [];
    comments.push({
      language,
      type: commentType,
      timestamp: new Date().toISOString()
    });
    contextMemory.save(this.id, 'comments', comments);
    
    return this.createSuccessResult(
      `I'll generate ${commentType} comments in ${language}.`,
      {
        task: 'codeComments',
        language,
        commentType,
        timestamp: new Date().toISOString(),
        ...context
      },
      [
        // Get code context if available
        ...(context.codeSnippet ? [{
          agentId: 'backend',
          input: 'Analyze this code for documentation:',
          priority: 1,
          context: { codeSnippet: context.codeSnippet, commentType }
        }] : [])
      ]
    );
  }

  // --- Helper Methods ---
  
  private isApiDocTask(input: string): boolean {
    const apiKeywords = ['api doc', 'api reference', 'endpoint', 'swagger', 'openapi'];
    return apiKeywords.some(keyword => input.toLowerCase().includes(keyword));
  }
  
  private isGuideTask(input: string): boolean {
    const guideKeywords = ['guide', 'tutorial', 'how to', 'walkthrough', 'example'];
    return guideKeywords.some(keyword => input.toLowerCase().includes(keyword));
  }
  
  private isCodeCommentTask(input: string): boolean {
    const commentKeywords = ['comment', 'jsdoc', 'tsdoc', 'docblock', 'document this'];
    return commentKeywords.some(keyword => input.toLowerCase().includes(keyword));
  }
  
  private detectApiName(input: string): string | null {
    const apiMatch = input.match(/(?:for|about|document)\s+(?:the\s+)?([A-Za-z0-9\s]+?)(?:\s+API)?(?:\s+documentation)?/i);
    return apiMatch ? apiMatch[1].trim() : null;
  }
  
  private detectDocFormat(input: string): string | null {
    const formatMatch = input.match(/(markdown|md|html|pdf|asciidoc|rst)/i);
    if (!formatMatch) return null;
    
    const format = formatMatch[1].toLowerCase();
    return format === 'md' ? 'markdown' : format;
  }
  
  private detectGuideType(input: string): string | null {
    const lowerInput = input.toLowerCase();
    if (lowerInput.includes('tutorial')) return 'tutorial';
    if (lowerInput.includes('guide')) return 'guide';
    if (lowerInput.includes('example') || lowerInput.includes('sample')) return 'example';
    if (lowerInput.includes('reference')) return 'reference';
    return null;
  }
  
  private detectTopic(input: string): string | null {
    const topicMatch = input.match(/(?:about|on|for|document)\s+(.+?)(?:\s+(?:guide|tutorial|example))?\s*$/i);
    return topicMatch ? topicMatch[1].trim() : null;
  }
  
  private detectSkillLevel(input: string): string | null {
    const lowerInput = input.toLowerCase();
    if (lowerInput.includes('beginner') || lowerInput.includes('basic')) return 'beginner';
    if (lowerInput.includes('intermediate') || lowerInput.includes('advanced')) return 'intermediate';
    if (lowerInput.includes('expert') || lowerInput.includes('advanced')) return 'advanced';
    return null;
  }
  
  private detectLanguage(input: string): string | null {
    const langMatch = input.match(/(typescript|javascript|python|java|c#|c\+\+|go|rust|ruby|php)/i);
    return langMatch ? langMatch[1].toLowerCase() : null;
  }
  
  private detectCommentType(input: string): string | null {
    const lowerInput = input.toLowerCase();
    if (lowerInput.includes('inline') || lowerInput.includes('single-line')) return 'inline';
    if (lowerInput.includes('block') || lowerInput.includes('multi-line')) return 'block';
    if (lowerInput.includes('jsdoc') || lowerInput.includes('tsdoc')) return 'jsdoc';
    return null;
  }
  
  private needsTechnicalDetails(input: string): boolean {
    const techKeywords = ['api', 'endpoint', 'function', 'method', 'class', 'component'];
    return techKeywords.some(keyword => input.toLowerCase().includes(keyword));
  }
}

// Self-registering the agent
import { agentRegistry } from '../../core/agentRegistry';
const docsAgent = new DocsAgent();
agentRegistry.registerAgent(docsAgent);

export default docsAgent;
