use alloy::{
    dyn_abi::DynSolEvent,
    primitives::{Address, U256},
    rpc::types::Log,
};

#[derive(Debug, Clone)]
pub struct Erc20Transfer {
    pub from: Address,
    pub to: Address,
    pub amount: U256,
}

impl Erc20Transfer {
    pub fn from_log(log: &Log) -> Option<Self> {
        let transfer_signature = alloy::primitives::FixedBytes::<32>::from([
            0xdd, 0xf2, 0x52, 0xad, 0x1b, 0xe2, 0xc8, 0x9b, 0x69, 0xc2, 0xb0, 0x68, 0xfc, 0x37,
            0x8d, 0xaa, 0x95, 0x2b, 0xa7, 0xf1, 0x63, 0xc4, 0xa1, 0x16, 0x28, 0xf5, 0x5a, 0x4d,
            0xf5, 0x23, 0xb3, 0xef,
        ]); // keccak256("Transfer(address,address,uint256)") in other words - hash of the event signature

        if log.topics().is_empty() || log.topics()[0] != transfer_signature {
            return None;
        }

        // For ERC-20 Transfer events:
        // topics[1] = from address (indexed)
        // topics[2] = to address (indexed)
        // data = amount (uint256)
        if log.topics().len() < 3 {
            return None;
        }

        let from = Address::from_slice(&log.topics()[1][12..]);
        let to = Address::from_slice(&log.topics()[2][12..]);

        if log.inner.data.data.len() < 32 {
            return None;
        }
        let amount_bytes: [u8; 32] = log.inner.data.data[..32].try_into().ok()?;
        let amount = U256::from_be_bytes(amount_bytes);

        Some(Erc20Transfer { from, to, amount })
    }
}
