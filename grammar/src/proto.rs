#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    #[prost(enumeration = "message::MessageType", tag = "1")]
    pub message_type: i32,
    /// The identifier used to correlate response messages to their related
    /// request messages.  correlation_id should be set to a random string
    /// for messages which are not responses to previously sent messages.  For
    /// response messages, correlation_id should be set to the same string as
    /// contained in the request message.
    #[prost(string, tag = "2")]
    pub correlation_id: std::string::String,
    /// The content of the message, defined by message_type.  In many
    /// cases, this data has been serialized with Protocol Buffers
    #[prost(bytes, tag = "3")]
    pub content: std::vec::Vec<u8>,
}
pub mod message {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MessageType {
        Default = 0,
        /// Component message
        ComponentContractRegister = 100,
        ComponentConsensusRegister = 101,
        ComponentUnregister = 102,
        /// Peer message
        PeerHandshakeRequest = 200,
        PeerHandshakeResponse = 201,
        PeerHeartbeatRequest = 202,
        PeerHeartbeatResponse = 203,
        PeerBroadcastTransaction = 204,
        PeerBroadcastBlockNumber = 205,
        /// pull blocks from other peer nodes
        PeerDeliverBlock = 206,
        /// Consensus notification messages
        ConsensusTransactionArrived = 300,
        ConsensusNotifyBlockCommit = 301,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComponentContractRegister {
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    #[prost(map = "string, bytes", tag = "2")]
    pub decorations: ::std::collections::HashMap<std::string::String, std::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComponentConsensusRegister {
    #[prost(string, tag = "1")]
    pub alg: std::string::String,
    #[prost(map = "string, bytes", tag = "2")]
    pub decorations: ::std::collections::HashMap<std::string::String, std::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComponentUnregister {}
