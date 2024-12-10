import * as borsh from 'borsh'

import * as anchor from "@coral-xyz/anchor";
import { PublicKey, Struct } from "@solana/web3.js";
import { Soltrade } from "../target/types/soltrade";
import type { MethodsBuilder } from '@coral-xyz/anchor/dist/cjs/program/namespace/methods'
import { OptionAssetItemSOL, OptionAssetItemSOLSchema, OptionAssetItemSPL, OptionAssetItemSPLSchema } from './sdk_struct';


// SDK Interface
export type AssetType = 'sol' | 'spl'
export interface AddAssetsToTradeAssetBase {
  // type: 'sol' | 'spl' | 'token',
}
export interface AddAssetsToTradeAssetSOL {
  type: 'sol',
  amount: number,
}
export interface AddAssetsToTradeAssetSPL {
  type: 'spl',
  mint: PublicKey,
  token: PublicKey,
}
export type AddAssetsToTradeAssets = AddAssetsToTradeAssetSOL | AddAssetsToTradeAssetSPL


// SDK
export class SolTradeSDK {
  PDA: ReturnType<typeof this.generatePDA>

  constructor (
    public program: anchor.Program<Soltrade>,
  ) {
    this.PDA = this.generatePDA(program.programId)
  }


  // methods
  async createTrade(
    opts?: {
      allowedUsers?: PublicKey[],
    }
  ) {
    const authority = await this.getAuthority()
    const trade = this.PDA.getTrade(authority.tradeCount);
    const builder = this.program.methods
      .createTrade(
        authority.tradeCount,
        opts?.allowedUsers || [],
      )
      .accounts({
        trade: trade.address,
        authority: this.PDA.getAuthority().address,
      })
    return {
      pda: {
        trade
      },
      builder,
    }
  }
  getAuthority = () => this.program.account.authority.fetch(this.PDA.getAuthority().address)
  getTrade = (address: PublicKey) => this.program.account.trade.fetch(address)
  getOffer = (address: PublicKey) => this.program.account.offer.fetch(address)
  getAssetTypeCode(asset_type: AssetType) {
    if (asset_type == 'sol') {
      return 1
    } else if (asset_type == 'spl') {
      return 2
    }
    throw new Error('Invalid asset type')
  }
  async addAssetsItem(
    tradeAddress: PublicKey,
    assets: AddAssetsToTradeAssets[],
    offerAddress?: PublicKey,
  ) {
    const trade = await this.getTrade(tradeAddress)

    // if trade = 1, offer = 2
    let from_type = 1
    let offer: anchor.IdlAccounts<Soltrade>['offer']|undefined
    {
      if (offerAddress) {
        offer = await this.program.account.offer.fetch(offerAddress)
        from_type = 2
      }
    }

    // builders
    const builders: MethodsBuilder<Soltrade, any>[] = []
    const assetItems: PublicKey[] = []
    const assetsAddress: PublicKey[] = []

    // opts
    let startAssetIndex = !offer ? trade.assetCount : offer.assetCount
    let assetIndex = startAssetIndex
    let assetTypeIndex = {
      sol: !offer ? trade.assetSolCount : offer.assetSolCount,
      spl: !offer ? trade.assetSplCount : offer.assetSplCount,
    }
    for (const item of assets) {
      // opts
      let serialized: Uint8Array|undefined
      if (item.type == 'sol') {
        const data = new OptionAssetItemSOL({
          amount: new anchor.BN(item.amount),
        })
        serialized = borsh.serialize(OptionAssetItemSOLSchema, data)
        // console.log('serialized', serialized)
        // const deserialized = borsh.deserialize(OptionAssetItemSOLSchema, OptionAssetItemSOL, Buffer.from(serialized))
        // console.log('deserialized', typeof deserialized, deserialized)
      } else if (item.type == 'spl') {
        const data = new OptionAssetItemSPL({
          mint: item.mint.toBase58(),
          token: item.token.toBase58(),
        })
        serialized = borsh.serialize(OptionAssetItemSPLSchema, data)
        // console.log('serialized', serialized)
        // const deserialized = borsh.deserialize(OptionAssetItemSPLSchema, OptionAssetItemSPL, Buffer.from(serialized))
        // console.log('deserialized', typeof deserialized, deserialized)
      }
      // return

      // check
      if (!serialized) {
        throw new Error('Invalid asset type')
      }

      // check
      const assetItemAddres = this.PDA.getTradeAssetItem(
        from_type == 1 ? tradeAddress : offerAddress,
        item.type, assetTypeIndex[item.type]
      ).address

      // build
      const assetTypeCode = this.getAssetTypeCode(item.type)
      const builder = this.program.methods
        .addAssetItem(
          from_type,
          assetIndex,
          assetTypeIndex[item.type],
          assetTypeCode,
          Buffer.from(serialized),
        )
        .accounts({
          tradeOrOffer: from_type == 1 ? tradeAddress : offerAddress,

          // assetItemBase: null,
          assetItemSol: item.type == 'sol' ? assetItemAddres : null,
          assetItemSpl: item.type == 'spl' ? assetItemAddres : null,
        })

      // push
      builders.push(builder)
      assetItems.push(assetItemAddres)

      // asset index
      assetIndex += 1
      assetTypeIndex[item.type] += 1
    }

    //
    return {
      builders,
      assetItems,

      startIndex: startAssetIndex,
      endIndex: assetIndex - 1,
    }
  }
  async getTradeOrOfferAssets(tradeOrOfferAddress: PublicKey) {
    const filters = [
      { memcmp: { offset: 8 + 4 + 4, bytes: tradeOrOfferAddress.toBase58() } }
    ]
    const items = [
      ...(await this.program.account.assetItemSol.all(filters)),
      ...(await this.program.account.assetItemSpl.all(filters)),
    ]
    return items
  }
  async getTradeOffersAddressFromTrade(tradeAddress: PublicKey, offerCount: number) {
    return new Array(offerCount).fill(0).map((_, i) => {
      return this.PDA.getTradeOffer(tradeAddress, i).address
    })
  }


