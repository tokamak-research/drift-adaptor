import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { DriftAdaptor } from "../../target/types/drift_adaptor";
import { build_initialize_token_tx } from "../lib/token-faucet-helper";
import { DRIFT_PROGRAM_ID, SOL_MINT, TOKEN_FAUCET_PROGRAM_ID, USDC_MINT, opts } from "../constants";
import { confirmTransaction, tx_logger, v0_pack } from "../helper";
import { AdminClient, encodeName, getDriftStateAccountPublicKey, getUserAccountPublicKeySync, getUserStatsAccountPublicKey } from "@drift-labs/sdk";
import { createAssociatedTokenAccountInstruction, getAssociatedTokenAddressSync } from "@solana/spl-token";
import { initializeQuoteSpotMarket, initializeSolSpotMarket, mockOracle } from "../lib/drift-helper";

const program = anchor.workspace.DriftAdaptor as Program<DriftAdaptor>;
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
const admin = anchor.Wallet.local();

const adminClient = new AdminClient({
    connection: provider.connection,
    wallet: provider.wallet,
    opts: { skipPreflight: true }
});

describe("Initialize Vault", () => {
    before("Create and Intialize everything for SOL", async () => {
        const create_SOL = await build_initialize_token_tx(
            admin.publicKey,
            "SOL",
            9,
            "SOLANA",
            "",
            new BN(1000 * 10 ** 9),
            new BN(1),
            TOKEN_FAUCET_PROGRAM_ID,
            provider
        );

        const [adaptorTokenMint] = PublicKey.findProgramAddressSync(
            [SOL_MINT.toBuffer(), Buffer.from([0])],
            program.programId
        );
        const [pricePDA] = PublicKey.findProgramAddressSync(
            [adaptorTokenMint.toBuffer()],
            program.programId
        );
        const [tokenMapPDA] = PublicKey.findProgramAddressSync(
            [Buffer.from("map"), adaptorTokenMint.toBuffer()],
            program.programId
        );
        const init_SOL = await program.methods.initializeToken({
            decimals: 9,
            tokenType: 0
        }).accounts({
            signer: admin.publicKey,
            adaptorToken: adaptorTokenMint,
            price: pricePDA,
            tokenMap: tokenMapPDA,
            baseToken: SOL_MINT,
        }).instruction();
        create_SOL.push(init_SOL);
        const tx = await provider.connection.sendTransaction(await v0_pack(create_SOL, admin), opts);
        tx_logger(tx);
        await confirmTransaction(tx);
    });

    before("Intialize Drift USDC, SOL market", async () => {
        await adminClient.initialize(USDC_MINT, false);
        await adminClient.subscribe();
        await initializeQuoteSpotMarket(adminClient, USDC_MINT);
        const solUsd = await mockOracle(1000);
        const tx2 = await initializeSolSpotMarket(adminClient, solUsd, SOL_MINT);
        tx_logger(tx2);
    });

    after(async () => {
        await adminClient.unsubscribe();
    })

    it("Initialize SOL/USDC vault", async () => {
        const [vaultPDA] = PublicKey.findProgramAddressSync(
            [Buffer.from(encodeName("SOL/USDC"))],
            program.programId
        );

        const [adaptorBaseTokenMint] = PublicKey.findProgramAddressSync(
            [SOL_MINT.toBuffer(), Buffer.from([0])],
            program.programId
        );

        const [adaptorQuoteTokenMint] = PublicKey.findProgramAddressSync(
            [USDC_MINT.toBuffer(), Buffer.from([1])],
            program.programId
        );

        const [baseTokenMapPDA] = PublicKey.findProgramAddressSync(
            [Buffer.from("map"), adaptorBaseTokenMint.toBuffer()],
            program.programId
        );

        const [quoteTokenMapPDA] = PublicKey.findProgramAddressSync(
            [Buffer.from("map"), adaptorQuoteTokenMint.toBuffer()],
            program.programId
        );

        const userStatsKey = getUserStatsAccountPublicKey(
            DRIFT_PROGRAM_ID,
            vaultPDA
        );
        const userKey = getUserAccountPublicKeySync(
            DRIFT_PROGRAM_ID,
            vaultPDA
        );
        const driftState = await getDriftStateAccountPublicKey(
            DRIFT_PROGRAM_ID
        );

        const init_vault = await program.methods.initializeVault({
            name: encodeName("SOL/USDC"),
            baseSpotMarketIndex: 1,
            quoteSpotMarketIndex: 0
        }).accounts({
            signer: admin.publicKey,
            vault: vaultPDA,
            baseToken: SOL_MINT,
            quoteToken: USDC_MINT,
            adaptorBaseToken: adaptorBaseTokenMint,
            adaptorQuoteToken: adaptorQuoteTokenMint,
            baseTokenMap: baseTokenMapPDA,
            quoteTokenMap: quoteTokenMapPDA,
            // vaultBaseTokenAta: getAssociatedTokenAddressSync(SOL_MINT, vaultPDA, true),
            // vaultQuoteTokenAta: getAssociatedTokenAddressSync(USDC_MINT, vaultPDA, true),
            driftUser: userKey,
            driftUserStats: userStatsKey,
            driftState: driftState,
            driftProgram: DRIFT_PROGRAM_ID
        }).instruction();

        const init_vault_ata_SOL =
            createAssociatedTokenAccountInstruction(
                admin.publicKey, // payer
                getAssociatedTokenAddressSync(SOL_MINT, vaultPDA, true), // ata
                vaultPDA, // owner
                SOL_MINT // mint
            );

        const init_vault_ata_USDC =
            createAssociatedTokenAccountInstruction(
                admin.publicKey, // payer
                getAssociatedTokenAddressSync(USDC_MINT, vaultPDA, true), // ata
                vaultPDA, // owner
                USDC_MINT // mint
            );

        const tx = await provider.connection.sendTransaction(await v0_pack([init_vault, init_vault_ata_SOL, init_vault_ata_USDC], admin), opts);
        tx_logger(tx);
        await confirmTransaction(tx);
    });


});