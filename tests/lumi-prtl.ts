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

  let title = "shipment-001";
  let destination_location = "New York, NY"; // Example destination location
  let description = "Electronics shipment"; // Example description
  let current_location = "Los Angeles, CA"; // Example current location
  let destination_coordinates = { lat: 40.7128, lng: -74.0060 }; // Example coordinates
  let current_location_coordinates = 5.7;
  //{ lat: 34.0522, lng: -118.2437 }; // Example coordinates
  let status = "In Transit"; // Example status

  const depositAmountForLogistics = new BN(50_000_000); // 50 USD

  before(async () => {
    // Airdrop SOL to buyer
    await connection.requestAirdrop(buyer.publicKey, 2 * LAMPORTS_PER_SOL);
    const latestBlockhash = await connection.getLatestBlockhash();
    await connection.confirmTransaction({
      signature: await connection.requestAirdrop(
        buyer.publicKey,
        2 * LAMPORTS_PER_SOL
      ),
      ...latestBlockhash,
    });
    console.log("Airdrop for buyer completed", latestBlockhash);

    // Airdrop SOL seller
    await connection.requestAirdrop(seller.publicKey, 2 * LAMPORTS_PER_SOL);
    const latestBlockhashForSeller = await connection.getLatestBlockhash();
    await connection.confirmTransaction({
      signature: await connection.requestAirdrop(
        seller.publicKey,
        2 * LAMPORTS_PER_SOL
      ),
      ...latestBlockhashForSeller,
    });
    console.log("Airdrop for seller completed", latestBlockhashForSeller);


    // Airdrop SOL logistics
    /*await connection.requestAirdrop(logistics.publicKey, 2 * LAMPORTS_PER_SOL);
    const latestBlockhashForLogistics = await connection.getLatestBlockhash();
    await connection.confirmTransaction({
      signature: await connection.requestAirdrop(
        logistics.publicKey,
        2 * LAMPORTS_PER_SOL
      ),
      ...latestBlockhashForLogistics,
    });
    console.log("Airdrop for logistics completed", latestBlockhashForLogistics);*/

    // Create USD Mint
    mintUsd = await spl.createMint(connection, buyer, buyer.publicKey, null, 6);

    // Mint USD to Buyer
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

    // Derive Escrow PDA
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

    // Mint USD to Seller
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
      buyer, // Mint authority is the buyer. Should the Buyer really be the mint authority? Not Admin!
      1_000_000_000 // 1000 tokens of 6 decimals (1_000_000 USD)
    );
  });

  it("Buyer initialized Escrow offer!", async () => {
    // Transactions should be in try/catch block to catch errors in the transaction
    try {
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
          systemProgram: SystemProgram.programId,
        })
        .signers([buyer])
        .rpc();

      console.log("✅ Transaction successful:", tx);
    } catch (error) {
      console.log(`An error occured: ${error}`);
    }

    // Assertions 🧪
    const escrowAccount = await program.account.escrow.fetch(escrow);
    assert.equal(escrowAccount.buyer.toBase58(), buyer.publicKey.toBase58());
  });

  it("Seller initialized Shipment with the shipment account!", async () => {
    [shipmentPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(title), seller.publicKey.toBuffer()],
      program.programId
    );
    console.log("Shipment PDA:", shipmentPda.toBase58());
    const sellerTx = await program.methods
      .initShipment(title)
      .accountsPartial({
        seller: seller.publicKey,
        mintUsd,
        sellerAta,
        shipment: shipmentPda,
        systemProgram: SystemProgram.programId,
        associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: spl.TOKEN_PROGRAM_ID,
      })
      .signers([seller])
      .rpc();
    console.log("✅ Transaction successful:", sellerTx);
    // Assertions 🧪
    // Fetch the shipment account and check data
    const shipmentAccount = await program.account.shipment.fetch(shipmentPda);

    assert.equal(shipmentAccount.title, title);
    assert.equal(shipmentAccount.status, "In Transit");
    console.log("Shipment initialized with title:", shipmentAccount.title);
  });

  it ("Seller pays logistics for their service of transport and tracking", async () => {
    const logisticsTx = await program.methods.deposit(depositAmountForLogistics).accountsPartial({
      seller: seller.publicKey,
      logistics: logistics.publicKey,
      mintUsd,
      logisticsAta,
      sellerAta,
      associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
      tokenProgram: spl.TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
    }).signers([seller]).rpc();  
    console.log("✅ Transaction successful:", logisticsTx);
    // Assertions 🧪
  });

  it ("Logistics updates the shipment based on their tracking info", async () => {
    [shipmentPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(title), seller.publicKey.toBuffer()],
      program.programId
    );
    console.log("Shipment PDA:", shipmentPda.toBase58());
    const logisticsTx = await program.methods
      .updateShipment(
        title, 
        description, 
        destination_location,
        current_location,
        destination_coordinates.lat,
        destination_coordinates.lng,
        status
      )
      .accountsPartial({
        seller: seller.publicKey,
        logistics: logistics.publicKey,
        shipment: shipmentPda,
        mintUsd,
        logisticsAta,
        associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: spl.TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([logistics])
      .rpc();
    console.log("✅ Transaction successful:", logisticsTx);
    // Assertions 🧪
  })

  it ("Buyer can file a dispute in case of delay or loss of shipment", async () => {
    const reason = "Delay in shipment"; // Example reason for dispute
    const disputeTx = program.methods.dispute(title, reason).accountsPartial({
      escrow,
      shipment: shipmentPda,
      buyer: buyer.publicKey,
      logistics: logistics.publicKey
    })
    console.log("Dispute filed successfully:", disputeTx);
  })
  // Log or assert
  //console.log("Derived PDA:", shipmentPda.toBase58());*/
});