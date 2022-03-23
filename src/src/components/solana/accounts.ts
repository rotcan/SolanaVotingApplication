import { AccountChangeCallback, AccountInfo, Connection, Keypair, PublicKey, sendAndConfirmTransaction, SystemProgram, Transaction } from '@solana/web3.js';
import BN from 'bn.js';
import adminKey from '../../keys/admin.json';
import accountKey1 from '../../keys/key1.json';
import accountKey2 from '../../keys/key2.json';
import accountKey3 from '../../keys/key3.json';
import { commitment, OPTION_COUNT, OPTION_SIZE, Poll, PollCount, PollOption, PollVoter, TITLE_LENGTH } from './solutil';

export enum Users {
    Admin,
    User1,
    User2,
    User3
}

export enum TransactionResponseEnum {
    Success,
    TransferToBlockedAccount,
    AdminAccountNotCreated,
    ProgramNotSupported,
    AccountNotAllowed
}

// export const ADMIN_PUBKEY = "9cnSj92djjErbqUahBcoTxmc5mL31hcaePDq971AmDEF";

export const getUserKeyPair = (user: Users) => {
    switch (user) {
        case Users.Admin:
            return getKeyPair(adminKey);
        case Users.User1:
            return getKeyPair(accountKey1);
        case Users.User2:
            return getKeyPair(accountKey2);
        case Users.User3:
            return getKeyPair(accountKey3);
    }
}

export const getKeyPair = (key: any) => {
    //const secretKey = Uint8Array.from(JSON.parse(adminKey));
    const secretKey = Uint8Array.from(key);
    return Keypair.fromSecretKey(secretKey)
}

export const ADMIN_ACCOUNT_SEED = "1";

export const getAdminAccountPublicKey = async (programId: PublicKey, adminPubKey: PublicKey) => {
    const seedStr = ADMIN_ACCOUNT_SEED;
    const profileAccountPubKey = await PublicKey.createWithSeed(
        adminPubKey,
        seedStr,
        programId,
    );
    return profileAccountPubKey;
}

export const getAccountFromProgram = async (connection: Connection, programId: PublicKey, adminPubKey: PublicKey) => {

    const profileAccountPubKey = await getAdminAccountPublicKey(programId, adminPubKey);

    let accountInfo = await connection.getAccountInfo(profileAccountPubKey);
    console.log(accountInfo);
    console.log("profileAccountPubKey=" + (accountInfo == null));
    return accountInfo;
}

export const createAccountFromProgram = async (connection: Connection, programId: PublicKey, adminKeyPair: Keypair) => {
    const size = 32;
    const rent = await connection.getMinimumBalanceForRentExemption(size);
    const seedStr = ADMIN_ACCOUNT_SEED;

    const profileAccountPubKey = await PublicKey.createWithSeed(
        adminKeyPair.publicKey,
        seedStr,
        programId,
    );

    let accountInfo = await connection.getAccountInfo(profileAccountPubKey);
    console.log(profileAccountPubKey);
    if (accountInfo === null) {
        //console.log(aliceKeyPair.publicKey);
        //let createTx = create(ProgramId, aliceKeyPair, profileAccountPubKey, 1 + 32, await connection.getMinimumBalanceForRentExemption(1 + 32));
        const instruction = SystemProgram.createAccountWithSeed({
            fromPubkey: adminKeyPair.publicKey,
            basePubkey: adminKeyPair.publicKey,
            seed: seedStr,
            newAccountPubkey: profileAccountPubKey,
            lamports: rent,
            space: size,
            programId: programId,
        });

        let txc = new Transaction();
        txc.add(instruction);
        let signers2 = [adminKeyPair];
        let txidc = await sendAndConfirmTransaction(connection, txc, signers2, {
            skipPreflight: true,
            preflightCommitment: "singleGossip",
            commitment: "singleGossip",
        });
        console.log(txidc);
    }
    accountInfo = await connection.getAccountInfo(profileAccountPubKey)
    return accountInfo;
}

