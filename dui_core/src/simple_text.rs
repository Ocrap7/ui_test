use std::collections::HashMap;
use std::sync::Arc;

use vello::fello::raw::FontRef;
use vello::fello::MetadataProvider;
use vello::glyph::{Glyph, GlyphContext};
use vello::kurbo::{Affine, Rect};
use vello::peniko::{Blob, BrushRef, Font, StyleRef};
use vello::{peniko::Brush, SceneBuilder};

// This is very much a hack to get things working.
// On Windows, can set this to "c:\\Windows\\Fonts\\seguiemj.ttf" to get color emoji
const OPEN_SANS_DATA: &[u8] =
    include_bytes!("../../res/fonts/Open_Sans/static/OpenSans-Regular.ttf");
// const OPEN_SANS_DATA: &[u8] = include_bytes!("../../res/fonts/Open_Sans/OpenSans-VariableFont.ttf");

pub struct FontManager {
    gcx: GlyphContext,
    fonts: HashMap<String, Font>,
}

impl std::fmt::Debug for FontManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FontManager")
            .field("fonts", &self.fonts)
            .finish()
    }
}

#[derive(Clone, Copy)]
pub enum TextAlign {
    Left,
    Right,

    Top,
    Bottom,

    Center,
}

impl FontManager {
    pub fn new() -> Self {
        let mut fonts = HashMap::new();
        fonts.insert(
            "opensans".to_string(),
            Font::new(Blob::new(Arc::new(OPEN_SANS_DATA)), 0),
        );

        Self {
            gcx: GlyphContext::new(),
            fonts,
        }
    }

    // pub fn layout(&mut self, font: Option<&FontRef>, size: f32, text: &str, bounds: &Rect) -> Rect {
    //     let font = font.unwrap_or(&FontRef {
    //         data: FONT_DATA,
    //         offset: 0,
    //     });

    //     if let Ok(cmap) = font.cmap() {
    //         if let Ok(hmtx) = font.hmtx() {
    //             let upem = font.head().map(|head| head.units_per_em()).unwrap_or(1000) as f64;
    //             let scale = size as f64 / upem;
    //             let hmetrics = hmtx.h_metrics();

    //             let height = if let Ok(h) = font.hhea() {
    //                 h.ascender() as f64 * scale - h.descender() as f64 * scale
    //                     + h.line_gap() as f64 * scale
    //             } else {
    //                 size as f64
    //             }
    //             .ceil();

    //             let default_hadvance = hmetrics
    //                 .get(hmetrics.len().saturating_sub(1))
    //                 .map(|h| h.advance_width)
    //                 .unwrap_or(0);

    //             let mut words: Vec<_> = text
    //                 .split(' ')
    //                 .map(|f| {
    //                     f.chars().chain([' '].into_iter()).fold(0.0, |acc, b| {
    //                         acc + hmetrics
    //                             .get(cmap.map(b as u32).unwrap_or(0) as usize)
    //                             .map(|h| h.advance_width)
    //                             .unwrap_or(default_hadvance)
    //                             as f64
    //                             * scale
    //                     })
    //                 })
    //                 .chain([0.0].into_iter())
    //                 .collect();

    //             let mut pen_x = 0f64;
    //             let mut max_x = 0f64;
    //             let mut pen_y = 0f64;
    //             let mut word_index = 0;
    //             let mut overflow = false;

    //             for (ch, nxt) in text.chars().zip(text.chars()) {
    //                 let gid = cmap.map(ch as u32).unwrap_or(0);
    //                 let advance = hmetrics
    //                     .get(gid as usize)
    //                     .map(|h| h.advance_width)
    //                     .unwrap_or(default_hadvance) as f64
    //                     * scale;

    //                 // If overflow, go to next line
    //                 if pen_x + words[word_index + 1] > bounds.width() && ch == ' ' {
    //                     // if pen_x + advance > bounds.width() {
    //                     pen_x = 0.0;
    //                     pen_y += height;
    //                     overflow = true;
    //                 }

    //                 if ch == ' ' {
    //                     word_index += 1;
    //                 }

    //                 // If newline starts with space, don't add it
    //                 if ch == ' ' && pen_y > 0.0 && pen_x < 0.1 {
    //                     continue;
    //                 }

    //                 pen_x += advance.ceil();

