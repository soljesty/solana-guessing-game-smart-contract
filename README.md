/*
This is a smart contract developed to process transactions to use and interact with a developed POC guessing game.

Security analysis was done using Auditware / radar and other tools

Rust contract has been deployed on DevNet blockchain for testing prior to deployment on Solana PROD

Smart Contract Size:
root@ubuntu:~/solana-smart-contract/target/deploy# size solana_smart_contract.so
   text    data     bss     dec     hex filename
  64102    1576       0   65678   1008e solana_smart_contract.so

Smart Contract Rent-Exempt Fees:
root@ubuntu:~/solana-smart-contract/target/deploy# solana rent 65678
Rent-exempt minimum: 0.45800976 SOL

*/

![Alt Text](/src/DevNet_Smart_Contract.png)
