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

void init(const string& collision_meshes_folder) {
    RocketSim::Init(std::filesystem::path(collision_meshes_folder));
}

void Arenar::SetGoalScoreCallback(rust::Fn<void(Arenar&, Team)> callback) {
    a->SetGoalScoreCallback([callback](class Arena* arena, Team team, void* userInfo) {
        callback(*((Arenar*) userInfo), team);
    }, this);
}

size_t Arenar::get_car_index(uint32_t car_id) const {
    assert(car_id != 0);

    for (size_t i = 0; i < a->_cars.size(); i++) {
        if (a->_cars[i]->id == car_id) {
            return i;
        }
    }

    assert(false);
}

std::unique_ptr<std::vector<CarState>> Arenar::GetCars() {
    std::unique_ptr<std::vector<CarState>> states = std::make_unique<std::vector<CarState>>();
    for (Car* car : a->_cars) {
        states.get()->push_back(car->GetState());
    }
    return states;
}

CarState Arenar::GetCar(uint32_t carID) {
    Car* car = a->GetCarFromID(carID);
    assert(car != NULL);

    return car->GetState();
}

bool Arenar::SetCar(uint32_t carID, const CarState state) {
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

bool Arenar::SetCarControls(uint32_t carID, const CarControls controls) {
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
        curLockedCar = a->GetCarFromID(state.curLockedCarId);
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