    //                 if pen_x > max_x {
    //                     max_x = pen_x
    //                 }
    //             }

    //             if max_x > bounds.width() || overflow {
    //                 max_x = bounds.width();
    //             }
    //             return Rect::new(0.0, 0.0, max_x, pen_y + height);
    //         }
    //     }
    //     Rect::ZERO
    // }

    // pub fn get_adg(&mut self, font: Option<&FontRef>, size: f32) -> (f64, f64, f64) {
    //     let font = font.unwrap_or(&FontRef {
    //         data: FONT_DATA,
    //         offset: 0,
    //     });

    //     let upem = font.head().map(|head| head.units_per_em()).unwrap_or(1000) as f64;
    //     let scale = size as f64 / upem;

    //     if let Some(h) = font.os2() {
    //         (
    //             h.typographic_ascender() as f64 * scale,
    //             -h.typographic_descender() as f64 * scale,
    //             h.typographic_line_gap() as f64 * scale,
    //         )
    //     } else {
    //         (0.0, 0.0, 0.0)
    //     }
    // }

    // pub fn add(
    //     &mut self,
    //     builder: &mut SceneBuilder,
    //     _font: Option<&FontRef>,
    //     size: f32,
    //     brush: Option<&Brush>,
    //     transform: Affine,
    //     text: &str,
    //     bounds: &Rect,
    // ) {
    //     let font = _font.unwrap_or(&FontRef {
    //         data: FONT_DATA,
    //         offset: 0,
    //     });

    //     if let Some(cmap) = font.cmap() {
    //         if let Some(hmtx) = font.hmtx() {
    //             let upem = font.head().map(|head| head.units_per_em()).unwrap_or(1000) as f64;
    //             let scale = size as f64 / upem;

    //             let vars: [(pinot::types::Tag, f32); 0] = [];
    //             let mut provider = self.gcx.new_provider(font, None, size, false, vars);
    //             let hmetrics = hmtx.hmetrics();
    //             let default_advance = hmetrics
    //                 .get(hmetrics.len().saturating_sub(1))
    //                 .map(|h| h.advance_width)
    //                 .unwrap_or(0);

    //             let mut pen_x = 0.0f64;
    //             let mut pen_y = 0f64;

    //             let mut word_index = 0;
    //             // for text in words {
    //             //     println!("{}", text);
    //             // }

    //             let mut words: Vec<_> = text
    //                 .split(' ')
    //                 .map(|f| {
    //                     f.chars().chain([' '].into_iter()).fold(0.0, |acc, b| {
    //                         acc + hmetrics
    //                             .get(cmap.map(b as u32).unwrap_or(0) as usize)
    //                             .map(|h| h.advance_width)
    //                             .unwrap_or(default_advance) as f64
    //                             * scale
    //                     })
    //                 })
    //                 .chain([0.0].into_iter())
    //                 .collect();

    //             for ch in text.chars() {
    //                 let gid = cmap.map(ch as u32).unwrap_or(0);
    //                 let advance = hmetrics
    //                     .get(gid as usize)
    //                     .map(|h| h.advance_width)
    //                     .unwrap_or(default_advance) as f64
    //                     * scale;

    //                 if let Some(glyph) = provider.get(gid, brush) {
    //                     if pen_x + words[word_index + 1] > bounds.width() && ch == ' ' {
    //                         if let Some(vmtx) = font.hhea() {
    //                             let height = (vmtx.ascender() as f64 * scale
    //                                 - vmtx.descender() as f64 * scale
    //                                 + vmtx.line_gap() as f64);

    //                             pen_x = 0.0;
    //                             pen_y += height;
    //                         }
    //                     }

    //                     if ch == ' ' {
    //                         word_index += 1;
    //                     }
    //                     // Skip space on start of newline
    //                     if ch == ' ' && pen_y > 0.0 && pen_x < 0.1 {
    //                         continue;
    //                     }

    //                     let xform = transform
    //                         * Affine::translate((
    //                             pen_x,
    //                             (font.hhea().unwrap().ascender() as f64 * scale + pen_y).ceil(),
    //                         ))
    //                         * Affine::scale_non_uniform(1.0, -1.0);
    //                     builder.append(&glyph, Some(xform));
    //                 }

    //                 pen_x += advance.ceil();
    //             }
    //         }
    //     }
    // }

