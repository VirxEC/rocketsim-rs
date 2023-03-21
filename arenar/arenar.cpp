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

CarState Arenar::GetCarFromIndex(uint32_t index) {
    return a->_cars[index]->GetState();
}

CarState Arenar::GetCar(uint32_t carID) {
    Car* car = a->GetCarFromID(carID);
    assert(car != NULL);

    return car->GetState();
}

bool Arenar::SetCar(uint32_t carID, const CarState& state) {
    Car* car = a->GetCarFromID(carID);
    if (car == NULL) {
        return false;
    }

    car->SetState(state);
    return true;
}

bool Arenar::RemoveCar(uint32_t carID) {
    Car* car = a->GetCarFromID(carID);
    if (car == NULL) {
        return false;
    }

    a->RemoveCar(car);
    return true;
}

bool Arenar::SetCarControls(uint32_t carID, const CarControls& controls) {
    Car* car = a->GetCarFromID(carID);
    if (car == NULL) {
        return false;
    }

    car->controls = controls;
    return true;
}

bool Arenar::DemolishCar(uint32_t carID) {
    Car* car = a->GetCarFromID(carID);
    if (car == NULL) {
        return false;
    }

    car->Demolish();
    return true;
}

bool Arenar::RespawnCar(uint32_t carID, int32_t seed) {
    Car* car = a->GetCarFromID(carID);
    if (car == NULL) {
        return false;
    }

    car->Respawn(seed);
    return true;
}

BallState Arenar::GetBall() const {
    return a->ball->GetState();
}

void Arenar::SetBall(const BallState& state) {
    a->ball->SetState(state);
}

Vec Arenar::GetPadPos(uint32_t index) const {
    assert(index < a->_boostPads.size());
    return a->_boostPads[index]->pos;
}

bool Arenar::get_pad_is_big(uint32_t index) const {
    assert(index < a->_boostPads.size());
    return a->_boostPads[index]->isBig;
}

void Arenar::SetPadState(const EBoostPadState& state) {
    BoostPadState estate = BoostPadState {
        state.isActive,
        state.cooldown,
    };
    a->_boostPads[state.index]->SetState(estate);
}

EBoostPadState Arenar::GetPadState(uint32_t index) const {
    assert(index < a->_boostPads.size());
    BoostPadState state = a->_boostPads[index]->GetState();
    return EBoostPadState {
        index,
        state.isActive,
        state.cooldown,
    };
}