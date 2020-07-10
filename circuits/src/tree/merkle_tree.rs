use std::fmt;

use sapling_crypto_ce::{
    poseidon::{
        PoseidonEngine,
        QuinticSBox,
        poseidon_hash,
    }
};

pub struct PoseidonMerkleTree<'a, E: PoseidonEngine> {
    params: &'a E::Params,
    tree: Vec::<E::Fr>,
    depth: usize,
}

#[allow(dead_code)]
impl<'a, E> PoseidonMerkleTree<'a, E>
    where E: PoseidonEngine<SBox = QuinticSBox<E>>
{
    pub fn hash(&self, input: &[E::Fr]) -> E::Fr {
        let hash = poseidon_hash::<E>(self.params, input);
        hash[0].clone()
    }

    pub fn num_leaves(&self) -> usize {
        1 << self.depth
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn new(leaves: Vec::<Vec::<E::Fr>>, params: &'a E::Params) -> Self {
        let mut merkle_tree = PoseidonMerkleTree {
            params: params,
            tree: Vec::new(),
            depth: 0,
        };

        merkle_tree.tree = leaves
            .iter().map(
                |leaf| merkle_tree.hash(leaf)
            ).collect();

        let mut start_index = 0;
        let mut end_index = leaves.len();

        while end_index - start_index > 1 {
            for i in (start_index..end_index).step_by(2) {
                merkle_tree.tree.push(
                    merkle_tree.hash(
                        &merkle_tree.tree[i..i + 2]
                    )
                );
            }

            start_index = end_index;
            end_index = merkle_tree.tree.len();
            merkle_tree.depth += 1;
        }

        merkle_tree
    }

    pub fn get_leaf_indices(&self, leaf_index: usize) -> Vec::<bool> {
        assert!(leaf_index < self.num_leaves());

        let bin_str = format!("{:0w$b}", leaf_index, w=self.depth());
        let mut bin_array: Vec<_> = bin_str.chars()
            .into_iter().map(
                |x| x == '1'
            ).collect();
        bin_array.reverse();
        bin_array
    }

    pub fn get_leaf_path(&self, leaf_index: usize) -> Vec::<E::Fr> {
        assert!(leaf_index < self.num_leaves());

        let mut path = Vec::new();
        let mut level_start = 0;
        let mut level_nodes = self.num_leaves();
        let mut level_shift = leaf_index;

        while level_nodes > 1 {
            let neighbor_index = {
                let node_index = level_start + level_shift;
                if node_index % 2 == 0 {
                    node_index + 1
                } else {
                    node_index - 1
                }
            };

            path.push(self.tree[neighbor_index].clone());

            level_shift /= 2;
            level_start += level_nodes;
            level_nodes /= 2;
        }

        path
    }

    pub fn update_leaf(&mut self, leaf_index: usize, new_leaf: Vec::<E::Fr>) {
        assert!(leaf_index < self.num_leaves());

        self.tree[leaf_index] = self.hash(
            &new_leaf
        );

        let mut level_start = 0;
        let mut level_nodes = self.num_leaves();
        let mut level_shift = leaf_index;

        while level_nodes > 1 { 
            let parent_index = level_start + level_nodes + level_shift / 2;
            let even_node_index = {
                let node_index = level_start + level_shift;
                node_index - (node_index % 2)
            };

            self.tree[parent_index] = self.hash(
                &self.tree[even_node_index..even_node_index + 2]
            );

            level_shift /= 2;
            level_start += level_nodes;
            level_nodes /= 2;
        }
    }

    pub fn root(&self) -> E::Fr {
        let root_index = self.tree.len() - 1;
        self.tree[root_index].clone()
    }
}

impl<'a, E> Clone for PoseidonMerkleTree<'a, E>
    where E: PoseidonEngine<SBox = QuinticSBox<E>>
{
    fn clone(&self) -> Self {
        PoseidonMerkleTree {
            params: self.params,
            tree: self.tree.clone(),
            depth: self.depth,
        }
    }
}

impl<'a, E> fmt::Debug for PoseidonMerkleTree<'a, E>
    where E: PoseidonEngine<SBox = QuinticSBox<E>>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let mut level_end = self.num_leaves();
        let mut level_size = self.num_leaves();

        write!(f, "tree: [\n")?;
        for (i, node) in self.tree.clone().into_iter().enumerate() {
            if i == level_end {
                write!(f, "\n")?;
                level_size /= 2;
                level_end += level_size;
            }
            write!(f, "    {:?},\n", node)?;
        }
        write!(f, "]\n")?;

        Ok(())
    }
}
