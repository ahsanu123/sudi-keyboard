// Copyright 2023 QMK
// SPDX-License-Identifier: GPL-2.0-or-later

#include QMK_KEYBOARD_H
enum keymap_layer { BASE = 0, SHIFTED, UTIL };

const char PROGMEM stringKey[2 * MATRIX_ROWS * MATRIX_COLS][4]={
        "ESC,",  "TAB,",   "CAPS",   "RSFT",   "1,  ",   "Q,  ",  "A   ",
        "Z,  ",  "LGUI",   "2,  ",   "W,  ",   "S,  ",   "X,  ",  "LALT",
        "3,  ",  "E,  ",   "D,  ",   "C,  ",   "MO1 ",   "4,  ",  "R,  ",
        "F,  ",  "V,  ",   "ENT,",   "5,  ",   "T,  ",   "G,  ",  "B,  ",
        "SPC,",  "6,  ",   "Y,  ",   "GRV,",   "END,",   "HOME",  "PGUP",
        "PGDN",  "LCTL",   "LEFT",   "RGHT",   "TAB,",   "RSFT",  "LGUI",
        "    ",  "    ",   "    ",   "    ",   "    ",   "    ",  "    ",
        "BSPC",  "BSLS",   "ENT,",   "RSFT",   "EQL,",   "RBRC",  "QUOT",
        "SLSH",  "DLR,",   "MINS",   "LBRC",   "SCLN",   "DOT,",  "CIRC",
        "0,  ",  "P,  ",   "L,  ",   "COMM",   "DEL,",   "9,  ",  "O,  ",
        "K,  ",  "M,  ",   "BSPC",   "8,  ",   "I,  ",   "J,  ",  "N,  ",
        "LSFT",  "7,  ",   "U,  ",   "H,  ",   "ESC,",   "HOME",  "PGUP",
        "PGDN",  "LALT",   "UP, ",   "DOWN",   "F1, ",   "F4, ",  "F5  "

};

