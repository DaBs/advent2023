use nom::{
    IResult,
    branch::alt,
    bytes::streaming::take_while,
    bytes::complete::{tag, take_until},
    character::complete::digit1,
    character::is_alphabetic,
    multi::{
        separated_list0,
    },
};

enum WorkflowOperator {
    GreaterThan,
    LessThan,
}

struct WorkflowCondition {
    part_type: MachinePartType,
    operator: WorkflowOperator,
    value: u32,
}

// Example: a<2006
impl WorkflowCondition {
    fn parse(input: &str) -> IResult<&str, WorkflowCondition> {
        let (input, part_type) = alt((
            tag("a"), // Aerodynamic
            tag("x"), // Extreme
            tag("m"), // Musical
            tag("s"), // Shiny
        ))(input)?;
        let (input, operator) = alt((
            tag("<"),
            tag(">"),
        ))(input)?;
        let (input, value) = digit1(input)?;

        Ok((input, WorkflowCondition {
            part_type: match part_type {
                "a" => MachinePartType::Aerodynamic,
                "x" => MachinePartType::Extreme,
                "m" => MachinePartType::Musical,
                "s" => MachinePartType::Shiny,
                _ => panic!("Unknown part type"),
            },
            operator: match operator {
                "<" => WorkflowOperator::LessThan,
                ">" => WorkflowOperator::GreaterThan,
                _ => panic!("Unknown operator"),
            },
            value: value.parse().unwrap(),
        }))
    }
}

struct WorkflowRule {
    output_condition: Option<WorkflowCondition>,
    output_id: Option<String>,
}

// Example x>10:one, or could also be just A for accepted and R for rejected or just the ID of the output
impl WorkflowRule {
    fn parse(input: &str) -> IResult<&str, WorkflowRule> {
        if !input.contains(":") {
            let (input, output_id) = take_until("}")(input)?;
            Ok((input, WorkflowRule {
                output_condition: None,
                output_id: Some(output_id.to_string()),
            }))
        } else {
            let (input, output_condition) = WorkflowCondition::parse(input)?;
            let (input, _) = tag(":")(input)?;
            // Take rest of the string as output id
            let (input, output_id) = take_while(|c| is_alphabetic(c as u8))(input)?;
            Ok((input, WorkflowRule {
                output_condition: Some(output_condition),
                output_id: Some(output_id.to_string()),
            }))
        }
    }
}

struct Workflow {
    id: String,
    rules: Vec<WorkflowRule>,
}

// Example: px{a<2006:qkq,m>2090:A,rfg}
impl Workflow {
    fn parse(input: &str) -> IResult<&str, Workflow> {
        let (input, id) = take_until("{")(input)?;
        let (input, _) = tag("{")(input)?;
        let (input, rules) = separated_list0(tag(","), WorkflowRule::parse)(input)?;
        let (input, _) = tag("}")(input)?;
        Ok((input, Workflow {
            id: id.to_string(),
            rules,
        }))
    }
}

#[derive(Debug, PartialEq)]
enum MachinePartType {
    Extreme,
    Musical,
    Aerodynamic,
    Shiny,
}

// Example: 
#[derive(Debug)]
struct MachinePart {
    ratings: Vec<(MachinePartType, u32)>,
}

// Example: {x=787,m=2655,a=1222,s=2876}
impl MachinePart {
    fn parse(input: &str) -> IResult<&str, MachinePart> {
        let (input, _) = tag("{")(input)?;
        let (input, ratings) = separated_list0(tag(","), MachinePart::parse_rating)(input)?;
        let (input, _) = tag("}")(input)?;
        Ok((input, MachinePart {
            ratings,
        }))
    }

