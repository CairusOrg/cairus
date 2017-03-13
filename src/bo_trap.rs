/*
 * Cairus - a reimplementation of the cairo graphics library in Rust
 *
 * Copyright © 2017 CairusOrg
 *
 * This library is free software; you can redistribute it and/or
 * modify it either under the terms of the GNU Lesser General Public
 * License version 2.1 as published by the Free Software Foundation
 * (the "LGPL") or, at your option, under the terms of the Mozilla
 * Public License Version 2.0 (the "MPL"). If you do not alter this
 * notice, a recipient may use your version of this file under either
 * the MPL or the LGPL.
 *
 * You should have received a copy of the LGPL along with this library
 * in the file LICENSE-LGPL-2_1; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Suite 500, Boston, MA 02110-1335, USA
 * You should have received a copy of the MPL along with this library
 * in the file LICENSE-MPL-2_0
 *
 * The contents of this file are subject to the Mozilla Public License
 * Version 2.0 (the "License"); you may not use this file except in
 * compliance with the License. You may obtain a copy of the License at
 * http://www.mozilla.org/MPL/
 *
 * This software is distributed on an "AS IS" basis, WITHOUT WARRANTY
 * OF ANY KIND, either express or implied. See the LGPL or the MPL for
 * the specific language governing rights and limitations.
 *
 * The Original Code is the cairus graphics library.
 *
 * Contributor(s):
 *  Troy Routley <routley@pdx.edu>
 *  Bobby Eshleman <bobbyeshleman@gmail.com>
 *  DJ Sabo <sabodj@pdx.edu>
 *
 */


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
use common_geometry::{Edge, Point, LineSegment};
use std::cmp::Ordering;
use std::clone::Clone;
use trapezoid_rasterizer::Trapezoid;
extern crate linked_list;
use self::linked_list::{LinkedList, Cursor};

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
#[derive(Debug)]
pub struct Event {
    edge_left: Edge,
    edge_right: Vec<Edge>,
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
        let y_compare = self.point.y.partial_cmp(&other.point.y).unwrap_or(Ordering::Equal);
        if y_compare != Ordering::Equal   {
                return y_compare
        }

        let x_compare = self.point.x.partial_cmp(&other.point.x).unwrap_or(Ordering::Equal);
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

// Need to check this code
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
            edge_right: Vec::new(),
            event_type: event_type,
        }
    }
    // Creates a new Event for an Intersection
    fn new_intersection(edge_left: Edge, edge_right: Edge, point: &Point) -> Event {
        Event {
            point: *point,
            edge_left: edge_left,
            edge_right: vec![edge_right],
            event_type: EventType::Intersection,
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
                                       &Point::new(edge.line.point1.x, edge.line.point1.y),
                                       EventType::Start));
                events.push(Event::new(edge,
                                       &Point::new(edge.line.point2.x, edge.line.point2.y),
                                       EventType::End));
            }
            else {
                events.push(Event::new(edge,
                                       &Point::new(edge.line.point2.x, edge.line.point2.y),
                                       EventType::Start ));
                events.push(Event::new(edge,
                                       &Point::new(edge.line.point1.x, edge.line.point1.y),
                                       EventType::End ));
            }
        }

        if edge.top == edge.line.point1.y {
            // Point1 is start event
            events.push(Event::new(edge,
                                   &Point::new(edge.line.point1.x, edge.line.point1.y),
                                   EventType::Start ));
            events.push(Event::new(edge,
                                   &Point::new(edge.line.point2.x, edge.line.point2.y),
                                   EventType::End ));

        } else {
            // Point2 is start event
            events.push(Event::new(edge,
                                   &Point::new(edge.line.point2.x, edge.line.point2.y),
                                   EventType::Start ));
            events.push(Event::new(edge,
                                   &Point::new(edge.line.point1.x, edge.line.point1.y),
                                   EventType::End ));
        }
    }
    events.sort();
    events
}

/// Defines a SweepLineEdge for our SweepLineList
///
/// The SweepLineEdges will be used to create trapezoids.
/// Top will be set by our SweepLine to mark the top of our trapezoid.
/// Left will be set based on the leftmost point of our line to determine where in our SweepLineList
///     we need to insert our SweepLineEdge. This is used for sorting our SweepLineList and is updated
///     when it intersects another line.
/// Line is our current line.
/// Note: We may need to add a Right (right: Option<Box<LineSegment>>) to track the right side of
///     our trapezoid but for now we will let the SweepLineList determine this based on if there is a
///     SweepLineEdge after the current SweepLineEdge in our SweepLineList.
#[derive(Debug, Copy, Clone)]
pub struct SweepLineEdge {
    trap_top: f32,
    left: f32,
    edge: Edge,
}

