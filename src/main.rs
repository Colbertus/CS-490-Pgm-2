use std::io;
use std::collections::BinaryHeap;
use std::collections::VecDeque;
use rand::Rng;
use std::rc::Rc;

// The Process struct contains an id, priority, sleep_time, and description field
struct Process {
    id: i32,
    priority: i32,
    sleep_time: i32,
    description: String,
}

// In order to use the Process struct in a BinaryHeap, we need to implement the Ord, PartialOrd, Eq, and PartialEq traits
impl Ord for Process {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverse the order to create a min-heap
        other.priority.cmp(&self.priority)
    }
}

impl PartialOrd for Process {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Process {}

impl PartialEq for Process {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

fn main() {
    let mut nodes = String::new();
    println!("Hello! Please enter a number of process nodes to generate: ");


    io::stdin()
        .read_line(&mut nodes)
        .expect("Failed to read input");

    let nodes: i32 = match nodes.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please try again.");
            return;
        },
    };

    let mut binary_heap: BinaryHeap<Rc<Process>> = BinaryHeap::new();
    let mut vec_deque: VecDeque<Rc<Process>> = VecDeque::new();

    println!("Generating {} process nodes for both a Queue and Binary MinHeap", nodes);

    for i in 0..nodes {

        let process = Rc::new(Process {
            id: i,
            priority: rand::thread_rng().gen_range(0..100),
            sleep_time: rand::thread_rng().gen_range(100..2000),
            description: format!("Process Node {}", i),
        });
        

        binary_heap.push(process.clone());
        vec_deque.push_back(process.clone());
    }

    println!("Verifying the contents of the Queue and Binary MinHeap");

    if binary_heap.len() != nodes as usize {
        println!("Binary Heap does not contain the correct number of nodes");
        return;
    }

    if vec_deque.len() != nodes as usize {
        println!("VecDeque does not contain the correct number of nodes");
        return;
    }

    println!("Both data structures contain the correct number of nodes");
    println!("\nNow printing the contents of the Queue and Binary MinHeap\n");

    println!("Binary Heap: ");
    while let Some(process) = binary_heap.pop() {
        println!("Process ID: {}, Priority: {}, Sleep Time: {}, Description: {}", process.id, process.priority, process.sleep_time, process.description);
    }

    println!("\n");

    println!("VecDeque: ");
    while let Some(process) = vec_deque.pop_front() {
        println!("Process ID: {}, Priority: {}, Sleep Time: {}, Description: {}", process.id, process.priority, process.sleep_time, process.description);
    }

    println!("\n");
    println!("Goodbye!");
}
