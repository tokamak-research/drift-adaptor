import * as anchor from "@coral-xyz/anchor";
import { PublicKey } from '@solana/web3.js';
import { DriftAdaptor } from "../target/types/drift_adaptor";
import { encodeName } from "./helper";

const drift_adaptor = anchor.workspace.DriftAdaptor as anchor.Program<DriftAdaptor>;
export const TOKEN_FAUCET_PROGRAM_ID = new PublicKey("7MGkDRY9dhNS6KYRBSzrNadH9uuuPeazJm4TdESAwYsb");
export const DRIFT_PROGRAM_ID = new PublicKey("dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH");

export const [globalConfigPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("global_config")],
    drift_adaptor.programId
)

export const opts = { skipPreflight: true};

const loglevellist = ["none", "devnet", "localhost"];
export const loglevel = loglevellist[2];

export const [USDC_MINT] = PublicKey.findProgramAddressSync(
    [Buffer.from("mint"), Buffer.from("USDC")],
    TOKEN_FAUCET_PROGRAM_ID
)

export const [SOL_MINT] = PublicKey.findProgramAddressSync(
    [Buffer.from("mint"), Buffer.from("SOL")],
    TOKEN_FAUCET_PROGRAM_ID
);

export const marketList = [
    "SOL/USDC",
    "wBTC/USDC",
    "wETH/USDC",
    "mSOL/USDC",
    "jitoSOL/USDC",
    "BONK/USDC",
    "WIF/USDC",
]

export const token_decimals = [
    9,
    8,
    8,
    9,
    9,
    6,
    6
]