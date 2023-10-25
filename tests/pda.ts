import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Solvoucher } from "../target/types/solvoucher";

const textEncoder = new TextEncoder();

export const deriveConfig = (program: Program<Solvoucher>, collectionName: string) =>
    anchor.web3.PublicKey.findProgramAddressSync(
        [textEncoder.encode("config"), textEncoder.encode(collectionName)],
        program.programId
    );

export const deriveVoucher = (program: Program<Solvoucher>, collectionName: string, offset: number) =>
    anchor.web3.PublicKey.findProgramAddressSync(
        [textEncoder.encode("voucher"), textEncoder.encode(collectionName), new anchor.BN(offset).toArrayLike(Buffer, "le", 4)],
        program.programId
    );

export const deriveOwnerToVoucher = (program: Program<Solvoucher>, collectionName: string, payer: anchor.web3.PublicKey) =>
    anchor.web3.PublicKey.findProgramAddressSync(
        [textEncoder.encode("owner_to_voucher"), textEncoder.encode(collectionName), payer.toBuffer()],
        program.programId
    );
