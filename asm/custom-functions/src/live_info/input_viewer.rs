use crate::system::{
    button::*,
    gx::*,
    math::{self, C_MTXOrtho, Matrix34f, Matrix44f},
};
use crate::utils::{
    char_writer::{CharWriter, TextWriterBase},
    menu::get_y_bottom,
};
use core::fmt::Write;

extern "C" {
    fn SCGetAspectRatio() -> u8;
}

const PANEL_SCALE: f32 = 1.4;
const OUTLINE: f32 = 1.6;
const MAX_RECTS: usize = 6000;
const PANEL_MARGIN_Y: f32 = -22.0;
const PANEL_HEIGHT_UNITS: f32 = 72.0;
const PANEL_WIDTH_UNITS: f32 = 152.5;
const SCREEN_WIDTH: f32 = 640.0;
const STICK_NOTCH: f32 = 0.7071;
const TWO_PI: f32 = 6.283185307179586;
const LABEL_GLOBAL_X_SHIFT: f32 = 0.7;
const LABEL_GLOBAL_Y_SHIFT: f32 = -2.8;

const COLOR_WHITE: u32 = 0xFFFFFFFF;
const COLOR_A: u32 = 0xBFBFBFFF;
const COLOR_HOME: u32 = 0x00BFFFFF;
const COLOR_SHAKE_ACTIVE: u32 = 0x00FF00FF;
const COLOR_SHADOW: u32 = 0x00000060;
const COLOR_PRESS_TEXT: u32 = 0x00000060;

#[derive(Clone, Copy)]
struct RectCmd {
    x:     f32,
    y:     f32,
    w:     f32,
    h:     f32,
    color: u32,
}

const EMPTY_RECT: RectCmd = RectCmd {
    x:     0.0,
    y:     0.0,
    w:     0.0,
    h:     0.0,
    color: 0,
};

#[link_section = "data"]
static mut RECT_BUFFER: [RectCmd; MAX_RECTS] = [EMPTY_RECT; MAX_RECTS];

#[derive(Clone, Copy)]
struct Transform {
    base_x:  f32,
    base_y:  f32,
    scale:   f32,
    x_ratio: f32,
}

impl Transform {
    fn x(&self, x: f32) -> f32 {
        self.base_x + x * self.scale * self.x_ratio
    }
    fn y(&self, y: f32) -> f32 {
        self.base_y + y * self.scale
    }
    fn w(&self, w: f32) -> f32 {
        w * self.scale * self.x_ratio
    }
    fn h(&self, h: f32) -> f32 {
        h * self.scale
    }
}

fn clamp01(v: f32) -> f32 {
    if v < 0.0 {
        0.0
    } else if v > 1.0 {
        1.0
    } else {
        v
    }
}

fn min_f32(a: f32, b: f32) -> f32 {
    if a < b { a } else { b }
}

fn max_f32(a: f32, b: f32) -> f32 {
    if a > b { a } else { b }
}

fn push_rect(rects: &mut [RectCmd; MAX_RECTS], count: &mut usize, x: f32, y: f32, w: f32, h: f32, color: u32) {
    if w <= 0.0 || h <= 0.0 || (color & 0xFF) == 0 || *count >= MAX_RECTS {
        return;
    }
    rects[*count] = RectCmd { x, y, w, h, color };
    *count += 1;
}

fn push_outline(rects: &mut [RectCmd; MAX_RECTS], count: &mut usize, x: f32, y: f32, w: f32, h: f32, color: u32) {
    push_rect(rects, count, x, y, w, OUTLINE, color);
    push_rect(rects, count, x, y + h - OUTLINE, w, OUTLINE, color);
    push_rect(rects, count, x, y, OUTLINE, h, color);
    push_rect(rects, count, x + w - OUTLINE, y, OUTLINE, h, color);
}

