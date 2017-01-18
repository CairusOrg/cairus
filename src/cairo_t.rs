/*
 * Cairus - a reimplementation of the cairo graphics library in Rust
 *
 * Copyright © 20XX CairusOrg
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
 *	Sara Ferdousi <ferdousi@pdx.edu>
 *
 */

use cairo_status::cairo_status_t;

struct cairo_array{
    size: u64,
    num_elements: u64,
    element_size: u64,
    elements: &char

}

impl cairo_array{

    fn new(size: u64, num_elements: u64, element_size: u64, elements: &char)->cairo_array{
        cairo_array{size: size, num_elements: num_elements, element_size: element_size, elements: elements}
    }
}

pub struct cairo_t{

    pub ref_count: u64,
    pub cairo_status: cairo_status_t,
    pub user_data_array: cairo_array

}

impl cairo_t{

    fn new(ref_count: u64, cairo_status: cairo_status_t, user_data_array: cairo_array)->cairo_t{

        cairo_t{
            ref_count:ref_count,
            cairo_status: cairo_status,
            user_data_array: cairo_array::new(user_data_array.size, user_data_array.element_size, user_data_array.num_elements, user_data_array.elements)

        }
    }

    fn cairo_reference(&mut self) -> cairo_t{
        self.ref_count+=1; //increases reference count by one

        return self;
    }

    fn cairo_destroy(&mut self){
        if(self.ref_count != 0){
            self.ref_count-=1;
        }
        //else need to free cairo_t object and associated resources. Not sure how to implement that
    }

}
