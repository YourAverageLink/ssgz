use core::ffi::c_void;

// This is also known as Profile name for decomp purposes
#[allow(non_camel_case_types)]
#[repr(C)]
pub enum ActorID {
    TITLE,                         // 000 (0x000)
    E3_TITLE,                      // 001 (0x001)
    E3_GAMEEND,                    // 002 (0x002)
    THPPLAYER,                     // 003 (0x003)
    GAME,                          // 004 (0x004)
    STAGE_MANAGER,                 // 005 (0x005)
    STAGE,                         // 006 (0x006)
    STAGE_SELECT,                  // 007 (0x007)
    VIEW_CLIP_TAG,                 // 008 (0x008)
    START_TAG,                     // 009 (0x009)
    MAP_AREA_TAG,                  // 010 (0x00A)
    TRUCK_RAIL,                    // 011 (0x00B)
    TAG_STREAM,                    // 012 (0x00C)
    COL_BOMSLD,                    // 013 (0x00D)
    OBJ_STAGE_KRAKEN,              // 014 (0x00E)
    OBJ_STAGE_KRAKEN_PARTS,        // 015 (0x00F)
    OBJ_TIME_STONE,                // 016 (0x010)
    OBJ_SW,                        // 017 (0x011)
    OBJ_BLOCK_ROPE,                // 018 (0x012)
    OBJ_PUSH_BLOCK,                // 019 (0x013)
    OBJ_KIBAKO,                    // 020 (0x014)
    OBJ_LOG,                       // 021 (0x015)
    OBJ_LOG_WATER,                 // 022 (0x016)
    OBJ_BELT_CVR,                  // 023 (0x017)
    OBJ_DRUM,                      // 024 (0x018)
    OBJ_BELT_OBSTACLE,             // 025 (0x019)
    OBJ_HIMO,                      // 026 (0x01A)
    OBJ_SPIDER_LINE,               // 027 (0x01B)
    OBJ_WIND,                      // 028 (0x01C)
    OBJ_WIND03,                    // 029 (0x01D)
    OBJ_WIND04,                    // 030 (0x01E)
    OBJ_TORNADO,                   // 031 (0x01F)
    OBJ_SWITCH_WALL,               // 032 (0x020)
    OBJ_TOWER_D101,                // 033 (0x021)
    OBJ_DOOR_DUNGEON_D200,         // 034 (0x022)
    OBJ_DOOR_DUNGEON,              // 035 (0x023)
    OBJ_WOOD_BOARD,                // 036 (0x024)
    OBJ_CLAW_SHOT_TG,              // 037 (0x025)
    OBJ_BULB_SWITCH,               // 038 (0x026)
    OBJ_SIDE_SHUTTER,              // 039 (0x027)
    OBJ_HIT_LEVER_SW,              // 040 (0x028)
    OBJ_FENCE_IRON,                // 041 (0x029)
    OBJ_UPDOWN_LAVA,               // 042 (0x02A)
    OBJ_BB_OBJECTS,                // 043 (0x02B)
    OBJ_BRIDGE_BUILDING,           // 044 (0x02C)
    OBJ_CANNON,                    // 045 (0x02D)
    OBJ_ROULETTE_ISLAND_C,         // 046 (0x02E)
    OBJ_ROULETTE_ISLAND_R,         // 047 (0x02F)
    OBJ_BRIDGE_STRETCH,            // 048 (0x030)
    OBJ_IRON_STAGE,                // 049 (0x031)
    OBJ_UTAJIMA_STOPPER,           // 050 (0x032)
    OBJ_UTAJIMA_MAIN_MECHA,        // 051 (0x033)
    OBJ_UTAJIMA_PEDESTAL,          // 052 (0x034)
    OBJ_UTAJIMA_ISLAND,            // 053 (0x035)
    OBJ_CANNON_COVER,              // 054 (0x036)
    OBJ_UTAJIMA,                   // 055 (0x037)
    OBJ_UTAJIMA_LV2,               // 056 (0x038)
    OBJ_PUZZLE_ISLAND,             // 057 (0x039)
    OBJ_FENCE_BOKO,                // 058 (0x03A)
    OBJ_FENCE_BOKO2,               // 059 (0x03B)
    OBJ_WINDMILL,                  // 060 (0x03C)
    OBJ_PINWHEEL,                  // 061 (0x03D)
    OBJ_LIGHTHOUSE_HARP,           // 062 (0x03E)
    OBJ_FENCE_KONSAI,              // 063 (0x03F)
    OBJ_STAGE_SINK,                // 064 (0x040)
    OBJ_STAGE_WATER,               // 065 (0x041)
    OBJ_STAGE_COVER,               // 066 (0x042)
    OBJ_STAGE_CRACK,               // 067 (0x043)
    OBJ_TERRY_ISLAND,              // 068 (0x044)
    OBJ_INSECT_ISLAND,             // 069 (0x045)
    OBJ_SHRINE_AFTER,              // 070 (0x046)
    OBJ_SHRINE_BEFORE,             // 071 (0x047)
    OBJ_SHIP_WINDOW,               // 072 (0x048)
    OBJ_WATER_SURFACE,             // 073 (0x049)
    OBJ_PUMPKIN_BAR,               // 074 (0x04A)
    OBJ_TREASURE_ISLAND,           // 075 (0x04B)
    OBJ_SEALED_DOOR,               // 076 (0x04C)
    OBJ_EVIL_FIELD,                // 077 (0x04D)
    OBJ_MEGAMI_ISLAND,             // 078 (0x04E)
    OBJ_CITY,                      // 079 (0x04F)
    OBJ_BAMBOO_ISLAND,             // 080 (0x050)
    OBJ_STREAM_LAVA,               // 081 (0x051)
    OBJ_DOWN_LAVA,                 // 082 (0x052)
    OBJ_APPEAR_BRIDGE,             // 083 (0x053)
    OBJ_TRUCK_STOPPER,             // 084 (0x054)
    OBJ_ISLAND_NUSI,               // 085 (0x055)
    OBJ_ROCK_SKY,                  // 086 (0x056)
    OBJ_TREASURE_ISLAND_B,         // 087 (0x057)
    OBJ_WATER_F100,                // 088 (0x058)
    OBJ_BELL,                      // 089 (0x059)
    OBJ_SHRINE_BEF_INSIDE,         // 090 (0x05A)
    OBJ_WINDMILL_DESERT,           // 091 (0x05B)
    OBJ_CITY_WATER,                // 092 (0x05C)
    OBJ_MOLE_COVER,                // 093 (0x05D)
    OBJ_DESERT_DEBRIS,             // 094 (0x05E)
    OBJ_BB_BROKEN_PARTS,           // 095 (0x05F)
    OBJ_KUMITE_WALL,               // 096 (0x060)
    OBJ_WATER_SHIELD,              // 097 (0x061)
    OBJ_BSTONE,                    // 098 (0x062)
    OBJ_WIND02,                    // 099 (0x063)
    OBJ_LEAF_SWING,                // 100 (0x064)
    RIDE_ROCK_SET_TAG,             // 101 (0x065)
    OBJ_RIDE_ROCK,                 // 102 (0x066)
    OBJ_MOVE_LIFT_VOL,             // 103 (0x067)
    OBJ_TRUCK,                     // 104 (0x068)
    OBJ_TERRY_SHOP,                // 105 (0x069)
    OBJ_TRAP_ROCK_1,               // 106 (0x06A)
    OBJ_STOPPER_ROCK,              // 107 (0x06B)
    OBJ_SHUTTER_FENCE,             // 108 (0x06C)
    OBJ_SINK_FLOOR_F,              // 109 (0x06D)
    E_GUMARM,                      // 110 (0x06E)
    OBJ_STEP_GUMARM,               // 111 (0x06F)
    OBJ_BRIDGE_FALL,               // 112 (0x070)
    OBJ_BRIDGE_STEP,               // 113 (0x071)
    OBJ_BRIDGE_BONE,               // 114 (0x072)
    OBJ_BB_BRIDGE,                 // 115 (0x073)
    OBJ_BRIDGE_TIME,               // 116 (0x074)
    OBJ_BOAT,                      // 117 (0x075)
    OBJ_BALLISTA,                  // 118 (0x076)
    OBJ_BALLISTA_F3,               // 119 (0x077)
    OBJ_TIME_BOAT,                 // 120 (0x078)
    OBJ_GODDESS_STATUE,            // 121 (0x079)
    OBJ_STONE_STAND,               // 122 (0x07A)
    OBJ_TIME_STAGE_BG,             // 123 (0x07B)
    OBJ_WARP_HOLE,                 // 124 (0x07C)
    OBJ_GEAR,                      // 125 (0x07D)
    OBJ_DESERT,                    // 126 (0x07E)
    OBJ_D300,                      // 127 (0x07F)
    OBJ_SEA_F301,                  // 128 (0x080)
    OBJ_DESERT_AGO,                // 129 (0x081)
    OBJ_DESERT_METER,              // 130 (0x082)
    OBJ_NEEDLE_DESERT,             // 131 (0x083)
    OBJ_LOTUS,                     // 132 (0x084)
    OBJ_TARZAN_POLE,               // 133 (0x085)
    OBJ_STEP_TIME_SLIP,            // 134 (0x086)
    OBJ_TIME_BASE,                 // 135 (0x087)
    OBJ_SWITCH_SHUTTER,            // 136 (0x088)
    OBJ_WATERFALL_D101,            // 137 (0x089)
    OBJ_ROLL_PILLAR,               // 138 (0x08A)
    OBJ_CHEST,                     // 139 (0x08B)
    OBJ_ROCK_BOAT,                 // 140 (0x08C)
    OBJ_BLOCK_UNDERGROUND,         // 141 (0x08D)
    OBJ_UNDERGROUND,               // 142 (0x08E)
    OBJ_TROLLEY,                   // 143 (0x08F)
    OBJ_LAVA_PLATE,                // 144 (0x090)
    OBJ_SAND_FLOOR,                // 145 (0x091)
    OBJ_SW_SYAKO,                  // 146 (0x092)
    OBJ_SYAKO_SHUTTER,             // 147 (0x093)
    OBJ_DUNGEON_SHIP,              // 148 (0x094)
    OBJ_NEEDLE_UNDERGROUND,        // 149 (0x095)
    OBJ_STEP_STATUE,               // 150 (0x096)
    OBJ_GRAVE,                     // 151 (0x097)
    OBJ_SHED,                      // 152 (0x098)
    OBJ_GIRAHIMU_FLOOR,            // 153 (0x099)
    OBJ_TENIJIMA,                  // 154 (0x09A)
    OBJ_SAND_D301,                 // 155 (0x09B)
    OBJ_DOOR_BOSSD101,             // 156 (0x09C)
    OBJ_BOXCAGE_F300,              // 157 (0x09D)
    OBJ_TOWER_HAND_D101,           // 158 (0x09E)
    OBJ_DORMITORY_GATE,            // 159 (0x09F)
    OBJ_PISTON,                    // 160 (0x0A0)
    OBJ_FRUIT_TREE,                // 161 (0x0A1)
    OBJ_FARMLAND,                  // 162 (0x0A2)
    OBJ_PROPELLER_LIFT,            // 163 (0x0A3)
    OBJ_D3_DUMMY,                  // 164 (0x0A4)
    B_BIGBOSS_BASE,                // 165 (0x0A5)
    B_BIGBOSS,                     // 166 (0x0A6)
    B_BIGBOSS2,                    // 167 (0x0A7)
    B_BIGBOSS3,                    // 168 (0x0A8)
    B_VD,                          // 169 (0x0A9)
    OBJ_VDB,                       // 170 (0x0AA)
    E_CAPTAIN,                     // 171 (0x0AB)
    OBJ_TRUCK_RAIL_COL,            // 172 (0x0AC)
    BIRD,                          // 173 (0x0AD)
    BIRD_TARGET,                   // 174 (0x0AE)
    BIRD_NPC,                      // 175 (0x0AF)
    BIRD_KOBUNA,                   // 176 (0x0B0)
    BIRD_KOBUNB,                   // 177 (0x0B1)
    BIRD_RIVAL,                    // 178 (0x0B2)
    BIRD_ZELDA_TRAINING,           // 179 (0x0B3)
    AVATER_RACE_MNG,               // 180 (0x0B4)
    AVATER_BULLET,                 // 181 (0x0B5)
    NUSI_BASE,                     // 182 (0x0B6)
    NUSI_NPC,                      // 183 (0x0B7)
    B_NUSI,                        // 184 (0x0B8)
    B_NUSI_TENTAKLE,               // 185 (0x0B9)
    B_NUSI_BULLET,                 // 186 (0x0BA)
    OBJ_LIGHT_LINE,                // 187 (0x0BB)
    OBJ_LIGHT_SHAFT_SMALL,         // 188 (0x0BC)
    TAG_LIGHT_SHAFT_EFF,           // 189 (0x0BD)
    MEGAMI_DIVING_TAG,             // 190 (0x0BE)
    COMMON_BULLET,                 // 191 (0x0BF)
    E_SYAKOMAITO,                  // 192 (0x0C0)
    E_MR,                          // 193 (0x0C1)
    E_PH,                          // 194 (0x0C2)
    B_KR,                          // 195 (0x0C3)
    B_KRH,                         // 196 (0x0C4)
    B_KRA,                         // 197 (0x0C5)
    OBJ_FLYING_CLAWSHOT_TARGET,    // 198 (0x0C6)
    OBJ_DIS_SHIP,                  // 199 (0x0C7)
    PLAYER,                        // 200 (0x0C8)
    TAG_SHUTTER_FENCE_PERMISSION,  // 201 (0x0C9)
    SHUTTER,                       // 202 (0x0CA)
    OBJ_SHUTTER_CHANGE_SCENE,      // 203 (0x0CB)
    OBJ_DOOR_BOSS,                 // 204 (0x0CC)
    OBJ_DOOR,                      // 205 (0x0CD)
    OBJ_FENCE,                     // 206 (0x0CE)
    TAG_SHUTTER_FENCE_FORBIDDANCE, // 207 (0x0CF)
    OBJ_TROLLEY_SHUTTER,           // 208 (0x0D0)
    OBJ_TR_SHUTTER_CS,             // 209 (0x0D1)
    OBJ_BG,                        // 210 (0x0D2)
    BOOMERANG,                     // 211 (0x0D3)
    GENKI_MGR_TAG,                 // 212 (0x0D4)
    TAG_MIECHAN,                   // 213 (0x0D5)
    DEMO_NPC_BIRD,                 // 214 (0x0D6)
    NPC_RVL,                       // 215 (0x0D7)
    NPC_RIVAL_LOD,                 // 216 (0x0D8)
    NPC_KBN,                       // 217 (0x0D9)
    NPC_KBN2,                      // 218 (0x0DA)
    NPC_KOBUN_B_NIGHT,             // 219 (0x0DB)
    NPC_SKN,                       // 220 (0x0DC)
    NPC_SKN2,                      // 221 (0x0DD)
    NPC_GZL,                       // 222 (0x0DE)
    NPC_ZLD,                       // 223 (0x0DF)
    NPC_DSK,                       // 224 (0x0E0)
    NPC_DRB,                       // 225 (0x0E1)
    NPC_DRBC,                      // 226 (0x0E2)
    NPC_CE_FRIEND,                 // 227 (0x0E3)
    NPC_CE_LADY,                   // 228 (0x0E4)
    NPC_TOILET_GHOST,              // 229 (0x0E5)
    NPC_SORAJIMA_FATHER,           // 230 (0x0E6)
    NPC_SORAJIMA_MOTHER,           // 231 (0x0E7)
    NPC_SORAJIMA_GIRL,             // 232 (0x0E8)
    NPC_KYUI_WIZARD,               // 233 (0x0E9)
    NPC_KYUI_FIRST,                // 234 (0x0EA)
    NPC_ORD_KYUI,                  // 235 (0x0EB)
    NPC_KYUI_ELDER,                // 236 (0x0EC)
    NPC_KYUI_THIRD,                // 237 (0x0ED)
    NPC_KYUI4,                     // 238 (0x0EE)
    NPC_TMN,                       // 239 (0x0EF)
    NPC_SALESMAN_S,                // 240 (0x0F0)
    NPC_DOUGUYA_NIGHT,             // 241 (0x0F1)
    NPC_MED_WIFE_NIGHT,            // 242 (0x0F2)
    NPC_MED_HUS_NIGHT,             // 243 (0x0F3)
    NPC_JUNK_NIGHT,                // 244 (0x0F4)
    NPC_AZUKARIYA_NIGHT,           // 245 (0x0F5)
    NPC_DOUGUYA_MOTHER,            // 246 (0x0F6)
    NPC_DOUGUYA_MOTHER_LOD,        // 247 (0x0F7)
    NPC_JUNK_MOTHER,               // 248 (0x0F8)
    NPC_JUNK_MOTHER_LOD,           // 249 (0x0F9)
    NPC_SENPAIA_MOTHER,            // 250 (0x0FA)
    NPC_SENPAIA_MOTHER_LOD,        // 251 (0x0FB)
    NPC_SORAJIMA_MAN_E,            // 252 (0x0FC)
    NPC_SORAJIMA_MAN_D,            // 253 (0x0FD)
    NPC_AZUKARIYA_FATHER,          // 254 (0x0FE)
    NPC_DAISHINKAN_N,              // 255 (0x0FF)
    NPC_SORAJIMA_MALE,             // 256 (0x100)
    NPC_BDSW,                      // 257 (0x101)
    NPC_SORAJIMA_FEMALE,           // 258 (0x102)
    NPC_KENSEI,                    // 259 (0x103)
    NPC_TALK_KENSEI,               // 260 (0x104)
    NPC_BDZ,                       // 261 (0x105)
    NPC_OIM,                       // 262 (0x106)
    NPC_YIM,                       // 263 (0x107)
    NPC_BGR,                       // 264 (0x108)
    NPC_SLTK,                      // 265 (0x109)
    NPC_SLB2,                      // 266 (0x10A)
    NPC_SMA3,                      // 267 (0x10B)
    NPC_SMA2,                      // 268 (0x10C)
    NPC_PMA,                       // 269 (0x10D)
    NPC_PDU,                       // 270 (0x10E)
    NPC_ICGK,                      // 271 (0x10F)
    NPC_PCS,                       // 272 (0x110)
    NPC_FDR,                       // 273 (0x111)
    NPC_TDR,                       // 274 (0x112)
    NPC_TDS,                       // 275 (0x113)
    NPC_TDRB,                      // 276 (0x114)
    TAG_SWORD_BATTLE_GAME,         // 277 (0x115)
    TAG_SIREN_TIME_ATTACK,         // 278 (0x116)
    NPC_ADR,                       // 279 (0x117)
    NPC_GHM,                       // 280 (0x118)
    NPC_SHA,                       // 281 (0x119)
    NPC_GRA,                       // 282 (0x11A)
    NPC_GRC,                       // 283 (0x11B)
    NPC_GRD,                       // 284 (0x11C)
    NPC_SORAJIMA_BOY,              // 285 (0x11D)
    NPC_AKUMAKUN,                  // 286 (0x11E)
    NPC_AKU_HUMAN,                 // 287 (0x11F)
    NPC_SUISEI,                    // 288 (0x120)
    NPC_SUISEI_SUB,                // 289 (0x121)
    NPC_SUISEI_NORMAL,             // 290 (0x122)
    MOLE_MGR_TAG,                  // 291 (0x123)
    NPC_MOLE_MG,                   // 292 (0x124)
    NPC_MOLE,                      // 293 (0x125)
    NPC_MOLE_NORMAL,               // 294 (0x126)
    NPC_MOLE_NORMAL2,              // 295 (0x127)
    NPC_MOLE_ES_NML,               // 296 (0x128)
    NPC_MOLE_TACKLE,               // 297 (0x129)
    NPC_MOLE_TACKLE2,              // 298 (0x12A)
    NPC_CHEF,                      // 299 (0x12B)
    NPC_SLFB,                      // 300 (0x12C)
    NPC_SLRP,                      // 301 (0x12D)
    NPC_SLFL,                      // 302 (0x12E)
    NPC_TERRY,                     // 303 (0x12F)
    NPC_DIVE_GAME_JUDGE,           // 304 (0x130)
    KNIGHT_LEADER_BIRD,            // 305 (0x131)
    NPC_KNIGHT_LEADER,             // 306 (0x132)
    NPC_SENPAI,                    // 307 (0x133)
    NPC_SENPAI_B,                  // 308 (0x134)
    NPC_REGRET_RIVAL,              // 309 (0x135)
    NPC_RESCUE,                    // 310 (0x136)
    NPC_SLB,                       // 311 (0x137)
    FLY_SLB,                       // 312 (0x138)
    OBJ_PROPERA,                   // 313 (0x139)
    OBJ_ROULETTE,                  // 314 (0x13A)
    NPC_MOLE_ELDER,                // 315 (0x13B)
    NPC_SALBAGE_MORRY,             // 316 (0x13C)
    NPC_MOLE_SAL,                  // 317 (0x13D)
    OBJ_POT_SAL,                   // 318 (0x13E)
    OBJ_MOLE_SOIL,                 // 319 (0x13F)
    LITTLE_BIRD_MGR,               // 320 (0x140)
    LITTLE_BIRD,                   // 321 (0x141)
    FISH_MGR,                      // 322 (0x142)
    FISH,                          // 323 (0x143)
    EEL,                           // 324 (0x144)
    JSTUDIO_SYSOBJ,                // 325 (0x145)
    JSTUDIO_ACTOR,                 // 326 (0x146)
    B_BBSHWV,                      // 327 (0x147)
    NPC_BBRVL,                     // 328 (0x148)
    OBJ_BIGBOMB_FLOWER,            // 329 (0x149)
    OBJ_BBLARGEBOMB,               // 330 (0x14A)
    OBJ_BSTN,                      // 331 (0x14B)
    B_MG,                          // 332 (0x14C)
    B_LASTBOSS,                    // 333 (0x14D)
    J_TEST,                        // 334 (0x14E)
    E_AM,                          // 335 (0x14F)
    T_QUAKE,                       // 336 (0x150)
    T_KUMITE,                      // 337 (0x151)
    GROUP_TEST,                    // 338 (0x152)
    GROUP_SUMMON,                  // 339 (0x153)
    T_BCAL,                        // 340 (0x154)
    E_SM,                          // 341 (0x155)
    E_BEAMOS,                      // 342 (0x156)
    GEKO_TAG,                      // 343 (0x157)
    E_GEKO,                        // 344 (0x158)
    E_SIREN,                       // 345 (0x159)
    E_PO,                          // 346 (0x15A)
    OBJ_RING,                      // 347 (0x15B)
    E_OR,                          // 348 (0x15C)
    E_OR_CANNON,                   // 349 (0x15D)
    OR_CANN_BULLET,                // 350 (0x15E)
    E_EYE,                         // 351 (0x15F)
    OBJ_HOLE,                      // 352 (0x160)
    OBJ_INTO_HOLE,                 // 353 (0x161)
    E_SPARK,                       // 354 (0x162)
    E_MAGMA,                       // 355 (0x163)
    E_MAGUPPO,                     // 356 (0x164)
    MAGUPPO_BULLET,                // 357 (0x165)
    E_BS,                          // 358 (0x166)
    E_SF,                          // 359 (0x167)
    E_SF4,                         // 360 (0x168)
    E_ST,                          // 361 (0x169)
    E_ST_WIRE,                     // 362 (0x16A)
    ENEMY_CONTROL,                 // 363 (0x16B)
    KIESU_TAG,                     // 364 (0x16C)
    E_KS,                          // 365 (0x16D)
    E_HB,                          // 366 (0x16E)
    E_HB_LEAF,                     // 367 (0x16F)
    E_REMLY,                       // 368 (0x170)
    E_LIZARUFOS,                   // 369 (0x171)
    E_LIZA_TAIL,                   // 370 (0x172)
    E_HIDOKARI,                    // 371 (0x173)
    E_HIDOKARIS,                   // 372 (0x174)
    E_HYDRA,                       // 373 (0x175)
    E_GUNHO,                       // 374 (0x176)
    E_GUNHOB,                      // 375 (0x177)
    E_BFISH,                       // 376 (0x178)
    E_CACTUS,                      // 377 (0x179)
    E_HOC,                         // 378 (0x17A)
    E_OC,                          // 379 (0x17B)
    E_KGIRA,                       // 380 (0x17C)
    OBJ_PIPE,                      // 381 (0x17D)
    E_BC,                          // 382 (0x17E)
    E_BCE,                         // 383 (0x17F)
    E_BCAL,                        // 384 (0x180)
    E_BCARROW,                     // 385 (0x181)
    E_BCALARROW,                   // 386 (0x182)
    BCZ_TAG,                       // 387 (0x183)
    E_BCZ,                         // 388 (0x184)
    E_SKYTAIL,                     // 389 (0x185)
    E_HP,                          // 390 (0x186)
    E_CHB,                         // 391 (0x187)
    E_GUE,                         // 392 (0x188)
    GUE_BULLET,                    // 393 (0x189)
    E_GE,                          // 394 (0x18A)
    E_RUPEE_GUE,                   // 395 (0x18B)
    E_GEROCK,                      // 396 (0x18C)
    E_TN2,                         // 397 (0x18D)
    E_HIDORY,                      // 398 (0x18E)
    HIDORY_FIRE,                   // 399 (0x18F)
    E_WS,                          // 400 (0x190)
    NPC_BIRD,                      // 401 (0x191)
    B_GIRAHIMU_BASE,               // 402 (0x192)
    B_GIRAHIMU,                    // 403 (0x193)
    B_GIRAHIMU2,                   // 404 (0x194)
    B_GIRAHIMU3_BASE,              // 405 (0x195)
    B_GIRAHIMU3_FIRST,             // 406 (0x196)
    B_GIRAHIMU3_SECOND,            // 407 (0x197)
    B_GIRAHIMU3_THIRD,             // 408 (0x198)
    OBJ_GH_SW_L,                   // 409 (0x199)
    OBJ_GH_KNIFE,                  // 410 (0x19A)
    OBJ_BIRD_SP_UP,                // 411 (0x19B)
    GH_SWORD_BEAM,                 // 412 (0x19C)
    B_ASURA,                       // 413 (0x19D)
    ASURA_ARM,                     // 414 (0x19E)
    ASURA_FOOT,                    // 415 (0x19F)
    ASURA_BULLET,                  // 416 (0x1A0)
    ASURA_SWORD,                   // 417 (0x1A1)
    ASURA_PILLAR,                  // 418 (0x1A2)
    INVISIBLE,                     // 419 (0x1A3)
    E_MR_SHIELD,                   // 420 (0x1A4)
    E_KG,                          // 421 (0x1A5)
    NPC_HONEYCOMB,                 // 422 (0x1A6)
    NPC_BEE,                       // 423 (0x1A7)
    HEART_FLOWER,                  // 424 (0x1A8)
    BOMBF,                         // 425 (0x1A9)
    BOMB,                          // 426 (0x1AA)
    OBJ_CARRY_STONE,               // 427 (0x1AB)
    OBJ_ROLL_ROCK,                 // 428 (0x1AC)
    COL_STP,                       // 429 (0x1AD)
    KANBAN,                        // 430 (0x1AE)
    OBJ_BAMBOO,                    // 431 (0x1AF)
    OBJ_SWHIT,                     // 432 (0x1B0)
    OBJ_SW_SWORD_BEAM,             // 433 (0x1B1)
    OBJ_SW_HARP,                   // 434 (0x1B2)
    OBJ_SIREN_BARRIER,             // 435 (0x1B3)
    OBJ_TOGE_TRAP,                 // 436 (0x1B4)
    PUMPKIN,                       // 437 (0x1B5)
    OBJ_PUMPKIN_LEAF,              // 438 (0x1B6)
    OBJ_WATER_NUT_LEAF,            // 439 (0x1B7)
    OBJ_WATER_NUT,                 // 440 (0x1B8)
    OBJ_TABLEWARE,                 // 441 (0x1B9)
    OBJ_SW_WHIPLEVER,              // 442 (0x1BA)
    OBJ_MUSHROOM,                  // 443 (0x1BB)
    WOODAREA_TAG,                  // 444 (0x1BC)
    OBJ_FRUIT,                     // 445 (0x1BD)
    OBJ_SKULL,                     // 446 (0x1BE)
    SOUND_TAG,                     // 447 (0x1BF)
    OBJ_ROCK_DRAGON,               // 448 (0x1C0)
    TAG_INSECT,                    // 449 (0x1C1)
    INSECT_LADYBUG,                // 450 (0x1C2)
    INSECT_DRAGONFLY,              // 451 (0x1C3)
    INSECT_BEETLE,                 // 452 (0x1C4)
    INSECT_GRASSHOPPER,            // 453 (0x1C5)
    INSECT_CICADA,                 // 454 (0x1C6)
    INSECT_ANT,                    // 455 (0x1C7)
    INSECT_BUTTERFLY,              // 456 (0x1C8)
    INSECT_SCARAB,                 // 457 (0x1C9)
    INSECT_FIREFLY,                // 458 (0x1CA)
    OBJ_SAIL,                      // 459 (0x1CB)
    OBJ_LOTUS_FLOWER,              // 460 (0x1CC)
    OBJ_LOTUS_SEED,                // 461 (0x1CD)
    OBJ_SHUTTER_LOCK,              // 462 (0x1CE)
    OBJ_LAMP,                      // 463 (0x1CF)
    TAG_ROCK_BOAT,                 // 464 (0x1D0)
    OBJ_TOWER_GEAR_D101,           // 465 (0x1D1)
    OBJ_SHUTTER_WATER_D101,        // 466 (0x1D2)
    OBJ_ANCIENT_JEWELS,            // 467 (0x1D3)
    OBJ_MG_PUMPKIN,                // 468 (0x1D4)
    OBJ_FLAG,                      // 469 (0x1D5)
    OBJ_CHANDELIER,                // 470 (0x1D6)
    TAG_PUMPKIN_CLAY,              // 471 (0x1D7)
    TAG_REACTION,                  // 472 (0x1D8)
    OBJ_SPORE,                     // 473 (0x1D9)
    OBJ_FRUIT_B,                   // 474 (0x1DA)
    OBJ_DIVINER_CRYSTAL,           // 475 (0x1DB)
    TAG_NOEFFECT_AREA,             // 476 (0x1DC)
    TAG_D3_SCENE_CHANGE,           // 477 (0x1DD)
    OBJ_DECOA,                     // 478 (0x1DE)
    OBJ_DECOB,                     // 479 (0x1DF)
    OBJ_SANDBAG,                   // 480 (0x1E0)
    OBJ_PAINT,                     // 481 (0x1E1)
    OBJ_CONTROL_PANEL,             // 482 (0x1E2)
    OBJ_UG_SWITCH,                 // 483 (0x1E3)
    OBJ_CLEARNESS_WALL,            // 484 (0x1E4)
    OBJ_RUINED_SAVE,               // 485 (0x1E5)
    OBJ_TRIFORCE,                  // 486 (0x1E6)
    OBJ_KANBAN_STONE,              // 487 (0x1E7)
    TBOX,                          // 488 (0x1E8)
    OBJ_BUBBLE,                    // 489 (0x1E9)
    OBJ_VSD,                       // 490 (0x1EA)
    OBJ_SOIL,                      // 491 (0x1EB)
    OBJ_IVY_ROPE,                  // 492 (0x1EC)
    OBJ_GRASS_COIL,                // 493 (0x1ED)
    OBJ_ROPE_IGAIGA,               // 494 (0x1EE)
    OBJ_FIRE,                      // 495 (0x1EF)
    OBJ_TUBO,                      // 496 (0x1F0)
    OBJ_TUBO_BIG,                  // 497 (0x1F1)
    OBJ_CHAIR,                     // 498 (0x1F2)
    TIME_AREA,                     // 499 (0x1F3)
    OBJ_BLAST_ROCK,                // 500 (0x1F4)
    OBJ_SW_DIR,                    // 501 (0x1F5)
    OBJ_SW_DIR_DOOR,               // 502 (0x1F6)
    OBJ_SW_BANK,                   // 503 (0x1F7)
    OBJ_SW_BANK_SMALL,             // 504 (0x1F8)
    T_FAIRY,                       // 505 (0x1F9)
    OBJ_FAIRY,                     // 506 (0x1FA)
    BIRD_MOB,                      // 507 (0x1FB)
    OBJ_BALLISTA_HANDLE,           // 508 (0x1FC)
    OBJ_TIME_BOAT_BULLET,          // 509 (0x1FD)
    OBJ_TIME_DOOR,                 // 510 (0x1FE)
    OBJ_TIME_DOOR_BEFORE,          // 511 (0x1FF)
    TAG_TIME_DOOR_BEAM,            // 512 (0x200)
    OBJ_COL,                       // 513 (0x201)
    OBJ_DAYNIGHT,                  // 514 (0x202)
    OBJ_BUILDING,                  // 515 (0x203)
    OBJ_OCT_GRASS,                 // 516 (0x204)
    OBJ_OCT_GRASS_LEAF,            // 517 (0x205)
    OBJ_TUMBLE_WEED,               // 518 (0x206)
    TUMBLE_WEED_TAG,               // 519 (0x207)
    OBJ_FLOWER_ANCIENT,            // 520 (0x208)
    OBJ_BARREL,                    // 521 (0x209)
    OBJ_WARP,                      // 522 (0x20A)
    OBJ_WATER_MARK,                // 523 (0x20B)
    OBJ_WATER_JAR,                 // 524 (0x20C)
    OBJ_STOPPING_ROPE,             // 525 (0x20D)
    OBJ_TRAP_BIRD_WOOD,            // 526 (0x20E)
    OBJ_TACKLE,                    // 527 (0x20F)
    TACKLE_TAG,                    // 528 (0x210)
    OBJ_VORTEX,                    // 529 (0x211)
    OBJ_TOWER_BOMB,                // 530 (0x212)
    OBJ_SEAT_SWORD,                // 531 (0x213)
    OBJ_POLE_STONY,                // 532 (0x214)
    OBJ_SWORD_CANDLE,              // 533 (0x215)
    OBJ_SAVE,                      // 534 (0x216)
    OBJ_POOL_COCK,                 // 535 (0x217)
    OBJ_FIREWALL,                  // 536 (0x218)
    HARP_TAG,                      // 537 (0x219)
    OBJ_SWORD_STAB,                // 538 (0x21A)
    OBJ_GODDESS_CUBE,              // 539 (0x21B)
    OBJ_TIME_BLOCK,                // 540 (0x21C)
    OBJ_MOVE_ELEC,                 // 541 (0x21D)
    OBJ_LAVA_D201,                 // 542 (0x21E)
    OBJ_HARP_HINT,                 // 543 (0x21F)
    OBJ_F302_LIGHT,                // 544 (0x220)
    OBJ_TOD3_STONE,                // 545 (0x221)
    OBJ_B300_SAND,                 // 546 (0x222)
    T_DOWSING,                     // 547 (0x223)
    T_MAP_MARK,                    // 548 (0x224)
    BEETLE_TAG,                    // 549 (0x225)
    EFFECT_GEN_TAG,                // 550 (0x226)
    TAG_TIME_AREA_CHECK,           // 551 (0x227)
    TAG_RESTART_TIME_STONE,        // 552 (0x228)
    SHOP_SAMPLE,                   // 553 (0x229)
    OBJ_TERRY_GIMMICK,             // 554 (0x22A)
    OBJ_TERRY_SWITCH,              // 555 (0x22B)
    OBJ_TERRY_HOLE,                // 556 (0x22C)
    OBJ_TERRY_BIKE,                // 557 (0x22D)
    OBJ_JUNK_REPAIR,               // 558 (0x22E)
    CO_TEST,                       // 559 (0x22F)
    OBJ_ARROW_SWITCH,              // 560 (0x230)
    OBJ_VENT_FAN,                  // 561 (0x231)
    OBJ_ELECTRIC_LIGHT,            // 562 (0x232)
    OBJ_WATER_SWITCH,              // 563 (0x233)
    OBJ_ROTATION_LIGHT,            // 564 (0x234)
    OBJ_HOLE_MINIGAME,             // 565 (0x235)
    OBJ_CLOUD_DIVE,                // 566 (0x236)
    OBJ_MUSASABI,                  // 567 (0x237)
    OBJ_FORTUNE_RING,              // 568 (0x238)
    OBJ_BLOW_COAL,                 // 569 (0x239)
    OBJ_SPIKE,                     // 570 (0x23A)
    OBJ_WATER_SPOUT,               // 571 (0x23B)
    OBJ_SMOKE,                     // 572 (0x23C)
    OBJ_LIGHTHOUSE_LIGHT,          // 573 (0x23D)
    OBJ_WATER_IGAIGA,              // 574 (0x23E)
    OBJ_BLADE,                     // 575 (0x23F)
    OBJ_FIRE_OBSTACLE,             // 576 (0x240)
    OBJ_FIRE_PILLAR,               // 577 (0x241)
    OBJ_GUARD_LOG,                 // 578 (0x242)
    OBJ_SLICE_LOG,                 // 579 (0x243)
    OBJ_SLICE_LOG_PARTS,           // 580 (0x244)
    OBJ_STAGE_DEBRIS,              // 581 (0x245)
    OBJ_GROUND_COVER,              // 582 (0x246)
    OBJ_CUMUL_CLOUD,               // 583 (0x247)
    OBJ_UNDER_CLOUD,               // 584 (0x248)
    OBJ_WATERFALL_F102,            // 585 (0x249)
    OBJ_GOD_MARK,                  // 586 (0x24A)
    OBJ_IMPA_DOOR,                 // 587 (0x24B)
    OBJ_WATERFALL_D100,            // 588 (0x24C)
    OBJ_GIRAHIM_FOOT,              // 589 (0x24D)
    OBJ_ISLAND_LOD,                // 590 (0x24E)
    OBJ_UTA_DEMO_PEDEST,           // 591 (0x24F)
    OBJ_LAVA_F200,                 // 592 (0x250)
    OBJ_ROPE_BASE,                 // 593 (0x251)
    OBJ_SUN_LIGHT,                 // 594 (0x252)
    OBJ_SIREN_2DMAP,               // 595 (0x253)
    OBJ_DISPLAY_ONLY_NBS,          // 596 (0x254)
    OBJ_AMBER,                     // 597 (0x255)
    OBJ_BIRD_STATUE,               // 598 (0x256)
    OBJ_F400_GATE_LEAF,            // 599 (0x257)
    OBJ_F400_GATE_SEAL,            // 600 (0x258)
    OBJ_MAPPARTS,                  // 601 (0x259)
    OBJ_RO_AT_TARGET,              // 602 (0x25A)
    RO_AT_TAR_MANAGER_TAG,         // 603 (0x25B)
    TAG_MUSASABI,                  // 604 (0x25C)
    TAG_MAP_INST,                  // 605 (0x25D)
    TAG_AUTO_MESSAGE,              // 606 (0x25E)
    TAG_SHIP_SLOPE,                // 607 (0x25F)
    TAG_SHIP_FLOOD,                // 608 (0x260)
    TAG_BARREL,                    // 609 (0x261)
    TAG_BARREL_POS,                // 610 (0x262)
    TAG_HEAT_RESIST,               // 611 (0x263)
    TAG_HOLY_WATER,                // 612 (0x264)
    TAG_BELT_OBSTACLE,             // 613 (0x265)
    TAG_DRUM,                      // 614 (0x266)
    TAG_ROLL_ATTACK_LOG,           // 615 (0x267)
    TAG_SHIP_WINDOW,               // 616 (0x268)
    ARROW,                         // 617 (0x269)
    MASS_OBJ_TAG,                  // 618 (0x26A)
    SOUND_AREA_MGR,                // 619 (0x26B)
    TAG_SOUND_AREA,                // 620 (0x26C)
    ATT_TAG,                       // 621 (0x26D)
    TLP_TAG,                       // 622 (0x26E)
    SKYENEMY_T,                    // 623 (0x26F)
    TOUCH_TAG,                     // 624 (0x270)
    CAMERA_TAG,                    // 625 (0x271)
    CAMERA2_TAG,                   // 626 (0x272)
    ACTION_TAG,                    // 627 (0x273)
    SC_CHANGE_TAG,                 // 628 (0x274)
    GATE2GND_TAG,                  // 629 (0x275)
    ALLDIE_TAG,                    // 630 (0x276)
    SW_TAG,                        // 631 (0x277)
    PL_RESTART,                    // 632 (0x278)
    SW_AREA_TAG,                   // 633 (0x279)
    SIREN_TAG,                     // 634 (0x27A)
    TAG_TKEVNT,                    // 635 (0x27B)
    MOLE_PROHIBIT_TAG,             // 636 (0x27C)
    TAG_DEFEAT_BOSS,               // 637 (0x27D)
    TAG_TIMER,                     // 638 (0x27E)
    TAG_FENCE_SYNCHRONIZER,        // 639 (0x27F)
    TAG_GENKI_DOWSING_TARGET,      // 640 (0x280)
    ITEM,                          // 641 (0x281)
    OBJ_ITEM_HEART_CONTAINER,      // 642 (0x282)
    OBJ_CLEF,                      // 643 (0x283)
    OBJ_FRUIT_GUTS_LEAF,           // 644 (0x284)
    OBJ_SWRD_PRJ,                  // 645 (0x285)
    OBJ_VACU_DUST_PARTS,           // 646 (0x286)
    OBJ_VACU_DUST,                 // 647 (0x287)
    OBJ_RAIL_POST,                 // 648 (0x288)
    OBJ_RAIL_END,                  // 649 (0x289)
    OBJ_TENI_RAIL,                 // 650 (0x28A)
    OBJ_TENI_RAIL_POST,            // 651 (0x28B)
    OBJ_FORCE_SIGN,                // 652 (0x28C)
    TAG_FORCE_GET_FLAG,            // 653 (0x28D)
    TAG_CLEF_MANAGER,              // 654 (0x28E)
    TAG_CLEF_GAME,                 // 655 (0x28F)
    TAG_MINIGAME_INSECT_CAPTURE,   // 656 (0x290)
    CAMERA,                        // 657 (0x291)
    WEATHER_TAG,                   // 658 (0x292)
    SPORE_TAG,                     // 659 (0x293)
    MIST_TAG,                      // 660 (0x294)
    SPARKS_TAG,                    // 661 (0x295)
    SPARKS2_TAG,                   // 662 (0x296)
    KYTAG_TAG,                     // 663 (0x297)
    LBTHUNDER_TAG,                 // 664 (0x298)
    PLTCHG_TAG,                    // 665 (0x299)
    PLIGHT_TAG,                    // 666 (0x29A)
    VRBOX_TAG,                     // 667 (0x29B)
    NPC_INV,                       // 668 (0x29C)
    NPC_TKE,                       // 669 (0x29D)
    NPC_STR,                       // 670 (0x29E)
    MESSAGE_ACTOR,                 // 671 (0x29F)
    LIGHT_OBJECT,                  // 672 (0x2A0)
    MESSAGE,                       // 673 (0x2A1)
    LYT_CONTROL_GAME,              // 674 (0x2A2)
    LYT_DEMO_DOWSING,              // 675 (0x2A3)
    LYT_CONTROL_TITLE,             // 676 (0x2A4)
    LYT_DROP_LINE,                 // 677 (0x2A5)
    LYT_FORCE_LINE,                // 678 (0x2A6)
    LYT_ENEMY_ICON,                // 679 (0x2A7)
    LYT_MINI_GAME,                 // 680 (0x2A8)
    LYT_SUIRYU_SCORE,              // 681 (0x2A9)
    LYT_SUIRYU_SCORE_COMP,         // 682 (0x2AA)
    LYT_BOSS_CAPTION,              // 683 (0x2AB)
    LYT_PAUSE,                     // 684 (0x2AC)
    LYT_GAMEOVER_MGR,              // 685 (0x2AD)
    LYT_SAVE_MGR,                  // 686 (0x2AE)
    TITLE_MANAGER,                 // 687 (0x2AF)
    LYT_TITLE_BG,                  // 688 (0x2B0)
    LYT_SHOP,                      // 689 (0x2B1)
    LYT_DEPOSIT,                   // 690 (0x2B2)
    LYT_DEMO_TITLE,                // 691 (0x2B3)
    LYT_END_ROLL,                  // 692 (0x2B4)
    LYT_SEEKER_STONE,              // 693 (0x2B5)
    LYT_FILESELECT,                // 694 (0x2B6)
    SKB,                           // 695 (0x2B7)
    EVENT_TAG,                     // 696 (0x2B8)
    EVENTF_TAG,                    // 697 (0x2B9)
    C_GAME,                        // 698 (0x2BA)
    C_BASE,                        // 699 (0x2BB)
    BOOT,                          // 700 (0x2BC)
    ROOM,                          // 701 (0x2BD)
    LAST,                          // 702 (0x2BE)
    NUMBER_OF_ACTORS,              // 703
    INVALID,
}

// probably wrong
#[repr(C)]
struct EnemyLinkedList {
    prev: *mut c_void,
    next: *mut c_void,
}

extern "C" {
    fn findActorByActorType(actor_type: i32, start_actor: *const c_void) -> *mut c_void;
    static ENEMY_LIST: EnemyLinkedList;
}

pub fn get_first_enemy() -> Option<*mut c_void> {
    unsafe {
        if ENEMY_LIST.prev.is_null() {
            return None;
        }

        Some(ENEMY_LIST.prev)
    }
}

pub fn find_actor_by_type(actor_type: i32, start_actor: *const c_void) -> *mut c_void {
    unsafe { findActorByActorType(actor_type, start_actor) }
}
