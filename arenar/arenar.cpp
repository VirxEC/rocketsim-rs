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

ECarState getECarState(CarState carstate) {
    return ECarState {
        carstate.pos,
        carstate.rotMat,
        carstate.vel,
        carstate.angVel,
        carstate.isOnGround,
        carstate.hasJumped, carstate.hasDoubleJumped, carstate.hasFlipped,
        carstate.lastRelDodgeTorque,
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
        carstate.worldContact.contactNormal,
        carstate.carContact.otherCarID,
        carstate.carContact.cooldownTimer,
        carstate.isDemoed,
        carstate.demoRespawnTimer,
        carstate.lastHitBallTick,
        carstate.lastControls
    };
}

ECarState Arenar::GetCarFromIndex(uint32_t index) {
    return getECarState(a->_cars[index]->GetState());
}

ECarState Arenar::GetCar(uint32_t carID) {
    Car* car = a->GetCarFromID(carID);
    assert(car != NULL);

    return getECarState(car->GetState());
}

bool Arenar::SetCar(uint32_t carID, const ECarState& state) {
    Car* car = a->GetCarFromID(carID);
    if (car == NULL) {
        return false;
    }

    CarState estate = {
        state.pos,
        state.rotMat,
        state.vel,
        state.angVel,
        state.isOnGround,
        state.hasJumped, state.hasDoubleJumped, state.hasFlipped,
        state.lastRelDodgeTorque,
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
        state.contactNormal,
        state.otherCarID,
        state.cooldownTimer,
        state.isDemoed,
        state.demoRespawnTimer,
        state.lastHitBallTick,
        state.lastControls
    };

    car->SetState(estate);
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

bool Arenar::RespawnCar(uint32_t carID, int seed) {
    Car* car = a->GetCarFromID(carID);
    if (car == NULL) {
        return false;
    }

    car->Respawn(seed);
    return true;
}

BallState Arenar::get_ball_state() const {
    return a->ball->GetState();
}

void Arenar::set_ball_state(const BallState& state) {
    a->ball->SetState(state);
}

Vec Arenar::get_boost_pad_pos(uint32_t id) const {
    assert(id < a->_boostPads.size());
    return a->_boostPads[id]->pos;
}

bool Arenar::get_boost_pad_is_big(uint32_t id) const {
    assert(id < a->_boostPads.size());
    return a->_boostPads[id]->isBig;
}

void Arenar::set_boost_pad_state(const EBoostPadState& state) {
    BoostPadState estate = BoostPadState {
        state.isActive,
        state.cooldown,
    };
    a->_boostPads[state.id]->SetState(estate);
}

EBoostPadState Arenar::get_boost_pad_state(uint32_t id) const {
    assert(id < a->_boostPads.size());
    BoostPadState state = a->_boostPads[id]->GetState();
    return EBoostPadState {
        id,
        state.isActive,
        state.cooldown,
    };
}
