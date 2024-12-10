<script lang="ts" setup>
import { WalletMultiButton } from "solana-wallets-vue";
import { Keypair } from '@solana/web3.js';

const assets = ref([
  {
    type: 'token_sol',
    name: 'SOL',
    properties: {
      amount: 1
    },
  },
  // nft
  {
    type: 'nft',
    name: 'NFT',
    properties: {
      mint: Keypair.generate().publicKey.toBase58(),
    },
  },
]);
</script>

<template>
  <div class="min-h-screen w-full flex flex-col">
    <header class="sticky top-0 left-0 w-screen h-[58px] flex">
      <div class="flex-1 flex items-center border-b border-gray-500/30">
        <Container class="flex-1 flex justify-between items-center">
          <UButton
            to="/"
            class="text-2xl font-bold"
            variant="ghost"
            icon="ph:triangle-dashed-duotone"
          >
            SolTrade
          </UButton>
          <div class="flex gap-4">
            <ClientOnly>
              <WalletMultiButton dark />
            </ClientOnly>
          </div>
        </Container>
      </div>
    </header>

    <main class="flex-1 flex flex-col">
      <Container class="flex-1 flex flex-col justify-center">
        <div class="w-full text-center">
          <h1 class="text-4xl font-bold mt-8">Welcome to SolTrade</h1>
          <p class="text-lg text-gray-500 mt-2">
            Trade your assets peer-to-peer on the Solana blockchain.
          </p>
          <div class="mt-12 flex gap-6 justify-center items-center">
            <div class="flex flex-col gap-4 items-center justify-center">
              <button
                class="h-64 w-64 rounded border border-dashed border-gray-500/30 text-gray-500"
              >
                Your asset
              </button>
              <div class="w-full h-4 flex items-start justify-end">
              </div>
            </div>
            <div class="flex flex-col gap-4 items-center justify-center mb-4">
              <div class="h-16 w-0.5 bg-orange-500" />
              <UIcon name="ph:arrows-left-right-duotone" class="text-orange-500" />
              <div class="h-16 w-0.5 bg-orange-500" />
            </div>
            <div class="flex flex-col gap-4 items-center justify-center">
              <button
                class="h-64 w-64 rounded border border-dashed border-gray-500/30 text-gray-500"
              >
                Asset from others
              </button>
              <div class="w-full h-4 flex items-start">
              </div>
            </div>
          </div>
          <div class="flex justify-center items-center gap-4">
            <UButton
              label="Create Trade Link"
              variant="solid"
            />
            <UButton
              label="Join Trade Link"
              variant="soft"
            />
          </div>
        </div>
      </Container>
    </main>

    <footer class="flex items-center justify-center h-10 border-t border-gray-500/30">
      <Container>
        <p class="text-sm text-gray-500">
          © 2021 SolTrade. All rights reserved.
        </p>
      </Container>
    </footer>
  </div>
</template>