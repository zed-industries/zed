import { agentRegistry } from './core/agentRegistry';
import { orchestrator } from './core/orchestrator';
import { contextMemory } from './core/contextMemory';

// Import agents to register them with the agent registry
import './agents/frontend/frontendAgent';
import './agents/backend/backendAgent';
import './agents/database/databaseAgent';
import './agents/devops/devopsAgent';
import './agents/docs/docsAgent';

/**
 * Main function to demonstrate the multi-agent system
 */
async function main() {
  console.log('=== CodeOrbit Multi-Agent System ===\n');

  // Example 1: Simple task routing
  console.log('Example 1: Simple task routing\n');
  await processTask('Create a new React component for user profile');
  
  // Example 2: Backend API creation
  console.log('\nExample 2: Backend API creation\n');
  await processTask('Create a REST API endpoint for user authentication');
  
  // Example 3: Database schema design
  console.log('\nExample 3: Database schema design\n');
  await processTask('Design a database schema for an e-commerce application');
  
  // Example 4: DevOps deployment
  console.log('\nExample 4: DevOps deployment\n');
  await processTask('Deploy the application to production on AWS');
  
  // Example 5: Documentation generation
  console.log('\nExample 5: Documentation generation\n');
  await processTask('Generate API documentation for the user service');
  
  // Example 6: Complex task with subtasks
  console.log('\nExample 6: Complex task with subtasks\n');
  await processTask('Set up a new feature with frontend form, backend API, and database table');
  
  // Display context memory state
  console.log('\n=== Context Memory State ===');
  const agents = contextMemory.getAgentIds();
  for (const agentId of agents) {
    console.log(`\n${agentId} context:`, contextMemory.getAll(agentId));
  }
  
  // Display registered agents
  console.log('\n=== Registered Agents ===');
  agentRegistry.listAgents().forEach(agent => {
    console.log(`- ${agent.name} (${agent.id}): ${agent.description}`);
  });
}

/**
 * Process a task through the orchestrator and display the result
 */
async function processTask(task: string) {
  console.log(`Processing task: "${task}"`);
  console.log('---');
  
  try {
    const result = await orchestrator.receivePrompt(task);
    
    console.log('\nResult:');
    console.log('- Status:', result.success ? '✅ Success' : '❌ Error');
    console.log('- Output:', result.output);
    
    if (result.data) {
      console.log('- Data:', JSON.stringify(result.data, null, 2));
    }
    
    if (result.subtasks && result.subtasks.length > 0) {
      console.log('\nGenerated Subtasks:');
      result.subtasks.forEach((subtask, index) => {
        console.log(`  ${index + 1}. [${subtask.agentId}] ${subtask.input}`);
      });
    }
  } catch (error) {
    console.error('Error processing task:', error);
  }
  
  console.log('\n' + '='.repeat(50) + '\n');
}

// Run the main function
main().catch(console.error);
