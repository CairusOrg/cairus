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
use std::clone::Clone;
use trapezoid_rasterizer::Trapezoid;
extern crate linked_list;
use self::linked_list::LinkedList;

#[derive(Eq, PartialEq, Debug)]
pub enum EventType {
    Start,
    End,
    Intersection
}

impl PartialOrd for EventType {
    fn partial_cmp(&self, other: &EventType) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for EventType {
    fn cmp(&self, other: &EventType) -> Ordering {
        match *self {
            EventType::Start =>
                match *other {
                    EventType::Start => Ordering::Equal,
                    EventType::End => Ordering::Greater,
                    EventType::Intersection => Ordering::Greater,
                },
            EventType::End =>
                match *other {
                    EventType::Start => Ordering::Less,
                    EventType::End => Ordering::Equal,
                    EventType::Intersection => Ordering::Less,
                },
            EventType::Intersection =>
                match *other {
                    EventType::Start => Ordering::Less,
                    EventType::End => Ordering::Greater,
                    EventType::Intersection => Ordering::Equal,
                },
        }
    }
}

#[derive(Copy)]
pub struct Edge {
    line: LineSegment,
    top: f32, // highest y value
    bottom: f32, // lowest y value
    direction: i32, // positive or negative
}

impl Clone for Edge {
    fn clone(&self) -> Edge { *self }
}


pub struct Event {
    edge_left: Edge,
    edge_right: Option<Box<Edge>>,
    point: Point,
    event_type: EventType
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Event) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Event) -> Ordering {
        let y_compare = match self.point.y.partial_cmp(&other.point.y){
            Some(val) => val,
            None => Ordering::Equal, // We choose an Ordering that isn't Ordering::Less because
                                     // this will cause these events to be compared by other fields
        };

        if y_compare != Ordering::Equal   {
                return y_compare
        }

        let x_compare = match self.point.x.partial_cmp(&other.point.x){
            Some(val) => val,
            None => Ordering::Equal, // We choose an Ordering that isn't Ordering::Less because
                                     // this will cause these events to be compared by other fields
        };

        if x_compare != Ordering::Equal   {
                return x_compare
        }

        let type_compare = self.event_type.cmp(&other.event_type);
        if type_compare == Ordering::Equal {
            return Ordering::Greater
        }
        type_compare
    }
}


impl PartialEq for Event {
    fn eq(&self, other:&Event) -> bool {
        true
    }
}

impl Eq for Event {}

impl Event {
    fn new(edge_left: Edge, point: &Point, event_type: EventType) -> Event {
        Event {
            point: *point,
            edge_left: edge_left,
            edge_right: None,
            event_type: event_type,
        }
    }
}

fn event_list_from_edges(edges: Vec<Edge>) -> Vec<Event> {
    let mut events = Vec::new();
    for edge in edges {
        if edge.top == edge.bottom {
            // Is horizontal
            if edge.line.point1.x < edge.line.point2.x {
                // let start_event = Event::new();
                events.push(Event::new(edge,
                                       &Point::create(edge.line.point1.x, edge.line.point1.y),
                                       EventType::Start));
                events.push(Event::new(edge,
                                       &Point::create(edge.line.point2.x, edge.line.point2.y),
                                       EventType::End));
            }
            else {
                events.push(Event::new(edge,
                                       &Point::create(edge.line.point2.x, edge.line.point2.y),
                                       EventType::Start ));
                events.push(Event::new(edge,
                                       &Point::create(edge.line.point1.x, edge.line.point1.y),
                                       EventType::End ));
            }
        }

        if edge.top == edge.line.point1.y {
            // Point1 is start event
            events.push(Event::new(edge,
                                   &Point::create(edge.line.point1.x, edge.line.point1.y),
                                   EventType::Start ));
            events.push(Event::new(edge,
                                   &Point::create(edge.line.point2.x, edge.line.point2.y),
                                   EventType::End ));

        } else {
            // Point2 is start event
            events.push(Event::new(edge,
                                   &Point::create(edge.line.point2.x, edge.line.point2.y),
                                   EventType::Start ));
            events.push(Event::new(edge,
                                   &Point::create(edge.line.point1.x, edge.line.point1.y),
                                   EventType::End ));
        }
    }
    events.sort();
    events
}

pub struct ScanLineEdge {
    top: f32,
    line: LineSegment,
    right: Option<Box<LineSegment>>,
}

impl ScanLineEdge {
    fn new(top: f32, line: LineSegment) -> ScanLineEdge {
        ScanLineEdge {
            top: top,
            line: line,
            right: None,
        }
    }
}

pub fn scan(edges: Vec<Edge>) -> Vec<Trapezoid> {
    // Create the empty Scan Line Linked List
    let mut sl_list: LinkedList<ScanLineEdge> = LinkedList::new();
    // Create the list of events
    let mut events = event_list_from_edges(edges);
    // Keep looping until the Event List is empty
    while !events.is_empty() {
        // Get the current event
        let event = events.remove(0);
        // Set the scan line to the events y value
        let scan_line = event.point.y;
        // Process Event
        if event.event_type == EventType::Start{
            // create a new node and add it to the list
            let mut sl_edge = ScanLineEdge::new(scan_line, event.edge_left.line);
            // Insert the node into the linked list. Need to work on the logic for where to add it.
            sl_list.push_back(sl_edge);
            println!("Added Start to the scan line at y: {}", scan_line);
        }
        else if event.event_type == EventType::End {

        }

        println!("Scan Line: {}", scan_line);
    }


   Vec::new()
}