fn push_line(
    rects: &mut [RectCmd; MAX_RECTS],
    count: &mut usize,
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    thickness: f32,
    color: u32,
) {
    let dx = x1 - x0;
    let dy = y1 - y0;
    let abs_dx = if dx < 0.0 { -dx } else { dx };
    let abs_dy = if dy < 0.0 { -dy } else { dy };
    let steps = max_f32(abs_dx, abs_dy) as i32 + 1;
    if steps <= 0 {
        push_rect(
            rects,
            count,
            x0 - thickness * 0.5,
            y0 - thickness * 0.5,
            thickness,
            thickness,
            color,
        );
        return;
    }
    let inv = 1.0 / steps as f32;
    let mut i = 0;
    while i <= steps {
        let t = i as f32 * inv;
        let x = x0 + dx * t;
        let y = y0 + dy * t;
        push_rect(
            rects,
            count,
            x - thickness * 0.5,
            y - thickness * 0.5,
            thickness,
            thickness,
            color,
        );
        i += 1;
    }
}

fn push_ellipse_outline(
    rects: &mut [RectCmd; MAX_RECTS],
    count: &mut usize,
    cx: f32,
    cy: f32,
    rx: f32,
    ry: f32,
    segments: i32,
    thickness: f32,
    color: u32,
) {
    let mut i = 0;
    while i < segments {
        let a0 = TWO_PI * (i as f32) / segments as f32;
        let a1 = TWO_PI * ((i + 1) as f32) / segments as f32;
        let x0 = cx + rx * math::cos(a0);
        let y0 = cy + ry * math::sin(a0);
        let x1 = cx + rx * math::cos(a1);
        let y1 = cy + ry * math::sin(a1);
        push_line(rects, count, x0, y0, x1, y1, thickness, color);
        i += 1;
    }
}

fn push_ellipse_filled(
    rects: &mut [RectCmd; MAX_RECTS],
    count: &mut usize,
    cx: f32,
    cy: f32,
    rx: f32,
    ry: f32,
    color: u32,
) {
    if rx <= 0.0 || ry <= 0.0 {
        return;
    }
    let mut iy = -(ry as i32);
    while iy <= ry as i32 {
        let y = iy as f32;
        let y_norm = y / ry;
        let xr = rx * math::sqrt(max_f32(1.0 - y_norm * y_norm, 0.0));
        push_rect(rects, count, cx - xr, cy + y, xr * 2.0, 1.2, color);
        iy += 1;
    }
}

fn push_circle_outline(
    rects: &mut [RectCmd; MAX_RECTS],
    count: &mut usize,
    cx: f32,
    cy: f32,
    r: f32,
    segments: i32,
    thickness: f32,
    color: u32,
) {
    push_ellipse_outline(rects, count, cx, cy, r, r, segments, thickness, color);
}

fn push_circle_filled(
    rects: &mut [RectCmd; MAX_RECTS],
    count: &mut usize,
    cx: f32,
    cy: f32,
    r: f32,
    color: u32,
) {
    push_ellipse_filled(rects, count, cx, cy, r, r, color);
}

fn push_octagon_outline(
    rects: &mut [RectCmd; MAX_RECTS],
    count: &mut usize,
    cx: f32,
    cy: f32,
    rx: f32,
    ry: f32,
    thickness: f32,
    color: u32,
) {
    let pts = [
        (cx - rx * 0.42, cy - ry),
        (cx + rx * 0.42, cy - ry),
        (cx + rx, cy - ry * 0.42),
        (cx + rx, cy + ry * 0.42),
        (cx + rx * 0.42, cy + ry),
        (cx - rx * 0.42, cy + ry),
        (cx - rx, cy + ry * 0.42),
        (cx - rx, cy - ry * 0.42),
    ];
    let mut i = 0usize;
    while i < pts.len() {
        let (x0, y0) = pts[i];
        let (x1, y1) = pts[(i + 1) % pts.len()];
        push_line(rects, count, x0, y0, x1, y1, thickness, color);
        i += 1;
    }
}

fn push_stick_outline_tpgz(
    rects: &mut [RectCmd; MAX_RECTS],
    count: &mut usize,
    cx: f32,
    cy: f32,
    rx: f32,
    ry: f32,
    thickness: f32,
    color: u32,
) {
    let pts = [
        (cx, cy - ry),
        (cx + rx * STICK_NOTCH, cy - ry * STICK_NOTCH),
        (cx + rx, cy),
        (cx + rx * STICK_NOTCH, cy + ry * STICK_NOTCH),
        (cx, cy + ry),
        (cx - rx * STICK_NOTCH, cy + ry * STICK_NOTCH),
        (cx - rx, cy),
        (cx - rx * STICK_NOTCH, cy - ry * STICK_NOTCH),
    ];
    let mut i = 0usize;
    while i < pts.len() {
        let (x0, y0) = pts[i];
        let (x1, y1) = pts[(i + 1) % pts.len()];
        push_line(rects, count, x0, y0, x1, y1, thickness, color);
        i += 1;
    }
}

