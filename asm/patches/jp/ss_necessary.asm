.open "main.dol"

.org @NextFreeSpace
.global custom_main_additions
; .global is_instant_text


; 0x80062f40 in JP 1.0
; 0x80062e60 in US 1.0
.org @MainInjection
bl custom_main_additions

.org 0x80053838 ; end of callback after rel initialization
b load_custom_rel

;.org 0x80064660
;lis r3, 0x16

;.org 0x80064690
;lis r3, 0x60

;.org 0x800646a0
;lis r3, 0xD0

;.org 0x801160bc ; instant text patch
;bl is_instant_text

.close