#[cfg(test)]
mod tests {
    use super::*;
    use common_geometry::{LineSegment, Point};
    use std::cmp::Ordering;

    fn create_edge(x1: f32, y1: f32, x2: f32, y2:f32) -> Edge{
        let mut top = y1;
        let mut bottom = y2;
        if y1 > y2 {
            top = y2;
            bottom = y1;
        }

        Edge{
            line: LineSegment::new(x1, y1, x2, y2),
            top: top,
            bottom: bottom,
            direction: 1,

        }
    }

    fn create_start_event(x1: f32, y1: f32, x2:f32, y2:f32) -> Event {
        let edge = create_edge(x1, y1, x2, y2);
        let point = Point::create(x1, y1);
        Event::new(edge, &point, EventType::Start)
    }

    #[test]
    fn event_compare_y_lesser(){
        let lesser = create_start_event(0., 0., 3., 3.);
        let greater = create_start_event(1., 1., 0., 2.);
        assert_eq!(lesser.cmp(&greater), Ordering::Less);
    }

    #[test]
    fn event_compare_y_greater(){
        let lesser = create_start_event(0., 0., 3., 3.);
        let greater = create_start_event(1., 1., 0., 2.);
        assert_eq!(greater.cmp(&lesser), Ordering::Greater);
    }

    #[test]
    fn event_compare_x_lesser(){
        let lesser = create_start_event(0., 0., 0., 0.);
        let greater = create_start_event(1., 0., 0., 0.);
        assert_eq!(lesser.cmp(&greater), Ordering::Less);
    }

    #[test]
    fn event_compare_x_greater(){
        let lesser = create_start_event(0., 0., 0., 0.);
        let greater = create_start_event(1., 0., 0., 0.);
        assert_eq!(greater.cmp(&lesser), Ordering::Greater);
    }

    #[test]
    fn event_compare_type_greater(){
        let dummy = create_start_event(0., 0., 0., 0.);
        assert_eq!(dummy.cmp(&dummy), Ordering::Greater);
    }

    #[test]
    fn event_sorting() {
        let mut event_list = vec![
            create_start_event(0., 1., 0., 3.),
            create_start_event(0., 0., 1., 2.),
            create_start_event(0., 0., 0., 1.)
        ];

        event_list.sort();
        assert_eq!(event_list.get(0).unwrap().edge_left.line.point2.y, 1.);
        assert_eq!(event_list.get(1).unwrap().edge_left.line.point2.y, 2.);
        assert_eq!(event_list.get(2).unwrap().edge_left.line.point2.y, 3.);
    }


    #[test]
    fn event_list_from_edges_sorted_test_size() {
        // Verify event list is the correct size
        let edges = vec![
            create_edge(3., 4., 1., 2.),
            create_edge(0., 1., 6., 6.),
            create_edge(0., 0., 5., 5.),
        ];

        let event_list = event_list_from_edges(edges);
        assert_eq!(event_list.len(), 6);
    }

    #[test]
    fn event_list_from_edges_sorted_test_order() {
        // Verify event list is the correct order
        let edges = vec![
        create_edge(3., 4., 1., 2.),
        create_edge(0., 1., 6., 6.),
        create_edge(0., 0., 5., 5.),
        ];

        let event_list = event_list_from_edges(edges);
        assert_eq!(event_list.get(0).unwrap().point, Point::create(0., 0.));
        assert_eq!(event_list.get(1).unwrap().point, Point::create(0., 1.));
        assert_eq!(event_list.get(2).unwrap().point, Point::create(1., 2.));
        assert_eq!(event_list.get(3).unwrap().point, Point::create(3., 4.));
        assert_eq!(event_list.get(4).unwrap().point, Point::create(5., 5.));
        assert_eq!(event_list.get(5).unwrap().point, Point::create(6., 6.));
    }

    #[test]
    fn event_list_from_edges_sorted_test_types() {
        // Verify event list events have the correct start/end types
        let edges = vec![
        create_edge(3., 4., 1., 2.),
        create_edge(0., 1., 6., 6.),
        create_edge(0., 0., 5., 5.),
        ];

        let event_list = event_list_from_edges(edges);
        assert_eq!(event_list.get(0).unwrap().event_type, EventType::Start);
        assert_eq!(event_list.get(1).unwrap().event_type, EventType::Start);
        assert_eq!(event_list.get(2).unwrap().event_type, EventType::Start);
        assert_eq!(event_list.get(3).unwrap().event_type, EventType::End);
        assert_eq!(event_list.get(4).unwrap().event_type, EventType::End);
        assert_eq!(event_list.get(5).unwrap().event_type, EventType::End);
    }


    #[test]
    fn event_constructor() {
        let edge = create_edge(0., 0., 0., 0.);
        let point = Point{x: 0., y: 0.};
        let event = Event::new(edge, &point, EventType::Start);
        assert_eq!(event.edge_left.line.point1, edge.line.point1);
        assert_eq!(event.point, point);
        assert_eq!(event.event_type, EventType::Start);
    }

    #[test]
    fn scan_test() {
        let edges = vec![
        create_edge(3., 4., 1., 2.),
        create_edge(0., 1., 6., 6.),
        create_edge(0., 0., 5., 5.),
        ];

        scan(edges);

    }
}
