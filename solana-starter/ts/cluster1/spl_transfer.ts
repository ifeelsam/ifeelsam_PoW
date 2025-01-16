import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import wallet from "../../Turbin3-wallet.json"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";
import { convertToObject } from "typescript";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("GTqNXAmwfN4v9LSYhNgonuoNbnzju7nTsLqvhNMrU4kJ");

// Your ata is: 7r34SpDMDqJiv7xguP1bPr2onT4HMiRYuSU9a2sWs2Nh
// Your mint txid: 2fj2PFENj15iReH1N8RBSdrn6raoKPmcLQig5a1agYLJorvZPE83YYg8gXwrQYHxtmCPHx8kGG3VE8tFHsCM6Y2B
// Recipient address
const to = new PublicKey("4LVyud6zUyACiFvhK3tshYkN6H23wSJmM4GYHorABpa1");

(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it
        const fromTokenAccoun= await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            keypair.publicKey
        )
        // Get the token account of the toWallet address, and if it does not exist, create it
        const toTokenAccount = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            to
        )
        // Transfer the new token to the "toTokenAccount" we just created
        const signature = await transfer(
            connection,
            keypair,
            fromTokenAccoun.address,
            toTokenAccount.address,
            keypair,
            1 * Math.pow(10, 6)
        )
        console.log("Token sign", signature)
// Token sign 5iioyLdrVQFnCDK82U1oaRr7akAp1Mhbyp2Ps12A19bwgTbD4QVnUB71qX7TQ6E4i1uB94RzArEC9UywrLfWa62i
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();