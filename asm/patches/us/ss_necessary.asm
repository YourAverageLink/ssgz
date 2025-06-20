.open "main.dol"

.org @NextFreeSpace
.global custom_main_additions
.global handle_instant_text
.global finish_instant_text
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

; 0x80062f40 in JP 1.0
; 0x80062e60 in US 1.0
.org @MainInjection
bl custom_main_additions

.org 0x80053728 ; end of callback after rel initialization
b load_custom_rel

;.org 0x80064660
;lis r3, 0x16

;.org 0x80064690
;lis r3, 0x60

;.org 0x800646a0
;lis r3, 0xD0

.org 0x80115A04 ; instant text patch
b handle_instant_text


;b instant_text_if_cheat

.close