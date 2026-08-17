import { Admin } from 'react-admin';
import { Layout } from './Layout';
import { authProvider } from './authProvider';


export const App = () => (
    <Admin
        layout={Layout}
        	authProvider={authProvider}
	>
        
    </Admin>
);

    