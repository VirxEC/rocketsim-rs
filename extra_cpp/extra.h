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

struct ECarState {
	std::unique_ptr<Vec> pos;
	Angle angles;
	std::unique_ptr<Vec> vel;
	std::unique_ptr<Vec> angVel;
	bool isOnGround;
	bool hasJumped, hasDoubleJumped, hasFlipped;
	std::unique_ptr<Vec> lastRelDodgeTorque;
	float jumpTimer, flipTimer;
	bool isJumping;
	float airTimeSinceJump;
	float boost;
	float timeSpentBoosting;
	bool isSupersonic;
	float handbrakeVal;
	CarControls lastControls;

};

std::unique_ptr<ECarState> getCarState(Arena& arena, uint32_t carID);

/// @brief Sets the state of a car in the arena
/// @param arena 
/// @param state 
/// @param carID 
/// @return True if the car was found and the state was set, false otherwise
bool setCarState(Arena& arena, uint32_t carID, const ECarState& state);

uint32_t addCar(Arena& arena, Team team, const CarConfig& config);

// extra ball stuff

struct EBallState {
    std::unique_ptr<Vec> pos;
    std::unique_ptr<Vec> vel;
    std::unique_ptr<Vec> angvel;
};

std::unique_ptr<EBallState> getBallState(const Arena& arena);

void setBallState(Arena& arena, const EBallState& state);
