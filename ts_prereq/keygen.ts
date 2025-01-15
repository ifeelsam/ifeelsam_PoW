import { Keypair } from "@solana/web3.js";

let keypair = Keypair.generate();

const pubKey = keypair.publicKey.toBase58();
const sk = keypair.secretKey;

console.log("you wallet address", pubKey)
console.log(`[${sk}]`)
