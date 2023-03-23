use std::pin::Pin;

use crate::{
    sim::{arena::Arena, CarControls},
    NoCarFound,
};

impl Arena {
    #[inline]
    pub fn set_all_controls(mut self: Pin<&mut Self>, controls: &[(u32, CarControls)]) -> Result<(), NoCarFound> {
        controls.iter().try_for_each(|&(car_id, car_controls)| self.as_mut().set_car_controls(car_id, car_controls))
    }
}
