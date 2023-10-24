import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Solvoucher } from "../target/types/solvoucher";
import {deriveConfig} from "./pda";
import {expect} from "chai";
import {initialize} from "./instruction";

describe("solvoucher", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Solvoucher as Program<Solvoucher>;

  // Test collections
  const collectionName1: string = "testCollection1";

  // Some wallets we will use for testing
  const accAdmin = (program.provider as anchor.AnchorProvider).wallet;
  const accAlice = anchor.web3.Keypair.generate();
  const accBob = anchor.web3.Keypair.generate();
  const accCharlie = anchor.web3.Keypair.generate();

  const [accConfig, bumpConfig] = deriveConfig(program, collectionName1);

  it("Airdrop works.", async () => {
    const airdrop1 = await program.provider.connection.requestAirdrop(accAlice.publicKey, 1_000_000_000);// 1 SOL
    const airdrop2 = await program.provider.connection.requestAirdrop(accBob.publicKey, 10_000_000_000);// 10 SOL
    const airdrop3 = await program.provider.connection.requestAirdrop(accCharlie.publicKey, 1_000_000_000);// 1 SOL

    await program.provider.connection.confirmTransaction(airdrop1);
    await program.provider.connection.confirmTransaction(airdrop2);
    await program.provider.connection.confirmTransaction(airdrop3);
  });

  it("Can initialize a new voucher collection. Only once.", async () => {
    const config = await initialize(program, accAlice, accConfig, collectionName1, "");
    expect(config).to.not.be.undefined;

    const duplicate = await initialize(program, accBob, accConfig, collectionName1, "already in use");
    expect(duplicate).to.be.undefined;
  });
});
