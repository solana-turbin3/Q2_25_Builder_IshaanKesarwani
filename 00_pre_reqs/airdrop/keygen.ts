import {Keypair} from "@solana/web3.js"

let kp = Keypair.generate()
console.log("New solana wallet's public key : ", kp.publicKey.toBase58())

//private key
console.log(`[${kp.secretKey}]`)