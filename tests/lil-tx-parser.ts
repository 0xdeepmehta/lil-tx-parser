import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { LilTxParser } from "../target/types/lil_tx_parser";

describe("lil-tx-parser", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.LilTxParser as Program<LilTxParser>;

  it("Is initialized!", async () => {
    let preAccount = anchor.web3.Keypair.generate();
    let postAccount = anchor.web3.Keypair.generate();
    let crntAccount = anchor.web3.Keypair.generate();

    const tx = await program.methods.curIx().accounts({
      curntAccountId: crntAccount.publicKey,
      instructionSysvarAccount: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY
    }).preInstructions([
      await program.methods.preIx().accounts({
        preAccountId: preAccount.publicKey,
      })
      .instruction(),
    ]).postInstructions([
      await program.methods.postIx().accounts({
        postAccountId: postAccount.publicKey
      })
      .instruction()
    ]).rpc();
    console.log("Your transaction signature", tx);
  });
});