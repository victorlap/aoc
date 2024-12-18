export type PriorityQueueElement<T> = { element: T, priority: number }

export class PriorityQueue<T> {
  items: PriorityQueueElement<T>[] = [];

  enqueue(element: T, priority: number) {
    const queueElement = {
      element: element,
      priority: priority,
    };
    let added = false;
    for (let i = 0; i < this.items.length; i++) {
      if (queueElement.priority < this.items[i].priority) {
        this.items.splice(i, 0, queueElement);
        added = true;
        break;
      }
    }
    if (!added) {
      this.items.push(queueElement);
    }
  };

  dequeue() {
    if (this.isEmpty()) {
      throw "Underflow";
    }
    return this.items.shift()!;
  };

  isEmpty() {
    return this.items.length === 0
  }

}
