pub mod biffle;
pub mod rehnberger;
pub mod thaumant;


#[cfg(test)]
mod test {
    use crate::biffle;
    use crate::rehnberger;
    use crate::thaumant;

    fn round12(f: f64) -> f64 {
        (f * 1e12).round() / 1e12
    }
    
    #[test]
    fn compare() {
        let mut biffle_bodies     = biffle::STARTING_STATE;
        let mut rehnberger_bodies = rehnberger::STARTING_STATE;
        let mut thaumant_bodies   = thaumant::STARTING_STATE;

        let mut rehnberger_sim = rehnberger::BodiesAdvance::new();

        biffle::offset_momentum(&mut biffle_bodies);
        rehnberger::offset_momentum(&mut rehnberger_bodies);
        thaumant::offset_momentum(&mut thaumant_bodies);

        for _ in 0..1000 {
            let biffle_energy     = round12(biffle::compute_energy(&mut biffle_bodies));
            let rehnberger_energy = round12(rehnberger::compute_energy(&mut rehnberger_bodies));
            let thaumant_energy   = round12(thaumant::compute_energy(&mut thaumant_bodies));

            assert_eq!(biffle_energy, rehnberger_energy);
            assert_eq!(biffle_energy, thaumant_energy);

            biffle::advance(&mut biffle_bodies);
            rehnberger_sim.advance(&mut rehnberger_bodies, 0.01);
            thaumant::advance(&mut thaumant_bodies, 1);
        }
    }
}
