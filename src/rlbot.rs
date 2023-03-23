use glam::Vec3A;

use crate::sim::arena::Arena;

#[derive(Clone, Copy, Debug, Default)]
pub struct BoostPad {
    pub location: Vec3A,
    pub is_full_boost: bool,
}

#[derive(Clone, Debug, Default)]
pub struct FieldInfo {
    pub boost_pads: Vec<BoostPad>,
    pub num_boosts: usize,
}

impl Arena {
    #[inline]
    pub fn get_field_info(&self) -> FieldInfo {
        FieldInfo {
            boost_pads: self
                .iter_pad_static()
                .map(|(is_full_boost, pos)| BoostPad {
                    location: pos.into(),
                    is_full_boost,
                })
                .collect(),
            num_boosts: self.num_pads(),
        }
    }
}
