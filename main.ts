import { agentRegistry } from './core/agentRegistry';
import { Orchestrator } from './core/orchestrator';
import { FrontendAgent } from './agents/frontend/frontendAgent';
import { BackendAgent } from './agents/backend/backendAgent';
import { DatabaseAgent } from './agents/database/databaseAgent';
import { DevOpsAgent } from './agents/devops/devopsAgent';
import { DocsAgent } from './agents/docs/docsAgent';

// Register all agents
agentRegistry.registerAgent('frontend', new FrontendAgent());
agentRegistry.registerAgent('backend', new BackendAgent());
agentRegistry.registerAgent('database', new DatabaseAgent());
agentRegistry.registerAgent('devops', new DevOpsAgent());
agentRegistry.registerAgent('docs', new DocsAgent());

async function main() {
  const orchestrator = new Orchestrator();
  const prompt = 'Build a login page using React and store users in a database';
  const result = await orchestrator.receivePrompt(prompt);
  console.log('\n--- Result ---');
  console.log(result);
}

main().catch(err => console.error(err));
