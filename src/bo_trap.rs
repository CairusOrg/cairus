/*
input: list of edges
output: list of trapezoids

Sweep line is a horizontal line going from top (minimum y) to bottom (maximum y)

LineSegment defined in common_geometry.rs contains 2 points
edge is a line + top, bot, dir
    dir is a direction and should come from whatever initially 'drew' the lines
        in a pinch, we could generate dir from a sequence of line segments assuming
        each segment's first point is the previous segment's end point.
        dir should be +1 for a segment that is being drawn in the positive y direction,
        -1 for a segment being drawn in the negative y direction, and 0 for horizontal lines
        (horizontal lines don't actually matter since we will never cross them with a
        horizontal ray)
    For example: A clockwise drawn square would have a right side with a +1 dir,
        and a left side with a -1 dir.
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
/*
add_to_traps(SL_edge edge, float bot, int mask, traps *traps)
    //mask is 0xFFFFFFFF if using winding rule, 0x1 if using even/odd rule
    //only output traps with positive area
    if edge.deferred_trap.top >= bot
        return
    //count edge directions for ray right to infinity
    in_out = 0
    pos = edge.deferred_trap->right (or pos = edge->next? should be same, no?)
    while (pos != null)
        in_out += pos.dir
        pos = pos.deferred_trap->right (or pos = pos->next? should be same, no?)
    //in_out & mask is zero means do not fill (0 or even)
    if in_out & mask != 0
        LineSegment left, right
        left = edge->LineSegment
        right = edge.deferred_trap->right->LineSegment
        traps_push(left, right, edge.deferred_trap.top, bot)
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
    edge_right: Option<Box<Edge>>,
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
    use common_geometry::{LineSegment, Point};

    fn create_edge(x1: f32, y1: f32, x2: f32, y2:f32) -> Edge{
        Edge{
            line: LineSegment::new(x1, y1, x2, y2),
            top: y1,
            bottom: y2,
            direction: 1,

        }
    }

    fn create_start_event(x1: f32, y1: f32, x2:f32, y2:f32) -> Event {
        Event {
            edge_left: create_edge(x1, y1, x2, y2),
            edge_right: None,
            point: Point::create(x1, y1),
            event_type: EventType::Start
        }
    }

    #[test]
    fn event_compare(){

    }

}