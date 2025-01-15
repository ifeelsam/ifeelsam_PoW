import { Connection, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";

import wallet from "./devnet-wallet.json"

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
console.log(keypair.publicKey)
const connection = new Connection("https://api.devnet.solana.com");

; (async () => {
  try {
    console.log("here")
    const txhash = await connection.requestAirdrop(keypair.publicKey, 2 * LAMPORTS_PER_SOL)
    console.log(`you got 2 sol ${txhash}`)
  } catch (error) {
    console.log(error)
  }

})();