    pub fn add_run<'a>(
        &mut self,
        builder: &mut SceneBuilder,
        font: Option<&Font>,
        size: f32,
        brush: impl Into<BrushRef<'a>>,
        transform: Affine,
        glyph_transform: Option<Affine>,
        style: impl Into<StyleRef<'a>>,
        text: &str,
    ) {
        self.add_var_run(
            builder,
            font,
            size,
            &[],
            brush,
            transform,
            glyph_transform,
            style,
            text,
        );
    }

    pub fn add_var_run<'a>(
        &mut self,
        builder: &mut SceneBuilder,
        font: Option<&Font>,
        size: f32,
        variations: &[(&str, f32)],
        brush: impl Into<BrushRef<'a>>,
        transform: Affine,
        glyph_transform: Option<Affine>,
        style: impl Into<StyleRef<'a>>,
        text: &str,
    ) {
        let default_font = self.fonts.get("opensans").unwrap();
        // let default_font = if variations.is_empty() {
        //     &self.roboto
        // } else {
        //     &self.inconsolata
        // };
        let font = font.unwrap_or(default_font);
        let font_ref = to_font_ref(font).unwrap();
        let brush = brush.into();
        let style = style.into();
        let axes = font_ref.axes();
        let fello_size = vello::fello::Size::new(size);
        let coords = axes
            .normalize(variations.iter().copied())
            .collect::<Vec<_>>();
        let charmap = font_ref.charmap();
        let metrics = font_ref.metrics(fello_size, coords.as_slice().into());
        let line_height = metrics.ascent - metrics.descent + metrics.leading;
        let glyph_metrics = font_ref.glyph_metrics(fello_size, coords.as_slice().into());
        let mut pen_x = 0f32;
        let mut pen_y = 0f32;
        builder
            .draw_glyphs(font)
            .font_size(size)
            .transform(transform)
            .glyph_transform(glyph_transform)
            .normalized_coords(&coords)
            .brush(brush)
            .draw(
                style,
                text.chars().filter_map(|ch| {
                    if ch == '\n' {
                        pen_y += line_height;
                        pen_x = 0.0;
                        return None;
                    }
                    let gid = charmap.map(ch).unwrap_or_default();
                    let advance = glyph_metrics.advance_width(gid).unwrap_or_default();
                    let x = pen_x;
                    pen_x += advance;
                    Some(Glyph {
                        id: gid.to_u16() as u32,
                        x,
                        y: pen_y,
                    })
                }),
            );
    }

    pub fn layout(
        &self,
        font: Option<&Font>,
        size: f32,
        scale: f32,
        // transform: Affine,
        bounds: Rect,
        text: &str,
    ) -> Rect {
        let default_font = self.fonts.get("opensans").unwrap();
        // let default_font = FontRef::new(ROBOTO_FONT).unwrap();
        let font = font
            .and_then(to_font_ref)
            .unwrap_or(to_font_ref(default_font).unwrap());

        let fello_size = vello::fello::Size::new(size * scale);
        let charmap = font.charmap();
        let metrics = font.metrics(fello_size, Default::default());
        let line_height = metrics.ascent - metrics.descent + metrics.leading;
        let glyph_metrics = font.glyph_metrics(fello_size, Default::default());

        let words: Vec<_> = text
            .split(' ')
            .map(|f| {
                f.chars().chain([' '].into_iter()).fold(0.0, |acc, b| {
                    acc + glyph_metrics
                        .advance_width(charmap.map(b as u32).unwrap())
                        .unwrap() as f64
                })
            })
            .chain([0.0].into_iter())
            .collect();

        let mut pen_x = 0f64;
        let mut pen_y = 0f64;
        let mut max_x = 0f64;
        let mut word_index = 0;
        let mut overflow = false;

        // let vars: [(&str, f32); 0] = [];
        // let mut provider = self.gcx.new_provider(&font, None, size, false, vars);

        for ch in text.chars() {
            if ch == '\n' {
                pen_y += line_height as f64;
                pen_x = 0.0;
                continue;
            }

            let gid = charmap.map(ch).unwrap_or_default();
            let advance = glyph_metrics.advance_width(gid).unwrap_or_default() as f64;

            if pen_x + words[word_index + 1] > bounds.width() && ch == ' ' {
                // if pen_x + advance > bounds.width() {
                pen_x = 0.0;
                pen_y += line_height as f64;
                overflow = true;
            }

            if ch == ' ' {
                word_index += 1;
            }

            // If newline starts with space, don't add it
            if ch == ' ' && pen_y > 0.0 && pen_x < 0.1 {
                continue;
            }

            pen_x += advance;

            if pen_x > max_x {
                max_x = pen_x
            }
        }

        if max_x > bounds.width() || overflow {
            max_x = bounds.width();
        }

        Rect::from_origin_size(bounds.origin(), (max_x.ceil(), (pen_y + line_height as f64).ceil()))
    }

    pub fn add(
        &mut self,
        builder: &mut SceneBuilder,
        font: Option<&Font>,
        size: f32,
        scale: f32,
        brush: Option<&Brush>,
        transform: Affine,
        text: &str,
        bounds: Rect,
    ) {
        let default_font = self.fonts.get("opensans").unwrap();
        // let default_font = FontRef::new(ROBOTO_FONT).unwrap();
        let font = font
            .and_then(to_font_ref)
            .unwrap_or(to_font_ref(default_font).unwrap());

        let fello_size = vello::fello::Size::new(size * scale);
        let charmap = font.charmap();
        let metrics = font.metrics(fello_size, Default::default());
        let line_height = metrics.ascent - metrics.descent + metrics.leading;
        let glyph_metrics = font.glyph_metrics(fello_size, Default::default());

        let words: Vec<_> = text
            .split(' ')
            .map(|f| {
                f.chars().chain([' '].into_iter()).fold(0.0, |acc, b| {
                    acc + glyph_metrics
                        .advance_width(charmap.map(b as u32).unwrap())
                        .unwrap() as f64
                })
            })
            .chain([0.0].into_iter())
            .collect();

        let mut pen_x = 0f64;
        let mut pen_y = 0f64;
        let mut word_index = 0;

        let vars: [(&str, f32); 0] = [];
        let mut provider = self
            .gcx
            .new_provider(&font, None, size * scale, false, vars);

        for ch in text.chars() {
            if ch == '\n' {
                pen_y += line_height as f64;
                pen_x = 0.0;
                continue;
            }

            let gid = charmap.map(ch).unwrap_or_default();
            let advance = glyph_metrics.advance_width(gid).unwrap_or_default() as f64;

            if pen_x + words[word_index + 1] > bounds.width() && ch == ' ' {
                pen_x = 0.0;
                pen_y += line_height as f64;
            }

            if ch == ' ' {
                word_index += 1;
            }
            // Skip space on start of newline
            if ch == ' ' && pen_y > 0.0 && pen_x < 0.1 {
                continue;
            }

            if let Some(glyph) = provider.get(gid.to_u16(), brush) {
                let xform = transform
                    * Affine::translate((bounds.x0, bounds.y0 + metrics.ascent as f64))
                    * Affine::translate((pen_x, pen_y))
                    * Affine::scale_non_uniform(1.0, -1.0);
              
                builder.append(&glyph, Some(xform));
            }

            pen_x += advance;
        }
    }
}

