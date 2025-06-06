# CodeOrbit Multi-Agent System

A modular, extensible multi-agent system for the CodeOrbit IDE extension, enabling intelligent task routing and collaboration between specialiCodeOrbit AI agents.

## Overview

This system implements a multi-agent architecture where different agents handle specific domains of functionality. The agents can work independently or collaborate on complex tasks by breaking them down into subtasks.

## Core Components

### 1. Agent System
- **BaseAgent**: Abstract class that all agents inherit from
- **Agent Registry**: Singleton that manages agent registration and lookup
- **Context Memory**: Persistent storage for agent state and context
- **Orchestrator**: Routes tasks to appropriate agents and manages execution flow

### 2. Domain Agents
- **Frontend Agent**: Handles UI components, layouts, and frontend logic
- **Backend Agent**: Manages API development and server-side logic
- **Database Agent**: Handles database design, queries, and migrations
- **DevOps Agent**: Manages deployment and infrastructure tasks
- **Documentation Agent**: Handles documentation generation and updates

## Getting Started

### Prerequisites
- Node.js (v14+)
- TypeScript (v4+)

### Installation

1. Clone the repository
2. Install dependencies:
   ```bash
   npm install
   ```
3. Compile TypeScript:
   ```bash
   npx tsc
   ```

### Running the Example

```bash
node dist/main.js
```

This will run the example scenarios demonstrating the multi-agent system in action.

## Usage

### Creating a New Agent

1. Create a new TypeScript file in the appropriate domain directory under `src/agents/`
2. Create a class that extends `BaseAgent`
3. Implement the required `execute` method
4. Register the agent with the agent registry:

```typescript
import { agentRegistry } from '../../core/agentRegistry';
const myAgent = new MyAgent();
agentRegistry.registerAgent(myAgent);
```

### Processing Tasks

Use the orchestrator to process tasks:

```typescript
import { orchestrator } from './core/orchestrator';

async function handleTask(task: string) {
  const result = await orchestrator.receivePrompt(task);
  console.log('Task result:', result);
}
```

## Agent Communication

Agents can communicate and collaborate through:

1. **Context Memory**: Share state and information
2. **Subtasks**: Delegate work to other agents
3. **Events**: Subscribe to and emit events (future enhancement)

## Example Scenarios

The `main.ts` file includes several example scenarios that demonstrate the system's capabilities:

1. Frontend component creation
2. Backend API development
3. Database schema design
4. Application deployment
5. Documentation generation
6. Complex multi-agent tasks

## Extending the System

### Adding New Capabilities

1. **New Agent Types**: Create new agent classes for specific domains
2. **Enhanced Context**: Add new context types to the context memory
3. **Custom Orchestration**: Extend the orchestrator for complex workflows

### Integration Points

- **IDE Integration**: Connect to the CodeOrbit IDE extension
- **External Services**: Integrate with external APIs and services
- **AI/ML Models**: Add specialiCodeOrbit AI capabilities

## Best Practices

1. **Single Responsibility**: Each agent should handle a specific domain
2. **Stateless Design**: Store state in context memory, not in agent instances
3. **Error Handling**: Implement robust error handling and logging
4. **Testing**: Write unit and integration tests for agents
5. **Documentation**: Document agent capabilities and usage

## License

This project is part of the CodeOrbit IDE extension and is licensed under the [MIT License](LICENSE).
