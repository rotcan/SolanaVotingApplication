import { Connection, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { PhantomProvider } from "./phantom";
import { Buffer } from 'buffer';
const BufferLayout = require("buffer-layout");
export const cluster = "http://localhost:8899";
export const commitment = "confirmed";
export const connection = new Connection(cluster, commitment);
export const poll_count_seed = "PollCount";
export const poll_seed = "Poll";


export const TITLE_LENGTH = 100;
export const OPTION_LENGTH = 50;
export const OPTION_COUNT = 4;
export const OPTION_SIZE = (OPTION_LENGTH + 1 + 1 + 8);

export const getProvider = (): PhantomProvider | undefined => {
    if ("solana" in window) {
        // @ts-ignore
        const provider = window.solana as any;
        if (provider.isPhantom) return provider as PhantomProvider;
    }
};

export const airdropSOL = async () => {

    var provider = await getProvider();
    console.log("Public key of the emitter: ", provider!.publicKey!.toBase58());

    // Establishing connection
    // Airdrop some SOL to the sender's wallet, so that it can handle the txn fee
    var airdropSignature = await connection.requestAirdrop(
        provider!.publicKey!,
        LAMPORTS_PER_SOL,
    );

    // Confirming that the airdrop went through
    await connection.confirmTransaction(airdropSignature);
    await getBalance(provider!.publicKey!);
}

export const getAccountBalance = async (connection: Connection, pubkey: PublicKey) => {
    const val = await connection.getBalance(pubkey);
    return val;
}

export const getBalance = async (accountKey: PublicKey): Promise<Number> => {
    const value = (await getAccountBalance(connection, accountKey)) / LAMPORTS_PER_SOL;
    return value
}

export const getPollCountSeedArray = (): Buffer[] => {
    return [Buffer.from(poll_count_seed)]
}


export const uint64 = (property = "uint64") => {
    return BufferLayout.blob(8, property);
}

const string_len = (property = "string", len: number) => {
    return BufferLayout.blob(len, property);
}

export const VOTE_ACCOUNT = BufferLayout.struct([
    BufferLayout.u8("isInitialized"),
    BufferLayout.u8("vote_group"),
    uint64("vote1"),
    uint64("vote2"),
    BufferLayout.u8("bump")
])

export const VOTER_ACCOUNT = BufferLayout.struct([
    BufferLayout.u8("vote_group"),
    BufferLayout.u8("vote_count"),
])


export const POLL_COUNT_ACCOUNT = BufferLayout.struct([
    BufferLayout.u8("isInitialized"),
    BufferLayout.u8("poll_count")
])

export const POLL_ACCOUNT = BufferLayout.struct([
    BufferLayout.u8("isInitialized"),
    BufferLayout.u8("id"),
    string_len("title", 100),
    BufferLayout.u8("title_length"),
    string_len("options", 60 * 4),
    BufferLayout.u8("options_length"),
    BufferLayout.u8("bump"),

])


export const POLL_VOTER_ACCOUNT = BufferLayout.struct([
    BufferLayout.u8("isInitialized"),
    BufferLayout.u8("id"),
    BufferLayout.u8("selected"),
    BufferLayout.u8("bump"),
])

export interface PollCount {
    isInitialized: boolean,
    count: number,
    bump: number
}

export interface PollVoter {
    isInitialized: boolean,
    pollId: number,
    optionSelected: number,
    bump: number
}

export interface PollOption {
    id: number,
    title: string,
    titleLength: number,
    votes: number
}

export interface Poll {
    isInitialized: boolean,
    id: number,
    title: string,
    titleLength: number,
    options: PollOption[],
    optionsLength: number,
    bump: number
}