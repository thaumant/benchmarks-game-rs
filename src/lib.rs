pub mod biffle;
pub mod rehnberger;
pub mod thaumant1;
pub mod thaumant2;


#[cfg(test)]
mod test {
    use crate::biffle;
    use crate::rehnberger;
    use crate::thaumant1;
    use crate::thaumant2;

    fn round12(f: f64) -> f64 {
        (f * 1e12).round() / 1e12
    }
    
    #[test]
    fn compare() {
        let mut biffle_bodies     = biffle::STARTING_STATE;
        let mut rehnberger_bodies = rehnberger::STARTING_STATE;
        let mut thaumant1_bodies  = thaumant1::STARTING_STATE;
        let mut thaumant2_bodies  = thaumant2::STARTING_STATE;

        let mut rehnberger_sim = rehnberger::BodiesAdvance::new();

        biffle::offset_momentum(&mut biffle_bodies);
        rehnberger::offset_momentum(&mut rehnberger_bodies);
        thaumant1::offset_momentum(&mut thaumant1_bodies);
        thaumant2::offset_momentum(&mut thaumant2_bodies);

        for _ in 0..1000 {
            let biffle_energy     = round12(biffle::compute_energy(&mut biffle_bodies));
            let rehnberger_energy = round12(rehnberger::compute_energy(&mut rehnberger_bodies));
            let thaumant1_energy  = round12(thaumant1::compute_energy(&mut thaumant1_bodies));
            let thaumant2_energy  = round12(thaumant2::compute_energy(&mut thaumant2_bodies));

            assert_eq!(biffle_energy, rehnberger_energy);
            assert_eq!(biffle_energy, thaumant1_energy);
            assert_eq!(biffle_energy, thaumant2_energy);

            biffle::advance(&mut biffle_bodies);
            rehnberger_sim.advance(&mut rehnberger_bodies, 0.01);
            thaumant1::advance(&mut thaumant1_bodies);
            thaumant2::advance(&mut thaumant2_bodies);
        }
    }
}
