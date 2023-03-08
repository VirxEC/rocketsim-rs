#include <array>

#include "RocketSim.h"

// extra car config stuff

const CarConfig& getOctane();
const CarConfig& getDominus();
const CarConfig& getPlank();
const CarConfig& getBreakout();
const CarConfig& getHybrid();
const CarConfig& getMerc();

// extra car stuff

uint32_t numCars(const Arena& arena);

uint32_t getCarID(const Arena& arena, uint32_t index);

struct ECarState {
	std::unique_ptr<Vec> pos;
	Angle angles;
	std::unique_ptr<Vec> vel;
	std::unique_ptr<Vec> angVel;
	bool isOnGround;
	bool hasJumped, hasDoubleJumped, hasFlipped;
	std::unique_ptr<Vec> lastRelDodgeTorque;
	float jumpTime, flipTime;
	bool isJumping;
	float airTimeSinceJump;
	float boost;
	float timeSpentBoosting;
	bool isSupersonic;
	float supersonicTime;
	float handbrakeVal;
	bool isAutoFlipping;
	float autoFlipTimer;
	float autoFlipTorqueScale;
	bool hasContact;
	std::unique_ptr<Vec> contactNormal;
	bool isContactingCar;
	uint32_t otherCar;
	float cooldownTimer;
	bool isDemoed;
	float demoRespawnTimer;
	uint64_t lastHitBallTick;
	CarControls lastControls;
};

std::unique_ptr<ECarState> getCarFromIndex(Arena& arena, uint32_t index);

std::unique_ptr<ECarState> getCarState(Arena& arena, uint32_t carID);

/// @brief Sets the state of a car in the arena
/// @param arena 
/// @param state 
/// @param carID 
/// @return True if the car was found and the state was set, false otherwise
bool setCarState(Arena& arena, uint32_t carID, const ECarState& state);

uint32_t addCar(Arena& arena, Team team, const CarConfig& config);

/// @brief Sets the controls of a car for the next tick
/// @param arena 
/// @param state 
/// @param carID 
/// @return True if the car was found and the state was set, false otherwise
bool setCarControls(Arena& arena, uint32_t carID, const CarControls& controls);

bool demolishCar(Arena& arena, uint32_t carID);

bool respawnCar(Arena& arena, uint32_t carID, int seed);

// extra ball stuff

struct EBallState {
    std::unique_ptr<Vec> pos;
    std::unique_ptr<Vec> vel;
    std::unique_ptr<Vec> angvel;
};

std::unique_ptr<EBallState> getBallState(const Arena& arena);

void setBallState(Arena& arena, const EBallState& state);

// boost pad stuff

uint32_t numBoostPads(const Arena& arena);

bool getBoostPadIsBig(const Arena& arena, uint32_t id);
std::unique_ptr<Vec> getBoostPadPos(const Arena& arena, uint32_t id);

struct EBoostPadState {
	uint32_t id;
	bool isActive;
	float cooldown;
};

void setBoostPadState(Arena& arena, const EBoostPadState& state);

EBoostPadState getBoostPadState(const Arena& arena, uint32_t id);

// extra arena stuff

uint64_t getTickCount(const Arena& arena);

float getTickRate(const Arena& arena);
