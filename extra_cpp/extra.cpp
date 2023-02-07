#include "extra.h"

std::array<float, 3> btVector3ToArray(const btVector3& v) {
    return {v.x(), v.y(), v.z()};
}

std::unique_ptr<btVector3> arrayToBtVector3(const std::array<float, 3>& a) {
    return std::make_unique<btVector3>(a[0], a[1], a[2]);
}

const CarConfig& getOctane() {
    return CAR_CONFIG_OCTANE;
}

const CarConfig& getDominus() {
    return CAR_CONFIG_DOMINUS;
}

const CarConfig& getPlank() {
    return CAR_CONFIG_PLANK;
}

const CarConfig& getBreakout() {
    return CAR_CONFIG_BREAKOUT;
}

const CarConfig& getHybrid() {
    return CAR_CONFIG_HYBRID;
}

const CarConfig& getMerc() {
    return CAR_CONFIG_MERC;
}

uint32_t getCarID(const Car& car) {
    return car.id;
}

std::unique_ptr<CarState> getCarState(Arena& arena, uint32_t carID) {
    Car* car = arena.GetCarFromID(carID);
    if (car == NULL) {
        return NULL;
    }

    return std::make_unique<CarState>(car->GetState());
}

bool setCarState(Arena& arena, uint32_t carID, const CarState& state) {
    Car* car = arena.GetCarFromID(carID);
    if (car == NULL) {
        return false;
    }

    car->SetState(state);
    return true;
}

uint32_t addCar(Arena& arena, Team team, const CarConfig& config) {
    Car* car = arena.AddCar(team, config);
    return car->id;
}

// uint32_t addCarSetState(Arena& arena, Team team, const CarConfig& config, const CarState& state) {
//     Car* car = arena.AddCar(team, config);
//     car->SetState(state);
//     return car->id;
// }
