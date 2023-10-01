#include <cassert>

#include "arenar.h"

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

void init(rust::Str collision_meshes_folder) {
    RocketSim::Init(std::filesystem::path(std::string(collision_meshes_folder)));
}

Angle AngleFromRotMat(RotMat mat) {
    return Angle::FromRotMat(mat);
}

void Arenar::SetGoalScoreCallback(rust::Fn<void(Arenar&, Team, size_t)> callback, size_t user_info) {
    a->SetGoalScoreCallback([callback](class Arena* arena, Team team, void* userInfo) {
        std::pair<Arenar*, size_t>* userInfoPair = (std::pair<Arenar*, size_t>*) userInfo;
        callback(*(userInfoPair->first), team, userInfoPair->second);
    }, new std::pair(this, user_info));
}

void Arenar::SetCarBumpCallback(rust::Fn<void(Arenar&, uint32_t, uint32_t, bool, size_t)> callback, size_t user_info) {
    a->SetCarBumpCallback([callback](class Arena* arena, Car* bumper, Car* victim, bool isDemo, void* userInfo) {
        std::pair<Arenar*, size_t>* userInfoPair = (std::pair<Arenar*, size_t>*) userInfo;
        callback(*(userInfoPair->first), bumper->id, victim->id, isDemo, userInfoPair->second);
    }, new std::pair(this, user_info));
}

rust::Vec<uint32_t> Arenar::GetCars() const {
    rust::Vec<uint32_t> cars = rust::Vec<uint32_t>();
    for (Car* car : a->_cars) {
        cars.push_back(car->id);
    }
    return cars;
}

CarState Arenar::GetCar(uint32_t carID) {
    Car* car = a->GetCar(carID);
    assert(car != NULL);

    return car->GetState();
}

bool Arenar::SetCar(uint32_t carID, const CarState state) {
    Car* car = a->GetCar(carID);
    if (car == NULL) {
        return false;
    }

    car->SetState(state);
    return true;
}

bool Arenar::RemoveCar(uint32_t carID) {
    Car* car = a->GetCar(carID);
    if (car == NULL) {
        return false;
    }

    a->RemoveCar(car);
    return true;
}

bool Arenar::SetCarControls(uint32_t carID, const CarControls controls) {
    Car* car = a->GetCar(carID);
    if (car == NULL) {
        return false;
    }

    car->controls = controls;
    return true;
}

bool Arenar::DemolishCar(uint32_t carID) {
    Car* car = a->GetCar(carID);
    if (car == NULL) {
        return false;
    }

    car->Demolish();
    return true;
}

bool Arenar::RespawnCar(uint32_t carID, int32_t seed, float boostAmount) {
    Car* car = a->GetCar(carID);
    if (car == NULL) {
        return false;
    }

    car->Respawn(this->a->gameMode, seed, boostAmount);
    return true;
}

Vec Arenar::GetPadPos(size_t index) const {
    assert(index < a->_boostPads.size());
    return a->_boostPads[index]->pos;
}

bool Arenar::get_pad_is_big(size_t index) const {
    assert(index < a->_boostPads.size());
    return a->_boostPads[index]->isBig;
}

void Arenar::SetPadState(size_t index, const EBoostPadState state) {
    Car* curLockedCar = NULL;

    if (state.curLockedCarId != 0) {
        curLockedCar = a->GetCar(state.curLockedCarId);
    }

    BoostPadState estate = BoostPadState {
        state.isActive,
        state.cooldown,
        curLockedCar,
        state.prevLockedCarID,
    };
    a->_boostPads[index]->SetState(estate);
}

EBoostPadState Arenar::GetPadState(size_t index) const {
    assert(index < a->_boostPads.size());
    BoostPadState state = a->_boostPads[index]->GetState();
    return EBoostPadState {
        state.isActive,
        state.cooldown,
        state.curLockedCar ? state.curLockedCar->id : 0,
        state.prevLockedCarID,
    };
}
