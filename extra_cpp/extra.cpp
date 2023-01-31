#include "extra.h"

std::array<float, 3> btVector3ToArray(const btVector3& v) {
    return {v.x(), v.y(), v.z()};
}

std::unique_ptr<btVector3> arrayToBtVector3(const std::array<float, 3>& a) {
    return std::make_unique<btVector3>(a[0], a[1], a[2]);
}
