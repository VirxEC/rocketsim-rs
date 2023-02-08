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

std::unique_ptr<Vec> getCarStatePos(const CarState& state) {
    return std::make_unique<Vec>(state.pos);
}

const Vec& carStatePos(const CarState& state) {
    return state.pos;
}

void setCarStatePos(CarState& state, const Vec& pos) {
    state.pos = pos;
}

std::unique_ptr<Vec> getCarStateVel(const CarState& state) {
    return std::make_unique<Vec>(state.vel);
}

const Vec& carStateVel(const CarState& state) {
    return state.vel;
}

void setCarStateVel(CarState& state, const Vec& vel) {
    state.vel = vel;
}

std::unique_ptr<Vec> getCarStateAngVel(const CarState& state) {
    return std::make_unique<Vec>(state.angVel);
}

const Vec& carStateAngVel(const CarState& state) {
    return state.angVel;
}

void setCarStateAngVel(CarState& state, const Vec& angVel) {
    state.angVel = angVel;
}

std::unique_ptr<BallState> getBallState(const Arena& arena) {
    return std::make_unique<BallState>(arena.ball->GetState());
}

void setBallState(Arena& arena, const BallState& state) {
    arena.ball->SetState(state);
}

std::unique_ptr<Vec> getBallStatePos(const BallState& state) {
    return std::make_unique<Vec>(state.pos);
}

const Vec& ballStatePos(const BallState& state) {
    return state.pos;
}

void setBallStatePos(BallState& state, const Vec& pos) {
    state.pos = pos;
}

std::unique_ptr<Vec> getBallStateVel(const BallState& state) {
    return std::make_unique<Vec>(state.vel);
}

const Vec& ballStateVel(const BallState& state) {
    return state.vel;
}

void setBallStateVel(BallState& state, const Vec& vel) {
    state.vel = vel;
}

std::unique_ptr<Vec> getBallStateAngVel(const BallState& state) {
    return std::make_unique<Vec>(state.angVel);
}

const Vec& ballStateAngVel(const BallState& state) {
    return state.angVel;
}

void setBallStateAngVel(BallState& state, const Vec& angVel) {
    state.angVel = angVel;
}