fn begin_ui_batch(quad_count: u16) {
    let pos_mtx = &mut Matrix34f {
        m: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
        ],
    };
    let ortho_mtx = &mut Matrix44f::default();

    unsafe {
        C_MTXOrtho(ortho_mtx, 0.0, 480.0, 0.0, 640.0, 0.0, 1.0);
        GXSetProjection(ortho_mtx, 1);
        GXSetViewport(0.0, 0.0, 640.0, 480.0, 0.0, 1.0);
        GXSetScissor(0, 0, 640, 480);
        GXLoadPosMtxImm(pos_mtx, 0);
        GXSetCurrentMtx(0);
        GXClearVtxDesc();
        GXInvalidateVtxCache();
        GXSetVtxDesc(GXAttr::GX_VA_POS, GXAttrType::GX_DIRECT);
        GXSetVtxDesc(GXAttr::GX_VA_CLR0, GXAttrType::GX_DIRECT);
        GXSetVtxAttrFmt(
            GXVtxFmt::GX_VTXFMT0,
            GXAttr::GX_VA_POS,
            GXCompCnt::GX_POS_XYZ,
            GXCompType::GX_F32,
            0,
        );
        GXSetVtxAttrFmt(
            GXVtxFmt::GX_VTXFMT0,
            GXAttr::GX_VA_CLR0,
            GXCompCnt::GX_CLR_RGBA,
            GXCompType::GX_RGBA8,
            0,
        );
        GXSetNumChans(1);
        GXSetChanCtrl(
            GXChannelID::GX_COLOR0A0,
            GXBool::GX_FALSE,
            GXColorSrc::GX_SRC_VTX,
            GXColorSrc::GX_SRC_VTX,
            0,
            GXDiffuseFn::GX_DF_NONE,
            GXAttnFn::GX_AF_NONE,
        );
        GXSetNumTexGens(0);
        GXSetNumIndStages(0);
        __GXSetIndirectMask(0);
        GXSetNumTevStages(1);
        GXSetTevOp(GXTevStageID::GX_TEVSTAGE0, GXTevMode::GX_PASSCLR);
        GXSetTevOrder(
            GXTevStageID::GX_TEVSTAGE0,
            GXTexCoordID::GX_TEXCOORD_NULL,
            GXTexMapID::GX_TEXMAP_NULL,
            GXChannelID::GX_COLOR0A0,
        );
        GXSetBlendMode(
            GXBlendMode::GX_BM_BLEND,
            GXBlendFactor::GX_BL_SRC_ALPHA,
            GXBlendFactor::GX_BL_INV_SRC_ALPHA,
            GXLogicOp::GX_LO_SET,
        );
        GXSetColorUpdate(GXBool::GX_TRUE);
        GXSetAlphaUpdate(GXBool::GX_TRUE);
        GXSetZMode(GXBool::GX_FALSE, GXCompare::GX_NEVER, GXBool::GX_FALSE);
        GXSetCullMode(GXCullMode::GX_CULL_NONE);
        GXBegin(GXPrimitive::GX_QUADS, GXVtxFmt::GX_VTXFMT0, quad_count * 4);
    }
}

fn render_rect_batch(rects: &[RectCmd], count: usize) {
    if count == 0 {
        return;
    }
    begin_ui_batch(count as u16);
    for r in &rects[..count] {
        GXPosition3f32(r.x, r.y, 0.0);
        GXColor1u32(r.color);
        GXPosition3f32(r.x + r.w, r.y, 0.0);
        GXColor1u32(r.color);
        GXPosition3f32(r.x + r.w, r.y + r.h, 0.0);
        GXColor1u32(r.color);
        GXPosition3f32(r.x, r.y + r.h, 0.0);
        GXColor1u32(r.color);
    }
}

fn draw_label(writer: &mut TextWriterBase, x: f32, y: f32, color: u32, text: &str, scale: f32) {
    writer.set_scale(scale);
    writer.set_cursor([x, y, 0.0]);
    let mut line = CharWriter::new();
    line.set_bg_color(0x00000000);
    line.set_font_color(color);
    let _ = line.write_str(text);
    line.draw(writer);
}

