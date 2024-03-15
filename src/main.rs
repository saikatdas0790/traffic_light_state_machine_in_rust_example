#![allow(dead_code)]
use std::{thread, time::Duration};

#[derive(Debug)]
enum TrafficLightPossibleStates {
    Red,
    Yellow {
        going_to: Box<TrafficLightPossibleStates>,
    },
    Green,
}

#[derive(Debug)]
enum PedestrianLightPossibleStates {
    Walk,
    DontWalk,
}

enum TrafficLightEvents {
    LightChange,
    EmergencyStop,
}

const INITIAL_STATE: (TrafficLightPossibleStates, PedestrianLightPossibleStates) = (
    TrafficLightPossibleStates::Red,
    PedestrianLightPossibleStates::Walk,
);
const WAITING_TIME_AT_RED_AND_GREEN: Duration = Duration::from_secs(3);
const WAITING_TIME_AT_YELLOW: Duration = Duration::from_secs(1);

trait TrafficLightStateMachine {
    fn initialize_state_machine();
    fn handle_event(self, event: TrafficLightEvents);
    fn transition_to_new_state_after_waiting(
        &mut self,
        waiting_time: Duration,
        new_state: (TrafficLightPossibleStates, PedestrianLightPossibleStates),
    );
}

struct TrafficLightStateMachineImpl {
    current_light_state: TrafficLightPossibleStates,
    current_pedestrian_state: PedestrianLightPossibleStates,
}

impl TrafficLightStateMachine for TrafficLightStateMachineImpl {
    fn initialize_state_machine() {
        let initialized_machine = TrafficLightStateMachineImpl {
            current_light_state: INITIAL_STATE.0,
            current_pedestrian_state: INITIAL_STATE.1,
        };

        initialized_machine.handle_event(TrafficLightEvents::LightChange);
    }

    fn handle_event(mut self, event: TrafficLightEvents) {
        match event {
            TrafficLightEvents::LightChange => match self.current_light_state {
                TrafficLightPossibleStates::Red => {
                    self.transition_to_new_state_after_waiting(
                        WAITING_TIME_AT_RED_AND_GREEN,
                        (
                            TrafficLightPossibleStates::Yellow {
                                going_to: Box::new(TrafficLightPossibleStates::Green),
                            },
                            PedestrianLightPossibleStates::Walk,
                        ),
                    );
                    self.handle_event(TrafficLightEvents::LightChange);
                }
                TrafficLightPossibleStates::Yellow { ref going_to } => {
                    self.transition_to_new_state_after_waiting(
                        WAITING_TIME_AT_YELLOW,
                        match **going_to {
                            TrafficLightPossibleStates::Green => (
                                TrafficLightPossibleStates::Green,
                                PedestrianLightPossibleStates::DontWalk,
                            ),
                            TrafficLightPossibleStates::Red => (
                                TrafficLightPossibleStates::Red,
                                PedestrianLightPossibleStates::Walk,
                            ),
                            _ => panic!("Invalid state"),
                        },
                    );
                    self.handle_event(TrafficLightEvents::LightChange);
                }
                TrafficLightPossibleStates::Green => {
                    self.transition_to_new_state_after_waiting(
                        WAITING_TIME_AT_RED_AND_GREEN,
                        (
                            TrafficLightPossibleStates::Yellow {
                                going_to: Box::new(TrafficLightPossibleStates::Red),
                            },
                            PedestrianLightPossibleStates::DontWalk,
                        ),
                    );
                    self.handle_event(TrafficLightEvents::LightChange);
                }
            },
            TrafficLightEvents::EmergencyStop => {
                self.current_light_state = TrafficLightPossibleStates::Red;
                self.current_pedestrian_state = PedestrianLightPossibleStates::DontWalk;
            }
        };
    }

    fn transition_to_new_state_after_waiting(
        &mut self,
        waiting_time: Duration,
        new_state: (TrafficLightPossibleStates, PedestrianLightPossibleStates),
    ) {
        thread::sleep(waiting_time);
        println!(
            "Transitioning to new state after waiting for {:?}",
            waiting_time
        );

        self.current_light_state = new_state.0;
        self.current_pedestrian_state = new_state.1;
        println!(
            "Light state: {:?}, Pedestrian state: {:?}",
            self.current_light_state, self.current_pedestrian_state
        );
    }
}

fn main() {
    TrafficLightStateMachineImpl::initialize_state_machine();
}
