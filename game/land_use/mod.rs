use kay::ActorSystem;
use core::simulation::SimulationID;
use stagemaster::UserInterfaceID;
use planning_old::materialized_reality::MaterializedRealityID;

pub mod buildings;
pub mod zone_planning_old;
pub mod zone_planning_new;

pub fn setup(
    system: &mut ActorSystem,
    user_interface: UserInterfaceID,
    simulation: SimulationID,
    materialized_reality: MaterializedRealityID,
) {
    buildings::setup(system, user_interface, simulation, materialized_reality);
    zone_planning_old::setup(system);
}