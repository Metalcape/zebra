//! History node test vectors

#![allow(missing_docs)]

use lazy_static::lazy_static;

use std::collections::BTreeMap;

lazy_static! {
    pub static ref MAINNET_HISTORY_NODES: BTreeMap<String, Vec<&'static[u8]>> = [
        ("heartwood".to_owned(), Vec::from([
            MAINNET_HEARTWOOD_NODE_0.as_ref(),
            MAINNET_HEARTWOOD_NODE_1.as_ref(),
            MAINNET_HEARTWOOD_NODE_2.as_ref(),
            MAINNET_HEARTWOOD_NODE_3.as_ref(),
            MAINNET_HEARTWOOD_NODE_4.as_ref(),
            MAINNET_HEARTWOOD_NODE_5.as_ref(),
            MAINNET_HEARTWOOD_NODE_6.as_ref(),
            MAINNET_HEARTWOOD_NODE_7.as_ref(),
            MAINNET_HEARTWOOD_NODE_8.as_ref(),
            MAINNET_HEARTWOOD_NODE_9.as_ref(),
        ])),

        ("canopy".to_owned(), Vec::from([
            MAINNET_CANOPY_NODE_0.as_ref(),
            MAINNET_CANOPY_NODE_1.as_ref(),
            MAINNET_CANOPY_NODE_2.as_ref(),
            MAINNET_CANOPY_NODE_3.as_ref(),
            MAINNET_CANOPY_NODE_4.as_ref(),
            MAINNET_CANOPY_NODE_5.as_ref(),
            MAINNET_CANOPY_NODE_6.as_ref(),
            MAINNET_CANOPY_NODE_7.as_ref(),
            MAINNET_CANOPY_NODE_8.as_ref(),
            MAINNET_CANOPY_NODE_9.as_ref(),
        ])),

        ("nu5".to_owned(), Vec::from([
            MAINNET_NU5_NODE_0.as_ref(),
            MAINNET_NU5_NODE_1.as_ref(),
            MAINNET_NU5_NODE_2.as_ref(),
            MAINNET_NU5_NODE_3.as_ref(),
            MAINNET_NU5_NODE_4.as_ref(),
            MAINNET_NU5_NODE_5.as_ref(),
            MAINNET_NU5_NODE_6.as_ref(),
            MAINNET_NU5_NODE_7.as_ref(),
            MAINNET_NU5_NODE_8.as_ref(),
            MAINNET_NU5_NODE_9.as_ref(),
        ])),
    ].iter().cloned().collect();

    pub static ref TESTNET_HISTORY_NODES: BTreeMap<String, Vec<&'static[u8]>> = [
        ("heartwood".to_owned(), Vec::from([
            TESTNET_HEARTWOOD_NODE_0.as_ref(),
            TESTNET_HEARTWOOD_NODE_1.as_ref(),
            TESTNET_HEARTWOOD_NODE_2.as_ref(),
            TESTNET_HEARTWOOD_NODE_3.as_ref(),
            TESTNET_HEARTWOOD_NODE_4.as_ref(),
            TESTNET_HEARTWOOD_NODE_5.as_ref(),
            TESTNET_HEARTWOOD_NODE_6.as_ref(),
            TESTNET_HEARTWOOD_NODE_7.as_ref(),
            TESTNET_HEARTWOOD_NODE_8.as_ref(),
            TESTNET_HEARTWOOD_NODE_9.as_ref(),
        ])),

        ("canopy".to_owned(), Vec::from([
            TESTNET_CANOPY_NODE_0.as_ref(),
            TESTNET_CANOPY_NODE_1.as_ref(),
            TESTNET_CANOPY_NODE_2.as_ref(),
            TESTNET_CANOPY_NODE_3.as_ref(),
            TESTNET_CANOPY_NODE_4.as_ref(),
            TESTNET_CANOPY_NODE_5.as_ref(),
            TESTNET_CANOPY_NODE_6.as_ref(),
            TESTNET_CANOPY_NODE_7.as_ref(),
            TESTNET_CANOPY_NODE_8.as_ref(),
            TESTNET_CANOPY_NODE_9.as_ref(),
        ])),

        ("nu5".to_owned(), Vec::from([
            TESTNET_NU5_NODE_0.as_ref(),
            TESTNET_NU5_NODE_1.as_ref(),
            TESTNET_NU5_NODE_2.as_ref(),
            TESTNET_NU5_NODE_3.as_ref(),
            TESTNET_NU5_NODE_4.as_ref(),
            TESTNET_NU5_NODE_5.as_ref(),
            TESTNET_NU5_NODE_6.as_ref(),
            TESTNET_NU5_NODE_7.as_ref(),
            TESTNET_NU5_NODE_8.as_ref(),
            TESTNET_NU5_NODE_9.as_ref(),
        ])),
    ].iter().cloned().collect();

    // Mainnet

    // Heartwood
    pub static ref MAINNET_HEARTWOOD_NODE_0: Vec<u8> =
        hex::decode(include_str!("history-main-heartwood-0.txt").trim()).expect("history node is in valid hex representation");
    pub static ref MAINNET_HEARTWOOD_NODE_1: Vec<u8> =
        hex::decode(include_str!("history-main-heartwood-1.txt").trim()).expect("history node is in valid hex representation");
    pub static ref MAINNET_HEARTWOOD_NODE_2: Vec<u8> =
        hex::decode(include_str!("history-main-heartwood-2.txt").trim()).expect("history node is in valid hex representation");
    pub static ref MAINNET_HEARTWOOD_NODE_3: Vec<u8> =
        hex::decode(include_str!("history-main-heartwood-3.txt").trim()).expect("history node is in valid hex representation");
    pub static ref MAINNET_HEARTWOOD_NODE_4: Vec<u8> =
        hex::decode(include_str!("history-main-heartwood-4.txt").trim()).expect("history node is in valid hex representation");
    pub static ref MAINNET_HEARTWOOD_NODE_5: Vec<u8> =
        hex::decode(include_str!("history-main-heartwood-5.txt").trim()).expect("history node is in valid hex representation");
    pub static ref MAINNET_HEARTWOOD_NODE_6: Vec<u8> =
        hex::decode(include_str!("history-main-heartwood-6.txt").trim()).expect("history node is in valid hex representation");
    pub static ref MAINNET_HEARTWOOD_NODE_7: Vec<u8> =
        hex::decode(include_str!("history-main-heartwood-7.txt").trim()).expect("history node is in valid hex representation");
    pub static ref MAINNET_HEARTWOOD_NODE_8: Vec<u8> =
        hex::decode(include_str!("history-main-heartwood-8.txt").trim()).expect("history node is in valid hex representation");
    pub static ref MAINNET_HEARTWOOD_NODE_9: Vec<u8> =
        hex::decode(include_str!("history-main-heartwood-9.txt").trim()).expect("history node is in valid hex representation");

    // Canopy
    pub static ref MAINNET_CANOPY_NODE_0: Vec<u8> =
        hex::decode(include_str!("history-main-canopy-0.txt").trim()).expect("history node is in valid hex representation");
    pub static ref MAINNET_CANOPY_NODE_1: Vec<u8> =
        hex::decode(include_str!("history-main-canopy-1.txt").trim()).expect("history node is in valid hex representation");
    pub static ref MAINNET_CANOPY_NODE_2: Vec<u8> =
        hex::decode(include_str!("history-main-canopy-2.txt").trim()).expect("history node is in valid hex representation");
    pub static ref MAINNET_CANOPY_NODE_3: Vec<u8> =
        hex::decode(include_str!("history-main-canopy-3.txt").trim()).expect("history node is in valid hex representation");
    pub static ref MAINNET_CANOPY_NODE_4: Vec<u8> =
        hex::decode(include_str!("history-main-canopy-4.txt").trim()).expect("history node is in valid hex representation");
    pub static ref MAINNET_CANOPY_NODE_5: Vec<u8> =
        hex::decode(include_str!("history-main-canopy-5.txt").trim()).expect("history node is in valid hex representation");
    pub static ref MAINNET_CANOPY_NODE_6: Vec<u8> =
        hex::decode(include_str!("history-main-canopy-6.txt").trim()).expect("history node is in valid hex representation");
    pub static ref MAINNET_CANOPY_NODE_7: Vec<u8> =
        hex::decode(include_str!("history-main-canopy-7.txt").trim()).expect("history node is in valid hex representation");
    pub static ref MAINNET_CANOPY_NODE_8: Vec<u8> =
        hex::decode(include_str!("history-main-canopy-8.txt").trim()).expect("history node is in valid hex representation");
    pub static ref MAINNET_CANOPY_NODE_9: Vec<u8> =
        hex::decode(include_str!("history-main-canopy-9.txt").trim()).expect("history node is in valid hex representation");

    // NU5
    pub static ref MAINNET_NU5_NODE_0: Vec<u8> =
        hex::decode(include_str!("history-main-nu5-0.txt").trim()).expect("history node is in valid hex representation");
    pub static ref MAINNET_NU5_NODE_1: Vec<u8> =
        hex::decode(include_str!("history-main-nu5-1.txt").trim()).expect("history node is in valid hex representation");
    pub static ref MAINNET_NU5_NODE_2: Vec<u8> =
        hex::decode(include_str!("history-main-nu5-2.txt").trim()).expect("history node is in valid hex representation");
    pub static ref MAINNET_NU5_NODE_3: Vec<u8> =
        hex::decode(include_str!("history-main-nu5-3.txt").trim()).expect("history node is in valid hex representation");
    pub static ref MAINNET_NU5_NODE_4: Vec<u8> =
        hex::decode(include_str!("history-main-nu5-4.txt").trim()).expect("history node is in valid hex representation");
    pub static ref MAINNET_NU5_NODE_5: Vec<u8> =
        hex::decode(include_str!("history-main-nu5-5.txt").trim()).expect("history node is in valid hex representation");
    pub static ref MAINNET_NU5_NODE_6: Vec<u8> =
        hex::decode(include_str!("history-main-nu5-6.txt").trim()).expect("history node is in valid hex representation");
    pub static ref MAINNET_NU5_NODE_7: Vec<u8> =
        hex::decode(include_str!("history-main-nu5-7.txt").trim()).expect("history node is in valid hex representation");
    pub static ref MAINNET_NU5_NODE_8: Vec<u8> =
        hex::decode(include_str!("history-main-nu5-8.txt").trim()).expect("history node is in valid hex representation");
    pub static ref MAINNET_NU5_NODE_9: Vec<u8> =
        hex::decode(include_str!("history-main-nu5-9.txt").trim()).expect("history node is in valid hex representation");

    // Testnet

    // Heartwood
    pub static ref TESTNET_HEARTWOOD_NODE_0: Vec<u8> =
        hex::decode(include_str!("history-test-heartwood-0.txt").trim()).expect("history node is in valid hex representation");
    pub static ref TESTNET_HEARTWOOD_NODE_1: Vec<u8> =
        hex::decode(include_str!("history-test-heartwood-1.txt").trim()).expect("history node is in valid hex representation");
    pub static ref TESTNET_HEARTWOOD_NODE_2: Vec<u8> =
        hex::decode(include_str!("history-test-heartwood-2.txt").trim()).expect("history node is in valid hex representation");
    pub static ref TESTNET_HEARTWOOD_NODE_3: Vec<u8> =
        hex::decode(include_str!("history-test-heartwood-3.txt").trim()).expect("history node is in valid hex representation");
    pub static ref TESTNET_HEARTWOOD_NODE_4: Vec<u8> =
        hex::decode(include_str!("history-test-heartwood-4.txt").trim()).expect("history node is in valid hex representation");
    pub static ref TESTNET_HEARTWOOD_NODE_5: Vec<u8> =
        hex::decode(include_str!("history-test-heartwood-5.txt").trim()).expect("history node is in valid hex representation");
    pub static ref TESTNET_HEARTWOOD_NODE_6: Vec<u8> =
        hex::decode(include_str!("history-test-heartwood-6.txt").trim()).expect("history node is in valid hex representation");
    pub static ref TESTNET_HEARTWOOD_NODE_7: Vec<u8> =
        hex::decode(include_str!("history-test-heartwood-7.txt").trim()).expect("history node is in valid hex representation");
    pub static ref TESTNET_HEARTWOOD_NODE_8: Vec<u8> =
        hex::decode(include_str!("history-test-heartwood-8.txt").trim()).expect("history node is in valid hex representation");
    pub static ref TESTNET_HEARTWOOD_NODE_9: Vec<u8> =
        hex::decode(include_str!("history-test-heartwood-9.txt").trim()).expect("history node is in valid hex representation");

    // Canopy
    pub static ref TESTNET_CANOPY_NODE_0: Vec<u8> =
        hex::decode(include_str!("history-test-canopy-0.txt").trim()).expect("history node is in valid hex representation");
    pub static ref TESTNET_CANOPY_NODE_1: Vec<u8> =
        hex::decode(include_str!("history-test-canopy-1.txt").trim()).expect("history node is in valid hex representation");
    pub static ref TESTNET_CANOPY_NODE_2: Vec<u8> =
        hex::decode(include_str!("history-test-canopy-2.txt").trim()).expect("history node is in valid hex representation");
    pub static ref TESTNET_CANOPY_NODE_3: Vec<u8> =
        hex::decode(include_str!("history-test-canopy-3.txt").trim()).expect("history node is in valid hex representation");
    pub static ref TESTNET_CANOPY_NODE_4: Vec<u8> =
        hex::decode(include_str!("history-test-canopy-4.txt").trim()).expect("history node is in valid hex representation");
    pub static ref TESTNET_CANOPY_NODE_5: Vec<u8> =
        hex::decode(include_str!("history-test-canopy-5.txt").trim()).expect("history node is in valid hex representation");
    pub static ref TESTNET_CANOPY_NODE_6: Vec<u8> =
        hex::decode(include_str!("history-test-canopy-6.txt").trim()).expect("history node is in valid hex representation");
    pub static ref TESTNET_CANOPY_NODE_7: Vec<u8> =
        hex::decode(include_str!("history-test-canopy-7.txt").trim()).expect("history node is in valid hex representation");
    pub static ref TESTNET_CANOPY_NODE_8: Vec<u8> =
        hex::decode(include_str!("history-test-canopy-8.txt").trim()).expect("history node is in valid hex representation");
    pub static ref TESTNET_CANOPY_NODE_9: Vec<u8> =
        hex::decode(include_str!("history-test-canopy-9.txt").trim()).expect("history node is in valid hex representation");

    // NU5
    pub static ref TESTNET_NU5_NODE_0: Vec<u8> =
        hex::decode(include_str!("history-test-nu5-0.txt").trim()).expect("history node is in valid hex representation");
    pub static ref TESTNET_NU5_NODE_1: Vec<u8> =
        hex::decode(include_str!("history-test-nu5-1.txt").trim()).expect("history node is in valid hex representation");
    pub static ref TESTNET_NU5_NODE_2: Vec<u8> =
        hex::decode(include_str!("history-test-nu5-2.txt").trim()).expect("history node is in valid hex representation");
    pub static ref TESTNET_NU5_NODE_3: Vec<u8> =
        hex::decode(include_str!("history-test-nu5-3.txt").trim()).expect("history node is in valid hex representation");
    pub static ref TESTNET_NU5_NODE_4: Vec<u8> =
        hex::decode(include_str!("history-test-nu5-4.txt").trim()).expect("history node is in valid hex representation");
    pub static ref TESTNET_NU5_NODE_5: Vec<u8> =
        hex::decode(include_str!("history-test-nu5-5.txt").trim()).expect("history node is in valid hex representation");
    pub static ref TESTNET_NU5_NODE_6: Vec<u8> =
        hex::decode(include_str!("history-test-nu5-6.txt").trim()).expect("history node is in valid hex representation");
    pub static ref TESTNET_NU5_NODE_7: Vec<u8> =
        hex::decode(include_str!("history-test-nu5-7.txt").trim()).expect("history node is in valid hex representation");
    pub static ref TESTNET_NU5_NODE_8: Vec<u8> =
        hex::decode(include_str!("history-test-nu5-8.txt").trim()).expect("history node is in valid hex representation");
    pub static ref TESTNET_NU5_NODE_9: Vec<u8> =
        hex::decode(include_str!("history-test-nu5-9.txt").trim()).expect("history node is in valid hex representation");
}

#[cfg(test)]
mod test {
    use itertools::Itertools;

    use super::*;

    use std::collections::HashSet;

    use crate::init;

    #[test]
    fn history_node_test_vectors_unique() {
        let _init_guard = init();

        let mut all_history_nodes = MAINNET_HISTORY_NODES
            .iter()
            .map(|(_upgrade, nodes)| nodes.clone())
            .concat();
        let mut test_history_nodes = TESTNET_HISTORY_NODES
            .iter()
            .map(|(_upgrade, nodes)| nodes.clone())
            .concat();
        all_history_nodes.append(&mut test_history_nodes);
        let node_count = all_history_nodes.len();
        let node_set: HashSet<_> = all_history_nodes.iter().collect();

        assert_eq!(
            node_count,
            node_set.len(),
            "history node test vectors must be unique"
        );
    }
}
