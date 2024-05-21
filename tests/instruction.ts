import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Solhabits } from "../target/types/solhabits";

export const newHabit = async (program: Program<Solhabits>, signer: anchor.web3.Signer, accHabit: anchor.web3.PublicKey, mintAccount: anchor.web3.PublicKey, bobATA: anchor.web3.PublicKey, vaultATA: anchor.web3.PublicKey, amount: anchor.BN, description: string, judge: anchor.web3.PublicKey, to_success: anchor.web3.PublicKey, to_failure: anchor.web3.PublicKey, deadline: anchor.BN) => {
    let tx;

    try {
        tx = await program.methods
            .newHabit(amount,
            description,
            judge,
            to_success,
            to_failure,
            deadline)
            .accounts({
                signer: signer.publicKey,
                habit: accHabit,
                tokenSource: bobATA,
                tokenVault: vaultATA,
                tokenMint: mintAccount,
            })
            .signers([signer])
            .rpc();
    }
    catch (e) {
        console.log("Error: ", e, " TX: ", tx);
        return undefined;
    }

    return await program.account.habit.fetch(accHabit);
}