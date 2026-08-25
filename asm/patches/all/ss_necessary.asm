.open "main.dol"

.org @NextFreeSpace
.global custom_main_additions
.global handle_instant_text
.global finish_instant_text
.global hijack_rng
.global use_game_rng
.global handle_hide_ui
.global finish_ui

handle_instant_text:
lis r9, INSTANT_TEXT_ACTIVE@ha
li r4, 0
lbz r9, INSTANT_TEXT_ACTIVE@l(r9)
cmpwi r9, 0
beq finish_instant_text
li r4, 1
b finish_instant_text
finish_instant_text:
b returnForInstantText

hijack_rng:
lis r3, USE_RNG@ha
lbz r3, USE_RNG@l(r3)
cmpwi r3, 0
bne use_game_rng
lis r3, HARDCODED_RNG_FLOAT@ha
lfs f1, HARDCODED_RNG_FLOAT@l(r3)
blr

use_game_rng:
b rnd__2cMFv+0x8

handle_hide_ui:
lis r9, UI_HIDDEN@ha
li r4, 0
lbz r9, UI_HIDDEN@l(r9)
cmpwi r9, 0
beq finish_ui
li r3, 1
blr
finish_ui:
stwu r1, -0x10(r1)
b dLytMeterMain__draw + 0x4

.org IMPORTANT_UPDATE_FUNCTION+0x10
bl custom_main_additions

.org dvdCallback__4dDylFPv+0xD8 ; end of callback after rel initialization
b load_custom_rel

.org executeState_OutputText__15dLytMsgWindow_cFv+0x64 ; instant text patch
b handle_instant_text

.org rnd__2cMFv
b hijack_rng

; exploiting function alignment here - we have exactly 8 bytes here
; to copy the original rand function
.org rnd__2cMFv+0x8
la r3, s_rnd@sda21(r13)
b REST_OF_RNG_FUNC

.org draw__15dLytMeterMain_cFv ; dLytMeterMain__draw
b handle_hide_ui

.close