// @ts-check
import { defineConfig } from 'astro/config';

// Yayin hedefi Netlify: kok dizinde servis edildigi icin `base` yok.
// GitHub Pages'e donulurse `astro.config.githubpages.mjs.bak` geri alinir.
export default defineConfig({
  site: 'https://manipulens-rehber.netlify.app',
  trailingSlash: 'ignore',
  build: { format: 'directory' },
});
