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

void Init(rust::Str collision_meshes_folder, bool silent) {
    RocketSim::Init(std::filesystem::path(std::string(collision_meshes_folder)), silent);
}

void InitFromMem(rust::Slice<const rust::Slice<const byte>> soccar, rust::Slice<const rust::Slice<const byte>> hoops) {
    std::map<GameMode, std::vector<FileData>> gameModeMeshes;

    gameModeMeshes[GameMode::SOCCAR] = std::vector<FileData>(soccar.size());
    for (size_t i = 0; i < soccar.size(); i++) {
        gameModeMeshes[GameMode::SOCCAR][i] = FileData(soccar[i].begin(), soccar[i].end());
    }

    gameModeMeshes[GameMode::HOOPS] = std::vector<FileData>(hoops.size());
    for (size_t i = 0; i < hoops.size(); i++) {
        gameModeMeshes[GameMode::HOOPS][i] = FileData(hoops[i].begin(), hoops[i].end());
    }

    RocketSim::InitFromMem(gameModeMeshes);
}

Angle AngleFromRotMat(RotMat mat) {
    return Angle::FromRotMat(mat);
}

std::unique_ptr<Arenar> CreateArena(GameMode game_mode, EArenaConfig arenaConfig, uint8_t tick_rate) {
	return std::make_unique<Arenar>(game_mode, arenaConfig, tick_rate);
}

void Arenar::SetGoalScoreCallback(rust::Fn<void(Arenar&, Team, size_t)> callback, size_t user_info) {
    a->SetGoalScoreCallback([callback](class Arena* arena, Team team, void* userInfo) {
        std::pair<Arenar*, size_t>* userInfoPair = (std::pair<Arenar*, size_t>*) userInfo;
        callback(*(userInfoPair->first), team, userInfoPair->second);
    }, new std::pair(this, user_info));
}

void Arenar::SetCarBumpCallback(rust::Fn<void(Arenar&, uint32_t, uint32_t, bool, size_t)> callback, size_t user_info) {
    a->SetCarBumpCallback([callback](class Arena* arena, Car* bumper, Car* victim, bool isDemo, void* userInfo) {
        std::pair<Arenar*, size_t>* userInfoPair = (std::pair<Arenar*, size_t>*) userInfo;
        callback(*(userInfoPair->first), bumper->id, victim->id, isDemo, userInfoPair->second);
    }, new std::pair(this, user_info));
}

rust::Vec<uint32_t> Arenar::GetCars() const {
    rust::Vec<uint32_t> cars = rust::Vec<uint32_t>();
    for (Car* car : a->_cars) {
        cars.push_back(car->id);
    }
    return cars;
}

CarState Arenar::GetCar(uint32_t carID) {
    Car* car = a->GetCar(carID);
    assert(car != NULL);

    return car->GetState();
}

bool Arenar::SetCar(uint32_t carID, const CarState state) {
    Car* car = a->GetCar(carID);
    if (car == NULL) {
        return false;
    }

    car->SetState(state);
    return true;
}

bool Arenar::RemoveCar(uint32_t carID) {
    Car* car = a->GetCar(carID);
    if (car == NULL) {
        return false;
    }

    a->RemoveCar(car);
    return true;
}

bool Arenar::SetCarControls(uint32_t carID, const CarControls controls) {
    Car* car = a->GetCar(carID);
    if (car == NULL) {
        return false;
    }

    car->controls = controls;
    return true;
}

bool Arenar::DemolishCar(uint32_t carID) {
    Car* car = a->GetCar(carID);
    if (car == NULL) {
        return false;
    }

    car->Demolish();
    return true;
}

bool Arenar::RespawnCar(uint32_t carID, int32_t seed, float boostAmount) {
    Car* car = a->GetCar(carID);
    if (car == NULL) {
        return false;
    }

    car->Respawn(this->a->gameMode, seed, boostAmount);
    return true;
}

BoostPadConfig Arenar::GetPadConfig(size_t index) const {
    assert(index < a->_boostPads.size());
    return a->_boostPads[index]->config;
}

void Arenar::SetPadState(size_t index, const EBoostPadState state) {
    Car* curLockedCar = NULL;

    if (state.curLockedCarId != 0) {
        curLockedCar = a->GetCar(state.curLockedCarId);
    }

    BoostPadState estate = BoostPadState {
        state.isActive,
        state.cooldown,
        curLockedCar,
        state.prevLockedCarID,
    };
    a->_boostPads[index]->SetState(estate);
}

EBoostPadState Arenar::GetPadState(size_t index) const {
    assert(index < a->_boostPads.size());
    BoostPadState state = a->_boostPads[index]->GetState();
    return EBoostPadState {
        state.isActive,
        state.cooldown,
        state.curLockedCar ? state.curLockedCar->id : 0,
        state.prevLockedCarID,
    };
}
