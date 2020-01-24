pub mod biffle;
pub mod thaumant1;
pub mod thaumant2;


#[cfg(test)]
mod test {
    use crate::biffle;
    use crate::thaumant1;
    use crate::thaumant2;
    
    #[test]
    fn compare() {
        let mut biffle_bodies  = biffle::STARTING_STATE;
        let mut thaumant1_bodies = thaumant1::STARTING_STATE;
        let mut thaumant2_bodies = thaumant2::STARTING_STATE;

        biffle::offset_momentum(&mut biffle_bodies);
        thaumant1::offset_momentum(&mut thaumant1_bodies);
        thaumant2::offset_momentum(&mut thaumant2_bodies);

        for _ in 0..1000 {
            let biffle_energy  = biffle::compute_energy(&mut biffle_bodies);
            let thaumant1_energy = thaumant1::compute_energy(&mut thaumant1_bodies);
            let thaumant2_energy = thaumant2::compute_energy(&mut thaumant2_bodies);

            assert_eq!(biffle_energy, thaumant1_energy);
            assert_eq!(biffle_energy, thaumant2_energy);

            biffle::advance(&mut biffle_bodies);
            thaumant1::advance(&mut thaumant1_bodies);
            thaumant2::advance(&mut thaumant2_bodies);
        }
    }
}
