struct MinHeap {
    pub length: usize,
    data: Vec<i32>
}

impl MinHeap {
    fn new() -> MinHeap {
        MinHeap {
            length: 0,
            data: Vec::new()
        }
    }

    pub fn insert(&mut self, value: i32) {
        self.data.push(value);
        self.length += 1;
        self.heapify_up(self.length - 1);
    }

    pub fn pop(&mut self) -> Option<i32> {
        if self.length == 0 { return None; }

        let out = self.data[0];
        self.length -= 1;

        if self.length == 0 {
            self.data = Vec::new();
            return Some(out);
        }

        self.data[0] = self.data[self.length];
        self.heapify_down(0);

        return Some(out);
    }

    fn heapify_down(&mut self, index:usize) {
        let left_index = MinHeap::left(index);
        let right_index =  MinHeap::right(index);

        if left_index >= self.length && right_index >= self.length {
            return;
        }

        let left_value = self.data[left_index];
        let right_value = self.data[right_index];
        let value = self.data[index];

        if left_value > right_value && value > right_value {
            self.data[right_index] = value;
            self.data[index] = right_value;
            self.heapify_down(right_index);
        }

        if left_value < right_value && value > left_value {
            self.data[left_index] = value;
            self.data[index] = left_value;
            self.heapify_down(left_index);
        }
    }

    fn heapify_up(&mut self, index: usize) {
        if index == 0 { return; }

        let parent_index = MinHeap::parent(index);
        let parent_value = self.data[parent_index];
        let value = self.data[index];

        if parent_value > value {
            self.data[parent_index] = value;
            self.data[index] = parent_value;
            self.heapify_up(parent_index);
        }
    }

    fn parent(index: usize) -> usize {
        (index - 1) / 2
    }

    fn left(index: usize) -> usize {
        2 * index + 1
    }

    fn right(index:usize) -> usize {
        2 * index + 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut heap = MinHeap::new();

        assert_eq!(heap.length, 0);

        heap.insert(5);
        heap.insert(3);
        heap.insert(69);
        heap.insert(420);
        heap.insert(4);
        heap.insert(1);
        heap.insert(8);
        heap.insert(7);

        assert_eq!(heap.length, 8);
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.length, 4);
        assert_eq!(heap.pop(), Some(7));
        assert_eq!(heap.pop(), Some(8));
        assert_eq!(heap.pop(), Some(69));
        assert_eq!(heap.pop(), Some(420));
        assert_eq!(heap.length, 0);
    }
}