  // utils
  parseFromAnchorData(data: object) {
    // looping through the object, child objects
    // if instance of anchor.BN, convert to string
    // if instance of anchor.PublicKey, convert to string
    // if object, call the function recursively
    // if array, call the function recursively
    // else return the value

    let parsedData: any = {};
    for (let key in data) {
      if (data[key] instanceof anchor.BN) {
        parsedData[key] = data[key].toString();
      } else if (data[key] instanceof anchor.web3.PublicKey) {
        parsedData[key] = data[key].toString();
      } else if (typeof data[key] === "object") {
        parsedData[key] = this.parseFromAnchorData(data[key]);
      } else if (Array.isArray(data[key])) {
        parsedData[key] = data[key].map((item) => {
          return this.parseFromAnchorData(item);
        });
      } else {
        parsedData[key] = data[key];
      }
    }
    return parsedData;
  }
  private generatePDA = function (p: PublicKey) {
    const $this = ({
      build(programId: PublicKey, data: (string|number|PublicKey)[], schemas?: Map<number, 'u8'|'u32'>) {
        const seeds: Array<Buffer | Uint8Array> = []
        let i = 0

        const schemas_used: string[] = []
        for (const item of data) {
          // by schemas if provided
          if (schemas) {
            const schema = schemas.get(i)
            if (schema) {
              if (schema === 'u32') {
                const _index = Buffer.alloc(4)
                _index.writeUint32LE(item as number)
                seeds.push(_index)
                schemas_used.push(`u32`)
                continue
              } else if (schema === 'u8') {
                seeds.push(Buffer.from([item as number]))
                schemas_used.push(`u8`)
                continue
              }
            }
          }

          // by auto-detecting the type
          if (typeof item === 'string') {
            seeds.push(Buffer.from(item))
            schemas_used.push(`string`)
          } else if (item instanceof PublicKey) {
            seeds.push(item.toBuffer())
            schemas_used.push(`PublicKey`)
          }

          // increment
          i += 1
        }
        // console.log('schemas_used', schemas_used)

        const res = PublicKey.findProgramAddressSync(
          seeds,
          programId
        )
        return {
          address: res[0],
          nonce: res[1],
        }
      },
      getTrade(index: number) {
        return $this.build(p, ["trade", index], new Map([[1, 'u32']]))
      },
      getTradeAssetItem(trade: PublicKey, type: 'sol' | 'spl', asset_type_index: number, offer?: PublicKey) {
        return $this.build(p, [trade, "asset_item_" + type, asset_type_index], new Map([[2, 'u32']]))
      },
      getTradeOffer(trade: PublicKey, offer_index: number) {
        return $this.build(p, [trade, "offer", offer_index], new Map([[2, 'u32']]))
      },
      getAuthority() {
        return $this.build(p, ["authority", p])
      }
    })
    return $this
  }
}
