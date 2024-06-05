use crate::utils::traits::Creature;
pub struct SimpleCreature {}


/* trait impls */
impl Creature for SimpleCreature {}
impl Default for SimpleCreature {
    fn default() -> Self {
        SimpleCreature {}
    }
}

