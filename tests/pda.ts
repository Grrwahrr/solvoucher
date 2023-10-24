import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Solvoucher } from "../target/types/solvoucher";

const textEncoder = new TextEncoder();

export const deriveConfig = (program: Program<Solvoucher>, collectionName: string) =>
    anchor.web3.PublicKey.findProgramAddressSync(
        [textEncoder.encode("config"), textEncoder.encode(collectionName)],
        program.programId
    );

export const deriveTeacherProfile = (program: Program<Solvoucher>, payer: anchor.web3.PublicKey) =>
    anchor.web3.PublicKey.findProgramAddressSync(
        [textEncoder.encode("teacher"), payer.toBuffer()],
        program.programId
    );