fn draw_centered_label(
    writer: &mut TextWriterBase,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    pressed: bool,
    color: u32,
    text: &str,
    scale: f32,
) {
    draw_centered_label_offset(writer, x, y, w, h, pressed, color, text, scale, 0.0, 0.0);
}

fn draw_centered_label_offset(
    writer: &mut TextWriterBase,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    pressed: bool,
    color: u32,
    text: &str,
    scale: f32,
    off_x: f32,
    off_y: f32,
) {
    // Avoid rect query calls that can crash when font state is null.
    let est_w = 7.0 * scale * text.len() as f32;
    let est_h = 12.8 * scale;
    let tx = x + (w - est_w) * 0.5 + off_x + LABEL_GLOBAL_X_SHIFT;
    let ty = y + (h - est_h) * 0.5 + off_y + LABEL_GLOBAL_Y_SHIFT;
    draw_label(
        writer,
        tx,
        ty,
        if pressed { COLOR_PRESS_TEXT } else { color },
        text,
        scale,
    );
}

fn draw_centered_ellipse_label(
    writer: &mut TextWriterBase,
    cx: f32,
    cy: f32,
    rx: f32,
    ry: f32,
    pressed: bool,
    color: u32,
    text: &str,
    scale: f32,
) {
    draw_centered_ellipse_label_offset(writer, cx, cy, rx, ry, pressed, color, text, scale, 0.0, 0.0);
}

fn draw_centered_ellipse_label_offset(
    writer: &mut TextWriterBase,
    cx: f32,
    cy: f32,
    rx: f32,
    ry: f32,
    pressed: bool,
    color: u32,
    text: &str,
    scale: f32,
    off_x: f32,
    off_y: f32,
) {
    draw_centered_label_offset(
        writer,
        cx - rx,
        cy - ry,
        rx * 2.0,
        ry * 2.0,
        pressed,
        color,
        text,
        scale,
        off_x,
        off_y,
    );
}

fn draw_centered_circle_label(
    writer: &mut TextWriterBase,
    cx: f32,
    cy: f32,
    r: f32,
    pressed: bool,
    color: u32,
    text: &str,
    scale: f32,
) {
    let d = r * 2.0;
    draw_centered_label(writer, cx - r, cy - r, d, d, pressed, color, text, scale);
}

