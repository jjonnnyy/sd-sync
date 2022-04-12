import React from 'react';
import * as ReactDOMClient from 'react-dom/client';
import {App} from './App';

/* Start React */
const container = document.getElementById('app');
if (container) {
  const root = ReactDOMClient.createRoot(container);
  root.render(
      <React.StrictMode>
        <App />
      </React.StrictMode>
  );
}
