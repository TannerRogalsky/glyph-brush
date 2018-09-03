use super::*;

/// Builder for a [`GlyphBrush`](struct.GlyphBrush.html).
///
/// # Example
///
/// ```no_run
/// # extern crate gfx;
/// # extern crate gfx_window_glutin;
/// # extern crate glutin;
/// extern crate gfx_glyph;
/// use gfx_glyph::GlyphBrushBuilder;
/// # fn main() {
/// # let events_loop = glutin::EventsLoop::new();
/// # let (_window, _device, gfx_factory, _gfx_target, _main_depth) =
/// #     gfx_window_glutin::init::<gfx::format::Srgba8, gfx::format::Depth>(
/// #         glutin::WindowBuilder::new(),
/// #         glutin::ContextBuilder::new(),
/// #         &events_loop);
///
/// let dejavu: &[u8] = include_bytes!("../examples/DejaVuSans.ttf");
/// let mut glyph_brush = GlyphBrushBuilder::using_font_bytes(dejavu).build(gfx_factory.clone());
/// # let _ = glyph_brush;
/// # }
/// ```
pub struct GlyphBrushBuilder<'a, H = DefaultSectionHasher> {
    inner: glyph_brush::GlyphBrushBuilder<'a, H>,
    depth_test: gfx::state::Depth,
    texture_filter_method: texture::FilterMethod,
}

impl<'a> GlyphBrushBuilder<'a> {
    /// Specifies the default font data used to render glyphs.
    /// Referenced with `FontId(0)`, which is default.
    pub fn using_font_bytes<B: Into<SharedBytes<'a>>>(font_0_data: B) -> Self {
        Self::using_font(Font::from_bytes(font_0_data).unwrap())
    }

    pub fn using_fonts_bytes<B, V>(font_data: V) -> Self
    where
        B: Into<SharedBytes<'a>>,
        V: Into<Vec<B>>,
    {
        Self::using_fonts(
            font_data
                .into()
                .into_iter()
                .map(|data| Font::from_bytes(data).unwrap())
                .collect::<Vec<_>>(),
        )
    }

    /// Specifies the default font used to render glyphs.
    /// Referenced with `FontId(0)`, which is default.
    pub fn using_font(font_0: Font<'a>) -> Self {
        Self::using_fonts(vec![font_0])
    }

    pub fn using_fonts<V: Into<Vec<Font<'a>>>>(fonts: V) -> Self {
        GlyphBrushBuilder {
            inner: glyph_brush::GlyphBrushBuilder::using_fonts(fonts),
            depth_test: gfx::preset::depth::PASS_TEST,
            texture_filter_method: texture::FilterMethod::Bilinear,
        }
    }
}

