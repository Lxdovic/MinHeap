# MinHeap - A MinHeap implementation in Rust

This is a simple MinHeap implementation in Rust. Made for learning purposes.

## What is min heap?

A min heap is a binary tree with the following properties:

- It is a complete tree (all levels are completely filled except possibly for the last level, which is filled from left to right)
- The value of each node is greater than or equal to the value of its parent, with the minimum-value element at the root.

![Min Heap](docs/Min-heap.png)

### Insertion

When inserting a new element into the heap, it is added to the bottom level, in the leftmost available space. Then, the heap is reordered by swapping the new element with its parent until the heap property is restored.
In a worst-case scenario, the new element will have to be swapped with its parent all the way to the root of the heap, which will take O(log n) time.

### Deletion

When deleting the minimum element, the last element is moved to the root, and then the heap is reordered by swapping the new root with its children until the heap property is restored.
In a worst-case scenario, the new root will have to be swapped with one of its children all the way to the bottom of the heap, which will take O(log n) time.

## Usage

```rust
use min_heap::MinHeap;

fn main() {
    let mut heap = MinHeap::new();
    
    heap.push(5);
    heap.push(3);
    heap.push(7);
    heap.push(1);
    heap.push(2);
    heap.push(6);
    heap.push(4);

    assert_eq!(heap.pop(), Some(1));
    assert_eq!(heap.pop(), Some(2));
    assert_eq!(heap.pop(), Some(3));
    assert_eq!(heap.pop(), Some(4));
    assert_eq!(heap.pop(), Some(5));
    assert_eq!(heap.pop(), Some(6));
    assert_eq!(heap.pop(), Some(7));
    assert_eq!(heap.pop(), None);
}
```
