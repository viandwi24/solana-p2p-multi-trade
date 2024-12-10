import * as borsh from "borsh";

import * as anchor from "@coral-xyz/anchor";
import { PublicKey, Struct } from "@solana/web3.js";
import { Soltrade } from "../target/types/soltrade";
import type { MethodsBuilder } from "@coral-xyz/anchor/dist/cjs/program/namespace/methods";

// === OPTIONS =================
export class OptionAssetItemSOL {
  public amount: anchor.BN;

  constructor(args: { amount: anchor.BN }) {
    this.amount = args.amount;
  }
}
export class OptionAssetItemSPL {
  public mint: string;
  public token: string;

  constructor(args: { mint: string; token: string }) {
    this.mint = args.mint;
    this.token = args.token;
  }
}
export const OptionAssetItemSOLSchema = new Map([
  [OptionAssetItemSOL, { kind: "struct", fields: [["amount", "u64"]] }],
]);
export const OptionAssetItemSPLSchema = new Map([
  [
    OptionAssetItemSPL,
    {
      kind: "struct",
      fields: [
        ["mint", "string"],
        ["token", "string"],
      ],
    },
  ],
]);

// === SDK =================
// export class AssetItemBase {
//   index: number;
//   from: PublicKey;
//   user: PublicKey;
//   asset_type: number;

//   constructor(fields: {
//     index: number;
//     from: PublicKey;
//     user: PublicKey;
//     asset_type: number;
//   }) {
//     this.index = fields.index;
//     this.from = fields.from;
//     this.user = fields.user;
//     this.asset_type = fields.asset_type;
//   }

//   static schema = new Map([
//     [
//       AssetItemBase,
//       {
//         kind: "struct",
//         fields: [
//           ["index", "u32"],
//           ["from", [32]],
//           ["user", [32]],
//           ["asset_type", "u64"],
//         ],
//       },
//     ],
//   ]);
// }
