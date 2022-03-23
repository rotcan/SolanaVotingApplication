import { Connection, PublicKey, SystemProgram, Transaction, TransactionInstruction } from "@solana/web3.js";
import { PhantomProvider } from "./phantom";
import { Buffer } from 'buffer';

export const createPollInitInstruction = (feePayer: PublicKey, pollCountAccount: PublicKey, pollAccount: PublicKey, pid: PublicKey, instructionU8: Uint8Array) => {
    const t = new TransactionInstruction(
        {
            keys: [

                {
                    pubkey: pollCountAccount,
                    isSigner: false,
                    isWritable: true
                },
                {
                    pubkey: pollAccount,
                    isSigner: false,
                    isWritable: true
                },
                {
                    pubkey: SystemProgram.programId,
                    isSigner: false,
                    isWritable: false
                },
                {
                    pubkey: feePayer,
                    isSigner: false,
                    isWritable: false
                }
            ],
            //instruction
            //data: Buffer.from(new Uint8Array([0, group])),
            data: Buffer.from(instructionU8),
            programId: pid
        }
    );
    return t;
}

export const createPollInitInstructionData = (title_length: number, title: string, options_count: number, option_size: number[], options: string[]) => {
    //title len (u8) + title (title length)+ options_count (u8) + options_size (options_count) + options (opsize ..)
    console.log("title_length", title_length);
    console.log("title", title);
    console.log("options_count", options_count);
    console.log("option_size", option_size);
    console.log("options", options);
    var totalSize = 1 + 1 + title_length + 1 + options_count;
    for (let i = 0; i < option_size.length; i++) {
        totalSize += option_size[i];
    }

    console.log("totalSize", totalSize);
    var uarray = new Uint8Array(totalSize);
    let counter = 0;
    //instruction type
    uarray[counter++] = 0;
    uarray[counter++] = title_length;

    //set title
    var arr = Array.prototype.slice.call(Buffer.from(title), 0);
    console.log("arr", arr);
    for (let i = 0; i < arr.length; i++) {
        uarray[counter++] = arr[i];
    }

    //set options count
    uarray[counter++] = options_count;
    //set text size of  each option
    for (let i = 0; i < options_count; i++)
        uarray[counter++] = option_size[i];

    for (let i = 0; i < options_count; i++) {
        var arr2 = Array.prototype.slice.call(Buffer.from(options[i]), 0);
        for (let j = 0; j < arr2.length; j++) {
            uarray[counter++] = arr2[j];
        }
    }

    return uarray;

}


export const createPollVoteInstruction = (feePayer: PublicKey, pollAccount: PublicKey, voterAccount: PublicKey, pid: PublicKey, pollId: number, optionId: number) => {
    const t = new TransactionInstruction(
        {
            keys: [

                {
                    pubkey: pollAccount,
                    isSigner: false,
                    isWritable: true
                },
                {
                    pubkey: voterAccount,
                    isSigner: false,
                    isWritable: true
                },
                {
                    pubkey: feePayer,
                    isSigner: false,
                    isWritable: false
                },
                {
                    pubkey: SystemProgram.programId,
                    isSigner: false,
                    isWritable: false
                }
            ],
            //instruction
            data: Buffer.from(new Uint8Array([1, pollId, optionId])),
            //data: Buffer.from(instructionU8),
            programId: pid
        }
    );
    return t;
}

export const createProposal = async (connection: Connection, from: PhantomProvider, pollCountKey: PublicKey, pollKey: PublicKey, programId: PublicKey, title: string,
    options: string[]) => {
    let optionsSize: number[] = [];
    for (let i = 0; i < options.length; i++)
        optionsSize.push(options[i].length);
    const data = createPollInitInstructionData(title.length, title, options.length, optionsSize, options);
    const ix = createPollInitInstruction(from!.publicKey!, pollCountKey, pollKey, programId, data);
    let tx = new Transaction();
    tx.add(ix);
    tx.feePayer = await from!.publicKey!;
    let blockhashObj = await connection.getLatestBlockhash();
    tx.recentBlockhash = await blockhashObj.blockhash;

    // Transaction constructor initialized successfully
    if (tx) {
        console.log("Txn created successfully");
    }
    // Request creator to sign the transaction (allow the transaction)
    let signed = await from!.signTransaction(tx);
    // The signature is generated
    let signature = await connection.sendRawTransaction(signed.serialize());
    // Confirm whether the transaction went through or not

    const response = await connection.confirmTransaction(signature);
    console.log(response.value);
    return response
}

export const voteTransaction = async (connection: Connection, from: PhantomProvider, pollPublicKey: PublicKey, voterKey: PublicKey, programId: PublicKey, pollId: number, optionId: number) => {
    const transferIx = createPollVoteInstruction(from!.publicKey!, pollPublicKey, voterKey, programId, pollId, optionId);
    //signers.push(programAccountKey);
    let tx = new Transaction();
    tx.add(transferIx);
    // let signers = [from!]
    //const idx = Buffer.from(new Uint8Array([0]))

    // Setting the variables for the transaction
    tx.feePayer = await from!.publicKey!;
    let blockhashObj = await connection.getLatestBlockhash();
    tx.recentBlockhash = await blockhashObj.blockhash;

    // Transaction constructor initialized successfully
    if (tx) {
        console.log("Txn created successfully");
    }
    // Request creator to sign the transaction (allow the transaction)
    let signed = await from!.signTransaction(tx);
    // The signature is generated
    let signature = await connection.sendRawTransaction(signed.serialize());
    // Confirm whether the transaction went through or not

    const response = await connection.confirmTransaction(signature);
    console.log(response.value);
    return response
}

