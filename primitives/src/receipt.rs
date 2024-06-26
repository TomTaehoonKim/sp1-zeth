// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use alloy_primitives::{Address, Bloom, BloomInput, Bytes, TxNumber, B256, U256};
use alloy_rlp::Encodable;
use alloy_rlp_derive::RlpEncodable;
use serde::{Deserialize, Serialize};

/// Version of the deposit nonce field in the receipt.
pub const OPTIMISM_DEPOSIT_NONCE_VERSION: u32 = 1;

/// Represents an Ethereum log entry.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, RlpEncodable)]
pub struct Log {
    /// Contract that emitted this log.
    pub address: Address,
    /// Topics of the log. The number of logs depend on what `LOG` opcode is used.
    pub topics: Vec<B256>,
    /// Arbitrary length data.
    pub data: Bytes,
}

/// Payload of a [Receipt].
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, RlpEncodable)]
#[rlp(trailing)]
pub struct ReceiptPayload {
    /// Indicates whether the transaction was executed successfully.
    pub success: bool,
    /// Total gas used by the transaction.
    pub cumulative_gas_used: U256,
    /// A bloom filter that contains indexed information of logs for quick searching.
    pub logs_bloom: Bloom,
    /// Logs generated during the execution of the transaction.
    pub logs: Vec<Log>,
    /// Nonce of the Optimism deposit transaction persisted during execution.
    #[serde(default)]
    pub deposit_nonce: Option<TxNumber>,
    /// Version of the deposit nonce field in the receipt.
    #[serde(default)]
    pub deposit_nonce_version: Option<u32>,
}

/// Receipt containing result of transaction execution.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Receipt {
    /// Type of Receipt.
    pub tx_type: u8,
    /// Detailed payload of the receipt.
    pub payload: ReceiptPayload,
}

impl Encodable for Receipt {
    /// Encodes the receipt into the `out` buffer.
    #[inline]
    fn encode(&self, out: &mut dyn alloy_rlp::BufMut) {
        match self.tx_type {
            // For legacy transactions
            0 => self.payload.encode(out),
            // For EIP-2718 typed transactions
            tx_type => {
                // prepend the EIP-2718 transaction type
                out.put_u8(tx_type);
                // append the RLP-encoded payload
                self.payload.encode(out);
            }
        }
    }

    /// Returns the length of the encoded receipt in bytes.
    #[inline]
    fn length(&self) -> usize {
        let mut payload_length = self.payload.length();
        if self.tx_type != 0 {
            payload_length += 1;
        }
        payload_length
    }
}

impl Receipt {
    /// Constructs a new [Receipt].
    pub fn new(tx_type: u8, success: bool, cumulative_gas_used: U256, logs: Vec<Log>) -> Receipt {
        let mut logs_bloom = Bloom::default();
        for log in &logs {
            logs_bloom.accrue(BloomInput::Raw(log.address.as_slice()));
            for topic in &log.topics {
                logs_bloom.accrue(BloomInput::Raw(topic.as_slice()));
            }
        }

        Receipt {
            tx_type,
            payload: ReceiptPayload {
                success,
                cumulative_gas_used,
                logs_bloom,
                logs,
                deposit_nonce: None,
                deposit_nonce_version: None,
            },
        }
    }
    /// Adds a deposit nonce to the receipt.
    pub fn with_deposit_nonce(mut self, deposit_nonce: TxNumber) -> Self {
        self.payload.deposit_nonce = Some(deposit_nonce);
        self.payload.deposit_nonce_version = Some(OPTIMISM_DEPOSIT_NONCE_VERSION);
        self
    }
}
