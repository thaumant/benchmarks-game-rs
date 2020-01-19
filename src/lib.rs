pub mod original;
pub mod modified1;
pub mod modified2;


#[cfg(test)]
mod test {
    use crate::original;
    use crate::modified1;
    use crate::modified2;
    
    #[test]
    fn compare() {
        let mut original_bodies  = original::STARTING_STATE;
        let mut modified1_bodies = modified1::STARTING_STATE;
        let mut modified2_bodies = modified2::STARTING_STATE;

        original::offset_momentum(&mut original_bodies);
        modified1::offset_momentum(&mut modified1_bodies);
        modified2::offset_momentum(&mut modified2_bodies);

        for _ in 0..1000 {
            let original_energy  = original::compute_energy(&mut original_bodies);
            let modified1_energy = modified1::compute_energy(&mut modified1_bodies);
            let modified2_energy = modified2::compute_energy(&mut modified2_bodies);

            assert_eq!(original_energy, modified1_energy);
            assert_eq!(original_energy, modified2_energy);

            original::advance(&mut original_bodies);
            modified1::advance(&mut modified1_bodies);
            modified2::advance(&mut modified2_bodies);
        }
    }
}
