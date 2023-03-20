#pragma once

#include "RocketSim.h"

// extra car config stuff

const CarConfig& getOctane();
const CarConfig& getDominus();
const CarConfig& getPlank();
const CarConfig& getBreakout();
const CarConfig& getHybrid();
const CarConfig& getMerc();

struct EBoostPadState {
	uint32_t index;
	bool isActive;
	float cooldown;
};
