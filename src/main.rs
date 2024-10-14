/*
    Author: Colby McClure
    CS 490 Pgm 2
    Date: 10/23/2024
    Dependency: rand = "0.8.5"
    Environment: VS Code on Windows 11 
*/

// Needed crates and dependencies
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

// The Ord trait is used to compare two instances of a type
// For this use case, we want to compare the priority field of two Process instances
// We reverse the order to create a min-heap
impl Ord for Process {

    // Use the cmp method to compare the priority field of two Process instances
    // The output is an Ordering enum that represents the ordering of the two instances
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {

        // Reverse the order to create a min-heap
        other.priority.cmp(&self.priority)
    }
}

// The PartialOrd trait is used to compare two instances of a type, but allows for the possibility that the comparison might fail
// For this use case, we want to compare the priority field of two Process instances
// We reverse the order to create a min-heap
impl PartialOrd for Process {

    // Use the partial_cmp method to compare the priority field of two Process instances
    // The output is an Option enum that contains the ordering of the two instances
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {

        // Reverse the order to create a min-heap
        Some(other.priority.cmp(&self.priority))
    }
}

// The Eq trait is used to compare two instances of a type for equality
// We keep this empty since the Eq trait is automatically implemented for types that implement the PartialEq trait
impl Eq for Process {}

// The PartialEq trait is used to compare two instances of a type for equality
// For this use case, we want to compare the priority field of two Process instances
impl PartialEq for Process {

    // Use the eq method to compare the priority field of two Process instances
    // The output is a boolean value that represents the equality of the two instances
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

fn main() {

    // Initialize a mutable variable to store the number of process nodes to generate
    let mut nodes = String::new();
    println!("Hello! Please enter a number of process nodes to generate: ");

    // Read user input into the nodes variable
    io::stdin()
        .read_line(&mut nodes)
        .expect("Failed to read input");

    // Parse and validate user input
    // If the input is invalid, print an error message and end the program
    let nodes: i32 = match nodes.trim().parse() {
        Ok(num) => num,

        // Catch all case for invalid input
        Err(_) => {
            println!("Invalid input. Please try again.");
            return;
        },
    };

    // Initialize a BinaryHeap and VecDeque to store the process nodes
    let mut binary_heap: BinaryHeap<Rc<Process>> = BinaryHeap::new();
    let mut vec_deque: VecDeque<Rc<Process>> = VecDeque::new();

    println!("Generating {} process nodes for both a Queue and Binary MinHeap", nodes);

    // For each process node... 
    for i in 0..nodes {

        // Instantiate a Process struct with random values for priority and sleep_time
        // The description field is set to "Process Node i" where i is the current process node
        // In order to retain ownership of the Process struct, we wrap it in an Rc (Reference Counted) smart pointer
        // This pointer allows us to clone the Process struct and store it in both the BinaryHeap and VecDeque
        let process = Rc::new(Process {
            id: i,

            // Using the rand crate to generate random values for priority and sleep_time
            priority: rand::thread_rng().gen_range(0..100),
            sleep_time: rand::thread_rng().gen_range(100..2000),
            description: format!("Process Node {}", i),
        });
        
        // Push the cloned Process struct into the BinaryHeap and VecDeque
        // Since we are using the Rc smart pointer, we are only cloning the reference to the Process struct
        binary_heap.push(process.clone());
        vec_deque.push_back(process.clone());
    }

    println!("Verifying the contents of the Queue and Binary MinHeap");

    // Check if the BinaryHeap and VecDeque contain the correct number of nodes
    // If there is a mismatch, print an error message and end the program
    if binary_heap.len() != nodes as usize {
        println!("CRITICAL ERROR: Binary Heap does not contain the correct number of nodes");
        return;
    }

    if vec_deque.len() != nodes as usize {
        println!("CRITICAL ERROR: VecDeque does not contain the correct number of nodes");
        return;
    }

    // Output relevant information about the BinaryHeap and VecDeque
    println!("MinHeap contains {} nodes", binary_heap.len());
    println!("Queue contains {} nodes", vec_deque.len());
    println!("\nNow printing the contents of the Queue and Binary MinHeap\n");

    // Pop the Process nodes from the BinaryHeap and VecDeque and print their details
    println!("VecDeque: ");

    // Iterate through the VecDeque and pop each element from the front
    while let Some(process) = vec_deque.pop_front() {
        println!("Process ID: {}, Priority: {}, Sleep Time: {}, Description: {}", process.id, process.priority, process.sleep_time, process.description);
    }

    println!("\n");

    println!("Binary Heap: ");

    // Pop the Process nodes from the BinaryHeap and print their details
    while let Some(process) = binary_heap.pop() {
        println!("Process ID: {}, Priority: {}, Sleep Time: {}, Description: {}", process.id, process.priority, process.sleep_time, process.description);
    }

    println!("\n");
    println!("Goodbye!");
}