fn build_wii_viewer_geometry(
    t: Transform,
    shadow: bool,
    down: Buttons,
    stick_x: f32,
    stick_y: f32,
    nunchuck_shake: f32,
    wiimote_shake: f32,
    rects: &mut [RectCmd; MAX_RECTS],
    count: &mut usize,
) {
    let white = if shadow { COLOR_SHADOW } else { COLOR_WHITE };
    let a_color = if shadow { COLOR_SHADOW } else { COLOR_A };
    let home_color = if shadow { COLOR_SHADOW } else { COLOR_HOME };

    // Top shake sliders.
    let bar_w = t.w(35.0);
    let bar_h = t.h(7.0);
    let left_x = t.x(0.0);
    let right_x = t.x(117.5);
    let bar_y = t.y(0.0);
    push_outline(rects, count, left_x, bar_y, bar_w, bar_h, white);
    push_outline(rects, count, right_x, bar_y, bar_w, bar_h, white);
    let left_fill = clamp01(nunchuck_shake * 1.5);
    let right_fill = clamp01(wiimote_shake * 1.5);
    let left_color = if shadow {
        COLOR_SHADOW
    } else if left_fill > 0.2 {
        COLOR_SHAKE_ACTIVE
    } else {
        COLOR_WHITE
    };
    let right_color = if shadow {
        COLOR_SHADOW
    } else if right_fill > 0.2 {
        COLOR_SHAKE_ACTIVE
    } else {
        COLOR_WHITE
    };
    push_rect(
        rects,
        count,
        left_x + OUTLINE,
        bar_y + OUTLINE,
        (bar_w - OUTLINE * 2.0) * left_fill,
        bar_h - OUTLINE * 2.0,
        left_color,
    );
    let right_fill_w = (bar_w - OUTLINE * 2.0) * right_fill;
    push_rect(
        rects,
        count,
        right_x + bar_w - OUTLINE - right_fill_w,
        bar_y + OUTLINE,
        right_fill_w,
        bar_h - OUTLINE * 2.0,
        right_color,
    );

    // Left stick octagon + dot
    let stick_cx = t.x(17.5);
    let stick_cy = t.y(30.0);
    let stick_rx = t.w(17.5);
    let stick_ry = t.h(17.5);
    push_stick_outline_tpgz(rects, count, stick_cx, stick_cy, stick_rx, stick_ry, OUTLINE, white);
    push_ellipse_filled(
        rects,
        count,
        t.x(17.5 + stick_x * 10.0),
        t.y(30.0 - stick_y * 10.0),
        t.w(10.0),
        t.h(10.0),
        white,
    );

    // C and Z buttons
    push_ellipse_outline(
        rects,
        count,
        t.x(50.0),
        t.y(20.0),
        t.w(8.4),
        t.h(5.8),
        18,
        OUTLINE,
        white,
    );
    if down.contains(C) {
        push_ellipse_filled(
            rects,
            count,
            t.x(50.0),
            t.y(20.0),
            t.w(6.8),
            t.h(4.8),
            white,
    );
}
    push_outline(rects, count, t.x(40.0), t.y(30.0), t.w(20.0), t.h(15.0), white);
    if down.contains(Z) {
        push_rect(
            rects,
            count,
            t.x(40.0) + OUTLINE,
            t.y(30.0) + OUTLINE,
            t.w(20.0) - OUTLINE * 2.0,
            t.h(15.0) - OUTLINE * 2.0,
            white,
        );
    }

    // D-pad plus
    let dpx = t.x(75.0);
    let dpy = t.y(10.0);
    let s = t.h(25.0);
    let bw = 3.0 * s / 11.0;
    let bl = 4.0 * s / 11.0;
    let up_x = dpx + bl * t.x_ratio;
    let up_y = dpy;
    let up_w = bw * t.x_ratio;
    let up_h = bl;
    let left_x_arm = dpx;
    let left_y_arm = dpy + bl;
    let left_w = bl * t.x_ratio;
    let left_h = bw;
    let right_x_arm = dpx + (bl + bw) * t.x_ratio;
    let down_x = up_x;
    let down_y = dpy + bl + bw;

    push_outline(rects, count, left_x_arm, left_y_arm, left_w, left_h, white);
    if down.contains(DPAD_LEFT) {
        push_rect(rects, count, left_x_arm + OUTLINE, left_y_arm + OUTLINE, left_w - OUTLINE * 2.0, left_h - OUTLINE * 2.0, white);
    }
    push_outline(rects, count, up_x, up_y, up_w, up_h, white);
    if down.contains(DPAD_UP) {
        push_rect(rects, count, up_x + OUTLINE, up_y + OUTLINE, up_w - OUTLINE * 2.0, up_h - OUTLINE * 2.0, white);
    }
    push_outline(rects, count, right_x_arm, left_y_arm, left_w, left_h, white);
    if down.contains(DPAD_RIGHT) {
        push_rect(rects, count, right_x_arm + OUTLINE, left_y_arm + OUTLINE, left_w - OUTLINE * 2.0, left_h - OUTLINE * 2.0, white);
    }
    push_outline(rects, count, down_x, down_y, up_w, up_h, white);
    if down.contains(DPAD_DOWN) {
        push_rect(rects, count, down_x + OUTLINE, down_y + OUTLINE, up_w - OUTLINE * 2.0, up_h - OUTLINE * 2.0, white);
    }

    // Minus/Home/Plus circles
    let minus_cx = t.x(120.0);
    let home_cx = t.x(130.0);
    let plus_cx = t.x(140.0);
    let top_cy = t.y(20.0);
    push_ellipse_outline(rects, count, minus_cx, top_cy, t.w(5.0), t.h(5.0), 18, OUTLINE, white);
    push_ellipse_outline(rects, count, home_cx, top_cy, t.w(5.0), t.h(5.0), 18, OUTLINE, home_color);
    push_ellipse_outline(rects, count, plus_cx, top_cy, t.w(5.0), t.h(5.0), 18, OUTLINE, white);
    if down.contains(MINUS) {
        push_ellipse_filled(rects, count, minus_cx, top_cy, t.w(3.8), t.h(3.8), white);
    }
    if down.contains(HOME) {
        push_ellipse_filled(rects, count, home_cx, top_cy, t.w(3.8), t.h(3.8), home_color);
    }
    if down.contains(PLUS) {
        push_ellipse_filled(rects, count, plus_cx, top_cy, t.w(3.8), t.h(3.8), white);
    }

    // 1/2 circles
    let one_cx = t.x(127.5) + t.w(7.5);
    let one_cy = t.y(30.0) + t.h(7.5);
    let two_cx = t.x(127.5) + t.w(7.5);
    let two_cy = t.y(50.0) + t.h(7.5);
    push_ellipse_outline(rects, count, one_cx, one_cy, t.w(7.5), t.h(7.5), 22, OUTLINE, white);
    push_ellipse_outline(rects, count, two_cx, two_cy, t.w(7.5), t.h(7.5), 22, OUTLINE, white);
    if down.contains(ONE) {
        push_ellipse_filled(rects, count, one_cx, one_cy, t.w(6.0), t.h(6.0), white);
    }
    if down.contains(TWO) {
        push_ellipse_filled(rects, count, two_cx, two_cy, t.w(6.0), t.h(6.0), white);
    }

    // A/B
    let a_cx = t.x(70.0) + t.w(7.5);
    let a_cy = t.y(42.5) + t.h(7.5);
    push_ellipse_outline(rects, count, a_cx, a_cy, t.w(7.5), t.h(7.5), 22, OUTLINE, a_color);
    if down.contains(A) {
        push_ellipse_filled(rects, count, a_cx, a_cy, t.w(6.0), t.h(6.0), a_color);
    }
    push_outline(rects, count, t.x(93.0), t.y(40.0), t.w(13.0), t.h(20.0), white);
    if down.contains(B) {
        push_rect(
            rects,
            count,
            t.x(93.0) + OUTLINE,
            t.y(40.0) + OUTLINE,
            t.w(13.0) - OUTLINE * 2.0,
            t.h(20.0) - OUTLINE * 2.0,
            white,
        );
    }
}

