This is a simple Voting Application created on Solana Blockchain

It uses Program Derived Address to store voting data. 

Any user can connect to this application and create a voting proposal (title max len is 100 characters) with options (max 4 with max len of 50 characters each)

It uses a u8 field to keep track of number of proposals created so at max 255 will be created.

Voter can vote on a proposal only once. For each proposal a new PDA is created for the voter.
