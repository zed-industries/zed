import type { ReactNode } from 'react';
import {
  Layout as RALayout,
  CheckForApplicationUpdate,
} from "react-admin";

export const Layout = ({ children }: { children: ReactNode }) => (
  <RALayout>
    {children}
    <CheckForApplicationUpdate />
  </RALayout>
);
