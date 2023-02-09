#include <array>

#include "../RocketSim/src/BaseInc.h"
#include "../RocketSim/src/Sim/Car/CarConfig/CarConfig.h"
#include "../RocketSim/src/Sim/Car/Car.h"
#include "../RocketSim/src/Sim/Arena/Arena.h"

// extra vec stuff

std::array<float, 3> btVector3ToArray(const btVector3& v);

std::unique_ptr<btVector3> arrayToBtVector3(const std::array<float, 3>& a);

// extra car config stuff

const CarConfig& getOctane();
const CarConfig& getDominus();
const CarConfig& getPlank();
const CarConfig& getBreakout();
const CarConfig& getHybrid();
const CarConfig& getMerc();

// extra car stuff

uint32_t getCarID(const Car& car);

std::unique_ptr<CarState> getCarState(Arena& arena, uint32_t carID);

/// @brief Sets the state of a car in the arena
/// @param arena 
/// @param state 
/// @param carID 
/// @return True if the car was found and the state was set, false otherwise
bool setCarState(Arena& arena, uint32_t carID, const CarState& state);

uint32_t addCar(Arena& arena, Team team, const CarConfig& config);

std::unique_ptr<Vec> getCarStatePos(const CarState& state);

const Vec& carStatePos(const CarState& state);

void setCarStatePos(CarState& state, const Vec& pos);

std::unique_ptr<Vec> getCarStateVel(const CarState& state);

const Vec& carStateVel(const CarState& state);

void setCarStateVel(CarState& state, const Vec& vel);

std::unique_ptr<Vec> getCarStateAngVel(const CarState& state);

const Vec& carStateAngVel(const CarState& state);

void setCarStateAngVel(CarState& state, const Vec& angVel);

std::unique_ptr<Vec> getCarStateTorque(const CarState& state);

const Vec& carStateTorque(const CarState& state);

void setCarStateTorque(CarState& state, const Vec& torque);

// extra ball stuff

std::unique_ptr<BallState> getBallState(const Arena& arena);

void setBallState(Arena& arena, const BallState& state);

std::unique_ptr<Vec> getBallStatePos(const BallState& state);

const Vec& ballStatePos(const BallState& state);

void setBallStatePos(BallState& state, const Vec& pos);

std::unique_ptr<Vec> getBallStateVel(const BallState& state);

const Vec& ballStateVel(const BallState& state);

void setBallStateVel(BallState& state, const Vec& vel);

std::unique_ptr<Vec> getBallStateAngVel(const BallState& state);

const Vec& ballStateAngVel(const BallState& state);

void setBallStateAngVel(BallState& state, const Vec& angVel);
