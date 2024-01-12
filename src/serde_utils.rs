pub use serde;

use serde::{Serialize, Deserialize, ser::SerializeStruct};

// impl Serialize for Vec3 {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//         where
//             S: serde::Serializer {
//                 let mut s = serializer.serialize_struct("Vec3", 4)?;
//                 s.serialize_field("x", &self.x)?;
//                 s.serialize_field("y", &self.y)?;
//                 s.serialize_field("z", &self.z)?;
//                 s.serialize_field("_w", &self._w)?;
//                 s.end()
//     }
// }

// TODO: Angle, 
