import { Connection, PublicKey } from "@solana/web3.js";
import { useState } from "react";
import View from "./View";

const List = ({ connection, programId, pollId }: { connection: Connection, programId: PublicKey, pollId: number }) => {
    const [currentProposal, setCurrentProposal] = useState<number | null>();

    return <>
        <span>Existing Proposals</span>

        {(pollId > 0 && (
            <ul>
                {
                    Array.from(Array(pollId).keys()).map((val) => {
                        return <li key={val}>
                            <button onClick={() => { if (currentProposal === val + 1) setCurrentProposal(null); else setCurrentProposal(val + 1); }}>{(currentProposal !== null && currentProposal === val + 1 && "Hide Proposal") || "View Proposal"} : {val + 1}</button>
                            {currentProposal !== null && currentProposal === val + 1 && <View connection={connection} programId={programId} pollId={val + 1} />}
                        </li>
                    })
                }
            </ul>
        ))
            || <span>No proposals till now</span>}
    </>
}

export default List;