fn draw_wii_viewer_text(writer: &mut TextWriterBase, t: Transform, shadow: bool, down: Buttons, stick_x: f32, stick_y: f32) {
    let white = if shadow { COLOR_SHADOW } else { COLOR_WHITE };
    let a_color = if shadow { COLOR_SHADOW } else { COLOR_A };

    draw_centered_label_offset(
        writer,
        t.x(40.0),
        t.y(30.0),
        t.w(20.0),
        t.h(15.0),
        down.contains(Z),
        white,
        "Z",
        0.42,
        -3.2,
        -2.6,
    );
    draw_centered_label_offset(
        writer,
        t.x(93.0),
        t.y(40.0),
        t.w(13.0),
        t.h(20.0),
        down.contains(B),
        white,
        "B",
        0.42,
        -3.2,
        -2.6,
    );

    draw_centered_ellipse_label_offset(
        writer,
        t.x(50.0),
        t.y(20.0),
        t.w(8.4),
        t.h(5.8),
        down.contains(C),
        white,
        "C",
        0.38,
        -3.8,
        -2.2,
    );
    draw_centered_ellipse_label_offset(
        writer,
        t.x(70.0) + t.w(7.5),
        t.y(42.5) + t.h(7.5),
        t.w(7.5),
        t.h(7.5),
        down.contains(A),
        a_color,
        "A",
        0.42,
        -3.6,
        -2.6,
    );
    draw_centered_ellipse_label_offset(
        writer,
        t.x(127.5) + t.w(7.5),
        t.y(30.0) + t.h(7.5),
        t.w(7.5),
        t.h(7.5),
        down.contains(ONE),
        white,
        "1",
        0.42,
        -3.0,
        -2.6,
    );
    draw_centered_ellipse_label_offset(
        writer,
        t.x(127.5) + t.w(7.5),
        t.y(50.0) + t.h(7.5),
        t.w(7.5),
        t.h(7.5),
        down.contains(TWO),
        white,
        "2",
        0.42,
        -2.8,
        -2.6,
    );
    draw_centered_ellipse_label_offset(
        writer,
        t.x(120.0),
        t.y(20.0),
        t.w(5.0),
        t.h(5.0),
        down.contains(MINUS),
        white,
        "-",
        0.38,
        -2.1,
        -2.2,
    );
    draw_centered_ellipse_label_offset(
        writer,
        t.x(140.0),
        t.y(20.0),
        t.w(5.0),
        t.h(5.0),
        down.contains(PLUS),
        white,
        "+",
        0.38,
        -2.8,
        -2.2,
    );

    let dpx = t.x(75.0);
    let dpy = t.y(10.0);
    let s = t.h(25.0);
    let bw = 3.0 * s / 11.0;
    let bl = 4.0 * s / 11.0;
    let up_x = dpx + bl * t.x_ratio;
    let up_y = dpy;
    let up_w = bw * t.x_ratio;
    let up_h = bl;
    let left_x_arm = dpx;
    let left_y_arm = dpy + bl;
    let left_w = bl * t.x_ratio;
    let left_h = bw;
    let right_x_arm = dpx + (bl + bw) * t.x_ratio;
    let down_x = up_x;
    let down_y = dpy + bl + bw;

    draw_centered_label_offset(writer, up_x, up_y, up_w, up_h, down.contains(DPAD_UP), white, "|", 0.38, -0.4, -2.2);
    draw_centered_label_offset(
        writer,
        down_x,
        down_y,
        up_w,
        up_h,
        down.contains(DPAD_DOWN),
        white,
        "|",
        0.38,
        -0.4,
        -2.2,
    );
    draw_centered_label_offset(
        writer,
        left_x_arm,
        left_y_arm,
        left_w,
        left_h,
        down.contains(DPAD_LEFT),
        white,
        "-",
        0.38,
        -2.2,
        -2.2,
    );
    draw_centered_label_offset(
        writer,
        right_x_arm,
        left_y_arm,
        left_w,
        left_h,
        down.contains(DPAD_RIGHT),
        white,
        "-",
        0.38,
        -2.2,
        -2.2,
    );

    let sx = (stick_x * 99.0) as i32;
    let sy = (stick_y * 99.0) as i32;
    let mut line = CharWriter::new();
    line.set_bg_color(0x00000000);
    line.set_font_color(white);
    let _ = line.write_fmt(format_args!("{sx:>3} {sy:>3}"));
    writer.set_scale(0.35);
    writer.set_cursor([t.x(0.0), t.y(65.0), 0.0]);
    line.draw(writer);
}