fn to_font_ref(font: &Font) -> Option<FontRef<'_>> {
    use vello::fello::raw::FileRef;
    let file_ref = FileRef::new(font.data.as_ref()).ok()?;
    match file_ref {
        FileRef::Font(font) => Some(font),
        FileRef::Collection(collection) => collection.get(font.index).ok(),
    }
}

pub fn xy_from_align(
    ascent: f64,
    descent: f64,
    size: Rect,
    vertical_align: TextAlign,
    horizontal_algin: TextAlign,
) -> (f64, f64) {
    let (mut x, mut y) = (0.0, 0.0);

    match vertical_align {
        // TextAlign::Top => y += size.max_y(),
        TextAlign::Center => y += (ascent + descent) / 2.0,
        _ => (),
    }

    match horizontal_algin {
        TextAlign::Right => x += -size.width(),
        TextAlign::Center => x += -size.width() / 2.0,
        _ => (),
    }

    (x, y)
}

pub fn transform_from_align(
    ascent: f64,
    descent: f64,
    size: Rect,
    vertical_align: TextAlign,
    horizontal_algin: TextAlign,
) -> Affine {
    // let mut transform = Affine::IDENTITY;
    let (x, y) = xy_from_align(ascent, descent, size, vertical_align, horizontal_algin);

    Affine::translate((x, y))
}