export const getPda = async (programId: PublicKey, seeds: Buffer[] | Uint8Array[]): Promise<[PublicKey, number]> => {
    return (await PublicKey.findProgramAddress(seeds, programId));
}

export const getPdaAccount = async (connection: Connection, programId: PublicKey, seeds: Buffer[] | Uint8Array[]): Promise<AccountInfo<Buffer> | null> => {
    const pda = (await PublicKey.findProgramAddress(seeds, programId))[0];
    return await connection.getAccountInfo(pda, commitment);
}


export const getPollCountAccount = (accountData: AccountInfo<Buffer> | null): PollCount | null => {
    if (accountData != null) {
        const data = accountData.data;
        const isInitialized = new BN(data.slice(0, 1), "le").toNumber() === 0 ? false : true;
        const count = new BN(data.slice(1, 2), "le").toNumber();
        const bump = new BN(data.slice(2, 3), "le").toNumber();
        const acc: PollCount = { isInitialized: isInitialized, count: count, bump: bump };
        console.log(acc);
        return acc;
    }
    return null;
}

export const getVoterPollAccount = (accountData: AccountInfo<Buffer> | null): PollVoter | null => {
    if (accountData != null) {
        const data = accountData.data;
        const isInitialized = new BN(data.slice(0, 1), "le").toNumber() === 0 ? false : true;
        const pollId = new BN(data.slice(1, 2), "le").toNumber();
        const optionSelected = new BN(data.slice(2, 3), "le").toNumber();
        const bump = new BN(data.slice(3, 4), "le").toNumber();
        const acc: PollVoter = { isInitialized, pollId, optionSelected, bump };
        return acc;
    }
    return null;
}

export const getPollAccount = (accountData: AccountInfo<Buffer> | null): Poll | null => {
    if (accountData != null) {
        const data = accountData.data;
        let size = 0;
        const isInitialized = new BN(data.slice(size, size + 1), "le").toNumber() === 0 ? false : true;
        size += 1;
        const id = new BN(data.slice(size, size + 1), "le").toNumber();
        size += 1;
        let title = data.slice(size, size + TITLE_LENGTH).toString();
        size += TITLE_LENGTH;
        const titleLength = new BN(data.slice(size, size + 1), "le").toNumber();
        title = title.substring(0, titleLength);
        size += 1;
        let optionsBuffer: Buffer = data.slice(size, size + OPTION_SIZE * OPTION_COUNT);
        size += OPTION_SIZE * OPTION_COUNT;
        const options_count = new BN(data.slice(size, size + 1), "le").toNumber();
        size += 1;
        const bump = new BN(data.slice(size, size + 1), "le").toNumber();


        let op_size = 0;
        let options: PollOption[] = [];

        for (let i = 0; i < options_count; i++) {
            const id = new BN(optionsBuffer.slice(op_size + 0, op_size + 1), "le").toNumber();
            let optionTitle = optionsBuffer.slice(op_size + 1, op_size + 51).toString();
            const optionTitleLength = new BN(optionsBuffer.slice(op_size + 51, op_size + 52), "le").toNumber();
            optionTitle = optionTitle.substring(0, optionTitleLength);
            const votes = new BN(optionsBuffer.slice(op_size + 52, op_size + 60), "le").toNumber();
            op_size += OPTION_SIZE;
            const option: PollOption = { id: id, title: optionTitle, titleLength: optionTitleLength, votes: votes };
            options.push(option);
        }

        const poll: Poll = { id: id, bump: bump, isInitialized: isInitialized, options: options, optionsLength: options_count, title: title, titleLength: titleLength };
        console.log(poll);
        return poll;
    }
    return null;
}


export function setAccountUpdateCallback(publicKey: PublicKey,
    callback: AccountChangeCallback,
    connection: Connection): void {
    connection.onAccountChange(publicKey, callback);
}
