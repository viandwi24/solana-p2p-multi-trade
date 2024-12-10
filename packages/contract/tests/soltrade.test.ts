import * as borsh from 'borsh'

import { describe, it } from "mocha"
// import { assert } from "chai"

import * as anchor from "@coral-xyz/anchor";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";

import { setup } from "./setup";
import { Soltrade } from '../target/types/soltrade';


describe("soltrade", async () => {
  const { solTradeSdk, program, log } = await setup();
  let $global = {
    tradeAddress: null as PublicKey,
    offerAddress: null as PublicKey,
  }

  it("is initialized", async () => {
    const authorityAddress = solTradeSdk.PDA.getAuthority().address;
    try {
      await solTradeSdk.getAuthority();
      log.debug("authority found, already initialized");
    } catch (e) {
      const tx = await program.methods
        .initialize()
        .accounts({
          authority: authorityAddress,
        })
        .rpc();

      log.debug("sig", tx);
    }

  });

  it("create trade", async () => {
    const trade = await solTradeSdk.createTrade();
    log.debug("trade", trade.pda.trade.address.toBase58());

    // execute
    {
      const tx = await trade.builder.rpc();
      log.debug("sig", tx);
      $global.tradeAddress = trade.pda.trade.address;
    }
  });

  it("add assets to trade", async () => {
    // create asset sol and transfer
    const assets = await solTradeSdk.addAssetsItem(
      $global.tradeAddress,
      [
        {
          type: 'sol',
          amount: LAMPORTS_PER_SOL * 1
        },
        // {
        //   type: 'sol',
        //   amount: 2
        // },
        // {
        //   type: 'spl',
        //   mint: Keypair.generate().publicKey,
        //   token: Keypair.generate().publicKey,
        // },
      ]
    );
    {
      for (const asset of assets.builders) {
        const index = assets.builders.indexOf(asset);
        const item = assets.assetItems[index];

        log.debug("add asset item to trade", item.toBase58());
        const sig = await asset.rpc({
          // skipPreflight: true,
        })
        log.debug("sig", sig);
        await new Promise((resolve) => setTimeout(resolve, 1000));
      }

      const assetItems = await solTradeSdk.getTradeOrOfferAssets($global.tradeAddress);
      log.debug("assetItems", assetItems.map(e => solTradeSdk.parseFromAnchorData(e)));
    }
  })

  it("create offer", async () => {
    const trade = await solTradeSdk.getTrade($global.tradeAddress);

    // create offer
    const offer = solTradeSdk.PDA.getTradeOffer($global.tradeAddress, trade.offerCount);
    $global.offerAddress = offer.address;
    {
      const sig = await solTradeSdk.program
        .methods.createOffer(trade.offerCount)
        .accounts({
          trade: $global.tradeAddress,
          offer: offer.address,
        })
        .rpc({
          // skipPreflight: true,
        })

      log.debug("sig", sig);
    }
  });

  it("add assets to offer", async () => {
    // create asset sol and transfer
    const assets = await solTradeSdk.addAssetsItem(
      $global.tradeAddress,
      [
        {
          type: 'sol',
          amount: LAMPORTS_PER_SOL * .5
        },
      ],
      $global.offerAddress,
    );
    {
      for (const asset of assets.builders) {
        const index = assets.builders.indexOf(asset);
        const item = assets.assetItems[index];

        log.debug("add asset item to trade", item.toBase58());
        const sig = await asset.rpc({
          skipPreflight: true,
        })
        log.debug("sig", sig);
        await new Promise((resolve) => setTimeout(resolve, 1000));
      }

      const assetItems = await solTradeSdk.getTradeOrOfferAssets($global.offerAddress);
      log.debug("assetItems", assetItems.map(e => solTradeSdk.parseFromAnchorData(e)));
    }
  });

  it("accept offer", async () => {
    // accept offer
    {
      const sig = await solTradeSdk.program.methods
        .acceptOffer()
        .accounts({
          trade: $global.tradeAddress,
          offer: $global.offerAddress,
        })
        .rpc({
          // skipPreflight: true,
        })

      log.debug("sig", sig);
    }
  });

  it("exchange from offer to trade", async () => {
    const trade = await solTradeSdk.getTrade($global.tradeAddress);

    const offer = await solTradeSdk.getOffer(trade.acceptedOffer);
    const offerAssets = await solTradeSdk.getTradeOrOfferAssets(trade.acceptedOffer);
    log.debug("offerAssets", offerAssets.map(e => solTradeSdk.parseFromAnchorData(e)));

    // exchange
    const from: 'offer' | 'trade' = 'offer';
    const from_type = from == 'offer' ? 2 : 1;
    for (const asset of offerAssets) {
      const sig = await solTradeSdk.program.methods
        .exchange(
          trade.index,
          offer.index,
          from_type,
          asset.account.assetType,
          asset.account.index,
          asset.account.typeIndex,
        )
        .accounts({
          trade: $global.tradeAddress,
          offer: trade.acceptedOffer,

          assetItemSol: asset.account.assetType == 1 ? asset.publicKey : null,
          assetItemSpl: asset.account.assetType == 2 ? asset.publicKey : null,

          userFrom: offer.user,
        })
        .rpc({
          // skipPreflight: true,
        })
      log.debug("sig", sig);
    }
  });

  it("exchange from trade to offer", async () => {
    const trade = await solTradeSdk.getTrade($global.tradeAddress);
    const offer = await solTradeSdk.getOffer(trade.acceptedOffer);
    const tradeAssets = await solTradeSdk.getTradeOrOfferAssets($global.tradeAddress);
    // const offerAssets = await solTradeSdk.getTradeOrOfferAssets(trade.acceptedOffer);

    // trade
    log.debug("offerAssets", tradeAssets.map(e => solTradeSdk.parseFromAnchorData(e)));

    // exchange
    const from: 'offer' | 'trade' = 'trade';
    const from_type = (from == 'trade') ? 1 : 2;
    for (const asset of tradeAssets) {
      const sig = await solTradeSdk.program.methods
        .exchange(
          trade.index,
          offer.index,
          from_type,
          asset.account.assetType,
          asset.account.index,
          asset.account.typeIndex,
        )
        .accounts({
          trade: $global.tradeAddress,
          offer: trade.acceptedOffer,

          assetItemSol: asset.account.assetType == 1 ? asset.publicKey : null,
          assetItemSpl: asset.account.assetType == 2 ? asset.publicKey : null,

          userFrom: offer.user,
        })
        .rpc({
          // skipPreflight: true,
        })
      log.debug("sig", sig);
    }
  });

  return;
});
