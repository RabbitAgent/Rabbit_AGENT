use bellman::{
    gadgets::{
        boolean::Boolean,
        sha256::sha256,
    },
    Circuit, ConstraintSystem, SynthesisError
};
use pairing::bls12_381::{Bls12, Fr};

struct MerkleMembershipCircuit {
    leaf: Option<[u8; 32]>,
    path: Vec<Option<[u8; 32]>>,
    index_bits: Vec<Option<bool>>,
    root: Option<[u8; 32]>,
}

impl Circuit<Fr> for MerkleMembershipCircuit {
    fn synthesize<CS: ConstraintSystem<Fr>>(
        self, 
        cs: &mut CS
    ) -> Result<(), SynthesisError> {
        let leaf = sha256(
            cs.namespace(|| "leaf"),
            &[Boolean::constant(false); 512] // Placeholder
        )?;

        let mut computed_hash = leaf;
        for (i, sibling) in self.path.into_iter().enumerate() {
            let direction_bit = Boolean::from(self.index_bits[i]);
            let sibling = sha256(
                cs.namespace(|| format!("sibling_{}", i)),
                &[Boolean::constant(false); 512]
            )?;
            
            computed_hash = merkle_switch(
                cs.namespace(|| format!("switch_{}", i)),
                &computed_hash,
                &sibling,
                &direction_bit
            )?;
        }

        let expected_root = sha256(
            cs.namespace(|| "root"),
            &[Boolean::constant(false); 512]
        )?;

        cs.enforce(
            || "root_equality",
            |lc| lc + computed_hash.get_variable(),
            |lc| lc + CS::one(),
            |lc| lc + expected_root.get_variable(),
        );
        
        Ok(())
    }
}

fn merkle_switch<CS: ConstraintSystem<Fr>>(
    mut cs: CS,
    left: &[Boolean],
    right: &[Boolean],
    bit: &Boolean,
) -> Result<Vec<Boolean>, SynthesisError> {
    let mut result = Vec::with_capacity(256);
    
    for (i, (l, r)) in left.iter().zip(right.iter()).enumerate() {
        let res = Boolean::from(AllocatedBit::alloc(
            cs.namespace(|| format!("bit_{}", i)),
            bit.get_value()
        )?);
        
        let selected = Bit::conditional_select(
            cs.namespace(|| format!("select_{}", i)),
            &bit,
            &r,
            &l
        )?;
        
        result.push(selected);
    }
    
    Ok(result)
}
