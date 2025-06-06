import express, { type Request, type Response, type Express } from 'express';
import path from 'path';
import { fileURLToPath } from 'url';
import cors, { type CorsOptions } from 'cors';
import morgan from 'morgan';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const app: Express = express();
const PORT: string | number = process.env.PORT || 3000;

// Middleware
const corsOptions: CorsOptions = {
  origin: '*',
  methods: ['GET', 'POST', 'PUT', 'DELETE', 'OPTIONS'],
  allowedHeaders: ['Content-Type', 'Authorization']
};

app.use(cors(corsOptions));
app.use(morgan('dev'));
app.use(express.json());
app.use(express.urlencoded({ extended: true }));

// Add response headers
app.use((_req: Request, res: Response, next: () => void) => {
  res.header('Access-Control-Allow-Origin', '*');
  res.header(
    'Access-Control-Allow-Headers',
    'Origin, X-Requested-With, Content-Type, Accept, Authorization'
  );
  next();
});

// Serve static files from the public directory
app.use(express.static(path.join(__dirname, '..', 'public')));

// Serve the logo from assets
app.use('/assets', express.static(path.join(__dirname, '..', 'assets')));

// API routes can be added here
app.get('/api/status', (req: Request, res: Response) => {
  res.json({
    status: 'ok',
    version: '1.0.0',
    name: 'CodeOrbit',
    timestamp: new Date().toISOString()
  });
});

// Handle SPA routing - return the index.html for all other routes
app.get('*', (req: Request, res: Response) => {
  res.sendFile(path.join(__dirname, '..', 'public', 'index.html'));
});

// Create HTTP server
const server = app.listen(PORT, () => {
  const address = server.address();
  const port = typeof address === 'string' ? address : address?.port || PORT;
  console.log(`🚀 CodeOrbit server is running on http://localhost:${port}`);
  console.log(`✨ Open your browser and navigate to the URL above to see the app`);
});

// Handle graceful shutdown
const signals: NodeJS.Signals[] = ['SIGINT', 'SIGTERM', 'SIGQUIT'];
signals.forEach(sig => {
  process.on(sig, () => {
    console.log(`\n🛑 Received ${sig}, shutting down gracefully...`);
    server.close(() => {
      console.log('👋 Server closed');
      process.exit(0);
    });
  });
});
