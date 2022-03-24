import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Keypair, PublicKey, SystemProgram, Connection } from "@solana/web3.js";
import { assert, expect } from "chai";
import { Foo } from "../../target/types/foo";
import { FooMaster } from "../../target/types/foo_master";

describe("Can the program modify and sign for the account? Yes, Pda derived from the program's id, and whose owner is the program.", () => {
  const provider = anchor.Provider.local("http://127.0.0.1:8899");

  anchor.setProvider(provider);

  const foo_program = anchor.workspace.Foo as Program<Foo>;
  const foo_master_program = anchor.workspace.FooMaster as Program<FooMaster>;
  const new_foo_acc = new Keypair();

  it("Is initialized! and FooMaster program sign for the foo account", async () => {
    // Add your test here.

    const [fooMasterPDA, fooMasterBump] = await PublicKey.findProgramAddress(
      [],
      foo_master_program.programId
    );

    await foo_program.methods
      .initialize()
      .accounts({
        foo: new_foo_acc.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([new_foo_acc])
      .rpc();

    // const tx = await foo_program.methods
    //   .setData(new anchor.BN("007"))
    //   .accounts({
    //     foo: new_foo_acc.publicKey,
    //     signer: provider.wallet.publicKey,
    //   })
    //   .rpc();

    const tx = await foo_master_program.methods
      .pullStrings(fooMasterBump, new anchor.BN("007"))
      .accounts({
        foo: new_foo_acc.publicKey,
        fooProgram: foo_program.programId,
        authority: fooMasterPDA,
      })
      .rpc();

    const foo_acc = await foo_program.account.data.fetch(new_foo_acc.publicKey);
    assert.ok(foo_acc.data.eq(new anchor.BN("007")));
  });

  it("Is intialized! and foomaster program modifies it own pda accounts", async () => {
    const [userPDA, _] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("user"),
        provider.wallet.publicKey.toBuffer(),
      ],
      foo_master_program.programId
    );

    await foo_master_program.methods
      .createUser("Alex")
      .accounts({
        user: userPDA,
        signer: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    await foo_master_program.methods
      .changerUserName("Brain")
      .accounts({
        user: userPDA,
        signer: provider.wallet.publicKey,
      })
      .rpc();

    const foo_mater_acc = await foo_master_program.account.user.fetch(userPDA);
    expect(foo_mater_acc.name).to.eq("Brain");
  });
});
