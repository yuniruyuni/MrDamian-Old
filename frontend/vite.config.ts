import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import { ecsstatic } from '@acab/ecsstatic/vite';
import tsconfigPaths from 'vite-tsconfig-paths';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react(), ecsstatic(), tsconfigPaths()],
});
