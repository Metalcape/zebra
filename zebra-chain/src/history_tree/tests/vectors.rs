use std::{collections::BTreeMap, sync::Arc};

use crate::{
    block::{
        Block, ChainHistoryBlockTxAuthCommitmentHash,
        Commitment::{self, ChainHistoryActivationReserved},
        Height,
    },
    history_tree::NonEmptyHistoryTree,
    orchard,
    parameters::{Network, NetworkUpgrade},
    primitives::zcash_history::Entry,
    sapling,
    serialization::ZcashDeserializeInto,
};

use color_eyre::eyre;
use eyre::Result;

/// Test the history tree using the activation block of a network upgrade
/// and its next block.
///
/// This test is very similar to the zcash_history test in
/// zebra-chain/src/primitives/zcash_history/tests/vectors.rs, but with the
/// higher level API.
#[test]
fn push_and_prune() -> Result<()> {
    for network in Network::iter() {
        push_and_prune_for_network_upgrade(network.clone(), NetworkUpgrade::Heartwood)?;
        push_and_prune_for_network_upgrade(network.clone(), NetworkUpgrade::Canopy)?;
        push_and_prune_for_network_upgrade(network, NetworkUpgrade::Nu5)?;
    }
    Ok(())
}

fn push_and_prune_for_network_upgrade(
    network: Network,
    network_upgrade: NetworkUpgrade,
) -> Result<()> {
    let (blocks, sapling_roots) = network.block_sapling_roots_map();
    let orchard_roots = network.orchard_anchors();

    let height = network_upgrade.activation_height(&network).unwrap().0;

    // Load first block (activation block of the given network upgrade)
    let first_block = Arc::new(
        blocks
            .get(&height)
            .expect("test vector exists")
            .zcash_deserialize_into::<Block>()
            .expect("block is structurally valid"),
    );

    // Check its commitment
    let first_commitment = first_block.commitment(&network)?;
    if network_upgrade == NetworkUpgrade::Heartwood {
        // Heartwood is the only upgrade that has a reserved value.
        // (For other upgrades we could compare with the expected commitment,
        // but we haven't calculated them.)
        assert_eq!(first_commitment, ChainHistoryActivationReserved);
    }

    // Build initial history tree with only the first block
    let first_sapling_root =
        sapling::tree::Root::try_from(**sapling_roots.get(&height).expect("test vector exists"))?;
    let first_orchard_root = match network_upgrade {
        NetworkUpgrade::Heartwood | NetworkUpgrade::Canopy => None,
        _ => {
            // Orchard root bytes are in big-endian display order
            let mut root_bytes = **orchard_roots.get(&height).expect("test vector exists");
            root_bytes.reverse();
            Some(orchard::tree::Root::try_from(root_bytes)?)
        }
    };
    let mut tree = NonEmptyHistoryTree::from_block(
        &network,
        first_block,
        &first_sapling_root,
        &first_orchard_root.unwrap_or_default(),
    )?;

    assert_eq!(tree.size(), 1);
    assert_eq!(tree.peaks().len(), 1);
    assert_eq!(tree.current_height().0, height);

    // Compute root hash of the history tree, which will be included in the next block
    let first_root = tree.hash();

    // Load second block (activation + 1)
    let second_block = Arc::new(
        blocks
            .get(&(height + 1))
            .expect("test vector exists")
            .zcash_deserialize_into::<Block>()
            .expect("block is structurally valid"),
    );

    // Check its commitment
    let second_commitment = second_block.commitment(&network)?;
    match network_upgrade {
        NetworkUpgrade::Heartwood | NetworkUpgrade::Canopy => {
            assert_eq!(second_commitment, Commitment::ChainHistoryRoot(first_root))
        }
        _ => {
            let auth_data_root = second_block.auth_data_root();
            let commitment_hash = Commitment::ChainHistoryBlockTxAuthCommitment(
                ChainHistoryBlockTxAuthCommitmentHash::from_commitments(
                    &first_root,
                    &auth_data_root,
                ),
            );
            assert_eq!(second_commitment, commitment_hash);
        }
    }

    // Append second block to history tree
    let second_sapling_root = sapling::tree::Root::try_from(
        **sapling_roots
            .get(&(height + 1))
            .expect("test vector exists"),
    )?;
    let second_orchard_root = match network_upgrade {
        NetworkUpgrade::Heartwood | NetworkUpgrade::Canopy => None,
        _ => {
            // Orchard root bytes are in big-endian display order
            let mut root_bytes = **orchard_roots
                .get(&(height + 1))
                .expect("test vector exists");
            root_bytes.reverse();
            Some(orchard::tree::Root::try_from(root_bytes)?)
        }
    };
    let entries = tree
        .push(
            second_block,
            &second_sapling_root,
            &second_orchard_root.unwrap_or_default(),
        )
        .unwrap();

    // Adding a second block will produce a 3-node tree (one parent and two leaves).
    assert_eq!(tree.size(), 3);
    // The tree must have been pruned, resulting in a single peak (the parent).
    assert_eq!(tree.peaks().len(), 1);
    assert_eq!(tree.current_height().0, height + 1);
    // There should be 2 returned nodes (the peak and the new leaf)
    assert_eq!(entries.len(), 2);

    Ok(())
}