impl<'a, H: BuildHasher> GlyphBrushBuilder<'a, H> {
    /// Adds additional fonts to the one added in [`using_font`](#method.using_font) /
    /// [`using_font_bytes`](#method.using_font_bytes).
    /// Returns a [`FontId`](struct.FontId.html) to reference this font.
    pub fn add_font_bytes<B: Into<SharedBytes<'a>>>(&mut self, font_data: B) -> FontId {
        self.inner.add_font_bytes(font_data)
    }

    /// Adds additional fonts to the one added in [`using_font`](#method.using_font) /
    /// [`using_font_bytes`](#method.using_font_bytes).
    /// Returns a [`FontId`](struct.FontId.html) to reference this font.
    pub fn add_font(&mut self, font_data: Font<'a>) -> FontId {
        self.inner.add_font(font_data)
    }

    /// Initial size of 2D texture used as a gpu cache, pixels (width, height).
    /// The GPU cache will dynamically quadruple in size whenever the current size
    /// is insufficient.
    ///
    /// Defaults to `(256, 256)`
    pub fn initial_cache_size(mut self, size: (u32, u32)) -> Self {
        self.inner = self.inner.initial_cache_size(size);
        self
    }

    /// Sets the maximum allowed difference in scale used for judging whether to reuse an
    /// existing glyph in the GPU cache.
    ///
    /// Defaults to `0.5`
    ///
    /// See rusttype docs for `rusttype::gpu_cache::Cache`
    pub fn gpu_cache_scale_tolerance(mut self, tolerance: f32) -> Self {
        self.inner = self.inner.gpu_cache_scale_tolerance(tolerance);
        self
    }

    /// Sets the maximum allowed difference in subpixel position used for judging whether
    /// to reuse an existing glyph in the GPU cache. Anything greater than or equal to
    /// 1.0 means "don't care".
    ///
    /// Defaults to `0.1`
    ///
    /// See rusttype docs for `rusttype::gpu_cache::Cache`
    pub fn gpu_cache_position_tolerance(mut self, tolerance: f32) -> Self {
        self.inner = self.inner.gpu_cache_position_tolerance(tolerance);
        self
    }

    /// Sets whether perform the calculation of glyph positioning according to the layout
    /// every time, or use a cached result if the input `Section` and `GlyphPositioner` are the
    /// same hash as a previous call.
    ///
    /// Improves performance. Should only disable if using a custom GlyphPositioner that is
    /// impure according to it's inputs, so caching a previous call is not desired. Disabling
    /// also disables [`cache_glyph_drawing`](#method.cache_glyph_drawing).
    ///
    /// Defaults to `true`
    pub fn cache_glyph_positioning(mut self, cache: bool) -> Self {
        self.inner = self.inner.cache_glyph_positioning(cache);
        self
    }

    /// Sets optimising drawing by reusing the last draw requesting an identical draw queue.
    ///
    /// Improves performance. Is disabled if
    /// [`cache_glyph_positioning`](#method.cache_glyph_positioning) is disabled.
    ///
    /// Defaults to `true`
    pub fn cache_glyph_drawing(mut self, cache: bool) -> Self {
        self.inner = self.inner.cache_glyph_drawing(cache);
        self
    }

    /// Sets the depth test to use on the text section **z** values.
    ///
    /// Defaults to: *Always pass the depth test, never write to the depth buffer write*
    ///
    /// # Example
    ///
    /// ```no_run
    /// # extern crate gfx;
    /// # extern crate gfx_glyph;
    /// # use gfx_glyph::GlyphBrushBuilder;
    /// # fn main() {
    /// # let some_font: &[u8] = include_bytes!("../examples/DejaVuSans.ttf");
    /// GlyphBrushBuilder::using_font_bytes(some_font).depth_test(gfx::preset::depth::LESS_EQUAL_WRITE)
    /// // ...
    /// # ;
    /// # }
    /// ```
    pub fn depth_test(mut self, depth_test: gfx::state::Depth) -> Self {
        self.depth_test = depth_test;
        self
    }

    /// Sets the texture filtering method.
    ///
    /// Defaults to `Bilinear`
    ///
    /// # Example
    /// ```no_run
    /// # extern crate gfx;
    /// # extern crate gfx_glyph;
    /// # use gfx_glyph::GlyphBrushBuilder;
    /// # fn main() {
    /// # let some_font: &[u8] = include_bytes!("../examples/DejaVuSans.ttf");
    /// GlyphBrushBuilder::using_font_bytes(some_font)
    ///     .texture_filter_method(gfx::texture::FilterMethod::Scale)
    /// // ...
    /// # ;
    /// # }
    /// ```
    pub fn texture_filter_method(mut self, filter_method: texture::FilterMethod) -> Self {
        self.texture_filter_method = filter_method;
        self
    }

    /// Sets the section hasher. `GlyphBrush` cannot handle absolute section hash collisions
    /// so use a good hash algorithm.
    ///
    /// This hasher is used to distinguish sections, rather than for hashmap internal use.
    ///
    /// Defaults to [seahash](https://docs.rs/seahash).
    ///
    /// # Example
    /// ```no_run
    /// # extern crate gfx;
    /// # extern crate gfx_glyph;
    /// # extern crate seahash;
    /// # use gfx_glyph::GlyphBrushBuilder;
    /// # fn main() {
    /// # let some_font: &[u8] = include_bytes!("../examples/DejaVuSans.ttf");
    /// # type SomeOtherBuildHasher = ::std::hash::BuildHasherDefault<seahash::SeaHasher>;
    /// GlyphBrushBuilder::using_font_bytes(some_font).section_hasher(SomeOtherBuildHasher::default())
    /// // ...
    /// # ;
    /// # }
    /// ```
    pub fn section_hasher<T: BuildHasher>(self, section_hasher: T) -> GlyphBrushBuilder<'a, T> {
        GlyphBrushBuilder {
            inner: self.inner.section_hasher(section_hasher),
            depth_test: self.depth_test,
            texture_filter_method: self.texture_filter_method,
        }
    }

    /// Builds a `GlyphBrush` using the input gfx factory
    pub fn build<R, F>(self, mut factory: F) -> GlyphBrush<'a, R, F, H>
    where
        R: gfx::Resources,
        F: gfx::Factory<R>,
    {
        let (cache_width, cache_height) = self.inner.initial_cache_size;
        let font_cache_tex = create_texture(&mut factory, cache_width, cache_height).unwrap();
        let program = factory
            .link_program(
                include_bytes!("shader/vert.glsl"),
                include_bytes!("shader/frag.glsl"),
            ).unwrap();

        GlyphBrush {
            font_cache_tex,
            texture_filter_method: self.texture_filter_method,
            glyph_brush: self.inner.build(),

            factory,
            program,
            draw_cache: None,

            depth_test: self.depth_test,
        }
    }
}
