use std::collections::{HashMap, VecDeque};

use nom::{
    branch::alt,
    bytes::{
        complete::{tag, take_until},
    },
    character::complete::{alpha1},
    multi::{separated_list1},
    combinator::{map},
    sequence::{preceded},
    IResult,
};

#[derive(Debug, PartialEq, Eq, Clone)]
enum PulseType {
    Low,
    High,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum ModuleType {
    FlipFlop, // Prefixed by %
    Conjunction, // Prefixed by &
    Broadcaster, // Simply called "broadcaster"
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum ModuleState {
    On,
    Off,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Module {
    id: String,
    module_type: ModuleType,
    state: ModuleState,
    outputs: Vec<String>,
    input_states: HashMap<String, PulseType>,
}

impl Module {
    fn parse(input: &str) -> IResult<&str, Module> {
        let (remaining_input, module_type) = alt((
            map(tag("%"), |_| ModuleType::FlipFlop),
            map(tag("&"), |_| ModuleType::Conjunction),
            map(tag("broadcaster"), |_| ModuleType::Broadcaster),
        ))(input)?;

        let (remaining_input, id) = match module_type {
            ModuleType::FlipFlop | ModuleType::Conjunction => {
                take_until(" ")(remaining_input)?
            }
            ModuleType::Broadcaster => {
                let (remaining, _) = take_until(" ")(remaining_input)?;
                (remaining, "broadcaster")
            },
        };

        let (remaining_input, _) = tag(" -> ")(remaining_input)?;
        // The outputs are separated by comma and space and continues to the end of the line, which is not included in the input str here
        let (remaining_input, outputs) = separated_list1(tag(", "), alpha1)(remaining_input)?;

        Ok((
            remaining_input,
            Module {
                id: id.to_string(),
                module_type,
                state: ModuleState::Off,
                outputs: outputs.iter().map(|s| s.to_string()).collect(),
                input_states: HashMap::new(),
            },
        ))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Pulse {
    from_id: String,
    to_id: String,
    pulse_type: PulseType,
}

struct Circuit {
    modules: Vec<Module>,
}

impl Circuit {
    fn parse(input: &str) -> IResult<&str, Circuit> {
        let (input, modules) = separated_list1(tag("\r\n"), Module::parse)(input)?;

        // Iterate over the modules and setup initial conjunction states
        let mapped_modules: Vec<Module> = modules
            .clone()
            .iter_mut()
            .map(|module| {
                if module.module_type == ModuleType::Conjunction {
                    let input_states: Vec<_> = modules.iter().filter(|m| m.outputs.contains(&module.id)).map(|m| (m.id.clone(), PulseType::Low)).collect();

                    module.input_states = input_states.into_iter().collect();
                }

                module.clone()
            })
            .collect();


        Ok((input, Circuit { modules: mapped_modules }))
    }

    fn resolve_broadcast(&mut self) -> (u32, u32) {
        let broadcaster = self.modules.iter().find(|m| m.module_type == ModuleType::Broadcaster).unwrap();

        let mut low_pulses_sent = 0;
        let mut high_pulses_sent = 0;

        let mut queue = VecDeque::new();

        queue.push_back(Pulse {
            from_id: "button".to_string(),
            to_id: broadcaster.id.clone(),
            pulse_type: PulseType::Low,
        });

        let mut outgoing_pulse_type = PulseType::Low;
    
        while let Some(pulse) = queue.pop_front() {
            //println!("Pulse: {:?}", pulse);
            let mut should_send_pulse = true;
    
            let receiving_module = self.modules.iter_mut().find(|m| m.id == pulse.to_id);

            if receiving_module.is_none() {
                continue;
            }

            let receiving_module = receiving_module.unwrap();
    
            match receiving_module.module_type {
                ModuleType::Broadcaster => {
                    outgoing_pulse_type = pulse.pulse_type.clone();
                }
                ModuleType::FlipFlop => {
                    match pulse.pulse_type {
                        PulseType::Low => {
                            if receiving_module.state == ModuleState::On {
                                receiving_module.state = ModuleState::Off;
                                outgoing_pulse_type = PulseType::Low;
                            } else {
                                receiving_module.state = ModuleState::On;
                                outgoing_pulse_type = PulseType::High;
                            }
                        },
                        PulseType::High => {
                            should_send_pulse = false;
                        }
                    }
                },
                ModuleType::Conjunction => {
                    receiving_module.input_states.insert(pulse.from_id, pulse.pulse_type.clone());
    
                    outgoing_pulse_type = if receiving_module.input_states.values().all(|v| v == &PulseType::High) {
                        PulseType::Low
                    } else {
                        PulseType::High
                    };
                }
            }

            if should_send_pulse {
                let new_pulses: Vec<_> = receiving_module.outputs
                    .clone()
                    .iter()
                    .map(|o| Pulse {
                        from_id: receiving_module.id.clone(),
                        to_id: o.clone(),
                        pulse_type: outgoing_pulse_type.clone(),
                    })
                    .collect();

                if outgoing_pulse_type == PulseType::Low {
                    low_pulses_sent += new_pulses.len() as u32;
                } else {
                    high_pulses_sent += new_pulses.len() as u32;
                }
    
                queue.extend(new_pulses);
            }
        }

        // Button always sends a low pulse to broadcaster
        (low_pulses_sent + 1, high_pulses_sent)
    }
}

fn part1(input: &str) -> u32 {
    let (_, mut circuit) = Circuit::parse(input).unwrap();

    // Run the broadcast 1000 times, recording the total number of low and high pulses sent
    let mut low_pulses_sent = 0;
    let mut high_pulses_sent = 0;

    for _ in 0..1000 {
        let (low, high) = circuit.resolve_broadcast();

        low_pulses_sent += low;
        high_pulses_sent += high;
    }

    low_pulses_sent * high_pulses_sent
}

fn main() {
    let input = include_str!("./input.txt");

    println!("Part 1: {}", part1(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_module() {
        let input = "%ghja -> x, y, z";
        let expected = Module {
            id: "ghja".to_string(),
            module_type: ModuleType::FlipFlop,
            state: ModuleState::Off,
            outputs: vec!["x".to_string(), "y".to_string(), "z".to_string()],
            input_states: HashMap::new(),
        };
        let (_, module) = Module::parse(input).unwrap();
        assert_eq!(module, expected);
    }

    #[test]
    fn test_example() {
        let input = include_str!("./example.txt");

        let (_, mut circuit) = Circuit::parse(input).unwrap();

        let (low_pulses_sent, high_pulses_sent) = circuit.resolve_broadcast();

        assert_eq!(low_pulses_sent, 8);
        assert_eq!(high_pulses_sent, 4);
    }

    #[test]
    fn test_part1_example() {
        let input = include_str!("./example.txt");

        assert_eq!(part1(input), 32000000);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("./input.txt");

        assert_eq!(part1(input), 879834312);
    }
}
