const anchor = require("@coral-xyz/anchor");
const { SystemProgram } = anchor.web3;

describe("Notwest Stable", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  const program = anchor.workspace.NotwestStable;
  let dataAccount = anchor.web3.Keypair.generate();

  it("Initializes the account!", async () => {
    await program.methods
      .initialize(new anchor.BN(100))
      .accounts({
        dataAccount: dataAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([dataAccount])
      .rpc();

    const account = await program.account.dataAccount.fetch(dataAccount.publicKey);
    console.log("✅ Initial value:", account.value.toString());
  });

  it("Deposits funds!", async () => {
    await program.methods
      .deposit(new anchor.BN(50))
      .accounts({
        dataAccount: dataAccount.publicKey,
      })
      .rpc();

    const account = await program.account.dataAccount.fetch(dataAccount.publicKey);
    console.log("💰 After deposit:", account.value.toString());
  });

  it("Withdraws funds!", async () => {
    await program.methods
      .withdraw(new anchor.BN(30))
      .accounts({
        dataAccount: dataAccount.publicKey,
      })
      .rpc();

    const account = await program.account.dataAccount.fetch(dataAccount.publicKey);
    console.log("💸 After withdraw:", account.value.toString());
  });
});
