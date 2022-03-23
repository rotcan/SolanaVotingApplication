import { Connection, PublicKey } from "@solana/web3.js";
import { useEffect, useState } from "react";
import { getPda, getPdaAccount, getPollAccount, getVoterPollAccount, setAccountUpdateCallback } from "../../solana/accounts";
import { getProvider, Poll, PollVoter, poll_seed } from "../../solana/solutil";
import { Buffer } from 'buffer';

import { voteTransaction } from "../../solana/transaction";

const View = ({ connection, programId, pollId }: { connection: Connection, programId: PublicKey, pollId: number }) => {

    const [poll, setPoll] = useState<Poll | null | undefined>();
    const [userVote, setUserVote] = useState<PollVoter | null | undefined>();

    const loadPoll = async () => {
        const seeds: Uint8Array[] = [Buffer.from(poll_seed), new Uint8Array([pollId])];
        const pda = await getPdaAccount(connection, programId, seeds);
        const acc = getPollAccount(pda);
        setPoll(acc);
    };

    const loadUserVote = async () => {
        const wallet = getProvider();
        const [poll_pda, bump] = await getPda(programId, [Buffer.from(poll_seed), new Uint8Array([pollId])]);
        const seeds: Uint8Array[] = [Buffer.from(poll_seed), new Uint8Array([pollId, bump]),
        wallet!.publicKey!.toBuffer()];
        const voterPdaAccount = await getPdaAccount(connection, programId, seeds);
        const acc = getVoterPollAccount(voterPdaAccount);
        if (acc !== null)
            setUserVote(acc);
    }

    const voteOption = async (optionId: number) => {
        const wallet = getProvider();
        const [poll_pda, bump] = await getPda(programId, [Buffer.from(poll_seed), new Uint8Array([pollId])]);
        const seeds: Uint8Array[] = [Buffer.from(poll_seed), new Uint8Array([pollId, bump]),
        wallet!.publicKey!.toBuffer()];
        const [voterPdaAccount, vBump] = await getPda(programId, seeds);
        await voteTransaction(connection, wallet!, poll_pda, voterPdaAccount, programId, pollId, optionId);
    }

    const pollAccountChangeCallback = async () => {
        await loadPoll();

    }


    const voteAccountChangeCallback = async () => {
        await loadUserVote();
    }


    const initCallback = async () => {
        const wallet = getProvider();
        const [poll_pda, bump] = await getPda(programId, [Buffer.from(poll_seed), new Uint8Array([pollId])]);
        const seeds: Uint8Array[] = [Buffer.from(poll_seed), new Uint8Array([pollId, bump]),
        wallet!.publicKey!.toBuffer()];
        const [voterPdaAccount, vBump] = await getPda(programId, seeds);
        setAccountUpdateCallback(voterPdaAccount, voteAccountChangeCallback, connection);
        setAccountUpdateCallback(poll_pda, pollAccountChangeCallback, connection);
    }

    useEffect(() => {
        loadPoll();
        loadUserVote();
        initCallback();
    }, [])


    return <> <div className="vote-view text-left">
        {
            (poll && poll?.id && (
                <>
                    <div>
                        Proposal Title: {poll.title}
                    </div>
                    <ul >{poll?.options.map((val) => {
                        return (<>
                            <li className="padding10" key={val.id}> ({val.id + 1}) {val.title} / {val.votes} vote(s)
                                {!userVote && <button onClick={() => { voteOption(val.id + 1) }}>Vote</button>}
                                {userVote && userVote.optionSelected === (val.id + 1) && <span>. You voted for this option!</span>}
                            </li>
                        </>)
                    })}
                    </ul>
                </>
            )
            )
            || "No proposal found"
        }
    </div></>;

}

export default View;