import { autoModeWatcher } from '@skeletonlabs/skeleton';
const script = document.createElement('script');
script.innerHTML = `${autoModeWatcher.toString()} autoModeWatcher();`;
document.head.append(script);

import './app.postcss';
import App from './App.svelte';

const app = new App({
  target: document.getElementById('app')!,
});

export default app;
