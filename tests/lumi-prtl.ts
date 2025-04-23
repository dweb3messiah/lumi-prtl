/*import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LumiPrtl } from "../target/types/lumi_prtl";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { BN } from "bn.js";
import * as spl from "@solana/spl-token";

describe("lumi-prtl", async () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();
  const connection = provider.connection;
  const program = anchor.workspace.LumiPrtl as Program<LumiPrtl>;

  let mintUsd: PublicKey; // the mint address for the first token
  let buyerAtaA: PublicKey;
  let sellerAta: PublicKey;  // taker's associated token account for usd
  let logisticsAta: PublicKey; // logistics associated token account for usd
  let vault: PublicKey;   // the vault address for the token
  let escrow: PublicKey;  // the escrow address

  const buyer = Keypair.generate();
  const seller = Keypair.generate();
  const logistics = Keypair.generate();
  const seed = new anchor.BN(1);
  const depositAmount = new anchor.BN(50); // amount of tokens to deposit
  const receiveAmount = new BN(50); // amount of tokens to receive by the logistics for the service

  before(async () => {
    // Airdrop some SOL
    const buyerAirdrop = await connection.requestAirdrop(
      buyer.publicKey,
      7 * LAMPORTS_PER_SOL
    );
    const sellerAirdrop = await connection.requestAirdrop(
      seller.publicKey,
      7 * LAMPORTS_PER_SOL
    );

    const latestBlockhash = await connection.getLatestBlockhash(); // get the latest blockhash of the cluster we are connected to
    await connection.confirmTransaction({ // confirm the transaction of airdropping SOL
      signature: buyerAirdrop,
      blockhash: latestBlockhash.blockhash,
      lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
    });
    await connection.confirmTransaction({
      signature: sellerAirdrop,
      blockhash: latestBlockhash.blockhash,
      lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
    });

    // Create tokens
    mintUsd = await spl.createMint(
      connection,
      buyer,
      buyer.publicKey,
      null,
      6,
      undefined,
      undefined,
      spl.TOKEN_PROGRAM_ID
    );

    // Create token accounts
    //buyerAtaA = await spl.createAccount(connection, buyer, mintUsd, buyer.publicKey);
    const buyerAtaAccount = await spl.getOrCreateAssociatedTokenAccount(
      connection,
      buyer, // fee payer
      mintUsd,
      buyer.publicKey
    );
    buyerAtaA = buyerAtaAccount.address;
    

    sellerAta = await spl.createAccount(connection, seller, mintUsd, seller.publicKey);

    logisticsAta = await spl.createAccount(connection, logistics, mintUsd, logistics.publicKey);

    // Mint tokens
    /*await spl.mintTo(connection, buyer, mintUsd, buyerAtaA, buyer, 1000);
    await spl.mintTo(connection, seller, mintUsd, sellerAta, seller, 1000);
    await spl.mintTo(connection, logistics, mintUsd, logisticsAta, logistics, 1000);*/
   /* const amount = 1_000_000_000; // 1000 tokens with 6 decimals

  await spl.mintTo(connection, buyer, mintUsd, buyerAtaA, buyer, amount);
  await spl.mintTo(connection, seller, mintUsd, sellerAta, seller, amount);
  await spl.mintTo(connection, logistics, mintUsd, logisticsAta, logistics, amount);


    // Derive PDA
    [escrow] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("escrow"), buyer.publicKey.toBuffer(), seed.toArrayLike(Buffer, "le", 8)],
      program.programId
    );

    // Derive vault
    vault = await anchor.utils.token.associatedAddress({
      mint: mintUsd,
      owner: escrow,
    });
  });

  const buyerBalance = await spl.getAccount(connection, buyerAtaA);
  console.log("Buyer balance:", buyerBalance.amount.toString());

  const vaultBalance = await connection.getTokenAccountBalance(vault).catch(() => null);
  console.log("Vault balance before:", vaultBalance?.value?.uiAmountString ?? "Vault doesn't exist yet");



  it("Buyer initialized Escrow offer!", async () => {
    // Add your test here.
    const tx = await program.methods
    .buy(seed, depositAmount)
    .accountsPartial({
      buyer: buyer.publicKey,
      mintUsd,
      buyerAtaA,
      escrow,
      vault,
      associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
      tokenProgram: spl.TOKEN_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).rpc();
    console.log("Your transaction signature", tx);
  });
});*/
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LumiPrtl } from "../target/types/lumi_prtl";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
} from "@solana/web3.js";
import { BN } from "bn.js";
import * as spl from "@solana/spl-token";
import { assert } from "chai";
import { title } from "process";

