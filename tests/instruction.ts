import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Solvoucher } from "../target/types/solvoucher";

export const initialize = async (program: Program<Solvoucher>, txPayer: anchor.web3.Signer, accConfig: anchor.web3.PublicKey, collectionName1: string, expectedError: string) => {
    let tx;
    try {
        tx = await program.methods
            .configInitialize(collectionName1)
            .accounts({
                payer: txPayer.publicKey,
                config: accConfig,
            })
            .signers([txPayer])
            .rpc();
    }
    catch (e) {
        if (expectedError === "" || !e.toString().includes(expectedError)) {
            console.log("Error: ", e, " TX: ", tx);
        }
        return undefined;
    }

    return await program.account.config.fetch(accConfig);
}