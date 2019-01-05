//! Basic example of rendering on Cairo.

use std::fs::File;

use kurbo::{BezPath, Line};

use cairo::{Context, Format, ImageSurface};

use piet::{FillRule, FontBuilder, RenderContext, TextLayout, TextLayoutBuilder};
use piet_cairo::CairoRenderContext;

const TEXTURE_WIDTH: i32 = 400;
const TEXTURE_HEIGHT: i32 = 200;

const HIDPI: f64 = 2.0;

fn draw_pretty_picture<R: RenderContext>(rc: &mut R) {
    rc.clear(0xFF_FF_FF);
    let brush = rc.solid_brush(0x00_00_80_FF);
    rc.stroke(&Line::new((10.0, 10.0), (100.0, 50.0)), &brush, 1.0, None);

    let mut path = BezPath::new();
    path.moveto((50.0, 10.0));
    path.quadto((60.0, 50.0), (100.0, 90.0));
    let brush = rc.solid_brush(0x00_80_00_FF);
    rc.stroke(&path, &brush, 1.0, None);

    let mut path = BezPath::new();
    path.moveto((10.0, 20.0));
    path.curveto((10.0, 80.0), (100.0, 80.0), (100.0, 60.0));
    let brush = rc.solid_brush(0x00_00_80_C0);
    rc.fill(&path, &brush, FillRule::NonZero);

    let font = rc.new_font_by_name("Segoe UI", 12.0).build();
    let layout = rc.new_text_layout(&font, "Hello piet-cairo!").build();
    let w: f64 = layout.width().into();
    let brush = rc.solid_brush(0x80_00_00_C0);
    rc.draw_text(&layout, (80.0, 10.0), &brush);

    rc.stroke(
        &Line::new((80.0, 12.0), (80.0 + w, 12.0)),
        &brush,
        1.0,
        None,
    );
}

fn main() {
    let surface = ImageSurface::create(Format::ARgb32, TEXTURE_WIDTH, TEXTURE_HEIGHT)
        .expect("Can't create surface");
    let mut cr = Context::new(&surface);
    cr.scale(HIDPI, HIDPI);
    let mut piet_context = CairoRenderContext::new(&mut cr);
    draw_pretty_picture(&mut piet_context);
    let mut file = File::create("temp-cairo.png").expect("Couldn't create 'file.png'");
    surface
        .write_to_png(&mut file)
        .expect("Error writing image file");
}