    fn parse_rating(input: &str) -> IResult<&str, (MachinePartType, u32)> {
        let (input, part_type) = alt((
            tag("x"), // Extreme
            tag("m"), // Musical
            tag("a"), // Aerodynamic
            tag("s"), // Shiny
        ))(input)?;
        let (input, _) = tag("=")(input)?;
        let (input, value) = digit1(input)?;
        Ok((input, (match part_type {
            "x" => MachinePartType::Extreme,
            "m" => MachinePartType::Musical,
            "a" => MachinePartType::Aerodynamic,
            "s" => MachinePartType::Shiny,
            _ => panic!("Unknown part type"),
        }, value.parse().unwrap())))
    }
}

fn run_machine_parts_through_workflows<'a>(workflows: &'a Vec<Workflow>, machine_parts: &'a Vec<MachinePart>, starting_workflow_id: &'a str) -> (Vec<&'a MachinePart>, Vec<&'a MachinePart>) {
    let mut accepted_machine_parts = Vec::new();
    let mut rejected_machine_parts = Vec::new();

    for machine_part in machine_parts {

        let mut current_workflow_id = starting_workflow_id;

        while let Some(workflow) = workflows.iter().find(|workflow| workflow.id == current_workflow_id) {
            let mut output_id = None;
            for rule in &workflow.rules {
                if let Some(output_condition) = &rule.output_condition {
                    let rating = machine_part.ratings.iter().find(|(part_type, _)| part_type == &output_condition.part_type).unwrap().1;
                    match output_condition.operator {
                        WorkflowOperator::GreaterThan => {
                            if rating > output_condition.value {
                                output_id = Some(rule.output_id.as_ref().unwrap());
                                break;
                            }
                        },
                        WorkflowOperator::LessThan => {
                            if rating < output_condition.value {
                                output_id = Some(rule.output_id.as_ref().unwrap());
                                break;
                            }
                        },
                    }
                } else {
                    output_id = Some(rule.output_id.as_ref().unwrap());
                    break;
                }
            }

            println!("{:?} -> {:?}", machine_part.ratings, output_id);

            if let Some(output_id) = output_id {
                if output_id == "A" {
                    accepted_machine_parts.push(machine_part);
                    break;
                } else if output_id == "R" {
                    rejected_machine_parts.push(machine_part);
                    break;
                }

                current_workflow_id = output_id;
            } else {
                break;
            }
        }
    }

    (accepted_machine_parts, rejected_machine_parts)
}

fn parse_input(input: &str) -> (Vec<Workflow>, Vec<MachinePart>) {

    let parts = input.split("\r\n\r\n").collect::<Vec<&str>>();

    let workflows = parts[0].lines()
        .map(|line| Workflow::parse(line).unwrap().1)
        .collect();

    let machine_parts = parts[1].lines()
        .map(|line| MachinePart::parse(line).unwrap().1)
        .collect();

    (workflows, machine_parts)
}

fn part1(input: &str) -> u32 {
    let (workflows, machine_parts) = parse_input(input);

    let (accepted_machine_parts, _) = run_machine_parts_through_workflows(&workflows, &machine_parts, "in");

    accepted_machine_parts.iter()
        .map(|machine_part| machine_part.ratings.iter().map(|(_, rating)| rating).sum::<u32>())
        .sum()
}

fn part2(input: &str) -> u64 {
    // TODO: Implement part 2
    0
}

fn main() {
    let input = include_str!("./input.txt");

    let result = part1(input);

    println!("Part 1 result: {}", result);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("./example.txt");

        let (workflows, machine_parts) = parse_input(input);

        assert_eq!(workflows.len(), 11);
        assert_eq!(machine_parts.len(), 5);

        let (accepted_machine_parts, rejected_machine_parts) = run_machine_parts_through_workflows(&workflows, &machine_parts, "in");

        assert_eq!(accepted_machine_parts.len(), 3);
        assert_eq!(rejected_machine_parts.len(), 2);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("./input.txt");

        let result = part1(input);

        assert_eq!(result, 425811);
    }

    #[test]
    fn test_example_part2() {
        let input = include_str!("./example.txt");

        let result = part2(input);

        assert_eq!(result, 167409079868000);
    } 

    #[test]
    fn test_part2() {

    }
}