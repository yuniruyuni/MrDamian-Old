import React from 'react';
import { createRoot } from 'react-dom/client';
import App from './App';
import './styles.css';
import 'semantic-ui-css/semantic.min.css';

document.addEventListener('contextmenu', (event) => event.preventDefault());

const root = document.getElementById('root') as HTMLElement;

createRoot(root).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