pub fn display() {
    let x_ratio = if unsafe { SCGetAspectRatio() == 0 } { 1.0 } else { 0.75 };
    let t = Transform {
        base_x:  (SCREEN_WIDTH - PANEL_WIDTH_UNITS * PANEL_SCALE * x_ratio) * 0.5,
        base_y:  get_y_bottom() - PANEL_HEIGHT_UNITS * PANEL_SCALE - PANEL_MARGIN_Y,
        scale:   PANEL_SCALE,
        x_ratio,
    };


    let down = raw_buttons_down();
    let stick = raw_stick_pos();
    let stick_x = stick[0].clamp(-1.0, 1.0);
    let stick_y = stick[1].clamp(-1.0, 1.0);
    let nunchuck_shake = raw_nunchuck_shake();
    let wiimote_shake = raw_wiimote_shake();

    let mut rect_count = 0usize;
    let rects = unsafe { &mut RECT_BUFFER };

    // TPGZ-style shadow pass.
    let shadow_t = Transform {
        base_x: t.base_x + 1.0,
        base_y: t.base_y + 1.0,
        scale: t.scale,
        x_ratio: t.x_ratio,
    };
    build_wii_viewer_geometry(
        shadow_t,
        true,
        down,
        stick_x,
        stick_y,
        nunchuck_shake,
        wiimote_shake,
        rects,
        &mut rect_count,
    );
    render_rect_batch(rects, rect_count);

    rect_count = 0;
    build_wii_viewer_geometry(
        t,
        false,
        down,
        stick_x,
        stick_y,
        nunchuck_shake,
        wiimote_shake,
        rects,
        &mut rect_count,
    );
    render_rect_batch(rects, rect_count);

    let mut writer = TextWriterBase::new();
    writer.set_position(0.0, 0.0);
    draw_wii_viewer_text(&mut writer, shadow_t, true, down, stick_x, stick_y);
    draw_wii_viewer_text(&mut writer, t, false, down, stick_x, stick_y);
}
