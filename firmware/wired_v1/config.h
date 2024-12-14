// Copyright 2023 sudi (@ahsanu)
// SPDX-License-Identifier: GPL-2.0-or-later

#pragma once
/* key matrix size */
#define MATRIX_ROWS 12
#define MATRIX_COLS 7

#define MATRIX_ROWS_PER_SIDE (MATRIX_ROWS / 2)
/* #define MATRIX_COLS_PER_SIDE (MATRIX_COLS) */
/*
 * Feature disable options
 *  These options are also useful to firmware size reduction.
 *
        "cols": ["F5","F6","F7","B1", "B3", "B2", "B6"],
        "rows": ["D4","C6","D7","E6","B4","B5"]
 */

#define MATRIX_ROW_PINS_MCU \
    { D4, C6, D7, E6, B4, B5 }
#define MATRIX_COL_PINS_MCU \
    { F5, F6, F7, B1, B3, B2, B6 }

#define DEBOUNCE 15
/* #define MATRIX_ROW_PINS \ */
/*     { B4, B5, F5, F5 } */
/* #define MATRIX_COL_PINS \ */
/*     { B2, B6, F6, F6 } */

/* disable debug print */
// #define NO_DEBUG

/* disable print */
// #define NO_PRINT

/* disable action features */
// #define NO_ACTION_LAYER
// #define NO_ACTION_TAPPING
// #define NO_ACTION_ONESHOT
