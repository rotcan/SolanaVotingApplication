import { PublicKey } from '@solana/web3.js';
import React, { useState } from 'react';
import './App.css';
import { programId } from './components/solana/program';
import { connection } from './components/solana/solutil';
import Airdrop from './components/ui/Airdrop';
import Vote from './components/ui/Vote';
import Wallet from './components/ui/Wallet';

function App() {

  const [walletKey, setWalletKey] = useState<PublicKey | null>();
  return (
    <div className="App">
      <header className="App-header">
        <Wallet setWalletKey={setWalletKey} walletKey={walletKey}></Wallet>
      </header>
      <div>
        {walletKey && <Airdrop connection={connection} publicKey={walletKey} />}
      </div>
      <div className='text-left padding10'>
        {walletKey && <Vote connection={connection} programId={programId} />}

      </div>
    </div>
  );
}

export default App;
