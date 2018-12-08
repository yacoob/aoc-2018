use aoc::*;

// Tree elements. Technically don't need the parent link for part A, but I have a feeling[tm].
// expected_metadata_count is there only for sanity checking afterwards.
#[derive(Debug)]
struct Node {
    index: usize,
    parent: usize,
    expected_metadata_count: usize,
    metadata: Vec<usize>,
}

impl Node {
    fn new(index: usize, parent: usize, expected_metadata_count: usize) -> Node {
        Node {
            index,
            parent,
            expected_metadata_count,
            metadata: Vec::with_capacity(expected_metadata_count),
        }
    }

    fn verify_metadata(&self) {
        assert_eq!(self.metadata.len(), self.expected_metadata_count);
    }
}

// Structure for parser stack. As we go through the input, we'll be trying to parse the numbers
// into one of the Elements:
enum Element {
    // A new Node, with specified index.
    NodeElement(usize),
    // A metadata entry for Node of specific index.
    MetadataElement(usize),
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
    // Parser stack. We'll push to it things we expect to see next in the input.
    let mut expected_elements: Vec<Element> = Vec::new();
    // Allocate the root node, and push expectations about it to the stack.
    nodes.push(Node::new(0, 0, 0));
    expected_elements.push(Element::NodeElement(0));
    // Are we expecting anything?
    while !expected_elements.is_empty() {
        match expected_elements.pop().unwrap() {
            // We're expecting data for a Node with specified index and parent.
            Element::NodeElement(index) => {
                let kid_count = numbers.pop().unwrap() as usize;
                let metadata_count = numbers.pop().unwrap() as usize;
                // Update metadata count expectations in the Node.
                nodes[index].expected_metadata_count = metadata_count;
                // Push our expectations about upcoming elements on the stack. Note the reverse
                // order; metadata entries will come last, so we push it first.
                for _ in 1..=metadata_count {
                    expected_elements.push(Element::MetadataElement(index));
                }
                // Allocate new expected kids.
                let nodes_before_adding_kids = nodes.len();
                for _ in 1..=kid_count {
                    let kid_index = nodes.len();
                    nodes.push(Node::new(kid_index, index, 0));
                }
                // Add expectations about kid nods. Again, reversed order.
                for kid_index in (nodes_before_adding_kids..nodes.len()).rev() {
                    expected_elements.push(Element::NodeElement(kid_index));
                }
            }
            // We're expecting metadata element for node[i]
            Element::MetadataElement(i) => {
                let metadata_entry = numbers.pop().unwrap();
                nodes[i].metadata.push(metadata_entry);
            }
        }
    }
    // At this point tree should be complete, and all input should have been consumed.
    assert!(numbers.is_empty());
    // All Nodes should have gotten as many metadata entries as they initially expected.
    nodes.iter().for_each(|n| n.verify_metadata());
    nodes
}

fn part1(nodes: &[Node]) -> usize {
    // FIXME: why do I need a type hint for that first sum? metadata is typed, isn't that enough?
    nodes.iter().map(|n| n.metadata.iter().sum::<usize>()).sum()
}

// fn part2(foo: &i32) -> i32 {
//     *foo
// }

fn main() {
    let filename = "inputs/08";
    let input = read_file(filename);
    let nodes = parse_input(&input);

    let answer1 = part1(&nodes);
    assert_eq!(answer1, 40746);
    println!("Sum of all metadata entries: {}", answer1);

    // let answer2 = part2(&foo);
    // assert_eq!(answer2, 3671);
    // println!("Part 2: {}", answer2);
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

    // #[test]
    // fn test_part2() {
    //     let lyrics = parse_input(INPUT);
    //     assert_eq!(part1(&lyrics), 94);
    // }
}
