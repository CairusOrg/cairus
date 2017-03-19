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
 *	Kyle J. Kneitinger <kyle@kneit.in>
 *
 */

use path::{Path,Data};
use common_geometry::{Edge,Point};
use splines;
use context::Context;
use bo_trap::sweep;
use trapezoid_rasterizer::mask_from_trapezoids;


pub struct Filler {
    edges: Vec<Edge>,
}

impl Filler {

    pub fn new() -> Filler {
        Filler {
            edges: Vec::new(),
        }
    }

    fn fill(&self, mut context: Context) {
        let traps = sweep(self.edges.as_slice());
        let mask = mask_from_trapezoids(&traps, 100, 100);
    }

}

