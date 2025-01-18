/*
Deployment of Smart Contract to Solana Devnet Environment:
    -- Download and install WSL if on Windows Environment
    -- Install Solana Client:
        sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"
    -- Connect to Solana Devnet 
        solana config set --url https://api.devnet.solana.com
    -- Navigate to project folder and build
        cargo build-sbf
    -- Deploy program to Devnet
        solana program deploy ./target/deploy/solana_guessing_game.so
    -- Check deployed programs on Solana Devnet:
        solana program show --programs
    -- Once deployed, you'll receive a program address. This address is used to interact with your smart contract on the Devnet.

