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
 *  Evan Smelser <evanjsmelser@gmail.com>
 */

enum status{

    SUCCESS = 0,
    NO_MEMORY,
    INVALID_RESTORE,
    INVALID_POP_GROUP,
    NO_CURRENT_POINT,
    INVALID_MATRIX,
    INVALID_STATUS,
    NULL_POINTER,
    INVALID_STRING,
    INVALID_PATH_DATA,
    READ_ERROR,
    WRITE_ERROR,
    SURFACE_FINISHED,
    SURFACE_TYPE_MISMATCH,
    PATTERN_TYPE_MISMATCH,
    INVALID_CONTENT,
    INVALID_FORMAT,
    INVALID_VISUAL,
    FILE_NOT_FOUND,
    INVALID_DASH,
    INVALID_DSC_COMMENT,
    INVALID_INDEX,
    CLIP_NOT_REPRESENTABLE,
    TEMP_FILE_ERROR,
    INVALID_STRIDE,
    FONT_TYPE_MISMATCH,
    USER_FONT_IMMUTABLE,
    USER_FONT_ERROR,
    NEGATIVE_COUNT,
    INVALID_CLUSTERS,
    INVALID_SLANT,
    INVALID_WEIGHT,
    INVALID_SIZE,
    USER_FONT_NOT_IMPLEMENTED,
    DEVICE_TYPE_MISMATCH,
    DEVICE_ERROR,
    INVALID_MESH_CONSTRUCTION,
    DEVICE_FINISHED,
    JBIG2_GLOBAL_MISSING,
    PNG_ERROR,
    FREETYPE_ERROR,
    WIN32_GDI_ERROR,
    TAG_ERROR,

    LAST_STATUS
}
