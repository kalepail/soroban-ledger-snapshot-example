1. Install `stellar-cli >=v21.3.0`  
`cargo install --locked stellar-cli --features opt`
2. Create 2 G address accounts and fund them on the Testnet  
![]()
3. Generate a ledger snapshot containing both of those G addresses and the native XLM Stellar Asset Contract (SAC) address (Please note snapshots are taken from checkpoints which occur every 5 minutes, so you'll need to wait up to 5 minutes before your 2 G addresses will be included in the snapshot)  
    ```bash
    soroban snapshot create \
    --network testnet \
    --address CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC \
    --address GBTFJPSA4GC7WLQIOFQ35345LJTCRZPD4ZLFCDEWVZ7WY4MVM7SCHIEQ \
    --address GBYEUXTQY4S7XN6O3RIELIL25AMWMTBGLEDZLHL5UHDDNEY7LK5EJTL2 \
    --output json \
    --out snapshot.json
    ```