impl SweepLineEdge {
    fn new(trap_top: f32, left: f32, edge: Edge) -> SweepLineEdge {
        SweepLineEdge {
            trap_top: trap_top,
            left: left,
            edge: edge,
        }
    }
}

/// /sweep will loop over all of the Edges in the vector and build Trapezoids out of them.
pub fn sweep(edges: Vec<Edge>) -> Vec<Trapezoid> {
    // Create the empty sweep Line Linked List
    let mut sl_list: LinkedList<SweepLineEdge> = LinkedList::new();
    // Create a cursor to move over the list
    let mut cursor = sl_list.cursor();
    // Create the list of events
    let mut events = event_list_from_edges(edges);
    // Create empty traps list for eventual return
    let mut traps: Vec<Trapezoid> = Vec::new();
    // Keep looping until the Event List is empty
    while !events.is_empty() {
        // Get the current event
        let event = events.remove(0);
        // Set the sweep line to the events y value
        let sweep_line = event.point.y;

        // Process Event
        // START CASE
        if event.event_type == EventType::Start{
            println!("Starting START case for point: ({},{})", event.point.x, event.point.y);
            println!("Sweep Line is: {}", sweep_line );
            // find the left most point of the edge_left line
            let left = event.edge_left.line.min_x_point().x;
            // create a new node and add it to the list
            let mut sl_edge = SweepLineEdge::new(sweep_line, left, event.edge_left);
            // Set the cursor back to the beginning
            cursor.reset();
            if cursor.peek_next().is_none() {
                cursor.insert(sl_edge);
            } else {
                while find_line_place(event.point, event.edge_left, *cursor.peek_next().unwrap()) == Comparator::Less {
                    cursor.next();
                    if cursor.peek_next().is_none() {
                        break;
                    }
                }
                // **** ADD TRAPEZOID *****
                // If before we add our new sl_edge there is a previous and next we need to make a
                // new Trapezoid and set the prev top
                if cursor.peek_prev().is_some() && cursor.peek_next().is_some() {
                    // passing -1 for mask as winding rule default 0xFFFFFFFF
                    add_to_traps(&mut cursor, sweep_line, -1 , &mut traps);
                    cursor.peek_prev().unwrap().trap_top = sweep_line;
                }

                // Add the new edge to our sweep line list
                cursor.insert(sl_edge);

                // **** CHECK FOR INTERSECTIONS ****
                // Check to see if the new edge intersects with the previous or next
                // if it does after the current sweep line then we add it to our event list.
                // Check if there is an intersection with the left line, if yes check for interaction

                // Move the cursor to just before our newly inserted sweep line edge
                cursor.prev();
                // If it has a previous before our new event there way be an intersection
                if cursor.peek_prev().is_some() {
                    check_for_intersection(sweep_line, &mut cursor, &mut events);
                }
                // Move the cursor to after the newly added sweep line edge
                cursor.next();
                // If there is a sweep line edge after the new one, check for intersections
                if cursor.peek_next().is_some() {
                    check_for_intersection(sweep_line, &mut cursor, &mut events);
                }

            }
            println!("Finished START Case");
         //   println!("current x, y value: {} {}",cursor.next().unwrap().edge.line.current_x_for_y(sweep_line), sweep_line );
        }

        // END CASE
        else if event.event_type == EventType::End {
            println!("Starting END case for point: ({},{})", event.point.x, event.point.y);
            println!("Sweep Line is: {}", sweep_line );
            // when we call remove on the cursor it will remove the next element.
            // when we call cursor.next or cursor.prev it moves the cursor left or right
            // when we call cursor.peek_left or right it gets the next element without moving the cursor
            // the events will always be sorted by the current left point
            // We know what line to remove based on the current event which will tell us what that
            // left point will be

            // REMOVE FROM SL_LIST
            // if our event line is equal to our cursor_left line then see if our lines are equal, if yes remove
            // if no then we need to see which direction to move...
            // if our event line is greater then our cursor left line then we need to move right and repeat
            // if our event line is less then our cursor left line then we need to move left

            // Move the cursor to before the sweep line edge we wish to delete
            move_cursor_to_line(event.point, event.edge_left, &mut cursor);

            let line = cursor.peek_next().unwrap().edge.line.clone();
            println!("Cursor Next point is: ({},{})", line.current_x_for_y(sweep_line), sweep_line);

            // **** CREATE TRAPEZOIDS *****
            // we will be at the point of removal here, so we need to see about building trapezoids
            // before and after this point before we remove it. We will want to update the TOP of the
            // node before if we create a trapezoid
            if cursor.peek_prev().is_some() {
                // passing -1 for mask as winding rule default 0xFFFFFFFF
                println!("Calling add_to_traps for trap before current cursor");
                add_to_traps(&mut cursor, sweep_line, -1 , &mut traps);
                cursor.peek_prev().unwrap().trap_top = sweep_line;
            }
            if cursor.peek_next().is_some() {
                cursor.next();
                if cursor.peek_next().is_some() {
                    if cursor.peek_prev().is_none() {
                        println!("HOW IS THIS POSSIBLE??");
                    }
                    println!("Calling add_to_traps for trap after current cursor");
                    let line_before = cursor.peek_prev().unwrap().edge.line.clone();
                    let line_after = cursor.peek_next().unwrap().edge.line.clone();
                    println!("   Line Previous point is: ({},{})", line_before.current_x_for_y(sweep_line), sweep_line);
                    println!("   Line Next point is: ({},{})", line_after.current_x_for_y(sweep_line), sweep_line);
                    // passing -1 for mask as winding rule default 0xFFFFFFFF
                    add_to_traps(&mut cursor, sweep_line, -1, &mut traps);
                    cursor.peek_prev().unwrap().trap_top = sweep_line;
                }
                cursor.prev();
            }
            cursor.remove();

            // ****** CHECK FOR INTERSECTIONS ****
            // After we remove it we will want to see if there is any intersections with the lines
            // before and after the cursor. If yes, and it happens after our current y we add it to
            // our event list.
            if cursor.peek_prev().is_some() && cursor.peek_next().is_some() {
                check_for_intersection(sweep_line, &mut cursor, &mut events);
            }

            println!("Finished END Case");
        }


        // INTERSECT CASE
            else if event.event_type == EventType::Intersection {
                println!("Starting INTERSECT case for point: ({},{})", event.point.x, event.point.y);
                println!("Sweep Line is: {}", sweep_line);

                // move the cursor between the two edges
                // *** Issue: How do i access the element inside of a BOX? ***
                move_cursor_to_line(event.point, *event.edge_right.get(0).unwrap(), &mut cursor );
                let line_before = cursor.peek_prev().unwrap().edge.line.clone();
                let line_after = cursor.peek_next().unwrap().edge.line.clone();
                println!("   Line Previous point is: ({},{})", line_before.current_x_for_y(sweep_line), sweep_line);
                println!("   Line Next point is: ({},{})", line_after.current_x_for_y(sweep_line), sweep_line);
                // check for traps before
                println!("Starting trap checks");
                cursor.prev();
                if cursor.peek_prev().is_some() {
                    add_to_traps(&mut cursor, sweep_line, -1 , &mut traps);
                    cursor.peek_prev().unwrap().trap_top = sweep_line;
                }
                // check for traps between
                cursor.next();
                add_to_traps(&mut cursor, sweep_line, -1 , &mut traps);
                cursor.peek_prev().unwrap().trap_top = sweep_line;

                // check for traps after
                cursor.next();
                if cursor.next().is_some() {
                    add_to_traps(&mut cursor, sweep_line, -1 , &mut traps);
                    cursor.peek_prev().unwrap().trap_top = sweep_line;
                }
                println!("Ending trap checks");

                // move the cursor back between our edges
                cursor.prev();

                if  cursor.peek_prev().is_none() || cursor.peek_next().is_none() {
                    println! ("**** ERROR WHAT HAPPENED TO THE CURSOR ****");
                    move_cursor_to_line(event.point, *event.edge_right.get(0).unwrap(), &mut cursor );
                }
                let line_before = cursor.peek_prev().unwrap().edge.line.clone();
                let line_after = cursor.peek_next().unwrap().edge.line.clone();
                println!("   Line Previous point is: ({},{})", line_before.current_x_for_y(sweep_line), sweep_line);
                println!("   Line Next point is: ({},{})", line_after.current_x_for_y(sweep_line), sweep_line);

                // swap
                let swap_sl_edge = cursor.remove().unwrap();
                cursor.prev();
                cursor.insert(swap_sl_edge);

                // check for intersections before set
                cursor.prev();
                if cursor.peek_prev().is_some() {
                    check_for_intersection(sweep_line, &mut cursor, &mut events);
                }
                cursor.next();

                // check for intersections after set
                cursor.next();
                if cursor.peek_next().is_some() {
                    check_for_intersection(sweep_line, &mut cursor, &mut events);
                }

                println!("Finished INTERSECT Case");
            }


        // print the Sweep Line List
        cursor.reset();
        println!("***Printing Sweep Line List***");
        let mut index = 0;
        while cursor.peek_next().is_some(){
            let line = cursor.peek_next().unwrap().edge.line.clone();
            let top = cursor.peek_next().unwrap().trap_top;
            println!("     Index {}:  x:{}  Top:({},{})", index, line.current_x_for_y(sweep_line), line.current_x_for_y(top), top) ;
            index = index + 1;
            cursor.next();
        }
        println!("********");

        println!("EVENT COMPLETE at sweep: {}", sweep_line);
        println!("")
    }
    // Return the list of trapezoids
    traps
}

