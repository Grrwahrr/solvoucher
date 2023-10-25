import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Solvoucher } from "../target/types/solvoucher";

export const initialize = async (program: Program<Solvoucher>, txPayer: anchor.web3.Signer, accConfig: anchor.web3.PublicKey, collectionName: string, expectedError: string) => {
    let tx;
    try {
        tx = await program.methods
            .configInitialize(collectionName)
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

export const updateConfig = async (program: Program<Solvoucher>, txPayer: anchor.web3.Signer, accConfig: anchor.web3.PublicKey, collectionName: string, expectedError: string) => {
    let tx;
    try {
        tx = await program.methods
            .configUpdate(collectionName, { minting: {} })
            .accounts({
                admin: txPayer.publicKey,
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

export const mintVoucher = async (program: Program<Solvoucher>, txPayer: anchor.web3.Signer, accConfig: anchor.web3.PublicKey, accVoucher: anchor.web3.PublicKey, accOwnerToVoucher: anchor.web3.PublicKey, collectionName: string, data: string, expectedError: string) => {
    let tx;
    try {
        tx = await program.methods
            .voucherMint(collectionName, data)
            .accounts({
                payer: txPayer.publicKey,
                config: accConfig,
                voucher: accVoucher,
                ownerToVoucher: accOwnerToVoucher,
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

    return await program.account.voucher.fetch(accVoucher);
}