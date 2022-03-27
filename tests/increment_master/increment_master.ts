import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Increment } from "../../target/types/increment";
import { IncrementMaster } from "../../target/types/increment_master";
import { Connection, Keypair, SystemProgram, PublicKey } from "@solana/web3.js";

describe("PDA as a authority", () => {
  let new_signer = new Keypair();
  const connection = new Connection("http://127.0.0.1:8899");
  let wallet = new anchor.Wallet(new_signer);
  const provider = new anchor.Provider(connection, wallet, {
    preflightCommitment: "processed",
  });

  anchor.setProvider(provider);

  const inc_program = anchor.workspace.Increment as Program<Increment>;
  const inc_master_program = anchor.workspace.Prob as Program<IncrementMaster>;

  it("Is initialized!", async () => {
    const [userPDA, userBump] = await PublicKey.findProgramAddress(
      [],
      inc_master_program.programId
    );

    const sig = await connection.requestAirdrop(wallet.publicKey, 10e9);
    await connection.confirmTransaction(sig);

    let incAcc = new Keypair();

    const txId = await inc_program.methods
      .initializeAcc(new anchor.BN(10), userPDA)
      .accounts({
        user: incAcc.publicKey,
        signer: wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([incAcc])
      .rpc();

    console.log("txId ", txId);

    console.log("IncAcc ", incAcc.publicKey.toBase58());

    // const t = await inc_program.methods
    //   .incrementAcc()
    //   .accounts({
    //     user: incAcc.publicKey,
    //     signer: wallet.publicKey,
    //   })
    //   .rpc();

    const tx = await inc_master_program.methods
      .incrementCounter(userBump)
      .accounts({
        user: incAcc.publicKey,
        incrementProgram: inc_program.programId,
        authority: userPDA,
      })
      .rpc();

    console.log("Tx ", tx);
  });
});
