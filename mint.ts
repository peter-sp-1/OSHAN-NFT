import * as anchor from "@coral-xyz/anchor";
import { Connection, PublicKey, SystemProgram } from "@solana/web3.js";

async function initializeNFT() {
  const wallet = pg.wallet;
  const program = pg.program;

  const [nftAccount, bump] = await PublicKey.findProgramAddressSync(
    [Buffer.from("nft"), wallet.publicKey.toBuffer()],
    program.programId
  );

  try {
    const tx = await program.methods
      .mintNft(
        "https://raw.githubusercontent.com/peter-sp-1/NFT/refs/heads/main/metadata.json", 
        new anchor.BN(1 * anchor.web3.LAMPORTS_PER_SOL),
        bump
      )
      .accounts({
        nftAccount: nftAccount,
        owner: wallet.publicKey,
        systemProgram: SystemProgram.programId
      })
      .rpc();

    console.log("NFT Minted. Transaction Signature:", tx);
  } catch (err) {
    console.error("Minting failed:", err);
  }
}
initializeNFT();
