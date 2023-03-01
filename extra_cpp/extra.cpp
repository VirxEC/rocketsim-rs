#include "extra.h"

std::array<float, 3> btVector3ToArray(const btVector3& v) {
    return {v.x(), v.y(), v.z()};
}

std::unique_ptr<btVector3> arrayToBtVector3(const std::array<float, 3>& a) {
    return std::make_unique<btVector3>(a[0], a[1], a[2]);
}

std::unique_ptr<btVector3> cloneBtVector3(const btVector3& v) {
    return std::make_unique<btVector3>(v.x(), v.y(), v.z());
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

uint32_t numCars(const Arena& arena) {
    return arena._cars.size();
}

uint32_t getCarID(const Arena& arena, uint32_t index) {
    return arena._cars[index]->id;
}

std::unique_ptr<ECarState> getECarState(CarState carstate) {
    bool isContactingCar = false;
    uint32_t otherCar = 0;

    if (carstate.carContact.otherCar != NULL) {
        isContactingCar = true;
        otherCar = carstate.carContact.otherCar->id;
    }

    return std::make_unique<ECarState>(ECarState {
        std::make_unique<Vec>(carstate.pos),
        carstate.angles,
        std::make_unique<Vec>(carstate.vel),
        std::make_unique<Vec>(carstate.angVel),
        carstate.isOnGround,
        carstate.hasJumped, carstate.hasDoubleJumped, carstate.hasFlipped,
        std::make_unique<Vec>(carstate.lastRelDodgeTorque),
        carstate.jumpTime, carstate.flipTime,
        carstate.isJumping,
        carstate.airTimeSinceJump,
        carstate.boost,
        carstate.timeSpentBoosting,
        carstate.isSupersonic,
        carstate.supersonicTime,
        carstate.handbrakeVal,
        carstate.isAutoFlipping,
        carstate.autoFlipTimer,
        carstate.autoFlipTorqueScale,
        carstate.worldContact.hasContact,
        std::make_unique<Vec>(carstate.worldContact.contactNormal),
        isContactingCar,
        otherCar,
        carstate.carContact.cooldownTimer,
        carstate.isDemoed,
        carstate.demoRespawnTimer,
        carstate.lastHitBallTick,
        carstate.lastControls
    });
}

std::unique_ptr<ECarState> getCarFromIndex(Arena& arena, uint32_t index) {
    return getECarState(arena._cars[index]->GetState());
}

std::unique_ptr<ECarState> getCarState(Arena& arena, uint32_t carID) {
    Car* car = arena.GetCarFromID(carID);
    if (car == NULL) {
        return NULL;
    }

    return getECarState(car->GetState());
}

bool setCarState(Arena& arena, uint32_t carID, const ECarState& state) {
    Car* car = arena.GetCarFromID(carID);
    if (car == NULL) {
        return false;
    }

    Car* otherCar = NULL;

    if (state.isContactingCar) {
        otherCar = arena.GetCarFromID(state.otherCar);
    }

    CarState estate = {
        *state.pos,
        state.angles,
        *state.vel,
        *state.angVel,
        state.isOnGround,
        state.hasJumped, state.hasDoubleJumped, state.hasFlipped,
        *state.lastRelDodgeTorque,
        state.jumpTime, state.flipTime,
        state.isJumping,
        state.airTimeSinceJump,
        state.boost,
        state.timeSpentBoosting,
        state.isSupersonic,
        state.supersonicTime,
        state.handbrakeVal,
        state.isAutoFlipping,
        state.autoFlipTimer,
        state.autoFlipTorqueScale,
        state.hasContact,
        *state.contactNormal,
        otherCar,
        state.cooldownTimer,
        state.isDemoed,
        state.demoRespawnTimer,
        state.lastHitBallTick,
        state.lastControls
    };

    car->SetState(estate);
    return true;
}

uint32_t addCar(Arena& arena, Team team, const CarConfig& config) {
    Car* car = arena.AddCar(team, config);
    return car->id;
}

bool setCarControls(Arena& arena, uint32_t carID, const CarControls& controls) {
    Car* car = arena.GetCarFromID(carID);
    if (car == NULL) {
        return false;
    }

    car->controls = controls;
    return true;
}

bool demolishCar(Arena& arena, uint32_t carID) {
    Car* car = arena.GetCarFromID(carID);
    if (car == NULL) {
        return false;
    }

    car->Demolish();
    return true;
}

bool respawnCar(Arena& arena, uint32_t carID, int seed) {
    Car* car = arena.GetCarFromID(carID);
    if (car == NULL) {
        return false;
    }

    car->Respawn(seed);
    return true;
}

std::unique_ptr<EBallState> getBallState(const Arena& arena) {
    BallState state = arena.ball->GetState();
    return std::make_unique<EBallState>(EBallState {
        std::make_unique<Vec>(state.pos),
        std::make_unique<Vec>(state.vel),
        std::make_unique<Vec>(state.angVel),
    });
}

void setBallState(Arena& arena, const EBallState& state) {
    BallState estate = BallState {
        *state.pos,
        *state.vel,
        *state.angvel,
    };

    arena.ball->SetState(estate);
}

uint32_t numBoostPads(const Arena& arena) {
    return arena._boostPads.size();
}

std::unique_ptr<Vec> getBoostPadPos(const Arena& arena, uint32_t id) {
    return std::make_unique<Vec>(arena._boostPads[id]->GetPos());
}

bool getBoostPadIsBig(const Arena& arena, uint32_t id) {
    return arena._boostPads[id]->isBig;
}

void setBoostPadState(Arena& arena, const EBoostPadState& state) {
    BoostPadState estate = BoostPadState {
        state.isActive,
        state.cooldown,
    }; 
    arena._boostPads[state.id]->SetState(estate);
}

EBoostPadState getBoostPadState(const Arena& arena, uint32_t id) {
    BoostPadState state = arena._boostPads[id]->GetState();
    return EBoostPadState {
        id,
        state.isActive,
        state.cooldown,
    };
}
