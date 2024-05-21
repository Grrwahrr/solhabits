import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Solhabits } from "../target/types/solhabits";

const textEncoder = new TextEncoder();

export const deriveHabit = (program: Program<Solhabits>, creator: anchor.web3.PublicKey, description: Uint8Array) =>
    anchor.web3.PublicKey.findProgramAddressSync(
        [textEncoder.encode("habit"), creator.toBuffer(), description],
        program.programId
    );