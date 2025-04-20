pub struct NodeReputation {
    scores: HashMap<NodeId, ReputationScore>,
    decay_factor: f32,
}

impl NodeReputation {
    pub fn adjust_score(&mut self, node: NodeId, delta: i32) {
        let entry = self.scores.entry(node).or_insert(ReputationScore::default());
        entry.update(delta, self.decay_factor);
    }

    pub fn get_ranked_nodes(&self) -> Vec<NodeRanking> {
        let mut nodes: Vec<_> = self.scores.iter().collect();
        nodes.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
        nodes.into_iter()
            .map(|(id, score)| NodeRanking {
                node_id: *id,
                score: *score,
            })
            .collect()
    }
}