const uint16_t PROGMEM keymaps[][MATRIX_ROWS][MATRIX_COLS] = {

    [BASE] = LAYOUT(
                KC_ESC,  KC_TAB,    KC_CAPS,  KC_RSFT,   KC_1,      KC_Q,     KC_A,
                KC_Z,    KC_LGUI,   KC_2,     KC_W,      KC_S,      KC_X,     KC_LALT,
                KC_3,    KC_E,      KC_D,     KC_C,      MO(1),     KC_4,     KC_R,
                KC_F,    KC_V,      KC_ENT,   KC_5,      KC_T,      KC_G,     KC_B,
                KC_SPC,  KC_6,      KC_Y,     KC_GRV,    KC_END,    KC_HOME,  KC_PGUP,
                KC_PGDN, KC_LCTL,   KC_LEFT,  KC_RGHT,   KC_TAB,    KC_RSFT,  KC_LGUI,

                KC_BSPC, KC_BSLS,   KC_ENT,   KC_RSFT,   KC_EQL,    KC_RBRC,  KC_QUOT,
                KC_SLSH, KC_DLR,    KC_MINS,  KC_LBRC,   KC_SCLN,   KC_DOT,   KC_CIRC,
                KC_0,    KC_P,      KC_L,     KC_COMM,   KC_LALT,    KC_9,     KC_O,
                KC_K,    KC_M,      KC_DEL,   KC_8,      KC_I,      KC_J,     KC_N,
                KC_LSFT, KC_7,      KC_U,     KC_H,      KC_END,    KC_HOME,  KC_PGUP,
                KC_PGDN, KC_BSPC,   KC_UP,    KC_DOWN,   KC_F1,     KC_F4,    KC_F5
                ),

    [SHIFTED] = LAYOUT(
                KC_ESC,  KC_NO,    KC_NO,   KC_NO,    KC_EXLM,  KC_NO,   KC_NO,
                KC_NO,   KC_AT,    KC_NO,   KC_NO,    KC_NO,    KC_NO,   KC_NO,
                KC_HASH, KC_NO,    KC_NO,   KC_NO,    KC_NO,    KC_DLR,  KC_NO,
                KC_NO,   KC_NO,    KC_NO,   KC_PERC,  KC_NO,    KC_NO,   KC_NO,
                KC_NO,   KC_CIRC,  KC_NO,   KC_TILD,  KC_NO,    KC_NO,   KC_NO,
                KC_NO,   KC_NO,   KC_MS_L,  KC_MS_R,  KC_NO,    KC_NO,   KC_NO,

                KC_BSPC, KC_PIPE,  KC_NO,   KC_NO,    KC_PLUS,  KC_RCBR, KC_DQUO,
                KC_QUES, KC_NO,    KC_UNDS, KC_LCBR,  KC_COLN,  KC_RABK, KC_NO,
                KC_RPRN, KC_NO,    KC_WH_R, KC_LABK,  KC_NO,    KC_LPRN, KC_NO,
                KC_WH_U, KC_BTN2,  KC_NO,   KC_ASTR,  KC_NO,    KC_WH_D, KC_BTN1,
                KC_NO,   KC_AMPR,  KC_NO,   KC_WH_L,  KC_NO,    KC_NO,   KC_NO,
                KC_NO,   KC_NO,    KC_MS_U, KC_MS_D,  KC_NO,    QK_RBT,  QK_BOOT
                )


};
bool process_record_user(uint16_t keycode, keyrecord_t *record) {
    uprintf("KL: col: %u, row: %u, pressed: %u\n", record->event.key.col, record->event.key.row, record->event.pressed);

//todo: display each ketcode/pressed key to oled

    oled_write_P(stringKey[record->event.key.row * 6 + record->event.key.col], false);

    return true;
}
static void render_logo(void) {
    static const char PROGMEM raw_logo[] = {
        0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,252,252,252,220,220,220,220,222,140,252,254,252,  0,  0,  0,  0,252,254,252,252,254,252, 28, 28, 28,252,252,240,252,254,252,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
        0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, 57, 57, 57, 57, 57, 57, 63, 63, 63,  7, 63, 63, 56, 56, 56, 56, 63, 63,  7, 63, 63, 63, 56, 56, 56, 63, 63,  7, 63, 63, 63,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
        0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,248,252,252,128,128,224,252,124, 28,252,252,252,156,156,156, 28, 28, 24, 28,124,252,224,128,128,240,252,124, 24,252,252,252,156,156,188,252,248, 96,224,252,252, 28, 28, 28, 60,252,248,224,224,252,252,156,156,252,252,240,224,252,252,252,156,156,252,252,240, 96,252,252,252, 28, 28,252,252,224,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
        0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, 63,127,127,  3,  3, 15,127,124,112,127,127,127,115,115,115,113,112, 48,  0,  0,  3, 63,127,127,  3,  1,  0,  0,127,127,127,115,115,123,127, 63, 12, 15,127,127,112,112,112,120,127, 63, 15,127,127,127,  3,  3, 63,127,127, 63,127,127,127,  3,  3, 63,127,126, 60,127,127,127,112,112,127,127, 15,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    };
    oled_write_raw_P(raw_logo, sizeof(raw_logo));
}

bool initialized = false;
bool oled_task_user(void) {

    oled_write_P(PSTR("Layer\n"), false);
    led_t led_state = host_keyboard_led_state();
    oled_write_P(led_state.caps_lock ? PSTR("CAPS\n") : PSTR("    \n"), false);

     switch (get_highest_layer(layer_state)){
         case BASE:
             oled_write_P(PSTR("BASE\n"), false);
             break;

         case SHIFTED:
             oled_write_P(PSTR("SHFT\n"), false);
             break;
         default:
             // Or use the write_ln shortcut over adding '\n' to the end of your string
             oled_write_ln_P(PSTR("UNDF\n"), false);
            break;
     }

    return false;
}
oled_rotation_t oled_init_user(oled_rotation_t rotation){

    render_logo();

    return OLED_ROTATION_270;
}
