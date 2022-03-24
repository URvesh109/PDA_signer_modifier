import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Bar } from "../../target/types/bar";
import { BarMaster } from "../../target/types/bar_master";
import { PublicKey, Keypair, SystemProgram, Connection } from "@solana/web3.js";
import { expect } from "chai";

describe("1. Can the program sign for the account? Yes, Pda derived from the program's id, and whose owner is a different program. 2. Can the program modify the account? No, Pda derived from the program's id, and whose owner is a different program.", () => {
  // Configure the client to use the local cluster.

  let newSigners = new Keypair();
  const connection = new Connection("http://127.0.0.1:8899/");

  let wallet = new anchor.Wallet(newSigners);

  const provider = new anchor.Provider(connection, wallet, {
    preflightCommitment: "processed",
  });

  anchor.setProvider(provider);

  const bar_program = anchor.workspace.Probability as Program<Bar>;
  const bar_master_program = anchor.workspace.ProbMaster as Program<BarMaster>;

  it("Is initialized!", async () => {
    const [userPDA, useBump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("userdata"),
        provider.wallet.publicKey.toBuffer(),
      ],
      bar_master_program.programId
    );

    const sig = await connection.requestAirdrop(newSigners.publicKey, 1e9);

    await connection.confirmTransaction(sig);
    console.log("usePda ", userPDA.toBase58());

    /**
     * Create pda using bar_master and assigned to bar
     */
    const tx = await bar_master_program.methods
      .createUser(useBump, "Brain")
      .accounts({
        userdata: userPDA,
        signer: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
        probability: bar_program.programId,
      })
      .rpc();
    expect((await bar_program.account.user.fetch(userPDA)).name).to.eql(
      "Brain"
    );

    await bar_program.methods
      .changeName("Acton")
      .accounts({
        user: userPDA,
        signer: provider.wallet.publicKey,
      })
      .rpc();

    expect((await bar_program.account.user.fetch(userPDA)).name).to.eql(
      "Acton"
    );

    const tx3 = await bar_master_program.methods
      .changeName(useBump, "Working")
      .accounts({
        userdata: userPDA,
        signer: provider.wallet.publicKey,
        probability: bar_program.programId,
      })
      .rpc();

    console.log("tx ", tx3);

    expect((await bar_program.account.user.fetch(userPDA)).name).to.eql(
      "Working"
    );
  });
});
