import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { MonopolyOnChain } from "../target/types/monopoly_on_chain";

describe("monopoly-on-chain", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.MonopolyOnChain as Program<MonopolyOnChain>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
