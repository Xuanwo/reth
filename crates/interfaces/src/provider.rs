use reth_primitives::{Address, BlockHash, BlockNumber, TransitionId, TxNumber, H256};

/// KV error type. They are using u32 to represent error code.
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error, PartialEq, Eq, Clone)]
pub enum Error {
    /// The header hash is missing from the database.
    #[error("Block number {block_number} does not exist in database")]
    CanonicalHeader { block_number: BlockNumber },
    /// A header body is missing from the database.
    #[error("No header for block #{number}")]
    Header {
        /// The block number key
        number: BlockNumber,
    },
    /// The header number was not found for the given block hash.
    #[error("Block hash {block_hash:?} does not exist in Headers table")]
    BlockHash { block_hash: BlockHash },
    /// A block body is missing.
    #[error("Block body not found for block #{number}")]
    BlockBody { number: BlockNumber },
    /// The block transition id for a certain block number is missing.
    #[error("Block transition id does not exist for block #{block_number}")]
    BlockTransition { block_number: BlockNumber },
    /// The transition id was found for the given address and storage key, but the changeset was
    /// not found.
    #[error("Storage ChangeSet address: ({address:?} key: {storage_key:?}) for transition:#{transition_id} does not exist")]
    StorageChangeset {
        /// The transition id found for the address and storage key
        transition_id: TransitionId,
        /// The account address
        address: Address,
        /// The storage key
        storage_key: H256,
    },
    /// The transition id was found for the given address, but the changeset was not found.
    #[error("Account {address:?} ChangeSet for transition #{transition_id} does not exist")]
    AccountChangeset {
        /// Transition id found for the address
        transition_id: TransitionId,
        /// The account address
        address: Address,
    },
    /// The total difficulty for a block is missing.
    #[error("Total difficulty not found for block #{number}")]
    TotalDifficulty { number: BlockNumber },
    /// The transaction is missing
    #[error("Transaction #{id} not found")]
    Transaction {
        /// The transaction id
        id: TxNumber,
    },
    /// A ommers are missing.
    #[error("Block ommers not found for block #{number}")]
    Ommers {
        /// The block number key
        number: BlockNumber,
    },
    /// There is a gap in the transaction table, at a missing transaction number.
    #[error("Gap in transaction table. Missing tx number #{missing}.")]
    TransactionsGap { missing: TxNumber },
    /// There is a gap in the senders table, at a missing transaction number.
    #[error("Gap in transaction signer table. Missing tx number #{missing}.")]
    TransactionsSignerGap { missing: TxNumber },
    /// Reached the end of the transaction table.
    #[error("Got to the end of transaction table")]
    EndOfTransactionTable,
    /// Reached the end of the transaction sender table.
    #[error("Got to the end of the transaction sender table")]
    EndOfTransactionSenderTable,
}