/// Test the history tree works during a network upgrade using the block
/// of a network upgrade and the previous block from the previous upgrade.
#[test]
fn upgrade() -> Result<()> {
    // The history tree only exists Hearwood-onward, and the only upgrade for which
    // we have vectors since then is Canopy. Therefore, only test the Heartwood->Canopy upgrade.
    for network in Network::iter() {
        upgrade_for_network_upgrade(network, NetworkUpgrade::Canopy)?;
    }
    Ok(())
}

fn upgrade_for_network_upgrade(network: Network, network_upgrade: NetworkUpgrade) -> Result<()> {
    let (blocks, sapling_roots) = network.block_sapling_roots_map();

    let height = network_upgrade.activation_height(&network).unwrap().0;

    // Load previous block (the block before the activation block of the given network upgrade)
    let block_prev = Arc::new(
        blocks
            .get(&(height - 1))
            .expect("test vector exists")
            .zcash_deserialize_into::<Block>()
            .expect("block is structurally valid"),
    );

    // Build a history tree with only the previous block (activation height - 1)
    // This tree will not match the actual tree (which has all the blocks since the previous
    // network upgrade), so we won't be able to check if its root is correct.
    let sapling_root_prev =
        sapling::tree::Root::try_from(**sapling_roots.get(&height).expect("test vector exists"))?;
    let mut tree = NonEmptyHistoryTree::from_block(
        &network,
        block_prev,
        &sapling_root_prev,
        &Default::default(),
    )?;

    assert_eq!(tree.size(), 1);
    assert_eq!(tree.peaks().len(), 1);
    assert_eq!(tree.current_height().0, height - 1);

    // Load block of the activation height
    let activation_block = Arc::new(
        blocks
            .get(&height)
            .expect("test vector exists")
            .zcash_deserialize_into::<Block>()
            .expect("block is structurally valid"),
    );

    // Append block to history tree. This must trigger a upgrade of the tree,
    // which should be recreated.
    let activation_sapling_root = sapling::tree::Root::try_from(
        **sapling_roots
            .get(&(height + 1))
            .expect("test vector exists"),
    )?;
    tree.push(
        activation_block,
        &activation_sapling_root,
        &Default::default(),
    )
    .unwrap();

    // Check if the tree has a single node, i.e. it has been recreated.
    assert_eq!(tree.size(), 1);
    assert_eq!(tree.peaks().len(), 1);
    assert_eq!(tree.current_height().0, height);

    Ok(())
}

