/*
input: list of edges
output: list of trapezoids

Sweep line is a horizontal line going from top (minimum y) to bottom (maximum y)

LineSegment defined in common_geometry.rs contains 2 points
edge is a line + top, bot, dir
SL_edge has edge + *prev, *next, *colinear, deferred_trap (top, *right)

1. build event queue (EQ) (BST?)
    add event for each endpoint of lines in edge list.
        min(y of points) is START, max is END
    event is a point and associated edge or two and an enum event type
    sort events by point.y first, then by edge (top bottom, left right)

2. initialize sweep line list (SLL)
    SLL starts empty. Contains SL_edges. Is doubly linked list
    SL has *head, y, *current_SL_edge
    ? what about multiple lines intersecting at the same point?

while EQ not empty:
    Pop event off EQ.
    Set SL.y = event.y
    Process event:
        case: event.type = start
            insert event.edge into SLL (build SL_edge)
                building SL_edge:
                    SL_edge->edge = event.edge
                    if SL_edge->next != null start new trap:
                        SL_edge.deferred_trap->right = SL_edge->next.edge
                        SL_edge.deferred_trap.top = SL.y
                    if SL_edge->prev.deferred_trap.right != null (edge to left has
                                                deferred trap)
                        add_to_traps(SL_edge->prev, SL.y)
                    SL_edge->prev.deferred_trap.right = SL_edge
                    SL_edge->prev.deferred_trap.top = SL.y
            check if SL_edge.prev intersects with SL_edge
                add intersection to EQ
            check if SL_edge.next intersects with SL_edge
                add intersection to EQ (future? current?)

        case: event.type = end
            if SL_edge->prev intersects with SL_edge->next
                add intersection to EQ if it isn't already there
            if SL_edge.deferred_trap->right != null
                add_to_traps(SL_edge, SL.y)
            if SL_edge->prev.deferred_trap->right != null (should never be null
                                prob just check SL_edge->prev != null)
                add_to_traps(SL_edge->prev, SL.y)
                SL_edge->prev.deferred_trap->right = SL_edge.deferred_trap->right
                SL_edge->prev.deferred_trap.top = SL.y
            remove SL_edge from SLL:
                SL_edge->prev = SL_edge->next
                SL_edge->next = SL_edge->prev

        case: event.type = intersection
            if SL_edgeL.deferred_trap->right != null (should be SL_edgeR.edge)
                add_to_traps(SL_edgeL, SL.y)
            SL_edgeL.deferred_trap->right = SL_edgeR.deferred_trap->right
            SL_edgeL.deferred_trap.top = SL.y
            if SL_edgeR.deferred_trap->right != null
                add_to_traps(SL_edgeR, SL.y)
            SL_edgeR.deferred_trap->right = SL_edgeL->edge
            SL_edgeR.deferred_trap.top = SL.y
            if SL_edgeL->prev.deferred_trap->right != null (should be SL_edgeL.edge)
                add_to_traps(SL_edgeL->prev, SL.y)
            SL_edgeL->prev.deferred_trap->right = SL_edgeR->edge
            SL_edgeL->prev.deferred_trap.top = SL.y
            swap SL_edgeL and SL_edgeR:
                SL_edgeL->prev->next = SL_edgeR (if L->prev == null, SL->head = R)
                SL_edgeR->prev = SL_edgeL->prev
                SL_edgeL->next = SL_edgeR->next
                SL_edgeL->prev = SL_edgeR
                SL_edgeR->next->prev = SL_edgeL (if R->next != null)
                SL_edgeR->next = SL_edgeL
            check if SL_edgeR.prev intersects with SL_edgeR
                add intersection to EQ
            check if SL_edgeL.next intersects with SL_edgeL
                add intersection to EQ

In case of multiple lines crossing at same intersection point we have a couple problems:
    1. if order of event insertion is wrong, we may end up with non-adjacent edges in SLL being
        swapped
    2. we end up in an infinite loop adding the same intersections to the event queue over and over
does slope of lines help with this? investigate cairo code...

*/

use common_geometry::{Point, LineSegment};
use std::cmp::Ordering;
extern crate linked_list;

pub enum EventType {
    Start,
    End,
    Intersection
}

pub struct Edge {
    line: LineSegment,
    top: f32, // highest y value
    bottom: f32, // lowest y value
    direction: i32, // positive or negative
}

pub struct Event {
    edge_left: Edge,
    edge_right: Edge,
    point: Point,
    event_type: EventType

}

impl PartialOrd for Event {
    fn partial_cmp(&self, other:&Event) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Event {
    fn cmp(&self, other:&Event) -> Ordering {
        let top_compare = match self.edge_left.top.partial_cmp(&other.edge_left.top){
            Some(val) => val,
            None => Ordering::Equal,
        };
        if top_compare == Ordering::Less {
                return Ordering::Less
        }
        Ordering::Greater
    }
}


impl PartialEq for Event {
    fn eq(&self, other:&Event) -> bool {
        true
    }
}

impl Eq for Event {}

#[cfg(test)]
mod tests {
    use super::{EventType, Edge, Event};

    #[test]
    fn event_compare(){

    }

}