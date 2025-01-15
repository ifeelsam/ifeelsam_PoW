import { Transaction, SystemProgram, Connection, Keypair, LAMPORTS_PER_SOL, sendAndConfirmTransaction, PublicKey } from "@solana/web3.js";
import wallet from "./devnet-wallet.json";

const from = Keypair.fromSecretKey(new Uint8Array(wallet))
const to = new PublicKey("MovHj25KabjUuoYRGMWHsGxHjb1JgCLdefbVrPFQwwJ");
const connection = new Connection("https://api.devnet.solana.com");

(async () => {
  try {
    const balance = await connection.getBalance(from.publicKey)
    // const txn = new Transaction().add(
    //   SystemProgram.transfer({
    //     fromPubkey: from.publicKey,
    //     toPubkey: to,
    //     lamports: balance,
    //   })
    // );
    const txn = new Transaction();
    txn.recentBlockhash = (await connection.getLatestBlockhash("confirmed")).blockhash;
    txn.feePayer = from.publicKey;

    const fee = (await connection.getFeeForMessage(txn.compileMessage(), "confirmed")).value || 0;

    txn.add(
      SystemProgram.transfer({
        fromPubkey: from.publicKey,
        toPubkey: to,
        lamports: balance - fee,
      })
    );

    const sign = await sendAndConfirmTransaction(
      connection,
      txn,
      [from]
    );
    console.log("success", sign)
    console.log(`https://explorer.solana.com/tx/${sign}?cluster=devnet`)
  }
  catch (error) {
    console.log("something went wrong!!!!")
    console.log(error)
  }
})();
