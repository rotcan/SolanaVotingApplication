import { Connection, PublicKey } from "@solana/web3.js";
import { useEffect, useState } from "react";
import { getPda, getPdaAccount, getPollCountAccount, setAccountUpdateCallback } from "../solana/accounts";
import { getPollCountSeedArray, PollCount } from "../solana/solutil";
import Create from "./vote/Create";
import List from "./vote/List";

const Vote = ({ connection, programId }: { connection: Connection, programId: PublicKey }) => {
    const [createPoll, setCreatePoll] = useState<Boolean>(false);
    const [pollCount, setPollCount] = useState<PollCount | null>();

    const loadPollCountAccount = async () => {
        const pollSeeds = getPollCountSeedArray();
        const pda = await getPdaAccount(connection, programId, pollSeeds);
        const acc = getPollCountAccount(pda);
        setPollCount(acc);
    }

    const accountChangeCallback = async () => {
        await loadPollCountAccount();
    }

    const initCallback = async () => {
        setAccountUpdateCallback((await getPda(programId, getPollCountSeedArray()))[0], accountChangeCallback, connection);
    }

    useEffect(() => {
        loadPollCountAccount();
        initCallback();

    }, [])

    return (
        <>
            <div>
                <div className="padding10">
                    <button onClick={() => { setCreatePoll(true) }}>Create New Poll</button>
                    {createPoll && <Create connection={connection} programId={programId} newPollId={(pollCount && pollCount.count + 1) || 1} setCreatePoll={setCreatePoll} />}
                </div>
                <div className="padding10">
                    <List connection={connection} programId={programId} pollId={(pollCount && pollCount.count) || 0} />
                </div>

            </div>
        </>
    );
}

export default Vote;