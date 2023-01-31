#include <array>

#include "../RocketSim/src/BaseInc.h"

std::array<float, 3> btVector3ToArray(const btVector3& v);

std::unique_ptr<btVector3> arrayToBtVector3(const std::array<float, 3>& a);