// Checks to see if we should add the intersection to the event list
// Expects the cursor to be between the two lines that we want to check for intersection
pub fn check_for_intersection(sweep_line: f32, cursor: &mut Cursor<SweepLineEdge>, events: &mut Vec<Event>)  {
    // Verifies there is a previous and next before we check for intersections
    if cursor.peek_prev().is_none() || cursor.peek_next().is_none() {
        return;
    }
    println!("Starting Intersection Checks");
    let next_line = &cursor.peek_next().unwrap().edge.line.clone();
    let result = cursor.peek_prev().unwrap().edge.line.intersection(next_line);
    // Add the event if it exists
    if result.is_some() {
        let point = result.unwrap();
        // if the event has already happened, do not add it
        if point.y <= sweep_line {
            println!("Ending Intersection Checks: No Intersection");
            return;
        }
        // if the intersection happens at the end of either line, do not add it
        if point == cursor.peek_prev().unwrap().edge.line.max_y_point() {
            println!("Ending Intersection Checks: No Intersection");
            return;
        }
        if point == cursor.peek_next().unwrap().edge.line.max_y_point() {
            println!("Ending Intersection Checks: No Intersection");
            return;
        }
        // add the intersection
        println!("Adding intersect to events");
        events.push(Event::new_intersection(cursor.peek_prev().unwrap().edge, cursor.peek_next().unwrap().edge, &point));
        events.sort();
        println!("Ending Intersection Checks: Intersect Added");
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum Comparator {
    Greater,
    Less,
    Equal,
    Empty,
}

// Searches the sweep line list for a line matching the one if the edge
// point: the current event point
// edge: the edge we are trying to find a match to
// cursor: will be set to the position before the edge that is equal
pub fn move_cursor_to_line(point: Point, edge:Edge, cursor: &mut Cursor<SweepLineEdge> ) {
    // If we are at the end of the list move one position back so we have something to compare
    if cursor.peek_next().is_none() {
        cursor.prev();
    }
    let mut result = Comparator::Empty;
    while result != Comparator::Equal {
        result = find_line_place(point, edge, *cursor.peek_next().unwrap());

        if result == Comparator::Equal {
            break;
        } else if result == Comparator::Greater {
            cursor.prev();
        } else if result == Comparator::Less {
            cursor.next();
        } else {
            break;
        }
    }

}

// need to rename function. it will compare a line to the next one in the list
// may want to pass in a point as well so that we can use this same function for insert
// Returns Equal if line and next_sl_edge.line are equal
// Returns Greater if Next current x is greater then events, if points are equal compares slopes
// Returns Less if Next current x is less then events, if points are equal compares slopes
pub fn find_line_place(point: Point, edge: Edge, next_sl_edge : SweepLineEdge) -> Comparator {
    let next_line = next_sl_edge.edge.line;
    // if the lines are the same line we return equal because we have a duplicate
    // will probably need to change this for intersections
    if edge.line == next_line {
//        println!("Lines are equal");
        return Comparator::Equal;
    }
    // Get the point on the next line for the current y value we are at since that is how the
    // linked list is sorted.
    let next_x = next_sl_edge.edge.line.current_x_for_y(point.y);
    // if the point is the same as the next point or lines intersect and we need to look at the
    // slope to determine the sorting order. We already know they have the same y value so we just
    // look at the x values
    if point.x == next_x {
        // compare the slopes of the lines
        if edge.line.slope() < next_line.slope() {
 //           println!("Points are equal, Next slope is greater then Events");
            return Comparator::Greater;
        }
        else {
 //           println!("Points are equal, Next slope is less then Events");
            return Comparator::Less;
        }
        // if the point is not on the nextLine we just need to see if it comes before or after
    } else if point.x < next_x {
 //       println!("Next current x is greater then Events point.x");
        return Comparator::Greater;
    } else {
 //       println!("Next current x is less then Events point.x");
        return Comparator::Less;
    }

}

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
// *** QUESTION: Do we need to reset the sl_edge top when we add to the traps so that
//               the next one continues where the last left off?
fn add_to_traps(cursor: &mut Cursor<SweepLineEdge>, bottom: f32, mask: i32, traps: &mut Vec<Trapezoid>) {
    println!("Starting add_to_traps");
    if cursor.peek_prev().is_none() || cursor.peek_next().is_none() {
        println!("Error: add_to_traps called when it shouldnt have");
    }
    // We unwrap because it should be considered a bug if this gets called when the value is
    // incorrect
    let mut sl_edge = *cursor.peek_prev().unwrap();

    if sl_edge.trap_top >= bottom {
        return;
    }

    let mut right = *cursor.peek_next().unwrap();
    let mut in_out = 0;
    let mut count = 0;
    println!("   Starting cursor count loop");
    while let Some(edge) = cursor.next() {
        count += 1;
        in_out += edge.edge.direction;
    }
    println!("   Ending cursor count loop");

    in_out &= mask;

    // Add a trapezoid if in_out isn't zero
    if in_out != 0 {
        let left = sl_edge.edge.line;
        let right = right.edge.line;
        let top_y = sl_edge.trap_top;
        let trap = bo_trap_from_lines(&left, &right, top_y, bottom);
        traps.push(trap)
    }
    //rewind cursor to starting position (+1 because loop advances past end)
    cursor.seek_backward(count+1);
    println!("Ending add_to_traps");
}

fn bo_trap_from_lines(left: &LineSegment,
                      right: &LineSegment,
                      top: f32,
                      bottom: f32) -> Trapezoid {
    println!("Starting Create Trap");
    let min_x = left.min_x_point().x.min(right.min_x_point().x);
    let max_x = left.max_x_point().x.min(right.max_x_point().x);
//    let top_line = LineSegment::new(min_x, top, max_x, top);
//    let bottom_line = LineSegment::new(min_x, bottom, max_x, bottom);
    println!("left line: {:?}", left);
    println!("right line: {:?}", right);
    println!("top: {} bottom: {}", top, bottom);
    let top_left = Point::new(left.current_x_for_y(top),top);
    let top_right = Point::new(right.current_x_for_y(top),top);
    let bottom_left = Point::new(left.current_x_for_y(bottom),bottom);
    let bottom_right = Point::new(right.current_x_for_y(bottom),bottom);

    println!("Ending Create Trap");
    Trapezoid::from_points(top_left, top_right, bottom_left, bottom_right)
}

#[cfg(test)]
mod tests {
    use super::*;
    use common_geometry::{Edge, Point, LineSegment};
    use std::cmp::Ordering;
    use trapezoid_rasterizer::Trapezoid;

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
        let point = Point::new(x1, y1);
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
            create_start_event(0., 2., 9., 9.),
            create_start_event(0., 1., 9., 9.),
            create_start_event(0., 3., 9., 9.)
        ];

        event_list.sort();
        assert_eq!(event_list.get(0).unwrap().point.y, 1.);
        assert_eq!(event_list.get(1).unwrap().point.y, 2.);
        assert_eq!(event_list.get(2).unwrap().point.y, 3.);
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
        assert_eq!(event_list.get(0).unwrap().point, Point::new(0., 0.));
        assert_eq!(event_list.get(1).unwrap().point, Point::new(0., 1.));
        assert_eq!(event_list.get(2).unwrap().point, Point::new(1., 2.));
        assert_eq!(event_list.get(3).unwrap().point, Point::new(3., 4.));
        assert_eq!(event_list.get(4).unwrap().point, Point::new(5., 5.));
        assert_eq!(event_list.get(5).unwrap().point, Point::new(6., 6.));
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
    fn sweep_test() {
        let edges = vec![
        create_edge(0., 0., 5., 5.),
        create_edge(3., 4., 1., 2.),
        create_edge(0., 1., 6., 6.),
        ];

        let traps = sweep(edges);
        assert_eq!(traps.len(), 2);
    }

    #[test]
    fn sweep_test_traps_one() {
        // Expected to make 1 trap with the 2 lines
        let edges = vec![
        create_edge(0., 0., 1., 4.),
        create_edge(2., 0., 3., 4.),
        ];

        let traps = sweep(edges);
        assert_eq!(traps.len(), 1);
    }

    #[test]
    fn sweep_test_traps_two_a() {
        // Expected to make 2 traps with the 3 lines
        let edges = vec![
        create_edge(0., 0., 1., 4.),
        create_edge(2., 0., 3., 4.),
        create_edge(4., 0., 5., 4.),
        ];

        let traps = sweep(edges);
        assert_eq!(traps.len(), 2);
    }

    #[test]
    fn sweep_test_traps_two_b() {
        // Expected to make 2 traps with the 4 lines because of the winding rule
        let edges = vec![
        create_edge(0., 0., 1., 7.),
        create_edge(2., 0., 3., 6.),
        create_edge(4., 0., 5., 5.),
        create_edge(6., 0., 7., 4.),
        ];

        let traps = sweep(edges);
 //       assert_eq!(traps.len(), 2);
    }

    #[test]
    fn sweep_test_intersect_two() {
        // Expected to make 2 traps with the 2 lines that cross
        // need to fix after the Intersection case is complete
        // test needs to be re-worked. It should produce 2 traps with this set of points.
        let edges = vec![
        create_edge(0., 0., 2., 4.),
        create_edge(2., 0., 0., 4.),
        ];

        let traps = sweep(edges);
        assert_eq!(traps.len(), 2);
    }

    // Tests that add_to_traps doesn't change the traps vector if the SweepLineEdge's top
    // is greater than the `bottom` arg passed in.
    #[test]
    fn add_to_traps_edge_top_gt_bottom() {
        // Setup
        let edge = SweepLineEdge {
            trap_top: 1.,
            left: 0.,
            edge: Edge {
                line: LineSegment::new(0., 0., 0., 0.),
                top: 0.,
                bottom: 0.,
                direction: 0
            }
        };

        // bottom is less than edge.top!
        let bottom = 0.;
        let mask = 1;
        let mut traps: Vec<Trapezoid> = Vec::new();
        let mut sl_list: LinkedList<SweepLineEdge> = LinkedList::new();
        sl_list.push_front(edge);
        let mut cursor = sl_list.cursor();
        cursor.next();
        // Call
        add_to_traps(&mut cursor, bottom, mask, &mut traps);
        assert_eq!(traps.len(), 0);
    }

    #[test]
    fn add_to_traps_edge_top_lt_bottom() {

        let edge1 = SweepLineEdge {
            trap_top: 1.,
            left: 0.,
            edge: Edge {
                line: LineSegment::new(1., 1., 3., 8.),
                top: 1.,
                bottom: 0.,
                direction: 1
            }
        };

        let edge2 = SweepLineEdge {
            trap_top: 1.,
            left: 0.,
            edge: Edge {
                line: LineSegment::new(5., 1., 1., 8.),
                top: 1.,
                bottom: 0.,
                direction: -1
            }
        };

        let mut sl_list: LinkedList<SweepLineEdge> = LinkedList::new();
        sl_list.push_front(edge1);
        sl_list.push_back(edge2);

        let bottom = 20.;
        let mask = 1;
        let mut traps: Vec<Trapezoid> = Vec::new();

        let mut cursor = sl_list.cursor();
        cursor.next();
        // Call
        add_to_traps(&mut cursor, bottom, mask, &mut traps);
        assert!(traps.len() > 0);
    }
}