describe("lumi-prtl", async () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();
  const connection = provider.connection;
  const program = anchor.workspace.LumiPrtl as Program<LumiPrtl>;

  let mintUsd: PublicKey;
  let buyerAtaA: PublicKey;
  let vault: PublicKey;
  let escrow: PublicKey;
  

  const buyer = Keypair.generate();
  const seed = new BN(1);
  const depositAmount = new BN(50_000_000); // 50 USD


  let logisticsAta: PublicKey;
  let sellerAta: PublicKey;

  let shipmentPda: PublicKey;
  let bumpShipment: number;

  const seller = Keypair.generate();
  const logistics = Keypair.generate();

  const title = "shipment-001";

  const depositAmountForLogistics = new BN(50_000_000); // 50 USD


  before(async () => {
    await connection.requestAirdrop(buyer.publicKey, 2 * LAMPORTS_PER_SOL);

    const latestBlockhash = await connection.getLatestBlockhash()
    await connection.confirmTransaction({
      signature: await connection.requestAirdrop(
        buyer.publicKey,
        2 * LAMPORTS_PER_SOL
      ),
      ...latestBlockhash,
    });
    console.log("Airdrop for buyer completed", latestBlockhash);

    await connection.requestAirdrop(seller.publicKey, 2 * LAMPORTS_PER_SOL);

    const latestBlockhashForSeller = await connection.getLatestBlockhash()
    await connection.confirmTransaction({
      signature: await connection.requestAirdrop(
        seller.publicKey,
        2 * LAMPORTS_PER_SOL
      ),
      ...latestBlockhashForSeller,
    });
    console.log("Airdrop for seller completed", latestBlockhashForSeller);

    mintUsd = await spl.createMint(
      connection,
      buyer,
      buyer.publicKey,
      null,
      6
    );

    const buyerTokenAccount = await spl.getOrCreateAssociatedTokenAccount(
      connection,
      buyer,
      mintUsd,
      buyer.publicKey
    );
    buyerAtaA = buyerTokenAccount.address;

    console.log("Buyer ATA:", buyerAtaA.toBase58());

    await spl.mintTo(
      connection,
      buyer,
      mintUsd,
      buyerAtaA,
      buyer,
      1_000_000_000 // 1000 tokens of 6 decimals (1_000_000 USD)
    );

    [escrow] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("escrow"),
        buyer.publicKey.toBuffer(),
        seed.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );
    console.log("Escrow PDA:", escrow.toBase58());

    //vault = await spl.getAssociatedTokenAddress(mintUsd, escrow, true);

    const sellerTokenAccount = await spl.getOrCreateAssociatedTokenAccount(
      connection,
      seller,
      mintUsd,
      seller.publicKey
    );
    sellerAta = sellerTokenAccount.address;

    console.log("Seller ATA:", sellerAta.toBase58()); 

    await spl.mintTo(
      connection,
      seller,
      mintUsd,
      sellerAta,
      seller,
      1_000_000_000 // 1000 tokens of 6 decimals (1_000_000 USD)
    );


});

  it("Buyer initialized Escrow offer!", async () => {
    const tx = await program.methods
      .buy(seed, depositAmount)
      .accountsPartial({
        buyer: buyer.publicKey,
        mintUsd,
        buyerAtaA,
        escrow,
       // vault,
        associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: spl.TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([buyer])
      .rpc();

    console.log("✅ Transaction successful:", tx);

    // Assertions 🧪
    const escrowAccount = await program.account.escrow.fetch(escrow);
    assert.ok(escrowAccount.buyer.equals(buyer.publicKey));
    assert.ok(escrowAccount.mintUsd.equals(mintUsd));
    assert.strictEqual(escrowAccount.amount.toNumber(), 0); // since deposit isn't stored here directly
    assert.strictEqual(escrowAccount.isCompleted, false);

    const vaultBalance = await connection.getTokenAccountBalance(vault);
    assert.strictEqual(vaultBalance.value.uiAmount, 50);
    console.log("✅ Vault successfully funded with 50 USD");
  });

  

 it("Seller initialized Shipment!", async () => {
  console.log("Running second instruction");
  [shipmentPda] = PublicKey.findProgramAddressSync(
    [Buffer.from(title), seller.publicKey.toBuffer()],
    program.programId
 );
  const sellerTx = await program.methods.initShipment(title).accountsPartial({
    seller: seller.publicKey,
    mintUsd,
    sellerAta,
    shipment: shipmentPda,
    systemProgram: SystemProgram.programId,
    associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
    tokenProgram: spl.TOKEN_PROGRAM_ID,
  }).signers([seller]).rpc();
  console.log("✅ Transaction successful:", sellerTx);
  // Assertions 🧪
  // Fetch the shipment account and check data
  const shipmentAccount = await program.account.shipment.fetch(shipmentPda);
    
  assert.equal(shipmentAccount.title, title);
  assert.equal(shipmentAccount.status, "In Transit");
  console.log("Shipment initialized with title:", shipmentAccount.title);
  
 })
// Log or assert
//console.log("Derived PDA:", shipmentPda.toBase58());*/


});

