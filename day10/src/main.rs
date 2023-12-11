use petgraph::graph::{NodeIndex, UnGraph};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
#[repr(u8)]
enum PipeType {
    Vertical = b'|',
    Horizontal = b'-',
    NorthEast = b'L',
    NorthWest = b'J',
    SouthEast = b'F',
    SouthWest = b'7',
    Starting = b'S',
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Pipe {
    is_starting: bool,
    pipe_type: PipeType,
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Pipes {
    height: usize,
    width: usize,
    pipes: Vec<Pipe>,
}

impl Pipes {
    fn parse(input: &str) -> Pipes {
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();

        let continuous = input.lines().flat_map(|line| line.bytes()).collect::<Vec<u8>>();
        let pipes = continuous.iter().enumerate().filter_map(|(index, &byte)| {
            if byte == b' ' || byte == b'.' {
                None
            } else {
                let pipe_type = match byte {
                    b'|' => PipeType::Vertical,
                    b'-' => PipeType::Horizontal,
                    b'L' => PipeType::NorthEast,
                    b'J' => PipeType::NorthWest,
                    b'7' => PipeType::SouthWest,
                    b'F' => PipeType::SouthEast,
                    b'S' => PipeType::Starting,
                    _ => panic!("Unknown pipe type: {}", byte),
                };

                Some(Pipe {
                    is_starting: byte == b'S',
                    pipe_type: pipe_type,
                    x: index % width,
                    y: index / width,
                })
            }
        }).collect::<Vec<Pipe>>();

        Pipes {
            height: height,
            width: width,
            pipes: pipes,
        }
    }

    fn get_graph_map(&self) -> UnGraph<Pipe, ()> {
        // Depending on the pipe type, we need to add edges to a directed graph in different directions.
        // For example, a horizontal pipe will have edges to the left and right, but not up or down.
        // A north-east pipe will have edges to the north and east, but not south or west.
        // A starting pipe will have edges to the north, east, south, and west, if there are pipes connecting
        // in those directions.
        let mut graph = UnGraph::<Pipe, ()>::new_undirected();

        let mut node_indices = Vec::new();

        for pipe in &self.pipes {
            let node_index = graph.add_node(pipe.clone());
            node_indices.push(node_index);
        }

        for (pipe, node_indice) in self.pipes.iter().zip(node_indices.clone()) {
            let mut add_edge = |x: usize, y: usize| -> bool {
                if x >= self.width || y >= self.height {
                    return false;
                }

                let other_pipe_position = self.pipes.iter().position(|other_pipe| other_pipe.x == x && other_pipe.y == y);

                if other_pipe_position.is_none() {
                    return false;
                }

                let other_pipe_position = other_pipe_position.unwrap();
                let other_node_index = node_indices[other_pipe_position];

                let other_pipe = &self.pipes[other_pipe_position];

                graph.add_edge(node_indice, other_node_index, ());
                true
            };

            match pipe.pipe_type {
                PipeType::Vertical => {
                    if pipe.y > 0 {
                        add_edge(pipe.x, pipe.y - 1);
                    }
                    if pipe.y < self.height - 1 {
                        add_edge(pipe.x, pipe.y + 1);
                    }
                },
                PipeType::Horizontal => {
                    if pipe.x > 0 {
                        add_edge(pipe.x - 1, pipe.y);
                    }
                    if pipe.x < self.width - 1 {
                        add_edge(pipe.x + 1, pipe.y);
                    }
                },
                PipeType::NorthEast => {
                    if pipe.y > 0 {
                        add_edge(pipe.x, pipe.y - 1);
                    }
                    if pipe.x < self.width - 1 {
                        add_edge(pipe.x + 1, pipe.y);
                    }
                },
                PipeType::NorthWest => {
                    if pipe.y > 0 {
                        add_edge(pipe.x, pipe.y - 1);
                    }
                    if pipe.x > 0 {
                        add_edge(pipe.x - 1, pipe.y);
                    }
                },
                PipeType::SouthEast => {
                    if pipe.y < self.height - 1 {
                        add_edge(pipe.x, pipe.y + 1);
                    }
                    if pipe.x < self.width - 1 {
                        add_edge(pipe.x + 1, pipe.y);
                    }
                },
                PipeType::SouthWest => {
                    if pipe.y < self.height - 1 {
                        add_edge(pipe.x, pipe.y + 1);
                    }
                    if pipe.x > 0 {
                        add_edge(pipe.x - 1, pipe.y);
                    }
                },
                PipeType::Starting => {
                    if pipe.y > 0 {
                        add_edge(pipe.x, pipe.y - 1);
                    }
                    if pipe.y < self.height - 1 {
                        add_edge(pipe.x, pipe.y + 1);
                    }
                    if pipe.x < self.width - 1 {
                        add_edge(pipe.x + 1, pipe.y);
                    }
                    if pipe.x > 0 {
                        add_edge(pipe.x - 1, pipe.y);
                    }
                },
            }
        }

        graph
    }

    fn traverse_pipes_to_find_loop(&self) -> Vec<NodeIndex> {
        // We need to find the starting pipe, and then traverse the graph from there, eventually finding a loop back to the starting pipe.
        let graph = self.get_graph_map();

        let starting_pipe = self.pipes.iter().find(|pipe| pipe.is_starting).unwrap();

        let starting_node_index = graph.node_indices().find(|&node_index| graph[node_index] == *starting_pipe).unwrap();

        let mut visited = Vec::new();

        let mut queue = Vec::new();
        queue.push(starting_node_index);

        let mut steps = 0;

        while !queue.is_empty() {
            let node_index = queue.remove(0);

            if visited.contains(&node_index) {
                continue;
            }

            visited.push(node_index);

            let node = &graph[node_index];

            println!("Visiting node: {:?} from node {:?}", node, node_index);

            let mut neighbors = graph.neighbors(node_index).detach();

            while let Some(neighbor_index) = neighbors.next_node(&graph) {
                queue.push(neighbor_index);
            }

            steps += 1;
        }

        println!("Visited: {:?}", visited);
        println!("Steps: {}", steps);

        visited
    }
}


fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_example() {
        let input = include_str!("./example.txt");

        let pipes = Pipes::parse(input);

        assert_eq!(pipes.pipes.len(), 23);
    }

    #[test]
    fn test_example() {
        let input = include_str!("./example.txt");

        let pipes = Pipes::parse(input);

        let visited = pipes.traverse_pipes_to_find_loop();

        assert_eq!(visited.len(), 16);
    }
}
