
use sdl2::render::WindowCanvas;

pub mod sdlvalues;
pub mod texture;
pub mod textrenderer;
pub mod textcachepool;

pub use self::sdlvalues::SdlValues;
pub use self::texture::IconIdx;
pub use self::textrenderer::FontKind;
pub use self::textcachepool::TextCache;

use sdl2::rect::Rect;
use sdl2::render::Texture;
use common::objholder::Holder;

/// Wrapper for SDL drawing functions
pub struct Context<'a, 'b, 't, 'sdl> {
    pub canvas: &'a mut WindowCanvas,
    pub sv: &'b mut SdlValues<'t, 'sdl>,
}

impl<'a, 'b, 't, 'sdl> Context<'a, 'b, 't, 'sdl> {
    pub fn new(canvas: &'a mut WindowCanvas, sv: &'b mut SdlValues<'t, 'sdl>) -> Context<'a, 'b, 't, 'sdl> {
        Context {
            canvas, sv
        }
    }

    pub fn set_viewport<R: Into<Option<Rect>>>(&mut self, rect: R) {
        self.canvas.set_viewport(rect);
    }

    pub fn render_tex<I>(&mut self, idx: I, dest: Rect)
    where
        for<'th> self::texture::TextureHolder<'th>: common::objholder::Holder<I, ReturnType=Texture<'th>>
    {
        let tex = self.sv.tex().get(idx);
        check_draw!(self.canvas.copy(tex, None, dest));
    }

    pub fn render_tex_n<I, O>(&mut self, idx: I, dest: Rect, n_image: u32)
    where
        for<'th> self::texture::TextureHolder<'th>: common::objholder::Holder<I, ReturnType=Texture<'th>>,
        I: common::objholder::ObjectIndex<ObjectType = O> + Copy,
        O: common::obj::ImgObject + 'static
    {
        let tex = self.sv.tex().get(idx);
        let obj = common::gobj::get_obj(idx);
        let src: Rect = obj.img_rect_nth(n_image).into();
        check_draw!(self.canvas.copy(tex, src, dest));
    }

    pub fn render_tex_n_center<I, O>(&mut self, idx: I, dest: Rect, n_image: u32)
    where
        for<'th> self::texture::TextureHolder<'th>: common::objholder::Holder<I, ReturnType=Texture<'th>>,
        I: common::objholder::ObjectIndex<ObjectType = O> + Copy,
        O: common::obj::ImgObject + 'static
    {
        let tex = self.sv.tex().get(idx);
        let obj = common::gobj::get_obj(idx);
        let src: Rect = obj.img_rect_nth(n_image).into();
        let dest = Rect::new(
            dest.x + (dest.w - src.w) / 2,
            dest.y + (dest.h - src.h) / 2,
            src.w as u32, src.h as u32);
        check_draw!(self.canvas.copy(tex, src, dest));
    }

    pub fn render_tex_n_bottom<I, O>(&mut self, idx: I, dest: Rect, n_image: u32)
    where
        for<'th> self::texture::TextureHolder<'th>: common::objholder::Holder<I, ReturnType=Texture<'th>>,
        I: common::objholder::ObjectIndex<ObjectType = O> + Copy,
        O: common::obj::ImgObject + 'static
    {
        let tex = self.sv.tex().get(idx);
        let obj = common::gobj::get_obj(idx);
        let src: Rect = obj.img_rect_nth(n_image).into();
        let dest = Rect::new(
            dest.x + (dest.w - src.w) / 2,
            dest.y + dest.h - src.h,
            src.w as u32, src.h as u32);
        check_draw!(self.canvas.copy(tex, src, dest));
    }
}

