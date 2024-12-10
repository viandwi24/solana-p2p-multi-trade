// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2024-11-01',
  devtools: { enabled: true },

  modules: ['@nuxt/ui'],
  css: [
    '@/assets/css/main.css',
  ],

  vite: {
    esbuild: {
      target: "esnext",
    },
    build: {
      target: "esnext",
    },
    optimizeDeps: {
      include: [
        "@coral-xyz/anchor",
        "@solana/web3.js",
        "@metaplex-foundation/umi",
        "buffer",
        // "@metaplex-foundation/mpl-token-metadata",
        // "@metaplex-foundation/umi-bundle-defaults",
      ],
      esbuildOptions: {
        target: "esnext",
      },
    },
    define: {
      "process.env.BROWSER": true,
      // "globalThis": "window",
      // "global": "window",
    },
  },
})