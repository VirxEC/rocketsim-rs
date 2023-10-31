#pragma once

#include "RocketSim.h"
#include "cxx.h"

const CarConfig& getOctane();
const CarConfig& getDominus();
const CarConfig& getPlank();
const CarConfig& getBreakout();
const CarConfig& getHybrid();
const CarConfig& getMerc();

void init(rust::Str collision_meshes_folder);
Angle AngleFromRotMat(RotMat mat);

struct EBoostPadState {
	bool isActive;
	float cooldown;
	uint32_t curLockedCarId = 0;
	uint32_t prevLockedCarID = 0;
};

struct Arenar {
    Arena* a;

    Arenar(GameMode game_mode, ArenaMemWeightMode memWeightMode, float tick_rate) {
        a = Arena::Create(game_mode, memWeightMode, tick_rate);
    }

    ~Arenar() {
        delete a;
    }

	void SetGoalScoreCallback(rust::Fn<void(Arenar&, Team, size_t)> callback, size_t);
	void SetCarBumpCallback(rust::Fn<void(Arenar&, uint32_t, uint32_t, bool, size_t)> callback, size_t user_info);

    // No copy constructor
    Arenar(const Arenar & other) = delete;
    Arenar & operator =(const Arenar & other) = delete;

    // Move constructor
    Arenar(Arenar&& other) = default;
    Arenar& operator =(Arenar && other) = default;

	Arenar Clone(bool copy_callbacks) {
		return Arenar(a->Clone(copy_callbacks));
	}

	// extra car stuff
	size_t num_cars() const {
		return a->_cars.size();
	}

	rust::Vec<uint32_t> GetCars() const;

	CarConfig GetCarConfig(uint32_t car_id) const {
		return a->GetCar(car_id)->config;
	}

	Team GetCarTeam(uint32_t car_id) const {
		return a->GetCar(car_id)->team;
	}

	CarState GetCar(uint32_t car_id);
	/// @brief Sets the state of a car in the arena
	/// @param arena
	/// @param state
	/// @param carID
	/// @return True if the car was found and the state was set, false otherwise
	bool SetCar(uint32_t car_id, const CarState state);

	uint32_t AddCar(Team team, const CarConfig& config) {
		return a->AddCar(team, config)->id;
	}

	bool RemoveCar(uint32_t car_id);
	/// @brief Sets the controls of a car for the next tick
	/// @param arena
	/// @param state
	/// @param carID
	/// @return True if the car was found and the state was set, false otherwise
	bool SetCarControls(uint32_t car_id, const CarControls controls);
	bool DemolishCar(uint32_t car_id);
	bool RespawnCar(uint32_t car_id, int32_t seed, float boost_amount);

	// extra ball stuff

	BallState GetBall() {
		return a->ball->GetState();
	}

	void SetBall(const BallState state) {
		a->ball->SetState(state);
	}

	float get_ball_radius() const {
		return a->ball->GetRadius();
	}

	std::array<float, 4> GetBallRotation() const {
		btQuaternion rot = a->ball->_rigidBody.getOrientation();
		return {
			rot.getX(),
			rot.getY(),
			rot.getZ(),
			rot.getW()
		};
	}

	// boost pad stuff

	size_t num_pads() const {
		return a->_boostPads.size();
	}

	bool get_pad_is_big(size_t index) const;
	Vec GetPadPos(size_t index) const;
	void SetPadState(size_t index, const EBoostPadState state);
	EBoostPadState GetPadState(size_t index) const;

	// extra misc stuff

	void reset_tick_count() {
		a->tickCount = 0;
	}

	uint64_t get_tick_count() const {
		return a->tickCount;
	}

	float get_tick_rate() const {
		return 1 / a->tickTime;
	}

	GameMode get_game_mode() const {
		return a->gameMode;
	}

	void ResetToRandomKickoff(int32_t seed = -1) {
		a->ResetToRandomKickoff(seed);
	}

	void step(int32_t ticks = 1) {
		a->Step(ticks);
	}

	bool IsBallProbablyGoingIn(float maxTime = 2.f, float extraMargin = 0) const {
		return a->IsBallProbablyGoingIn(maxTime, extraMargin);
	}

	bool is_ball_scored() const {
		return a->IsBallScored();
	}

	MutatorConfig GetMutatorConfig() const {
		return a->GetMutatorConfig();
	}

	void SetMutatorConfig(MutatorConfig mutatorConfig) {
		a->SetMutatorConfig(mutatorConfig);
	}

private:
	Arenar(Arena* arena) {
		a = arena;
	}
};
