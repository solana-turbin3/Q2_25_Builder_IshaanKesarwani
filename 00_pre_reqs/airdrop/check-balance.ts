import { Connection, Keypair } from "@solana/web3.js";
import wallet from "./dev-wallet.json";

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
const connection = new Connection("https://api.devnet.solana.com");

(async () => {
  try {
    const balance = await connection.getBalance(keypair.publicKey);
    console.log(`Your wallet balance is: ${balance / 1_000_000_000} SOL`);
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();