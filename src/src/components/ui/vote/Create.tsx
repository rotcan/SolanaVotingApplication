import { Connection, PublicKey } from "@solana/web3.js";
import { Buffer } from 'buffer';

import { useEffect } from "react";
import { useFieldArray, useForm } from "react-hook-form";
import { getPda, getPdaAccount, getPollCountAccount } from "../../solana/accounts";
import { getPollCountSeedArray, getProvider, poll_seed } from "../../solana/solutil";
import { createProposal } from "../../solana/transaction";

const Create = ({ connection, programId, newPollId, setCreatePoll }: { connection: Connection, programId: PublicKey, newPollId: number, setCreatePoll: any }) => {

    type VoteForm = {
        title: string | null,
        options: string[] | null
    }
    const { register, control, handleSubmit, watch } = useForm();
    const { fields, append, remove } = useFieldArray({ name: "options", control });
    const onSubmit = async (data: VoteForm) => {
        const title = data.title;
        const options = data.options.map((val: any) => val.value);
        const pollSeeds = getPollCountSeedArray();
        const [countPda, bump] = await getPda(programId, pollSeeds);
        const countAcc = await getPollCountAccount(await getPdaAccount(connection, programId, pollSeeds));
        const [pollPda, bump2] = await getPda(programId, [Buffer.from(poll_seed), new Uint8Array([countAcc.count + 1])]);
        await createProposal(connection, getProvider(), countPda, pollPda, programId, title, options);
        setCreatePoll(false);
    }

    const optionCountWatch = watch("optionCount");

    useEffect(() => {
        const newVal = parseInt(optionCountWatch || 2);
        const oldVal = fields.length;
        if (newVal > oldVal) {
            for (let i = oldVal; i < newVal; i++)
                append("Option " + i);
        } else if (newVal < oldVal) {
            for (let i = oldVal; i > newVal; i--) {
                remove(i - 1);
            }
        }
    }, [optionCountWatch])

    return (<>
        <form onSubmit={handleSubmit(onSubmit)}>
            <div className="row">
                <div className="item">Proposal #: {(newPollId)}<div className="item">
                    <label>No. of Options</label>
                    <select name="optionCount" {...register("optionCount" as const)}>
                        {[2, 3, 4].map(i => <option key={i} value={i}>{i}</option>)}
                    </select>
                </div></div>
            </div>
            <div className="form-row">
                <div className="field">
                    <input name="title" {...register("title" as const)} type="text" className="form-control" placeholder="Title of proposal" />
                </div>
            </div>

            {fields.map((field, index) => (
                <div className="field" key={field.id}>
                    <input name="options" {...register(`options.${index}.value` as const)} type="text" className="form-control" placeholder={"Option " + (index + 1)} />
                </div>
            ))}
            <div className="form-submit"><input type="submit" value="Submit" /></div>

        </form>

    </>)
}

export default Create;