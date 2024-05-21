import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID, Account, getAssociatedTokenAddress,
} from "@solana/spl-token";
import { Solhabits } from "../target/types/solhabits";
import {deriveHabit} from "./pda";
import {castJudgement, newHabit} from "./instruction";
import { expect } from "chai";

function sleep(ms: number) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

async function wait(program, secs: number) {
  let currentSlot = await program.provider.connection.getSlot();
  let currentBlockTime = await program.provider.connection.getBlockTime(currentSlot);
  let waitUntil = currentBlockTime + secs;

  while (true) {
    currentSlot = await program.provider.connection.getSlot();
    currentBlockTime = await program.provider.connection.getBlockTime(currentSlot);
    if (currentBlockTime > waitUntil) {
      break;
    }
    await sleep(1000);
  }
}

describe("solhabits", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Solhabits as Program<Solhabits>;

  const accAlice = anchor.web3.Keypair.generate();
  const accBob = anchor.web3.Keypair.generate();
  const accCharlie = anchor.web3.Keypair.generate();
  const accCharityOne = anchor.web3.Keypair.generate();
  const accTokenCreator = anchor.web3.Keypair.generate();


  const habitOneTitle: string = "Stop FOMOing into meme coins";
  const habitOneTitleBuffer = new TextEncoder().encode(habitOneTitle);

  const [bobHabit1, bumpBobHabit1] = deriveHabit(program, accBob.publicKey, habitOneTitleBuffer);

  let mintAccount: anchor.web3.PublicKey;
  let bobATA: Account;
  let vaultATA: anchor.web3.PublicKey;

  before(async () => {
    const airdrop1 = await program.provider.connection.requestAirdrop(accTokenCreator.publicKey, 1_000_000_000);// 1 SOL
    await program.provider.connection.confirmTransaction(airdrop1);

    const airdrop2 = await program.provider.connection.requestAirdrop(accBob.publicKey, 1_000_000_000);// 1 SOL
    await program.provider.connection.confirmTransaction(airdrop2);

    mintAccount = await createMint(
        program.provider.connection,
        accTokenCreator,
        accTokenCreator.publicKey,
        null,
        6
    );

    bobATA = await getOrCreateAssociatedTokenAccount(
        program.provider.connection,
        accTokenCreator,
        mintAccount,
        accBob.publicKey
    );
    await mintTo(
        program.provider.connection,
        accTokenCreator,
        mintAccount,
        bobATA.address,
        accTokenCreator,
        1_000_000_000);

    vaultATA = await getAssociatedTokenAddress(
        mintAccount,
        bobHabit1,
        true
    );

  });

  it("Bob can create a new habit", async () => {
    const deadlineTime = Math.floor(Date.now() / 1000 + 1);
    let habitOne = await newHabit(program, accBob, bobHabit1, mintAccount, bobATA.address, vaultATA, new anchor.BN(1000), habitOneTitle, accCharlie.publicKey, accBob.publicKey, accCharityOne.publicKey, new anchor.BN(deadlineTime));
    expect(habitOne).to.not.be.undefined;
  });

  it("Charlie can judge on Bobs habit, but Alice can't", async () => {
    // Have to wait here until the delay passes
    await wait(program, 2);

    let habitOneFails = await castJudgement(program, accAlice, bobHabit1, mintAccount, bobATA.address, vaultATA, true, "A raw constraint was violated");

    let habitOne = await castJudgement(program, accCharlie, bobHabit1, mintAccount, bobATA.address, vaultATA, true, "");
    console.log(habitOne);
    expect(habitOne).to.not.be.undefined;
    expect(habitOne.outcome).to.be.true;
  });

  //TODO negative tests
});
