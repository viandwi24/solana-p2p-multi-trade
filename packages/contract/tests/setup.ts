import path from 'path';
import fs from 'fs';
import * as borsh from 'borsh'

import { PublicKey } from "@solana/web3.js";
import { createConsola, ConsolaReporter } from 'consola'
import * as anchor from "@coral-xyz/anchor";
import { Soltrade } from "../target/types/soltrade";

import { SolTradeSDK } from './sdk';



// SETUP
export const setup = async () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Soltrade as anchor.Program<Soltrade>;

  // log
  const log = createConsola({
    fancy: true,
    level: 5,
    reporters: [
      {
        log: (logObj) => {
          console.log(`  |`, ...logObj.args);
        },
      },
    ],
  })
  log.info("==========================================================");
  log.info("Program ID:", program.programId.toBase58());
  log.info("Cluster:", anchor.getProvider().connection.rpcEndpoint);
  log.info("==========================================================");

  return {
    program,
    log,
    solTradeSdk: new SolTradeSDK(program),
  }
}