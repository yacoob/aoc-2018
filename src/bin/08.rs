use aoc::*;

// Tree elements.
#[derive(Debug)]
struct Node {
    index: usize,
    children: Vec<usize>,
    metadata: Vec<usize>,
    value: Option<usize>,
    // We use expected_metadata_count only for sanity checking afterwards.
    expected_metadata_count: usize,
}

impl Node {
    fn new(index: usize, expected_metadata_count: usize) -> Node {
        Node {
            index,
            children: Vec::new(),
            metadata: Vec::with_capacity(expected_metadata_count),
            value: None,
            expected_metadata_count,
        }
    }

    fn verify_metadata(&self) {
        assert_eq!(self.metadata.len(), self.expected_metadata_count);
    }
}

// Structure for parser stack. As we go through the input, we'll be either trying to parse the numbers
// into a Node or metadata entrye, or calculate Node's value.
#[derive(Debug)]
enum Expectation {
    // A new Node, with specified index.
    NodeElement(usize),
    // A metadata entry for Node of specific index.
    MetadataElement(usize),
    // Request to calculate value for Node of specific index.
    EvaluateNode(usize),
}

fn parse_input(input: &str) -> Vec<Node> {
    let mut numbers: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    // Just for convenience, so I can use push/pop.
    numbers.reverse();
    // Minimal tree is one node, (0, 0).
    assert!(numbers.len() >= 2);
    // Nodes storage. This is our tree, just serialized in a vector.
    let mut nodes: Vec<Node> = Vec::with_capacity(numbers.len() / 2);
    // Parser stack. We'll push to it things we expect to happen next.
    let mut expectations: Vec<Expectation> = Vec::new();
    // Allocate the root node, and push an expectation to see it in the input to the stack.
    nodes.push(Node::new(0, 0));
    expectations.push(Expectation::NodeElement(0));
    // Are we expecting anything?
    while !expectations.is_empty() {
        match expectations.pop().unwrap() {
            // We're expecting data for a node[i]. It should already have been allocataed by its
            // parent.
            Expectation::NodeElement(i) => {
                let kid_count = numbers.pop().unwrap() as usize;
                let metadata_count = numbers.pop().unwrap() as usize;
                // Update metadata count expectations in the Node.
                nodes[i].expected_metadata_count = metadata_count;
                // Push an evaluation request for this node. It'll complete once we have all child
                // nodes and all metadata.
                expectations.push(Expectation::EvaluateNode(i));
                // Push our expectations about upcoming elements on the stack. Note the reverse
                // order; metadata entries will come last, so we push it first.
                for _ in 1..=metadata_count {
                    expectations.push(Expectation::MetadataElement(i));
                }
                // Allocate new expected kids.
                let nodes_before_adding_kids = nodes.len();
                for _ in 1..=kid_count {
                    let kid_index = nodes.len();
                    nodes.push(Node::new(kid_index, 0));
                    // Add new kid to list of parent's children.
                    nodes[i].children.push(kid_index);
                }
                // Add expectations about kid nods. Again, reversed order.
                for kid_index in (nodes_before_adding_kids..nodes.len()).rev() {
                    expectations.push(Expectation::NodeElement(kid_index));
                }
            }
            // We're expecting metadata element for node[i].
            Expectation::MetadataElement(i) => {
                let metadata_entry = numbers.pop().unwrap();
                nodes[i].metadata.push(metadata_entry);
            }
            // We're expecting to calculate value of node[i].
            Expectation::EvaluateNode(i) => {
                // FIXME: any sane way of saying:
                // let node = &mut nodes[i];
                // and using that instead of nodes[i] below, without tripping the borrowchecker?
                if nodes[i].children.is_empty() {
                    // No children? Just sum the metadata values.
                    nodes[i].value = Some(nodes[i].metadata.iter().sum());
                } else {
                    // If there are children, iterate through metadata, sum children they
                    // reference.
                    let mut sum = 0;
                    // FIXME:is "iter() and *m" idiomatic here, or is a different approach better?
                    for m in nodes[i].metadata.iter() {
                        // Which child are we referencing?
                        match nodes[i].children.get(m - 1) {
                            Some(child_node_index) => {
                                // By the time of this calculation, referenced Node should exist
                                // and has its value ready.
                                assert!(*child_node_index < nodes.len());
                                assert!(nodes[*child_node_index].value.is_some());
                                sum += nodes[*child_node_index].value.unwrap()
                            }
                            None => continue,
                        }
                    }
                    nodes[i].value = Some(sum);
                }
            }
        }
    }
    // At this point tree should be complete, and all input should have been consumed.
    assert!(numbers.is_empty());
    // All Nodes should have gotten as many metadata entries as they initially expected.
    nodes.iter().for_each(|n| n.verify_metadata());
    // assert!(false);
    nodes
}

fn part1(nodes: &[Node]) -> usize {
    // FIXME: why do I need a type hint for that first sum? metadata is typed, isn't that enough?
    nodes.iter().map(|n| n.metadata.iter().sum::<usize>()).sum()
}

fn part2(nodes: &[Node]) -> usize {
    nodes[0].value.unwrap()
}

fn main() {
    let nodes = parse_input(&read_file("inputs/08"));

    let answer1 = part1(&nodes);
    assert_eq!(answer1, 40746);
    println!("Sum of all metadata entries: {}", answer1);

    let answer2 = part2(&nodes);
    assert_eq!(answer2, 37453);
    println!("Value of the root node: {}", answer2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2
"#;

    #[test]
    fn test_part1() {
        let nodes = parse_input(INPUT);
        assert_eq!(part1(&nodes), 138);
    }

    #[test]
    fn test_part2() {
        let nodes = parse_input(INPUT);
        assert_eq!(part2(&nodes), 66);
    }
}
