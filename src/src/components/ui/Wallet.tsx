import { PublicKey } from "@solana/web3.js";
import { PhantomProvider } from "../solana/phantom";

const Wallet = ({ walletKey, setWalletKey }: { walletKey: PublicKey | null | undefined, setWalletKey: any }) => {




    const connectWallet = async () => {
        // @ts-ignore
        const { solana } = window;
        if (solana) {
            try {
                const response = await solana.connect();
                console.log('wallet account ', response.publicKey);
                setWalletKey(response.publicKey);
            } catch (err) {
                // { code: 4001, message: 'User rejected the request.' }
            }
        }
    };

    const disconnectWallet = async () => {
        // @ts-ignore
        const { solana } = window;

        if (walletKey && solana) {
            await (solana as PhantomProvider).disconnect();
            setWalletKey(undefined);
        }
    };

    return (
        <>
            <div className="right">{
                walletKey &&
                (
                    <><span className="address padding10">{walletKey.toBase58()}</span>
                        <button onClick={() => { disconnectWallet() }}>Disconnect</button></>

                )
            }
                {!walletKey && (<button onClick={() => { connectWallet() }}>Connect</button>)}</div>

        </>
    )

}


export default Wallet;
