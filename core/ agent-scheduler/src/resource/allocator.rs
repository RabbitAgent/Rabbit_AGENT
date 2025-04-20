use crate::blockchain::ResourceToken;
pub struct ResourceAuction {
    resource_pool: HashMap<NodeId, ResourceProfile>,
    token_contract: ResourceToken,
}

impl ResourceAuction {
    pub async fn allocate(
        &mut self,
        task: &TaskDescriptor,
        bid_strategy: BidStrategy,
    ) -> Result<AllocationReceipt> {
        let mut bids = self.collect_bids(task).await?;
        bids.sort_by(|a, b| b.bid_value.cmp(&a.bid_value));
        
        let winner = bids.first()
            .ok_or(AllocationError::NoBidders)?;

        let receipt = self.token_contract.lock_resources(
            winner.node_id,
            &task.resource_requirements,
            task.ttl
        ).await?;

        Ok(receipt)
    }

    async fn collect_bids(&self, task: &TaskDescriptor) -> Result<Vec<ResourceBid>> {
        let mut bids = Vec::new();
        
        for (node_id, profile) in &self.resource_pool {
            if profile.satisfies(&task.resource_requirements) {
                let bid_value = self.calculate_bid_value(node_id, task).await?;
                bids.push(ResourceBid {
                    node_id: *node_id,
                    bid_value,
                });
            }
        }
        
        Ok(bids)
    }
}
