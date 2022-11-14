use strum_macros::{EnumString, Display, EnumIter};

#[allow(non_camel_case_types)]
#[derive(EnumString, Debug, Display, EnumIter)]
pub enum Methods {
    // * Gossip Methods
    eth_blockNumber,
    eth_sendRawTransaction, // ?

    // * State Methods
    eth_getBalance,
    eth_getStorageAt,
    eth_getTranscationCount,
    eth_getCode,
    eth_call,
    eth_estimateGas,

    //* History Methods
    eth_getBlockTransactionCountByHash,
    eth_getBlockTransactionCountByNumber,
    eth_getUncleCountByBlockHash,
    eth_getUncleCountByBlockNumber,
    eth_getBlockByHash,
    eth_getBlockByNumber,
    eth_getTransactionByHash,
    eth_getTransactionByBlockHashAndIndex,
    eth_getTransactionByBlockNumberAndIndex,
    eth_getTransactionReceipt,
    eth_getUncleByBlockHashAndIndex,
    eth_getUncleByBlockNumberAndIndex,

    //* Other
    web3_clientVersion,
    eth_syncing,
    eth_coinbase,
    eth_accounts, // deprecated?
    eth_gasPrice,
}
