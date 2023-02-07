#include <array>

#include "../RocketSim/src/BaseInc.h"
#include "../RocketSim/src/Sim/Car/CarConfig/CarConfig.h"
#include "../RocketSim/src/Sim/Car/Car.h"
#include "../RocketSim/src/Sim/Arena/Arena.h"

std::array<float, 3> btVector3ToArray(const btVector3& v);

std::unique_ptr<btVector3> arrayToBtVector3(const std::array<float, 3>& a);

const CarConfig& getOctane();
const CarConfig& getDominus();
const CarConfig& getPlank();
const CarConfig& getBreakout();
const CarConfig& getHybrid();
const CarConfig& getMerc();

uint32_t getCarID(const Car& car);

std::unique_ptr<CarState> getCarState(Arena& arena, uint32_t carID);

/// @brief Sets the state of a car in the arena
/// @param arena 
/// @param state 
/// @param carID 
/// @return True if the car was found and the state was set, false otherwise
bool setCarState(Arena& arena, uint32_t carID, const CarState& state);

uint32_t addCar(Arena& arena, Team team, const CarConfig& config);

// uint32_t addCarSetState(Arena& arena, Team team, const CarConfig& config, const CarState& state);