/// Test that the tree built from saved nodes is equivalent to the one made from blocks.
#[test]
fn tree_from_cache() -> Result<()> {
    for network in Network::iter() {
        tree_from_cache_for_network_upgrade(network.clone(), NetworkUpgrade::Heartwood)?;
        tree_from_cache_for_network_upgrade(network, NetworkUpgrade::Nu5)?;
    }
    Ok(())
}

fn tree_from_cache_for_network_upgrade(
    network: Network,
    network_upgrade: NetworkUpgrade,
) -> Result<()> {
    let (blocks, sapling_roots) = network.block_sapling_roots_map();
    let orchard_roots = network.orchard_anchors();

    let height = network_upgrade.activation_height(&network).unwrap().0;

    let history_nodes = network.mainnet_history_nodes(network_upgrade).unwrap().clone();
    let n_nodes = history_nodes.len();

    // We use 6 blocks to build a tree with 10 nodes
    assert_eq!(n_nodes, 10);
    assert!(blocks.len() >= 6);
    let n_blocks = 6u32;

    let first_block = Arc::new(
        blocks
            .get(&height)
            .expect("test vector exists")
            .zcash_deserialize_into::<Block>()
            .expect("block is structurally valid"),
    );

    // Build initial history tree with only the first block
    let first_sapling_root =
        sapling::tree::Root::try_from(**sapling_roots.get(&height).expect("test vector exists"))?;
    let first_orchard_root = match network_upgrade {
        NetworkUpgrade::Heartwood | NetworkUpgrade::Canopy => None,
        _ => {
            // Orchard root bytes are in big-endian display order
            let mut root_bytes = **orchard_roots.get(&height).expect("test vector exists");
            root_bytes.reverse();
            Some(orchard::tree::Root::try_from(root_bytes)?)
        }
    };
    let mut tree_from_blocks = NonEmptyHistoryTree::from_block(
        &network,
        first_block,
        &first_sapling_root,
        &first_orchard_root.unwrap_or_default(),
    )?;

    // Build a tree from blocks
    let mut entries = Vec::new();
    for i in 1..n_blocks as u32 {
        let next_block = Arc::new(
            blocks
                .get(&(height + i))
                .expect("test vector exists")
                .zcash_deserialize_into::<Block>()
                .expect("block is structurally valid"),
        );

        let sapling_root = sapling::tree::Root::try_from(
            **sapling_roots
                .get(&(height + i))
                .expect("test vector exists"),
        )?;

        let orchard_root = match network_upgrade {
            NetworkUpgrade::Heartwood | NetworkUpgrade::Canopy => None,
            _ => {
                // Orchard root bytes are in big-endian display order
                let mut root_bytes = **orchard_roots
                    .get(&(height + i))
                    .expect("test vector exists");
                root_bytes.reverse();
                Some(orchard::tree::Root::try_from(root_bytes)?)
            }
        };

        let mut new_entries = tree_from_blocks
            .push(next_block, &sapling_root, &orchard_root.unwrap_or_default())
            .unwrap();
        entries.append(&mut new_entries);
    }

    // The returned entries should match the history node vector
    for (expected, returned) in history_nodes[1..].iter().zip(entries) {
        assert_eq!(*expected, returned.inner());
    }

    // Build a tree from nodes
    let peak_index = [6u32, 9u32];
    let peaks = BTreeMap::from([
        (
            peak_index[0],
            Entry::from(&Vec::from(history_nodes[peak_index[0] as usize])),
        ),
        (
            peak_index[1],
            Entry::from(&Vec::from(history_nodes[peak_index[1] as usize])),
        ),
    ]);
    let tree_from_cache = NonEmptyHistoryTree::from_cache(
        &network,
        n_nodes as u32,
        peaks,
        Height(height + n_blocks),
    )?;

    // Compare hashes and see if they match
    assert_eq!(tree_from_blocks.hash(), tree_from_cache.hash());

    Ok(())
}
