#pragma once

#include "RocketSim.h"

// extra car config stuff

const CarConfig& getOctane();
const CarConfig& getDominus();
const CarConfig& getPlank();
const CarConfig& getBreakout();
const CarConfig& getHybrid();
const CarConfig& getMerc();

struct ECarState {
	Vec pos;
	RotMat rotMat;
	Vec vel;
	Vec angVel;
	bool isOnGround;
	bool hasJumped, hasDoubleJumped, hasFlipped;
	Vec lastRelDodgeTorque;
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
	Vec contactNormal;
	uint32_t otherCarID;
	float cooldownTimer;
	bool isDemoed;
	float demoRespawnTimer;
	uint64_t lastHitBallTick;
	CarControls lastControls;
};

struct EBoostPadState {
	uint32_t id;
	bool isActive;
	float cooldown;
};
