import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { DriftAdaptor } from "../../target/types/drift_adaptor";
import { build_initialize_token_tx } from "../lib/token-faucet-helper";
import { TOKEN_FAUCET_PROGRAM_ID, USDC_MINT, opts } from "../constants";
import { confirmTransaction, tx_logger, v0_pack } from "../helper";

const program = anchor.workspace.DriftAdaptor as Program<DriftAdaptor>;
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
const admin = anchor.Wallet.local();

describe("Initialize Token", () => {
    before("Create USDC Token",async () => {
        const init_USDC = await build_initialize_token_tx(
            admin.publicKey,
            "USDC",
            6,
            "USD Coin",
            "",
            new BN(10000 * 10 ** 6),
            new BN(1),
            TOKEN_FAUCET_PROGRAM_ID,
            provider
        );
        const tx = await provider.connection.sendTransaction(await v0_pack(init_USDC, admin), opts );
        tx_logger(tx);
        await confirmTransaction(tx);
    });

    it("Initialize USDC, create adaptor USDC token, USDC supply vault map, adaptor USDC price account", async() => {
        const [adaptorTokenMint] = PublicKey.findProgramAddressSync(
            [USDC_MINT.toBuffer(), Buffer.from([1])],
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
        const init_USDC = await program.methods.initializeToken({
            decimals: 6,
            tokenType: 1
        }).accounts({
            signer: admin.publicKey,
            adaptorToken: adaptorTokenMint,
            price: pricePDA,
            tokenMap: tokenMapPDA,
            baseToken: USDC_MINT,
        }).instruction();
        const tx = await provider.connection.sendTransaction(await v0_pack([init_USDC], admin), opts);
        tx_logger(tx);
    });
});